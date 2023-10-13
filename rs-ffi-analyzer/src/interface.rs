use std::collections::HashMap;
use syn::{Field, parse_quote, Path, PathArguments, Type, TypeArray, TypePath, TypePtr, TypeReference};
use quote::{format_ident, quote, ToTokens};
use quote::__private::{TokenStream as TokenStream2};
use syn::__private::Span;

use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::helper::{ffi_struct_name, mangle_type, path_arguments_to_path_conversions, path_arguments_to_types};
use crate::type_conversion::TypeConversion;

pub trait Presentable where Self: Sized {
    fn present(self) -> TokenStream2;
}

/// token -> token
pub type MapPresenter = fn(field_name: TokenStream2) -> TokenStream2;
/// token + token -> token
pub type MapPairPresenter = fn(field_name: TokenStream2, conversion: TokenStream2) -> TokenStream2;
/// token + type -> token
pub type FieldPresenter = fn(field: &Field) -> TokenStream2;
// pub type DictionaryFieldPresenter = fn(field: &Field, dictionary: &HashMap<ImportType, HashSet<ImportConversion>>) -> TokenStream2;
pub type ScopeTreeFieldPresenter = fn(field: &Field, tree: &HashMap<TypeConversion, Type>) -> TokenStream2;

pub type TypePresenter = fn(field_type: &Type) -> TokenStream2;
// pub type DictionaryTypePresenter = fn(field_type: &Type, dictionary: &HashMap<ImportType, HashSet<ImportConversion>>) -> TokenStream2;

/// token + type -> token
pub type FieldTypedPresenter = fn(field_name: TokenStream2, field_type: &Type) -> TokenStream2;
// pub type DictionaryFieldTypedPresenter = fn(field_name: TokenStream2, field_type: &Type, dictionary: &HashMap<ImportType, HashSet<ImportConversion>>) -> TokenStream2;
pub type ScopeTreeFieldTypedPresenter = fn(field_name: TokenStream2, field_type: &Type, tree: &HashMap<TypeConversion, Type>) -> TokenStream2;
/// [token] -> token
pub type IteratorPresenter = fn(items: Vec<TokenStream2>) -> TokenStream2;

pub type ScopeTreeItemTypePresenter = fn(field_type: &Type, tree: &HashMap<TypeConversion, Type>) -> TokenStream2;
/// token + [token] -> token
/// type OwnerIteratorPresenter = fn(owner: TokenStream2, items: Vec<TokenStream2>) -> TokenStream2;
pub type OwnerIteratorPresenter = fn((TokenStream2, Vec<TokenStream2>)) -> TokenStream2;
pub type PathPresenter = fn(path: &Path) -> TokenStream2;
// pub type DictionaryPathPresenter = fn(path: &Path, dictionary: &HashMap<ImportType, HashSet<ImportConversion>>) -> TokenStream2;
pub type ScopeTreePathPresenter = fn(path: &Path, tree: &HashMap<TypeConversion, Type>) -> TokenStream2;
pub type PathArgumentsPresenter = fn(arguments: &PathArguments) -> TokenStream2;
// pub type DictionaryPathArgumentsPresenter = fn(arguments: &PathArguments, dictionary: &HashMap<ImportType, HashSet<ImportConversion>>) -> TokenStream2;
pub type ScopeTreePathArgumentsPresenter = fn(arguments: &PathArguments, tree: &HashMap<TypeConversion, Type>) -> TokenStream2;

pub type GenericVecPresenter = fn((TokenStream2, &Path)) -> TokenStream2;
pub type GenericMapPresenter = fn((TokenStream2, &Path, &Path, &Path)) -> TokenStream2;

pub type GenericPathPresenter = fn(path: &Path, arguments_presenter: ScopeTreePathArgumentsPresenter, tree: &HashMap<TypeConversion, Type>) -> TokenStream2;


