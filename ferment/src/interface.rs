use syn::{Field, parse_quote, Path, PathArguments, Type, TypeArray, TypePath, TypePtr, TypeReference};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::FieldType;

use crate::generic_path_conversion::GenericPathConversion;
use crate::path_conversion::PathConversion;
use crate::helper::{ffi_mangled_ident, path_arguments_to_types};
use crate::item_conversion::ItemContext;
use crate::type_conversion::TypeConversion;

/// token -> token
pub type MapPresenter = fn(field_name: TokenStream2) -> TokenStream2;
/// token + token -> token
pub type MapPairPresenter = fn(field_name: TokenStream2, conversion: TokenStream2) -> TokenStream2;

/// field + dictionary -> token
pub type ScopeTreeFieldPresenter = fn(field: &Field, context: &ItemContext) -> TokenStream2;

/// token + type + dictionary -> token
pub type ScopeTreeFieldTypedPresenter = fn(field_type: &FieldType, context: &ItemContext) -> TokenStream2;
/// [token] -> token
pub type IteratorPresenter = fn(items: Vec<TokenStream2>) -> TokenStream2;

pub type ScopeTreeItemTypePresenter = fn(field_type: &Type, context: &ItemContext) -> TokenStream2;
/// token + [token] -> token
pub type OwnerIteratorPresenter = fn((TokenStream2, Vec<TokenStream2>)) -> TokenStream2;

pub type ScopeTreePathPresenter = fn(path: &Path, context: &ItemContext) -> TokenStream2;
pub type ScopeTreePathArgumentsPresenter = fn(arguments: &PathArguments, context: &ItemContext) -> TokenStream2;

pub type GenericPathPresenter = fn(path: &Path, arguments_presenter: ScopeTreePathArgumentsPresenter, context: &ItemContext) -> TokenStream2;


/// Field Presenters
pub const UNNAMED_VARIANT_FIELD_PRESENTER: ScopeTreeFieldPresenter = |Field { ty, .. }, context| {
    let full_ty = context.ffi_full_type_for(ty);
    FFI_DICTIONARY_TYPE_PRESENTER(&full_ty, context)
};
pub const NAMED_VARIANT_FIELD_PRESENTER :ScopeTreeFieldPresenter = |Field { ident, ty: field_type, .. }, context| {
    let full_ty = context.ffi_full_type_for(field_type);
    NAMED_CONVERSION_PRESENTER(ident.clone().unwrap().to_token_stream(), FFI_DICTIONARY_TYPE_PRESENTER(&full_ty, context))
};