/// Field Presenters
pub const UNNAMED_VARIANT_FIELD_PRESENTER: ScopeTreeFieldPresenter = |Field { ty, .. }, tree|
    FFI_DICTIONARY_TYPE_PRESENTER(ty, tree);
pub const NAMED_VARIANT_FIELD_PRESENTER :ScopeTreeFieldPresenter = |Field { ident, ty: field_type, .. }, tree|
    NAMED_CONVERSION_PRESENTER(ident.clone().unwrap().to_token_stream(), FFI_DICTIONARY_TYPE_PRESENTER(field_type, tree));


/// Type Presenters
pub const FFI_DICTIONARY_TYPE_PRESENTER: ScopeTreeItemTypePresenter = |field_type, tree| {
    // let full_type = dictionary.fin
    let result = match field_type {
        Type::Path(TypePath { path, .. }) =>
            (match path.segments.last().unwrap().ident.to_string().as_str() {
                "Vec" | "BTreeMap" | "HashMap" => FFI_GENERIC_TYPE_PRESENTER,
                "Option" => OPTION_PATH_PRESENTER,
                // "String" =>
                "OpaqueContext" => OPAQUE_CONTEXT_PATH_PRESENTER,
                "OpaqueContextMut" => OPAQUE_CONTEXT_MUT_PATH_PRESENTER,
                _ => DEFAULT_DICT_PATH_PRESENTER,
            })(path, tree),
        Type::Array(TypeArray { elem, len, .. }) =>
            FFI_ARRAY_FIELD_TYPED_PRESENTER(quote!(#len), &elem),
        Type::Reference(TypeReference { elem, .. }) =>
            FFI_DICTIONARY_TYPE_PRESENTER(&**elem, tree),
        Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
            match &**elem {
                Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                    "c_void" => match (star_token, const_token, mutability) {
                        (_, Some(_const_token), None) => quote!(OpaqueContextFFI),
                        (_, None, Some(_mut_token)) => quote!(OpaqueContextMutFFI),
                        _ => panic!("extract_struct_field: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                    },
                    _ => quote!(*mut #path)
                },
                Type::Ptr(type_ptr) => quote!(*mut #type_ptr),
                _ => panic!("extract_struct_field: {} not supported", quote!(#elem))
            }
        _ => panic!("FFI_DICTIONARY_TYPE_PRESENTER: type not supported: {}", quote!(#field_type))
    };
    println!("FFI_DICTIONARY_TYPE_PRESENTER: {} --> {}", field_type.to_token_stream(), result);
    result
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
pub const DEFAULT_DOC_PRESENTER: MapPresenter = |target_name: TokenStream2| doc(target_name.to_string());


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
// pub const EMPTY_FIELD_TYPED_PRESENTER: FieldTypedPresenter = |_, _|
//     quote!();
// pub const DEFAULT_FIELD_PRESENTER: FieldTypedPresenter = |field_name, _|
//     quote!(#field_name);
// pub const DEFAULT_FIELD_TYPE_PRESENTER: FieldTypedPresenter = |_, field_type|
//     FFI_TYPE_PRESENTER(field_type);
// pub const NAMED_FIELD_TYPE_PRESENTER: FieldTypedPresenter = |field_name, field_type|
//     PUB_NAMED_CONVERSION_PRESENTER(field_name, FFI_TYPE_PRESENTER(field_type));

pub const FFI_ARRAY_FIELD_TYPED_PRESENTER: FieldTypedPresenter = |len, elem|
    quote!(*mut [#elem; #len]);

pub const EMPTY_DICT_FIELD_TYPED_PRESENTER: ScopeTreeFieldTypedPresenter = |_, _, _|
    quote!();
pub const DEFAULT_DICT_FIELD_PRESENTER: ScopeTreeFieldTypedPresenter = |field_name, _, _|
    quote!(#field_name);
pub const DEFAULT_DICT_FIELD_TYPE_PRESENTER: ScopeTreeFieldTypedPresenter = |_, field_type, tree|
    FFI_DICTIONARY_TYPE_PRESENTER(field_type, tree);
pub const NAMED_DICT_FIELD_TYPE_PRESENTER: ScopeTreeFieldTypedPresenter = |field_name, field_type, tree|
    PUB_NAMED_CONVERSION_PRESENTER(field_name, FFI_DICTIONARY_TYPE_PRESENTER(field_type, tree));



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
    // #[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Ord)]
    quote! {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum #enum_presentation
    }
};


/// PathArguments Presenters
pub const OPAQUE_CONTEXT_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |_, _|
    quote!(rs_ffi_interfaces::OpaqueContextFFI);
pub const OPAQUE_CONTEXT_MUT_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |_, _|
    quote!(rs_ffi_interfaces::OpaqueContextMutFFI);

pub const OPTION_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |arguments, tree| match path_arguments_to_types(arguments)[..] {
    [field_type] => FFI_DICTIONARY_TYPE_PRESENTER(field_type, tree),
    _ => panic!("OPTION_ARGUMENTS_PRESENTER: arguments: {} not supported", quote!(#arguments))
};
pub const MANGLE_MAP_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |arguments, tree| match &path_arguments_to_path_conversions(arguments)[..] {
    [key_conversion, value_conversion] => {
        let ident_string = format!("keys_{}_values_{}", key_conversion.mangled_map_ident(tree), value_conversion.mangled_map_ident(tree));
        syn::LitInt::new(&ident_string, Span::call_site()).to_token_stream()
    },
    _ => panic!("MANGLE_MAP_ARGUMENTS_PRESENTER: Map nested in Vec not supported yet"),
}.to_token_stream();


pub const MANGLE_VEC_ARGUMENTS_PRESENTER: ScopeTreePathArgumentsPresenter = |arguments, tree|
    path_arguments_to_path_conversions(arguments)
        .first()
        .unwrap()
        .mangled_vec_arguments(tree);

pub const GENERIC_PATH_PRESENTER: GenericPathPresenter = |path, arguments_presenter, dictionary|
    arguments_presenter(&path.segments.last().unwrap().arguments, dictionary);



/// Path Presenters
pub const DEFAULT_DICT_PATH_PRESENTER: ScopeTreePathPresenter = |path, _dictionary|
    PathConversion::from(path).as_ffi_type().to_token_stream();

pub const FFI_GENERIC_TYPE_PRESENTER: ScopeTreePathPresenter = |path, tree| {
    println!("FFI_GENERIC_TYPE_PRESENTER: {}", path.to_token_stream());
    match PathConversion::from(path) {
        PathConversion::Primitive(path) => path.to_token_stream(),
        PathConversion::Complex(path) => {
            println!("FFI_GENERIC_TYPE_PRESENTER: COMPLEX: {}", path.to_token_stream());
            path.to_token_stream()
        },
        PathConversion::Generic(GenericPathConversion::Map(path)) |
        PathConversion::Generic(GenericPathConversion::Vec(path)) => {
            let short_ty: Type = parse_quote!(#path);
            let found = tree.iter().find(|(tc, _full_ty)| short_ty.eq(&tc.0));
            match found {
                Some((_, full_type)) => {
                    let ident = mangle_type(full_type);
                    let full_ty = ffi_struct_name(&ident);
                    println!("FFI_GENERIC_TYPE_PRESENTER:: GENERIC FOUND: {} -> {} -> {}", path.to_token_stream(), quote!(#full_type), quote!(#full_ty));

                    quote!(*mut #full_ty)
                },
                _ => {
                    println!("FFI_GENERIC_TYPE_PRESENTER:: GENERIC NOT FOUND: {} -> {}", path.to_token_stream(), quote!(#short_ty));
                    quote!(*mut #short_ty)
                }
            }
            // PathConversion::Primitive(path) => parse_quote!(#path),
            // PathConversion::Complex(path) => {
            //     let ty = Scope::ffi_type_converted_or_same(&parse_quote!(#path));
            //     parse_quote!(*mut #ty)
            // },
            // PathConversion::Generic(GenericPathConversion::Map(path)) |
            //     PathConversion::Generic(GenericPathConversion::Vec(path)) => {
            //     let ty = Self::convert_to_ffi_type(path);
            //     parse_quote!(*mut #ty)
        }
    }
};
// PathConversion::from(path).make_full_qualified_ffi_type_if_need(dictionary)
//         .to_token_stream();

// pub const FFI_STRUCT_PATH_PRESENTER: PathPresenter = |path| {
//     (match path.segments.last().unwrap().ident.to_string().as_str() {
//         // "Vec" => VEC_PATH_PRESENTER, //|path| PathConversion::from(path).as_ffi_path(),
//         // "BTreeMap" | "HashMap" => MAP_PATH_PRESENTER,
//         "Vec" | "BTreeMap" | "HashMap" => FFI_GENERIC_TYPE_PRESENTER,
//         "Option" => OPTION_PATH_PRESENTER,
//         "OpaqueContext" => OPAQUE_CONTEXT_PATH_PRESENTER,
//         "OpaqueContextMut" => OPAQUE_CONTEXT_MUT_PATH_PRESENTER,
//         _ => DEFAULT_PATH_PRESENTER,
//     })(path)
// };

pub const FFI_TYPE_PATH_PRESENTER: PathPresenter = |path|
    FFI_TYPE_PATH_CONVERTER(path)
        .to_token_stream();

pub const FFI_TYPE_PATH_CONVERTER: fn(&Path) -> Path = |path|
    PathConversion::from(path)
        .as_ffi_path();

// pub const GENERIC_VEC_SIMPLE_PRESENTER: GenericVecPresenter = |(ffi_name, value_path)|
//     generics::vec_ffi_simple_expansion(ffi_name, value_path);
//
// pub const GENERIC_VEC_COMPLEX_PRESENTER: GenericVecPresenter = |(ffi_name, value_path)|
//     generics::vec_ffi_complex_expansion(ffi_name, value_path);
//
// pub const GENERIC_MAP_SIMPLE_PRESENTER: GenericMapPresenter = |(ffi_name, root, key_path, value_path)|
//     generics::map_ffi_simple_expansion(ffi_name, root, key_path, value_path);
//
// pub const GENERIC_MAP_SIMPLE_COMPLEX_PRESENTER: GenericMapPresenter = |(ffi_name, root, key_path, value_path)|
//     generics::map_ffi_simple_complex_expansion(ffi_name, root, key_path, value_path);
//
// pub const GENERIC_MAP_COMPLEX_SIMPLE_PRESENTER: GenericMapPresenter = |(ffi_name, root, key_path, value_path)|
//     generics::map_ffi_complex_simple_expansion(ffi_name, root, key_path, value_path);
//
// pub const GENERIC_MAP_COMPLEX_PRESENTER: GenericMapPresenter = |(ffi_name, root, key_path, value_path)|
//     generics::map_ffi_complex_expansion(ffi_name, root, key_path, value_path);

pub const MANGLE_INNER_PATH_PRESENTER: ScopeTreePathPresenter = |path, tree| match PathConversion::from(path) {
    PathConversion::Primitive(path) |
    PathConversion::Complex(path) => MANGLE_PATH_PRESENTER(&path, tree),
    PathConversion::Generic(GenericPathConversion::Vec(path)) => MANGLE_VEC_ARGUMENTS_PRESENTER(&path.segments.last().unwrap().arguments, tree),
    PathConversion::Generic(GenericPathConversion::Map(path)) => MANGLE_MAP_ARGUMENTS_PRESENTER(&path.segments.last().unwrap().arguments, tree)
};

pub const VEC_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, MANGLE_VEC_ARGUMENTS_PRESENTER, dictionary);

pub const MAP_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, MANGLE_MAP_ARGUMENTS_PRESENTER, dictionary);

pub const OPTION_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, OPTION_ARGUMENTS_PRESENTER, dictionary);

pub const OPAQUE_CONTEXT_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, OPAQUE_CONTEXT_ARGUMENTS_PRESENTER, dictionary);
pub const OPAQUE_CONTEXT_MUT_PATH_PRESENTER: ScopeTreePathPresenter = |path, dictionary|
    GENERIC_PATH_PRESENTER(path, OPAQUE_CONTEXT_MUT_ARGUMENTS_PRESENTER, dictionary);

pub const MANGLE_PATH_PRESENTER: ScopeTreePathPresenter = |path, _dictionary|
    format_ident!("{}",
        path.segments.iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<String>>().join("_"))
        .to_token_stream();


fn create_struct(name: TokenStream2, implementation: TokenStream2) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct #name #implementation
    }
}

pub fn doc(target_name: String) -> TokenStream2 {
    let comment = format!("FFI-representation of the {}", target_name);
    parse_quote! { #[doc = #comment] }
}

pub fn package() -> TokenStream2 {
    quote!(rs_ffi_interfaces)
}

pub fn interface() -> TokenStream2 {
    quote!(FFIConversion)
}

pub fn ffi() -> TokenStream2 {
    quote!(ffi)
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

pub fn boxed() -> TokenStream2 {
    quote!(boxed)
}

pub fn boxed_vec() -> TokenStream2 {
    quote!(boxed_vec)
}

pub fn unbox_any() -> TokenStream2 {
    quote!(unbox_any)
}

pub fn package_boxed() -> TokenStream2 {
    let package = package();
    let boxed = boxed();
    quote!(#package::#boxed)
}

pub fn package_unbox_any() -> TokenStream2 {
    let package = package();
    let unbox_any = unbox_any();
    quote!(#package::#unbox_any)
}

pub fn package_unbox_any_expression(expr: TokenStream2) -> TokenStream2 {
    let package_unbox_any = package_unbox_any();
    quote!(#package_unbox_any(#expr))
}

pub fn package_unbox_any_expression_terminated(expr: TokenStream2) -> TokenStream2 {
    let package_unbox_any_expr = package_unbox_any_expression(expr);
    quote!(#package_unbox_any_expr;)
}

pub fn package_unboxed_root() -> TokenStream2 {
    package_unbox_any_expression(ffi())
}

pub fn package_boxed_expression(expr: TokenStream2) -> TokenStream2 {
    let package_boxed = package_boxed();
    quote!(#package_boxed(#expr))
}

pub fn package_boxed_vec() -> TokenStream2 {
    let package = package();
    let boxed_vec = boxed_vec();
    quote!(#package::#boxed_vec)
}

pub fn package_boxed_vec_expression(expr: TokenStream2) -> TokenStream2 {
    let package_boxed_vec = package_boxed_vec();
    quote!(#package_boxed_vec(#expr))
}

pub fn iter_map_collect(iter: TokenStream2, mapper: TokenStream2) -> TokenStream2 {
    quote!(#iter.map(#mapper).collect())
}

pub fn unwrap_or(field_path: TokenStream2, or: TokenStream2) -> TokenStream2 {
    quote!(#field_path.unwrap_or(#or))
}


pub fn ffi_from_map_conversion(map_key_path: TokenStream2, acc_type: TokenStream2, key_conversion: TokenStream2, value_conversion: TokenStream2) -> TokenStream2 {
    quote! {{
        let map = &*#map_key_path;
        (0..map.count).fold(#acc_type::new(), |mut acc, i| {
            let key = #key_conversion;
            let value = #value_conversion;
            acc.insert(key, value);
            acc
        })
    }}
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

pub fn destroy_conversion(field_value: TokenStream2, ffi_type: TokenStream2, field_type: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let destroy = destroy();
    quote!(<#ffi_type as #package::#interface<#field_type>>::#destroy(#field_value))
}