/// Type Presenters
pub const FFI_DICTIONARY_TYPE_PRESENTER: ScopeTreeItemTypePresenter = |field_type, context| {
    match field_type {
        Type::Path(TypePath { path, .. }) =>
            (match path.segments.last().unwrap().ident.to_string().as_str() {
                "Vec" | "BTreeMap" | "HashMap" | "Result" => FFI_GENERIC_TYPE_PRESENTER,
                "Option" => OPTION_PATH_PRESENTER,
                "OpaqueContext" => OPAQUE_CONTEXT_PATH_PRESENTER,
                "OpaqueContextMut" => OPAQUE_CONTEXT_MUT_PATH_PRESENTER,
                _ => DEFAULT_DICT_PATH_PRESENTER,
            })(path, context),
        Type::Array(TypeArray { elem, len, .. }) =>
            quote!(*mut [#elem; #len]),
        Type::Reference(TypeReference { elem, .. }) =>
            FFI_DICTIONARY_TYPE_PRESENTER(elem, context),
        Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
            match &**elem {
                Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                    "c_void" => match (star_token, const_token, mutability) {
                        (_, Some(_const_token), None) => quote!(OpaqueContext_FFI),
                        (_, None, Some(_mut_token)) => quote!(OpaqueContextMut_FFI),
                        _ => panic!("extract_struct_field: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                    },
                    _ => quote!(*mut #path)
                },
                Type::Ptr(type_ptr) => quote!(*mut #type_ptr),
                _ => panic!("extract_struct_field: {} not supported", quote!(#elem))
            }
        _ => panic!("FFI_DICTIONARY_TYPE_PRESENTER: type not supported: {}", quote!(#field_type))
    }
};

/// Map Presenters
pub const EMPTY_MAP_PRESENTER: MapPresenter = |_| quote!();
pub const FFI_DEREF_FIELD_NAME: MapPresenter = |field_name| quote!(ffi_ref.#field_name);
pub const DEREF_FIELD_PATH: MapPresenter = |field_path| quote!(*#field_path);

pub const FROM_OFFSET_MAP_PRESENTER: MapPresenter = |field_path| quote!(#field_path.add(i));

pub const OBJ_FIELD_NAME: MapPresenter = |field_name| quote!(obj.#field_name);
pub const SIMPLE_PRESENTER: MapPresenter = |name| quote!(#name);
pub const SIMPLE_TERMINATED_PRESENTER: MapPresenter = |name| quote!(#name;);
pub const ROOT_DESTROY_CONTEXT_PRESENTER: MapPresenter = |_| package_unboxed_root();
pub const EMPTY_DESTROY_PRESENTER: MapPresenter = |_| quote!({});
pub const DEFAULT_DOC_PRESENTER: MapPresenter = |target_name: TokenStream2| {
    let comment = format!("FFI-representation of the {}", target_name);
    parse_quote! { #[doc = #comment] }
};


/// Map Pair Presenters
pub const EMPTY_PAIR_PRESENTER: MapPairPresenter = |_, _|
    quote!();
pub const SIMPLE_PAIR_PRESENTER: MapPairPresenter = |name, presentation|
    quote!(#name #presentation);
pub const SIMPLE_CONVERSION_PRESENTER: MapPairPresenter = |_, conversion|
    quote!(#conversion);
pub const NAMED_CONVERSION_PRESENTER: MapPairPresenter = |l_value, r_value|
    quote!(#l_value: #r_value);
pub const PUB_NAMED_CONVERSION_PRESENTER: MapPairPresenter = |l_value, r_value|
    quote!(pub #l_value: #r_value);
pub const LAMBDA_CONVERSION_PRESENTER: MapPairPresenter = |l_value, r_value|
    quote!(#l_value => #r_value);
pub const FFI_FROM_ROOT_PRESENTER: MapPairPresenter = |field_path: TokenStream2, conversions: TokenStream2|
    quote!(let ffi_ref = #field_path; #conversions);
pub const FFI_TO_ROOT_PRESENTER: MapPairPresenter = |_, conversions: TokenStream2|
    package_boxed_expression(conversions);

/// Field Type Presenters

pub const EMPTY_DICT_FIELD_TYPED_PRESENTER: ScopeTreeFieldTypedPresenter = |_, _|
    quote!();
pub const DEFAULT_DICT_FIELD_PRESENTER: ScopeTreeFieldTypedPresenter = |field_type, _|
    field_type.name();
pub const DEFAULT_DICT_FIELD_TYPE_PRESENTER: ScopeTreeFieldTypedPresenter = |field_type, context| {
    FFI_DICTIONARY_TYPE_PRESENTER(&context.ffi_full_type_for(field_type.ty()), context)
};
pub const NAMED_DICT_FIELD_TYPE_PRESENTER: ScopeTreeFieldTypedPresenter = |field_type, context| {
    let ffi_type = context.ffi_full_type_for(field_type.ty());
    PUB_NAMED_CONVERSION_PRESENTER(field_type.name(), FFI_DICTIONARY_TYPE_PRESENTER(&ffi_type, context))
};



/// Iterator Presenters
pub const EMPTY_ITERATOR_PRESENTER: IteratorPresenter = |_|
    quote!();
pub const DEFAULT_DESTROY_FIELDS_PRESENTER: IteratorPresenter = |destructors|
    quote!({#(#destructors)*});
pub const CURLY_ITER_PRESENTER: IteratorPresenter = |fields: Vec<TokenStream2>|
    quote!({ #(#fields,)* });
pub const ROUND_ITER_PRESENTER: IteratorPresenter = |fields: Vec<TokenStream2>|
    quote!(( #(#fields,)* ));
pub const STRUCT_DESTROY_PRESENTER: IteratorPresenter = |fields| match fields.len() {
    0 => quote!(),
    _ => quote!(let ffi_ref = self; #(#fields;)*)
};

pub const ENUM_DESTROY_PRESENTER: IteratorPresenter = |fields| match fields.len() {
    0 => quote!(),
    _ => MATCH_FIELDS_PRESENTER((quote!(self), fields))
};

/// Owner Iterator Presenters
pub const EMPTY_FIELDS_PRESENTER: OwnerIteratorPresenter = |_|
    quote!();
pub const CURLY_BRACES_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    SIMPLE_PAIR_PRESENTER(name, CURLY_ITER_PRESENTER(fields));
pub const ROUND_BRACES_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    SIMPLE_PAIR_PRESENTER(name, ROUND_ITER_PRESENTER(fields));
pub const MATCH_FIELDS_PRESENTER: OwnerIteratorPresenter = |(field_path, fields)|
    SIMPLE_PAIR_PRESENTER(quote!(match #field_path), CURLY_ITER_PRESENTER(fields));
pub const NO_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, _)|
    quote!(#name);
pub const ENUM_UNIT_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    quote!(#name = #(#fields)*);

pub const TYPE_ALIAS_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    create_struct(name, SIMPLE_TERMINATED_PRESENTER(ROUND_ITER_PRESENTER(fields)));
pub const TYPE_ALIAS_CONVERSION_FROM_PRESENTER: OwnerIteratorPresenter = |(_, fields)|
    quote!(#(#fields)*);
pub const TYPE_ALIAS_CONVERSION_TO_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    quote!(#name(#(#fields),*));

pub const UNNAMED_STRUCT_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    create_struct(name, SIMPLE_TERMINATED_PRESENTER(ROUND_ITER_PRESENTER(fields)));
pub const NAMED_STRUCT_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    create_struct(name, CURLY_ITER_PRESENTER(fields));
pub const ENUM_NAMED_VARIANT_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    SIMPLE_PAIR_PRESENTER(name, CURLY_ITER_PRESENTER(fields));
pub const ENUM_UNNAMED_VARIANT_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    SIMPLE_PAIR_PRESENTER(name, ROUND_ITER_PRESENTER(fields));
pub const ENUM_PRESENTER: OwnerIteratorPresenter = |(name, fields)| {
    let enum_presentation = CURLY_BRACES_FIELDS_PRESENTER((name, fields));
    quote! {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum #enum_presentation
    }
};

/// PathArguments Presenters
pub const OPAQUE_CONTEXT_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |_, _|
    quote!(ferment_interfaces::OpaqueContext_FFI);
pub const OPAQUE_CONTEXT_MUT_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |_, _|
    quote!(ferment_interfaces::OpaqueContextMut_FFI);

pub const OPTION_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |arguments, tree|
    match path_arguments_to_types(arguments)[..] {
        [field_type] => FFI_DICTIONARY_TYPE_PRESENTER(field_type, tree),
        _ => panic!("OPTION_ARGUMENTS_PRESENTER: arguments: {} not supported", quote!(#arguments))
};

pub const GENERIC_PATH_PRESENTER: GenericPathPresenter = |path, arguments_presenter, dictionary|
    arguments_presenter(&path.segments.last().unwrap().arguments, dictionary);



/// Path Presenters
pub const DEFAULT_DICT_PATH_PRESENTER: ScopeTreePathPresenter = |path, _context| {
    PathConversion::from(path)
        .as_ffi_type()
        .to_token_stream()
};


pub const FFI_GENERIC_TYPE_PRESENTER: ScopeTreePathPresenter = |path, tree| {
    match PathConversion::from(path) {
        PathConversion::Primitive(path) |
        PathConversion::Complex(path) =>
            path.to_token_stream(),
        PathConversion::Generic(GenericPathConversion::Result(path)) |
        PathConversion::Generic(GenericPathConversion::Map(path)) |
        PathConversion::Generic(GenericPathConversion::Vec(path)) => {
            let short_ty: Type = parse_quote!(#path);
            tree.scope_types.iter()
                .find_map(|(TypeConversion{ 0: other}, full_type)|
                    short_ty.to_token_stream().to_string().eq(other.to_token_stream().to_string().as_str())
                        .then_some(full_type))
                .map_or(quote!(*mut #short_ty), |full_type| {
                    let full_ty = ffi_mangled_ident(full_type);
                    quote!(*mut #full_ty)
                })
        }
    }
};


pub const OPTION_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, OPTION_ARGUMENTS_PRESENTER, dictionary);

pub const OPAQUE_CONTEXT_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, OPAQUE_CONTEXT_ARGUMENTS_PRESENTER, dictionary);
pub const OPAQUE_CONTEXT_MUT_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, OPAQUE_CONTEXT_MUT_ARGUMENTS_PRESENTER, dictionary);



fn create_struct(name: TokenStream2, implementation: TokenStream2) -> TokenStream2 {
    let path: Path = parse_quote!(#name);
    let ident = &path.segments.last().unwrap().ident;
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct #ident #implementation
    }
}

pub fn package() -> TokenStream2 {
    quote!(ferment_interfaces)
}

pub fn interface() -> TokenStream2 {
    quote!(FFIConversion)
}

pub fn obj() -> TokenStream2 {
    quote!(obj)
}

pub fn destroy() -> TokenStream2 {
    quote!(destroy)
}

pub fn ffi_from() -> TokenStream2 {
    quote!(ffi_from)
}

pub fn ffi_from_const() -> TokenStream2 {
    quote!(ffi_from_const)
}

pub fn ffi_from_opt() -> TokenStream2 {
    quote!(ffi_from_opt)
}

pub fn ffi_to() -> TokenStream2 {
    quote!(ffi_to)
}
pub fn ffi_to_const() -> TokenStream2 {
    quote!(ffi_to_const)
}

pub fn ffi_to_opt() -> TokenStream2 {
    quote!(ffi_to_opt)
}

pub fn package_unbox_any_expression(expr: TokenStream2) -> TokenStream2 {
    let package = package();
    quote!(#package::unbox_any(#expr))
}

pub fn package_unbox_any_expression_terminated(expr: TokenStream2) -> TokenStream2 {
    let package_unbox_any_expr = package_unbox_any_expression(expr);
    quote!(#package_unbox_any_expr;)
}

pub fn package_unboxed_root() -> TokenStream2 {
    package_unbox_any_expression(quote!(ffi))
}

pub fn package_boxed_expression(expr: TokenStream2) -> TokenStream2 {
    let package = package();
    quote!(#package::boxed(#expr))
}

pub fn package_boxed_vec_expression(expr: TokenStream2) -> TokenStream2 {
    let package = package();
    quote!(#package::boxed_vec(#expr))
}

pub fn iter_map_collect(iter: TokenStream2, mapper: TokenStream2) -> TokenStream2 {
    quote!(#iter.map(#mapper).collect())
}

pub fn ffi_from_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_from = ffi_from();
    quote!(#package::#interface::#ffi_from(#field_value))
}

pub fn ffi_to_conversion(field_path: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_to = ffi_to();
    quote!(#package::#interface::#ffi_to(#field_path))
}

pub fn ffi_from_opt_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_from_opt = ffi_from_opt();
    quote!(#package::#interface::#ffi_from_opt(#field_value))
}

pub fn ffi_to_opt_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_to_opt = ffi_to_opt();
    quote!(#package::#interface::#ffi_to_opt(#field_value))
}

// TODO: provide full type or make an import
pub fn destroy_conversion(field_value: TokenStream2, ffi_type: Type, field_type: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let destroy = destroy();
    // quote!(#package::#interface::#destroy(#field_value))
    quote!(<#ffi_type as #package::#interface<#field_type>>::#destroy(#field_value))
}
