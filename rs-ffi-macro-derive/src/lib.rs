extern crate proc_macro;
use proc_macro::TokenStream;
use std::string::ToString;
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, ItemFn, Meta, NestedMeta, Type, PathArguments, GenericArgument, TypePtr, TypeArray, Ident, TypePath, DataStruct, Fields, FieldsUnnamed, FieldsNamed, DataEnum, Expr, Path, ReturnType, FnArg, PatType, AngleBracketedGenericArguments, Pat, PatIdent, Field, TypeReference, Variant, Item, ItemType, TypeBareFn, BareFnArg, parse_quote};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use quote::__private::Span;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;

enum ConversionType {
    Simple(Path),
    Complex(Path),
    Map(Path),
    Vec(Path)
}

type ComposerContext = fn(context: &PresentationComposer) -> TokenStream2;
/// token -> token
type MapPresenter = fn(field_name: TokenStream2) -> TokenStream2;
/// token + token -> token
type MapPairPresenter = fn(field_name: TokenStream2, conversion: TokenStream2) -> TokenStream2;
/// token + type -> token
type FieldTypedPresenter = fn(field_name: TokenStream2, field_type: &Type) -> TokenStream2;
/// [token] -> token
type IteratorPresenter = fn(items: Vec<TokenStream2>) -> TokenStream2;
/// token + [token] -> token
/// type OwnerIteratorPresenter = fn(owner: TokenStream2, items: Vec<TokenStream2>) -> TokenStream2;
type OwnerIteratorPresenter = fn((TokenStream2, Vec<TokenStream2>)) -> TokenStream2;

type InterfacePresenter = fn(ffi_name: TokenStream2, target_name: TokenStream2, from_presentation: TokenStream2, to_presentation: TokenStream2, destroy_presentation: TokenStream2) -> TokenStream2;



const DEFAULT_INTERFACE_PRESENTER: InterfacePresenter = |ffi_name: TokenStream2, target_name: TokenStream2, from_presentation: TokenStream2, to_presentation: TokenStream2, destroy_presentation: TokenStream2|
    impl_interface(ffi_name, target_name, from_presentation, to_presentation, destroy_presentation);



// const EMPTY_COMPOSER_PRESENTER: ComposerContext = |_| quote!();
const EMPTY_MAP_PRESENTER: MapPresenter = |_| quote!();
const SIMPLE_PRESENTER: MapPresenter = |name| quote!(#name);
const SIMPLE_TERMINATED_PRESENTER: MapPresenter = |name| quote!(#name;);
const ROOT_DESTROY_CONTEXT_PRESENTER: MapPresenter = |_| package_unboxed_root();
const EMPTY_DESTROY_PRESENTER: MapPresenter = |_| quote!({});
// const FFI_TO_CONVERSION_PRESENTER: MapPresenter = |conversions: TokenStream2| package_boxed_expression(conversions);
const DEFAULT_DOC_PRESENTER: MapPresenter = |target_name: TokenStream2| doc(target_name.to_string());
const SAFETY_DOC_PRESENTER: MapPresenter = |target_name: TokenStream2| {
    let doc = DEFAULT_DOC_PRESENTER(target_name);
    quote! {
        #doc
        /// # Safety
    }
};
const FFI_DEREF_FIELD_NAME: MapPresenter = |field_name| quote!(ffi_ref.#field_name);
const OBJ_FIELD_NAME: MapPresenter = |field_name| quote!(obj.#field_name);

// const DROP_UNNAMED_PRESENTER: MapPresenter = |_| package_unbox_any_expression_terminated(quote!(self.0));

// const CURVY_PAIR_PRESENTER: MapPairPresenter = |name, presentation| quote!(#name { #presentation });
// const ROUND_PAIR_PRESENTER: MapPairPresenter = |name, presentation| quote!(#name ( #presentation ));

const EMPTY_PAIR_PRESENTER: MapPairPresenter = |_, _| quote!();
// const TYPE_ALIAS_DROP_PRESENTER: MapPairPresenter = |_, _| DROP_UNNAMED_PRESENTER(quote!());
const SIMPLE_PAIR_PRESENTER: MapPairPresenter = |name, presentation| quote!(#name #presentation);
const SIMPLE_CONVERSION_PRESENTER: MapPairPresenter = |_, conversion| quote!(#conversion);
const NAMED_CONVERSION_PRESENTER: MapPairPresenter = define_field;
const PUB_NAMED_CONVERSION_PRESENTER: MapPairPresenter = define_pub_field;
const LAMBDA_CONVERSION_PRESENTER: MapPairPresenter = define_lambda;
const DROP_INTERFACE_PRESENTER: MapPairPresenter = impl_drop;
const FFI_FROM_CONVERSION_PRESENTER: MapPairPresenter = |field_path: TokenStream2, conversions: TokenStream2| quote!(let ffi_ref = #field_path; #conversions);
const FFI_TO_CONVERSION_PRESENTER: MapPairPresenter = |_, conversions: TokenStream2| package_boxed_expression(conversions);
const EMPTY_FIELD_TYPED_PRESENTER: FieldTypedPresenter = |_, _| quote!();
const DEFAULT_FIELD_PRESENTER: FieldTypedPresenter = |field_name, _| quote!(#field_name);
const DEFAULT_FIELD_TYPE_PRESENTER: FieldTypedPresenter = |_, field_type| extract_struct_field(field_type);
const NAMED_FIELD_TYPE_PRESENTER: FieldTypedPresenter = |field_name, field_type| PUB_NAMED_CONVERSION_PRESENTER(field_name, extract_struct_field(field_type));

// Iterator Presenters
const EMPTY_ITERATOR_PRESENTER: IteratorPresenter = |_| quote!();
const DEFAULT_DESTROY_FIELDS_PRESENTER: IteratorPresenter = |destructors| quote!({#(#destructors)*});
const CURLY_ITER_PRESENTER: IteratorPresenter = |fields: Vec<TokenStream2>| quote!({ #(#fields,)* });
const ROUND_ITER_PRESENTER: IteratorPresenter = |fields: Vec<TokenStream2>| quote!(( #(#fields,)* ));
const STRUCT_DESTROY_PRESENTER: IteratorPresenter = |fields| match fields.len() {
    0 => quote!(),
    _ => quote!(let ffi_ref = self; #(#fields;)*)
};

const ENUM_DESTROY_PRESENTER: IteratorPresenter = |fields| match fields.len() {
    0 => quote!(),
    _ => MATCH_FIELDS_PRESENTER((quote!(self), fields))
};

// Owner Iterator Presenters
const EMPTY_FIELDS_PRESENTER: OwnerIteratorPresenter = |_| quote!();



const CURLY_BRACES_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)| SIMPLE_PAIR_PRESENTER(name, CURLY_ITER_PRESENTER(fields));
const ROUND_BRACES_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)| SIMPLE_PAIR_PRESENTER(name, ROUND_ITER_PRESENTER(fields));
const MATCH_FIELDS_PRESENTER: OwnerIteratorPresenter = |(field_path, fields)| SIMPLE_PAIR_PRESENTER(quote!(match #field_path), CURLY_ITER_PRESENTER(fields));
const NO_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, _)| quote!(#name);
const ENUM_UNIT_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)| quote!(#name = #(#fields)*);

const TYPE_ALIAS_PRESENTER: OwnerIteratorPresenter = |(name, fields)| create_struct(name, SIMPLE_TERMINATED_PRESENTER(ROUND_ITER_PRESENTER(fields)));
const UNNAMED_STRUCT_PRESENTER: OwnerIteratorPresenter = |(name, fields)| create_struct(name, SIMPLE_TERMINATED_PRESENTER(ROUND_ITER_PRESENTER(fields)));
const NAMED_STRUCT_PRESENTER: OwnerIteratorPresenter = |(name, fields)| create_struct(name, CURLY_ITER_PRESENTER(fields));
const ENUM_NAMED_VARIANT_PRESENTER: OwnerIteratorPresenter = |(name, fields)| quote!(#name { #(#fields),* });
const ENUM_UNNAMED_VARIANT_PRESENTER: OwnerIteratorPresenter = |(name, fields)| SIMPLE_PAIR_PRESENTER(name, ROUND_ITER_PRESENTER(fields));
const ENUM_PRESENTER: OwnerIteratorPresenter = |(name, fields)| {
    let enum_presentation = CURLY_BRACES_FIELDS_PRESENTER((name, fields));
    quote! {
        #[repr(C)]
        #[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Ord)]
        pub enum #enum_presentation
    }
};
const TYPE_ALIAS_CONVERSION_FROM_PRESENTER: OwnerIteratorPresenter = |(_, fields)| quote!((#(#fields)*));
const TYPE_ALIAS_CONVERSION_TO_PRESENTER: OwnerIteratorPresenter = |(name, fields)| quote!(#name(#(#fields),*));

struct PresentationComposer {
    pub ffi_name: TokenStream2,
    pub target_name: TokenStream2,
    pub conversion_from_path: TokenStream2,
    pub conversion_to_path: TokenStream2,

    pub from: MapPresenter,
    pub to: MapPresenter,
    pub destroy: MapPresenter,

    pub root_presenter: OwnerIteratorPresenter,
    pub root_conversion_from_presenter: OwnerIteratorPresenter,
    pub root_conversion_to_presenter: OwnerIteratorPresenter,
    pub root_destroy_presenter: IteratorPresenter,

    pub doc_presenter: MapPresenter,

    pub from_context_presenter: ComposerContext,
    pub to_context_presenter: ComposerContext,

    pub from_conversions_composer: MapPairPresenter,
    pub to_conversions_composer: MapPairPresenter,

    pub interface_presenter: InterfacePresenter,
    pub field_presenter: FieldTypedPresenter,
    pub conversion_presenter: MapPairPresenter,

    pub destroy_code_context_presenter: MapPresenter,
    pub destroy_presenter: MapPresenter,
    pub drop_presenter: MapPairPresenter,



    pub fields: Vec<TokenStream2>,
    pub conversions_from: Vec<TokenStream2>,
    pub conversions_to: Vec<TokenStream2>,
    pub destructors: Vec<TokenStream2>,

}

impl PresentationComposer {
    fn type_alias_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: FieldTypedPresenter,
        from_context_presenter: ComposerContext,
        to_context_presenter: ComposerContext,
        from_conversions_composer: MapPairPresenter,
        to_conversions_composer: MapPairPresenter,
        root_conversion_from_presenter: OwnerIteratorPresenter,
        root_conversion_to_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        root_destroy_presenter: IteratorPresenter,
        destroy_code_context_presenter: MapPresenter,
        conversions_composer: I
    ) -> Self {
        let new = Self::new(
            ffi_name,
            target_name,
            root_presenter,
            field_presenter,
            from_context_presenter,
            to_context_presenter,
            from_conversions_composer,
            to_conversions_composer,
            root_conversion_from_presenter,
            root_conversion_to_presenter,
            conversion_presenter,
            root_destroy_presenter,
            destroy_code_context_presenter,
            SIMPLE_PRESENTER,
            DROP_INTERFACE_PRESENTER,
            DEFAULT_DOC_PRESENTER,
            |_| quote!(ffi_ref.0),
            |_| obj(),
            FFI_DEREF_FIELD_NAME,
            conversions_composer
        );
        // new.to = |_| obj();
        // new.from = |_| quote!(ffi_ref.0);
        // new.conversions_to = conversions_to;
        // new.conversions_from = conversions_from;
        new
    }

    fn struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        from_context_presenter: ComposerContext,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: FieldTypedPresenter,
        destroy_code_context_presenter: MapPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        conversions_composer: I) -> Self {
        Self::new(
            ffi_name,
            target_name,
            root_presenter,
            field_presenter,
            from_context_presenter,
            |_| quote!(),
            FFI_FROM_CONVERSION_PRESENTER,
            FFI_TO_CONVERSION_PRESENTER,
            root_conversion_presenter,
            root_conversion_presenter,
            conversion_presenter,
            STRUCT_DESTROY_PRESENTER,
            destroy_code_context_presenter,
            SIMPLE_PRESENTER,
            DROP_INTERFACE_PRESENTER,
            DEFAULT_DOC_PRESENTER,
            FFI_DEREF_FIELD_NAME,
            OBJ_FIELD_NAME,
            FFI_DEREF_FIELD_NAME,
            conversions_composer
        )
    }

    fn enum_variant_default_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        root_presenter: OwnerIteratorPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_code_context_presenter: MapPresenter,
        destroy_presenter: MapPresenter,
        conversions_presenter: I) -> Self {
        Self::new(
            ffi_name,
            target_name,
            root_presenter,
            DEFAULT_FIELD_PRESENTER,
            |composer| composer.fields_from(),
            |composer| composer.fields_to(),
            LAMBDA_CONVERSION_PRESENTER,
            LAMBDA_CONVERSION_PRESENTER,
            root_conversion_presenter,
            root_conversion_presenter,
            conversion_presenter,
            DEFAULT_DESTROY_FIELDS_PRESENTER,
            destroy_code_context_presenter,
            destroy_presenter,
            LAMBDA_CONVERSION_PRESENTER,
            DEFAULT_DOC_PRESENTER,
            deref_field_path,
            SIMPLE_PRESENTER,
            |f| quote!(#f.to_owned()), conversions_presenter)
    }

    fn enum_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        root_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_code_context_presenter: MapPresenter,
        destroy_presenter: MapPresenter,
        conversions_composer: I) -> Self {
        Self::enum_variant_default_composer(
            ffi_name,
            target_name,
            root_presenter,
            root_presenter,
            conversion_presenter,
            destroy_code_context_presenter,
            destroy_presenter,
            conversions_composer)
    }

    fn primitive_composer(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        root_presenter: OwnerIteratorPresenter,
        root_conversion_from_presenter: OwnerIteratorPresenter,
        root_conversion_to_presenter: OwnerIteratorPresenter,
        destroy_code_context_presenter: MapPresenter,
        drop_presenter: MapPairPresenter,
        conversions_from: Vec<TokenStream2>,
        conversion_to_path: TokenStream2) -> Self {
        let new = Self {
            ffi_name,
            target_name: target_name.clone(),
            interface_presenter: DEFAULT_INTERFACE_PRESENTER,
            field_presenter: EMPTY_FIELD_TYPED_PRESENTER,
            root_presenter,
            from_context_presenter: |_| quote!(*ffi),
            to_context_presenter: |_| quote!(),
            from_conversions_composer: FFI_FROM_CONVERSION_PRESENTER,
            to_conversions_composer: FFI_TO_CONVERSION_PRESENTER,
            root_conversion_from_presenter,
            root_conversion_to_presenter,
            conversion_presenter: EMPTY_PAIR_PRESENTER,
            destroy_presenter: EMPTY_MAP_PRESENTER,
            root_destroy_presenter: EMPTY_ITERATOR_PRESENTER,
            drop_presenter,
            doc_presenter: DEFAULT_DOC_PRESENTER,
            from: EMPTY_MAP_PRESENTER,
            to: EMPTY_MAP_PRESENTER,
            destroy: EMPTY_MAP_PRESENTER,
            destroy_code_context_presenter,
            fields: vec![],
            conversions_from,
            conversion_from_path: target_name.clone(),
            conversions_to: vec![],
            conversion_to_path,
            destructors: vec![]
        };
        new
    }



    fn enum_unit_variant_composer(ffi_name: TokenStream2, target_name: TokenStream2, destroy_code_context_presenter: MapPresenter) -> Self {
        Self::enum_variant_default_composer(
            ffi_name,
            target_name,
            NO_FIELDS_PRESENTER,
            NO_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            destroy_code_context_presenter,
            EMPTY_DESTROY_PRESENTER,
            IntoIterator::into_iter(vec![])
        )
    }

    fn enum_unnamed_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(ffi_name: TokenStream2, target_name: TokenStream2, destroy_code_context_presenter: MapPresenter, conversions_presenter: I) -> Self {
        Self::enum_variant_composer(
            ffi_name,
            target_name,
            ROUND_BRACES_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            destroy_code_context_presenter,
            SIMPLE_TERMINATED_PRESENTER,
            conversions_presenter
        )
    }

    fn enum_named_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(ffi_name: TokenStream2, target_name: TokenStream2, destroy_code_context_presenter: MapPresenter, conversions_presenter: I) -> Self {
        Self::enum_variant_composer(
            ffi_name,
            target_name,
            CURLY_BRACES_FIELDS_PRESENTER,
            NAMED_CONVERSION_PRESENTER,
            destroy_code_context_presenter,
            SIMPLE_PRESENTER,
            conversions_presenter
        )
    }



    fn new<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: FieldTypedPresenter,
        from_context_presenter: ComposerContext,
        to_context_presenter: ComposerContext,
        from_conversions_composer: MapPairPresenter,
        to_conversions_composer: MapPairPresenter,
        root_conversion_from_presenter: OwnerIteratorPresenter,
        root_conversion_to_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        root_destroy_presenter: IteratorPresenter,
        destroy_code_context_presenter: MapPresenter,
        destroy_presenter: MapPresenter,
        drop_presenter: MapPairPresenter,
        doc_presenter: MapPresenter,
        from: MapPresenter,
        to: MapPresenter,
        destroy: MapPresenter,
        conversions_composer: I) -> Self where Self: Sized {
        let mut new = Self {
            ffi_name: ffi_name.clone(),
            target_name: target_name.clone(),
            interface_presenter: DEFAULT_INTERFACE_PRESENTER,
            field_presenter,
            root_presenter,
            from_context_presenter,
            to_context_presenter,
            from_conversions_composer,
            to_conversions_composer,
            root_conversion_from_presenter,
            root_conversion_to_presenter,
            conversion_presenter,
            destroy_code_context_presenter,
            destroy_presenter,
            root_destroy_presenter,
            drop_presenter,
            doc_presenter,
            from,
            to,
            destroy,
            fields: vec![],
            conversions_from: vec![],
            conversions_to: vec![],
            conversion_from_path: target_name.clone(),
            conversion_to_path: ffi_name.clone(),
            destructors: vec![]
        };
        for (field_type, field_name) in conversions_composer {
            new.add_conversion(&field_type, field_name);
        }
        println!("----->: fields: {:?}", new.fields);
        println!("----->: conversions_from: {:?}", new.conversions_from);
        println!("----->: conversions_to: {:?}", new.conversions_to);
        println!("----->: destructors: {:?}", new.destructors);

        new
    }

    fn add_conversion(&mut self, field_type: &Type, field_name: TokenStream2) {
        println!("add_conversion: {:?} {:?}", field_name, field_type);
        let field_path_to = (self.to)(field_name.clone());
        let field_path_from = (self.from)(field_name.clone());
        let field_path_destroy = (self.destroy)(field_name.clone());
        let (converted_field_to, converted_field_from, destructor) = match field_type {
            Type::Ptr(type_ptr) => (
                to_ptr(field_path_to, type_ptr),
                from_ptr(field_path_from, type_ptr),
                destroy_ptr(field_path_destroy, type_ptr)
            ),
            Type::Path(TypePath { path, .. }) => (
                to_path(field_path_to, path, None),
                from_path(field_path_from, path, None),
                destroy_path(field_path_destroy, path, None),
            ),
            Type::Reference(type_reference) => (
                to_reference(field_path_to, type_reference),
                from_reference(field_path_from, type_reference),
                destroy_reference(field_path_destroy, type_reference)
            ),
            Type::Array(type_array) => (
                to_array(field_path_to, type_array),
                from_array(field_path_from, type_array),
                destroy_array(field_path_destroy, type_array)
            ),
            _ => panic!("add_conversion: Unknown field {:?} {:?}", field_name, field_type),
        };

        self.fields.push((self.field_presenter)(field_name.clone(), field_type));
        self.conversions_to.push((self.conversion_presenter)(field_name.clone(), converted_field_to));
        self.conversions_from.push((self.conversion_presenter)(field_name.clone(), converted_field_from));
        self.destructors.push((self.destroy_presenter)(destructor));
    }

    fn fields_from(&self) -> TokenStream2 {
        self.fields(self.ffi_name.to_token_stream())
    }

    fn fields_to(&self) -> TokenStream2 {
        (self.root_presenter)((self.target_name.to_token_stream(), self.fields.clone()))
    }

    fn fields(&self, root: TokenStream2) -> TokenStream2 {
        (self.root_presenter)((root, self.fields.clone()))
    }

    fn conversions_from(&self) -> TokenStream2 {
        (self.root_conversion_from_presenter)((self.conversion_from_path.to_token_stream(), self.conversions_from.clone()))
    }

    fn conversions_to(&self) -> TokenStream2 {
        (self.root_conversion_to_presenter)((self.conversion_to_path.to_token_stream(), self.conversions_to.clone()))
    }

    fn destroy_presentation(&self) -> TokenStream2 {
        (self.root_destroy_presenter)(self.destructors.clone())
    }

    fn drop_presentation(&self, context: TokenStream2) -> TokenStream2 {
        (self.drop_presenter)(context, self.destroy_presentation())
    }

    fn doc_presentation(&self) -> TokenStream2 {
        (self.doc_presenter)(self.target_name.clone())
    }

    fn interface_presentation(&self, destroy_code: TokenStream2) -> TokenStream2 {
        (self.interface_presenter)(self.ffi_name.clone(), self.target_name.clone(), self.compose_from(), self.compose_to(), destroy_code)
    }

    fn compose_from(&self) -> TokenStream2 {
        let context = (self.from_context_presenter)(self);
        let conversions = self.conversions_from();
        (self.from_conversions_composer)(context, conversions)
    }

    fn compose_to(&self) -> TokenStream2 {
        let context = (self.to_context_presenter)(self);
        let conversions = self.conversions_to();
        (self.to_conversions_composer)(context, conversions)
    }

    fn expand(&self, input: TokenStream2) -> TokenStream {
        let comment = self.doc_presentation();
        let ffi_converted_input = self.fields_from();
        let ffi_conversion_presentation = self.interface_presentation((self.destroy_code_context_presenter)(quote!()));
        let drop_presentation = self.drop_presentation(self.ffi_name.clone());

        expansion(
            input.to_token_stream(),
            comment,
            ffi_converted_input,
            ffi_conversion_presentation,
            drop_presentation
        )

    }
}

fn expansion(input: TokenStream2, comment: TokenStream2, ffi_converted_input: TokenStream2, ffi_conversion_presentation: TokenStream2, drop_presentation: TokenStream2) -> TokenStream {
    let expanded = quote! {
        #input
        #comment
        #ffi_converted_input
        #ffi_conversion_presentation
        #drop_presentation
    };
    println!("{}", expanded);
    expanded.into()
}

fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}

fn package() -> TokenStream2 {
    quote!(rs_ffi_interfaces)
}

fn interface() -> TokenStream2 {
    quote!(FFIConversion)
}

fn ffi() -> TokenStream2 {
    quote!(ffi)
}

fn obj() -> TokenStream2 {
    quote!(obj)
}

fn destroy() -> TokenStream2 {
    quote!(destroy)
}

fn ffi_from() -> TokenStream2 {
    quote!(ffi_from)
}

fn ffi_from_const() -> TokenStream2 {
    quote!(ffi_from_const)
}

fn ffi_from_opt() -> TokenStream2 {
    quote!(ffi_from_opt)
}

fn ffi_to() -> TokenStream2 {
    quote!(ffi_to)
}
fn ffi_to_const() -> TokenStream2 {
    quote!(ffi_to_const)
}

fn ffi_to_opt() -> TokenStream2 {
    quote!(ffi_to_opt)
}

fn boxed() -> TokenStream2 {
    quote!(boxed)
}

fn boxed_vec() -> TokenStream2 {
    quote!(boxed_vec)
}

fn unbox_any() -> TokenStream2 {
    quote!(unbox_any)
}

fn package_boxed() -> TokenStream2 {
    let package = package();
    let boxed = boxed();
    quote!(#package::#boxed)
}

fn package_unbox_any() -> TokenStream2 {
    let package = package();
    let unbox_any = unbox_any();
    quote!(#package::#unbox_any)
}

fn package_unbox_any_expression(expr: TokenStream2) -> TokenStream2 {
    let package_unbox_any = package_unbox_any();
    quote!(#package_unbox_any(#expr))
}

fn package_unbox_any_expression_terminated(expr: TokenStream2) -> TokenStream2 {
    let package_unbox_any_expr = package_unbox_any_expression(expr);
    quote!(#package_unbox_any_expr;)
}

fn package_unboxed_root() -> TokenStream2 {
    package_unbox_any_expression(ffi())
}

fn package_boxed_expression(expr: TokenStream2) -> TokenStream2 {
    let package_boxed = package_boxed();
    quote!(#package_boxed(#expr))
}

fn package_boxed_vec() -> TokenStream2 {
    let package = package();
    let boxed_vec = boxed_vec();
    quote!(#package::#boxed_vec)
}

fn package_boxed_vec_expression(expr: TokenStream2) -> TokenStream2 {
    let package_boxed_vec = package_boxed_vec();
    quote!(#package_boxed_vec(#expr))
}

fn iter_map_collect(iter: TokenStream2, mapper: TokenStream2) -> TokenStream2 {
    quote!(#iter.map(#mapper).collect())
}

fn define_field(l_value: TokenStream2, r_value: TokenStream2) -> TokenStream2 {
    quote!(#l_value: #r_value)
}
fn define_pub_field(l_value: TokenStream2, r_value: TokenStream2) -> TokenStream2 {
    define_field(quote!(pub #l_value), r_value)
}

fn define_lambda(l_value: TokenStream2, r_value: TokenStream2) -> TokenStream2 {
    quote!(#l_value => #r_value)
}

fn unwrap_or(field_path: TokenStream2, or: TokenStream2) -> TokenStream2 {
    quote!(#field_path.unwrap_or(#or))
}

fn deref_field_path(field_path: TokenStream2) -> TokenStream2 {
    quote!(*#field_path)
}

fn create_struct(name: TokenStream2, implementation: TokenStream2) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        pub struct #name #implementation
    }
}

fn ffi_vec_field_type(value_type: TokenStream2) -> TokenStream2 {
    quote!(*mut rs_ffi_interfaces::VecFFI<#value_type>)
}

fn ffi_map_field_type(key_type: TokenStream2, value_type: TokenStream2) -> TokenStream2 {
    quote!(*mut rs_ffi_interfaces::MapFFI<#key_type, #value_type>)
}

fn ffi_from_map_conversion(map_key_path: TokenStream2, key_index: TokenStream2, acc_type: TokenStream2, key_conversion: TokenStream2, value_conversion: TokenStream2) -> TokenStream2 {
    quote! {{
        let map = &*#map_key_path;
        (0..map.count).fold(#acc_type::new(), |mut acc, #key_index| {
            let key = #key_conversion;
            let value = #value_conversion;
            acc.insert(key, value);
            acc
        })
    }}
}

fn path_arguments_to_generic_arguments(arguments: &PathArguments) -> Vec<&GenericArgument> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => map_args(args),
        _ => unimplemented!("map_arguments: arguments: {:?} not supported", arguments)
    }
}

fn path_arguments_to_types(arguments: &PathArguments) -> Vec<&Type> {
    match path_arguments_to_generic_arguments(arguments)[..] {
        [GenericArgument::Type(value_type)] =>
            vec![value_type],
        [GenericArgument::Type(key_type), GenericArgument::Type(value_type, .. )] =>
            vec![key_type, value_type],
        _ => unimplemented!("map_types: unexpected args: {:?}", arguments)
    }
}

fn path_arguments_to_paths(arguments: &PathArguments) -> Vec<&Path> {
    match path_arguments_to_types(arguments)[..] {
        [Type::Path(TypePath { path, .. })] =>
            vec![path],
        [Type::Path(TypePath { path: path_keys, .. }), Type::Path(TypePath { path: path_values, .. })] =>
            vec![path_keys, path_values],
        _ => unimplemented!("map_types: unexpected args: {:?}", arguments)
    }
}
fn path_arguments_to_conversion_types(arguments: &PathArguments) -> Vec<ConversionType> {
    match path_arguments_to_paths(arguments)[..] {
        [path] =>
            vec![conversion_type_for_path(path)],
        [path_keys, path_values] =>
            vec![conversion_type_for_path(path_keys), conversion_type_for_path(path_values)],
        _ => panic!("path_arguments_to_conversion_type: Bad argunments {:?}", arguments)
    }
}

fn destroy_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    match &path_arguments_to_conversion_types(arguments)[..] {
        [ConversionType::Simple(..) | ConversionType::Complex(..)] => package_unbox_any_expression_terminated(field_path),
        [ConversionType::Vec(path)] => destroy_vec(path, field_path),
        _ => panic!("destroy_vec: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn unbox_vec(var: TokenStream2, field_path: TokenStream2, conversion: TokenStream2) -> TokenStream2 {
    quote!({
        let #var = #field_path;
        #conversion
    })
}

fn box_vec(var: TokenStream2, field_path: TokenStream2, values_conversion: TokenStream2) -> TokenStream2 {
    package_boxed_expression(quote!({
        let #var = #field_path;
        rs_ffi_interfaces::VecFFI { count: #var.len(), values: #values_conversion }
    }))
}
fn from_simple_vec_conversion(field_path: TokenStream2, field_type: TokenStream2) -> TokenStream2 {
    quote!(std::slice::from_raw_parts(#field_path.values as *const #field_type, #field_path.count).to_vec())
}

fn from_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    let ffi_from_conversion = ffi_from_conversion(quote!(*#field_path.values.add(i)));
    iter_map_collect(quote!((0..#field_path.count)), quote!(|i| #ffi_from_conversion))
}

fn from_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    let conversion = match &path_arguments_to_conversion_types(arguments)[..] {
        [ConversionType::Simple(path)] => from_simple_vec_conversion(quote!(vec), path.segments.last().unwrap().ident.to_token_stream()),
        [ConversionType::Complex(..)] => from_complex_vec_conversion(quote!(vec)),
        [ConversionType::Vec(path)] => from_vec_vec_conversion(&path.segments.last().unwrap().arguments),
        _ => panic!("from_vec_vec_conversion: Bad arguments {:?}", arguments)
    };
    let unbox_conversion = unbox_vec(quote!(vec), quote!(&**vec.values.add(i)), conversion);
    iter_map_collect(quote!((0..vec.count)), quote!(|i| #unbox_conversion))
}

fn to_simple_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    quote!(#field_path.clone())
}

fn to_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    let conversion = ffi_to_conversion(quote!(o));
    iter_map_collect(quote!(#field_path.into_iter()),  quote!(|o| #conversion))
}

fn to_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    let values_conversion = package_boxed_vec_expression(match &path_arguments_to_conversion_types(arguments)[..] {
        [ConversionType::Simple(..)] => to_simple_vec_conversion(quote!(vec)),
        [ConversionType::Complex(..)] => to_complex_vec_conversion(quote!(vec)),
        [ConversionType::Vec(Path { segments, .. })] => to_vec_vec_conversion(&segments.last().unwrap().arguments),
        _ => panic!("to_vec_conversion: bad arguments {:?}", arguments)
    });
    let boxed_conversion = box_vec(quote!(vec), quote!(o), values_conversion);
    iter_map_collect(quote!(vec.into_iter()), quote!(|o| #boxed_conversion))
}


fn from_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    let conversion = match &path_arguments_to_conversion_types(arguments)[..] {
        [ConversionType::Simple(path)] => from_simple_vec_conversion(quote!(vec), path.segments.last().unwrap().ident.to_token_stream()),
        [ConversionType::Complex(..)] => from_complex_vec_conversion(quote!(vec)),
        [ConversionType::Vec(path)] => from_vec_vec_conversion(&path.segments.last().unwrap().arguments),
        [ConversionType::Map(..)] => panic!("from_vec (Map): Unknown field {:?} {:?}", field_path, arguments),
        _ => panic!("from_vec: Bad arguments {:?} {:?}", field_path, arguments)
    };
    let unbox_conversion = unbox_vec(quote!(vec), quote!(&*#field_path), conversion);
    unbox_conversion
}
fn destroy_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    match path_arguments_to_paths(arguments)[..] {
        [_path_keys, _path_values] => match conversion_type_for_path(path) {
            ConversionType::Simple(..) => package_unbox_any_expression_terminated(field_path),
            ConversionType::Complex(..) => package_unbox_any_expression_terminated(field_path),
            ConversionType::Vec(..) => destroy_vec(path, quote!(#field_path)),
            ConversionType::Map(..) => package_unbox_any_expression_terminated(field_path),
        },
        _ => panic!("destroy_map: Bad arguments {:?} {:?}", field_path, arguments)
    }

}

fn from_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let field_type = &last_segment.ident;
    let arguments = &last_segment.arguments;
    match path_arguments_to_paths(arguments)[..] {
        [inner_path_key_path, inner_path_value_path] => {
            let key_index = quote!(i);
            let simple_conversion = |buffer: TokenStream2| quote!(#buffer.add(#key_index));
            let key_simple_conversion = simple_conversion(quote!(*map.keys));
            let value_simple_conversion = simple_conversion(quote!(*map.values));
            let key_conversion = match conversion_type_for_path(inner_path_key_path) {
                ConversionType::Simple(..) => key_simple_conversion,
                ConversionType::Complex(..) => ffi_from_conversion(key_simple_conversion),
                ConversionType::Vec(path) => from_vec(&path, quote!(*map.values.add(#key_index))),
                ConversionType::Map(..) => panic!("Map not supported as Map key")
            };
            let inner_path_value_path_last_segment = inner_path_value_path.segments.last().unwrap();
            let value_conversion = match conversion_type_for_path(inner_path_value_path) {
                ConversionType::Simple(..) => value_simple_conversion,
                ConversionType::Complex(..) => ffi_from_conversion(value_simple_conversion),
                ConversionType::Vec(path) => from_vec(&path, quote!(*map.values.add(#key_index))),
                ConversionType::Map(..) => {
                    let field_type = &inner_path_value_path_last_segment.ident;
                    match path_arguments_to_paths(&inner_path_value_path_last_segment.arguments)[..] {
                        [inner_path_key_path, inner_path_value_path] => {
                            let key_index = quote!(i);
                            let simple_conversion = |buffer: TokenStream2| quote!(#buffer.add(#key_index));
                            let key_simple_conversion = simple_conversion(quote!(*map.keys));
                            let value_simple_conversion = simple_conversion(quote!(*map.values));

                            let converter = |inner_conversion: TokenStream2, inner_path: &Path| match conversion_type_for_path(inner_path) {
                                ConversionType::Simple(..) => inner_conversion,
                                ConversionType::Complex(..)  => ffi_from_conversion(inner_conversion),
                                ConversionType::Vec(path) => from_vec(&path, quote!(*map.values.add(#key_index))),
                                ConversionType::Map(..) => panic!("Vec/Map not supported as Map key")
                            };

                            let key_conversion = converter(key_simple_conversion, inner_path_key_path);
                            let value_conversion = converter(value_simple_conversion, inner_path_value_path);
                            let ccc = simple_conversion(quote!(map.values));
                            ffi_from_map_conversion(quote!(((*#ccc))), key_index, quote!(#field_type), key_conversion, value_conversion)
                        },
                        _ => panic!("from_map: Unknown field {:?} {:?}", field_path, arguments)
                    }
                }
            };
            ffi_from_map_conversion(quote!(#field_path), key_index, quote!(#field_type), key_conversion, value_conversion)
        },
        _ => panic!("from_map: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

// TODO: doesn't work
fn destroy_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let arguments = &last_segment.arguments;
    match path_arguments_to_paths(arguments)[..] {
        [path] => match path.segments.last() {
            Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                // std convertible
                // TODO: what to use? 0 or ::MAX
                "i8" | "u8" | "i16" | "u16" |
                "i32" | "u32" | "i64" | "u64" |
                "i128" | "u128" | "isize" | "usize" => quote!({}),
                // TODO: mmm shit that's incorrect
                "bool" => quote!({}),
                "Vec" => {
                    let conversion = destroy_vec(path, field_path.clone());
                    quote!(if !#field_path.is_null() { #conversion; })
                },
                _ => {
                    let conversion = package_unbox_any_expression_terminated(field_path.clone());
                    quote!(if !#field_path.is_null() { #conversion })
                }
            },
            _ => panic!("from_option: Unknown field {:?} {:?}", field_path, arguments)
        },
        _ => panic!("from_option: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

// TODO: Option<Map>
fn from_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let arguments = &last_segment.arguments;
    match path_arguments_to_paths(arguments)[..] {
        [path] => match path.segments.last() {
            Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                // std convertible
                // TODO: what to use? 0 or ::MAX
                "i8" | "u8" | "i16" | "u16" |
                "i32" | "u32" | "i64" | "u64" |
                "i128" | "u128" | "isize" | "usize" => quote!((#field_path > 0).then_some(#field_path)),
                // TODO: mmm shit that's incorrect
                "bool" => quote!((#field_path).then_some(#field_path)),
                "Vec" => {
                    let conversion = from_vec(path, field_path.clone());
                    quote!((!#field_path.is_null()).then_some(#conversion))
                },
                _ => ffi_from_opt_conversion(field_path)
            },
            _ => panic!("from_option: Bad arguments {:?} {:?}", field_path, arguments)
        },
        _ => panic!("from_option: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn from_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) => {
            let last_segment = segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "u8" => deref_field_path(field_path),
                _ => panic!("from_array: unsupported ident {:?} {:?}", field_path, last_segment.ident)
            }
        },
        _ => panic!("from_array: unsupported {:?} {:?}", field_path, type_array.elem)
    }
}

fn destroy_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(_type_path) => package_unbox_any_expression(quote!(#field_path)),
        _ => panic!("from_array: unsupported {:?} {:?}", field_path, type_array.elem)
    }
}

fn destroy_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => quote!({}),
        "VarInt" => quote!({}),
        "Option" => destroy_option(path, field_path),
        "Vec" => destroy_vec(path, field_path),
        "BTreeMap" | "HashMap" => destroy_map(path, field_path),
        "str" => destroy_conversion(field_path, convert_path_to_ffi_type(path), quote!(&#path)),
        _ => destroy_conversion(field_path, convert_path_to_ffi_type(path), quote!(#path))
    }
}

fn from_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => field_path,
        "VarInt" => quote!(#path(#field_path)),
        "Option" => from_option(path, field_path),
        "Vec" => from_vec(path, field_path),
        "BTreeMap" | "HashMap" => from_map(path, field_path),
        _ => ffi_from_conversion(field_path)
    }
}

fn destroy_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => destroy_ptr(field_path, type_ptr),
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, Some(type_ptr)),
        // _ => destroy_conversion(field_path)
        _ => panic!("Can't destroy_ptr: of type: {:?}", type_ptr)
    }
}

fn from_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => from_ptr(field_path, type_ptr),
        Type::Path(type_path) => from_path(field_path, &type_path.path, Some(type_ptr)),
        _ => ffi_from_conversion(field_path)
    }
}

fn destroy_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, None),
        _ => panic!("from_reference: unsupported type: {:?} {:?}", field_path, type_reference)
    }
}

fn from_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => from_path(field_path, &type_path.path, None),
        _ => panic!("from_reference: unsupported type: {:?} {:?}", field_path, type_reference)
    }
}


fn map_args(args: &Punctuated<GenericArgument, Comma>) -> Vec<&GenericArgument> {
    args.iter().collect::<Vec<_>>()
}

fn to_vec_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    let conversion = match &path_arguments_to_conversion_types(arguments)[..] {
        [ConversionType::Simple(..)] => to_simple_vec_conversion(quote!(vec)),
        [ConversionType::Complex(..)] => to_complex_vec_conversion(quote!(vec)),
        [ConversionType::Vec(path)] => to_vec_vec_conversion(&path.segments.last().unwrap().arguments),
        _ => panic!("to_vec_conversion: Map nested in Vec not supported yet"),
    };
    let values_conversion = package_boxed_vec_expression(conversion);
    let boxed_conversion = box_vec(quote!(vec), field_path, values_conversion);
    boxed_conversion
}

fn to_map_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    package_boxed_expression(match path_arguments_to_paths(arguments)[..] {
        [inner_path_key, inner_path_value] => {
            let mapper = |field_path: TokenStream2, path: &Path| {
                let conversion = match conversion_type_for_path(path) {
                    ConversionType::Simple(..) => field_path,
                    ConversionType::Complex(..) => ffi_to_conversion(field_path),
                    ConversionType::Vec(path) => to_vec_conversion(field_path, &path.segments.last().unwrap().arguments),
                    ConversionType::Map(path) => to_map_conversion(field_path, &path.segments.last().unwrap().arguments)
                };
                quote!(|o| #conversion)
            };
            let key_mapper = mapper(quote!(o), inner_path_key);
            let value_mapper = mapper(quote!(o),inner_path_value);
            let keys_conversion = package_boxed_vec_expression(quote!(map.keys().cloned().map(#key_mapper).collect()));
            let values_conversion = package_boxed_vec_expression(quote!(map.values().cloned().map(#value_mapper).collect()));
            quote!({let map = #field_path; rs_ffi_interfaces::MapFFI { count: map.len(), keys: #keys_conversion, values: #values_conversion }})
        },
        _ => panic!("to_map_conversion: Bad arguments {:?} {:?}", field_path, arguments)
    })
}

fn to_option_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    match path_arguments_to_paths(arguments)[..] {
        [inner_path] => {
            let last_segment = inner_path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                // TODO: MAX/MIN? use optional primitive?
                "i8" | "u8" | "i16" | "u16" |
                "i32" | "u32" | "i64" | "u64" |
                "i128" | "u128" | "isize" | "usize" => unwrap_or(field_path, quote!(0)),
                "bool" => unwrap_or(field_path, quote!(false)),
                "Vec" => match path_arguments_to_paths(&last_segment.arguments)[..] {
                    [path] => {
                        let transformer = match conversion_type_for_path(path) {
                            ConversionType::Simple(..) => quote!(clone()),
                            ConversionType::Complex(..) => {
                                let mapper = package_boxed_expression(ffi_to_conversion(quote!(o)));
                                iter_map_collect(quote!(iter()), quote!(|o| #mapper))
                            },
                            ConversionType::Map(..) => panic!("define_option: Map nested in Vec not supported yet"),
                            ConversionType::Vec(..) => panic!("define_option: Vec nested in Vec not supported yet"),
                        };
                        MATCH_FIELDS_PRESENTER((field_path, vec![
                            LAMBDA_CONVERSION_PRESENTER(quote!(Some(vec)), package_boxed_expression(quote!(rs_ffi_interfaces::VecFFI::new(vec.#transformer)))),
                            LAMBDA_CONVERSION_PRESENTER(quote!(None), quote!(std::ptr::null_mut())),
                        ]))
                    },
                    _ => panic!("to_option_conversion: Unknown args {:?}", last_segment)
                },
                _ => ffi_to_opt_conversion(field_path)
            }
        },
        _ => panic!("to_option_conversion: Bad arguments {:?}", arguments)
    }
}

fn to_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => field_path,
        "VarInt" => quote!(#field_path.0),
        "Vec" => to_vec_conversion(field_path, &last_segment.arguments),
        "BTreeMap" | "HashMap" => to_map_conversion(field_path, &last_segment.arguments),
        "Option" => to_option_conversion(field_path, &last_segment.arguments),
        _ => ffi_to_conversion(field_path)
    }
}

fn to_vec_ptr(ident: TokenStream2, _type_ptr: &TypePtr, _type_arr: &TypeArray) -> TokenStream2 {
    let expr = package_boxed_expression(quote!(o));
    package_boxed_vec_expression(iter_map_collect(OBJ_FIELD_NAME(ident), quote!(|o| #expr)))
}
fn destroy_conversion(field_value: TokenStream2, ffi_type: TokenStream2, field_type: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let destroy = destroy();
    quote!(<#ffi_type as #package::#interface<#field_type>>::#destroy(#field_value))
}

fn ffi_from_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_from = ffi_from();
    quote!(#package::#interface::#ffi_from(#field_value))
}

fn ffi_to_conversion(field_path: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_to = ffi_to();
    quote!(#package::#interface::#ffi_to(#field_path))
}

fn ffi_from_opt_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_from_opt = ffi_from_opt();
    quote!(#package::#interface::#ffi_from_opt(#field_value))
}

fn ffi_to_opt_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_to_opt = ffi_to_opt();
    quote!(#package::#interface::#ffi_to_opt(#field_value))
}

fn to_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Array(TypeArray { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
            _ => panic!("to_pointer: Unknown field (arr->) {:?} {:?}", field_path, elem),
        },
        Type::Ptr(TypePtr { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
            // Type::Ptr(type_ptr) => to_vec_ptr(f, type_ptr),
            Type::Array(type_arr) => to_vec_ptr(field_path, type_ptr, type_arr),
            _ => panic!("to_pointer: Unknown field (ptr->) {:?} {:?}", field_path, elem),
        },
        Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
        _ => panic!("to_pointer: Unknown field (path->) {:?} {:?}", field_path, type_ptr.elem),
    }
}

fn to_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => to_path(field_path, &type_path.path, None),
        _ => panic!("to_reference: Unknown field {:?} {:?}", field_path, type_reference.elem)
    }
}

fn to_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(type_path) => to_path(package_boxed_expression(field_path), &type_path.path, None),
        _ => panic!("to_array: Unknown field {:?} {:?}", field_path, type_array.elem)
    }
}

fn conversion_type_for_path(path: &Path) -> ConversionType {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => ConversionType::Simple(path.clone()),
        "BTreeMap" | "HashMap" => ConversionType::Map(path.clone()),
        "Vec" => ConversionType::Vec(path.clone()),
        _ => ConversionType::Complex(path.clone())
    }
}

fn convert_path_to_ffi_type(path: &Path) -> TokenStream2 {
    let mut cloned_segments = path.segments.clone();
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let field_type = &last_segment.ident;
    match field_type.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => quote!(#field_type),
        "str" | "String" => quote!(std::os::raw::c_char),
        "UInt128" => quote!([u8; 16]),
        "UInt160" => quote!([u8; 20]),
        "UInt256" => quote!([u8; 32]),
        "UInt384" => quote!([u8; 48]),
        "UInt512" => quote!([u8; 64]),
        "UInt768" => quote!([u8; 96]),
        "VarInt" => quote!(u64),
        _ => {
            last_segment.ident = Ident::new(&format!("{}FFI", last_segment.ident), last_segment.ident.span());
            let field_type = cloned_segments.into_iter().map(|segment| quote_spanned! { segment.span() => #segment }).collect::<Vec<_>>();
            let full_path = quote!(#(#field_type)::*);
            quote!(#full_path)
        }
    }
}

fn convert_path_to_field_type(path: &Path) -> TokenStream2 {
    let mut cloned_segments = path.segments.clone();
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let field_type = &last_segment.ident;
    match field_type.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => quote!(#field_type),
        "str" | "String" => quote!(*mut std::os::raw::c_char),
        "UInt128" => quote!(*mut [u8; 16]),
        "UInt160" => quote!(*mut [u8; 20]),
        "UInt256" => quote!(*mut [u8; 32]),
        "UInt384" => quote!(*mut [u8; 48]),
        "UInt512" => quote!(*mut [u8; 64]),
        "UInt768" => quote!(*mut [u8; 96]),
        "VarInt" => quote!(u64),
        _ => {
            last_segment.ident = Ident::new(&format!("{}FFI", last_segment.ident), last_segment.ident.span());
            let field_type = cloned_segments.into_iter().map(|segment| quote_spanned! { segment.span() => #segment }).collect::<Vec<_>>();
            let full_path = quote!(#(#field_type)::*);
            quote!(*mut #full_path)
        }
    }
}

fn ffi_struct_name(field_type: &Ident) -> Ident {
    format_ident!("{}FFI", field_type)
}

fn extract_map_arg_type(path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "BTreeMap" | "HashMap" => match path_arguments_to_paths(&last_segment.arguments)[..] {
            [path_keys, path_values] =>
                ffi_map_field_type(extract_map_arg_type(path_keys), extract_map_arg_type(path_values)),
            _ => panic!("convert_map_arg_type: Unknown args {:?}", last_segment)
        },
        "Vec" => match path_arguments_to_paths(&last_segment.arguments)[..] {
            [path] =>
                ffi_vec_field_type(extract_vec_arg_type(path)),
            _ => panic!("extract_vec_arg_type: Unknown args {:?}", path)
        },
        _ => convert_path_to_field_type(path)
    }
}

fn extract_vec_arg_type(path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "Vec" => match path_arguments_to_paths(&last_segment.arguments)[..] {
            [path] =>
                ffi_vec_field_type(extract_vec_arg_type(path)),
            _ => panic!("extract_vec_arg_type: Unknown args {:?}", path)
        },
        _ => convert_path_to_field_type(path)
    }
}

fn extract_struct_field(field_type: &Type) -> TokenStream2 {
    match field_type {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            let arguments = &last_segment.arguments;
            match last_segment.ident.to_string().as_str() {
                "Vec" => match path_arguments_to_paths(arguments)[..] {
                    [path] => ffi_vec_field_type(extract_vec_arg_type(path)),
                    _ => panic!("extract_struct_field: Vec: arguments: {:?} not supported", arguments)
                },
                "BTreeMap" | "HashMap" => match path_arguments_to_paths(arguments)[..] {
                    [path_keys, path_values] =>
                        ffi_map_field_type(extract_map_arg_type(path_keys), extract_map_arg_type(path_values)),
                    _ => panic!("extract_struct_field: Map: arguments: {:?} not supported", arguments)
                },
                "Option" => match path_arguments_to_types(arguments)[..] {
                    [field_type] => extract_struct_field(field_type),
                    _ => panic!("extract_struct_field: Option: {:?} not supported", arguments)
                },
                "OpaqueContext" => quote!(rs_ffi_interfaces::OpaqueContextFFI),
                "OpaqueContextMut" => quote!(rs_ffi_interfaces::OpaqueContextMutFFI),
                _ => convert_path_to_field_type(path),
            }
        },
        Type::Array(TypeArray { elem, len, .. }) => {
            quote!(*mut [#elem; #len])
        },
        Type::Reference(TypeReference { elem, .. }) => extract_struct_field(&**elem),
        // Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
        //     match &**elem {
        //         Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
        //             "c_void" => match (const_token, mutability) {
        //                 (Some(const_token), None) => quote!(OpaqueContextFFI),
        //                 (None, Some(mut_token)) => quote!(OpaqueContextMutFFI),
        //                 _ => panic!("extract_struct_field: c_void with {:?} {:?} not supported", const_token, mutability)
        //             },
        //             ptr => panic!("extract_struct_field: ptr {:?} not supported {:?} {:?} {:?}", ptr, star_token, const_token, mutability)
        //         },
        //         _ => panic!("extract_struct_field: {:?} not supported", elem)
        //     }
        _ => panic!("extract_struct_field: field type {:?} not supported", field_type)
    }
}





fn impl_interface(ffi_name: TokenStream2, target_name: TokenStream2, ffi_from_conversion: TokenStream2, ffi_to_conversion: TokenStream2, destroy_code: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi = ffi();
    let obj = obj();
    // let ffi_from = ffi_from();
    let ffi_from_const = ffi_from_const();
    // let ffi_to = ffi_to();
    let ffi_to_const = ffi_to_const();
    // let ffi_from_opt = ffi_from_opt();
    // let ffi_to_opt = ffi_to_opt();

    quote! {
        impl #package::#interface<#target_name> for #ffi_name {

            unsafe fn #ffi_from_const(#ffi: *const #ffi_name) -> #target_name { #ffi_from_conversion }
            unsafe fn #ffi_to_const(#obj: #target_name) -> *const #ffi_name { #ffi_to_conversion }
            // unsafe fn #ffi_from(#ffi: *mut #ffi_name) -> #target_name { #ffi_from_conversion }
            // unsafe fn #ffi_to(#obj: #target_name) -> *mut #ffi_name { #ffi_to_conversion }
            // unsafe fn #ffi_from_opt(#ffi: *mut #ffi_name) -> Option<#target_name> {
            //     (!#ffi.is_null()).then_some(<Self as #package::#interface<#target_name>>::#ffi_from(#ffi))
            // }
            // unsafe fn #ffi_to_opt(#obj: Option<#target_name>) -> *mut #ffi_name {
            //     #obj.map_or(std::ptr::null_mut(), |o| <Self as #package::#interface<#target_name>>::#ffi_to(o))
            // }
            unsafe fn destroy(#ffi: *mut #ffi_name) {
                #destroy_code;
            }
        }
    }
}

fn impl_drop(ffi_name: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    quote! { impl Drop for #ffi_name { fn drop(&mut self) { unsafe { #drop_code } } } }
}

fn from_enum(data_enum: &DataEnum, target_name: Ident, input: &DeriveInput) -> TokenStream {
    let variants = &data_enum.variants;
    let variants_count = variants.len();
    let ffi_name = ffi_struct_name(&target_name);
    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_fields = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut destroy_fields = Vec::<TokenStream2>::new();
    variants.iter().for_each(|Variant { ident: variant_name, fields, discriminant, ..}| {
        let target_variant_path = quote!(#target_name::#variant_name);
        let ffi_variant_path = quote!(#ffi_name::#variant_name);

        let composer = match fields {
            Fields::Unit => PresentationComposer::enum_unit_variant_composer(quote!(#ffi_variant_path), quote!(#target_variant_path), ROOT_DESTROY_CONTEXT_PRESENTER),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => PresentationComposer::enum_unnamed_variant_composer(
                quote!(#ffi_variant_path),
                quote!(#target_variant_path),
                ROOT_DESTROY_CONTEXT_PRESENTER,
                unnamed.iter().enumerate().map(|(index, Field { ty, .. })| (ty, format_ident!("o_{}", index).to_token_stream()))
            ),
            Fields::Named(FieldsNamed { named, .. }) => PresentationComposer::enum_named_variant_composer(
                quote!(#ffi_variant_path),
                quote!(#target_variant_path),
                ROOT_DESTROY_CONTEXT_PRESENTER,
                named.iter().map(|Field { ident, ty, .. }| (ty, ident.clone().unwrap().to_token_stream()))
            )
        };
        let (variants_presenter, fields) = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => (ENUM_UNIT_FIELDS_PRESENTER, vec![quote!(#lit)]),
            None => match fields {
                Fields::Unit => (NO_FIELDS_PRESENTER, vec![]),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (ENUM_UNNAMED_VARIANT_PRESENTER, unnamed.iter().map(|field: &Field| extract_struct_field(&field.ty)).collect::<Vec<_>>()),
                Fields::Named(FieldsNamed { named, .. }) => (ENUM_NAMED_VARIANT_PRESENTER, named.iter().map(|field: &Field| NAMED_CONVERSION_PRESENTER(field.ident.clone().unwrap().to_token_stream(), extract_struct_field(&field.ty))).collect::<Vec<_>>())
            },
            _ => panic!("Error variant discriminant")
        };
        variants_fields.push(variants_presenter((quote!(#variant_name), fields)));
        conversions_from_ffi.push(composer.compose_from());
        conversions_to_ffi.push(composer.compose_to());
        destroy_fields.push(composer.drop_presentation(composer.fields_from()));
    });

    // let from_context_presenter = |_| quote!(&*ffi);
    let comment = DEFAULT_DOC_PRESENTER(quote!(#target_name));
    let ffi_converted_input = ENUM_PRESENTER((quote!(#ffi_name), variants_fields));
    let from_conversions = MATCH_FIELDS_PRESENTER((quote!(ffi_ref), conversions_from_ffi));
    let to_conversions = MATCH_FIELDS_PRESENTER((quote!(obj), conversions_to_ffi));
    let ffi_from_conversion = FFI_FROM_CONVERSION_PRESENTER(quote!(&*ffi), from_conversions);
    let ffi_to_conversion = FFI_TO_CONVERSION_PRESENTER(quote!(), to_conversions);
    let ffi_conversion_presentation = DEFAULT_INTERFACE_PRESENTER(quote!(#ffi_name), quote!(#target_name), ffi_from_conversion, ffi_to_conversion, package_unboxed_root());
    let drop_presentation = DROP_INTERFACE_PRESENTER(quote!(#ffi_name), ENUM_DESTROY_PRESENTER(destroy_fields));

    expansion(
        input.to_token_stream(),
        comment,
        ffi_converted_input,
        ffi_conversion_presentation,
        drop_presentation
    )
}


#[proc_macro_attribute]
pub fn impl_ffi_conv(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attrs = parse_macro_input!(attr as AttributeArgs);
    let target_name = match attrs.first() {
        Some(NestedMeta::Lit(literal)) => format_ident!("{}", literal.to_token_stream().to_string()),
        Some(NestedMeta::Meta(Meta::Path(path))) => path.segments.first().unwrap().ident.clone(),
        _ => {
            // use default rules
            // for unnamed structs like UInt256 -> #target_name = [u8; 32]
            // for named structs -> generate ($StructName)FFI
            input.ident.clone()
        },
    };
    match input.data {
        Data::Struct(DataStruct { fields: ref f, ..}) => {
            let composer = match f {
                Fields::Unnamed(ref fields) => match target_name.clone().to_string().as_str() {
                    // Hack used to simplify some structures
                    // Main problem here that without special dictionary of predefined non-std structures
                    // we unable to filter out structures and provide them conversions when they are used as field types inside parent structures
                    // Solution would be to write build script to preprocess and collect dictionary before macro expansion
                    // in order to match struct field types with this predefined dictionary
                    "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" | "VarInt" => {
                        let (ffi_name,
                            ffi_from_presenter,
                            ffi_from_presentation_context,
                            ffi_to_presenter,
                            ffi_to_presentation_context,
                            destroy_code_context_presenter) = match fields.unnamed.first().unwrap().ty.clone() {
                            // VarInt
                            Type::Path(TypePath { path, .. }) => {
                                let ffi_name = ffi_struct_name(&target_name);
                                (
                                    quote!(#ffi_name),
                                    CURLY_BRACES_FIELDS_PRESENTER,
                                    vec![from_path(quote!(ffi_ref.0), &path, None)],
                                    CURLY_BRACES_FIELDS_PRESENTER,
                                    quote!(#ffi_name),
                                    ROOT_DESTROY_CONTEXT_PRESENTER,
                                )
                            },
                            // UInt256 etc
                            Type::Array(type_array) => (
                                quote!(#type_array),
                                ROUND_BRACES_FIELDS_PRESENTER,
                                vec![quote!(ffi_ref)],
                                NO_FIELDS_PRESENTER,
                                quote!(obj.0),
                                EMPTY_MAP_PRESENTER
                            ),
                            _ => unimplemented!("from_unnamed_struct: not supported {:?}", fields.unnamed.first().unwrap().ty)
                        };
                        PresentationComposer::primitive_composer(
                                quote!(#ffi_name),
                                quote!(#target_name),
                                EMPTY_FIELDS_PRESENTER,
                                ffi_from_presenter,
                                ffi_to_presenter,
                                destroy_code_context_presenter,
                                EMPTY_PAIR_PRESENTER,
                                ffi_from_presentation_context,
                                ffi_to_presentation_context,
                        )
                    },
                    _ =>
                        PresentationComposer::struct_composer(
                            ffi_struct_name(&target_name).to_token_stream(),
                            quote!(#target_name),
                            |_| quote!(&*ffi),
                            UNNAMED_STRUCT_PRESENTER,
                            DEFAULT_FIELD_TYPE_PRESENTER,
                            ROOT_DESTROY_CONTEXT_PRESENTER,
                            ROUND_BRACES_FIELDS_PRESENTER,
                            SIMPLE_CONVERSION_PRESENTER,
                            fields.unnamed.iter().enumerate().map(|(index, Field { ty, .. })| (ty, usize_to_tokenstream(index)))
                        ),
                },
                Fields::Named(ref fields) => PresentationComposer::struct_composer(
                        ffi_struct_name(&input.ident).to_token_stream(),
                        quote!(#target_name),
                        |_| quote!(&*ffi),
                        NAMED_STRUCT_PRESENTER,
                        NAMED_FIELD_TYPE_PRESENTER,
                        ROOT_DESTROY_CONTEXT_PRESENTER,
                        CURLY_BRACES_FIELDS_PRESENTER,
                        NAMED_CONVERSION_PRESENTER,
                        fields.named.iter().map(|Field { ident, ty, .. }| (ty, quote!(#ident)))
                ),
                Fields::Unit => panic!("Fields::Unit is not supported yet"),
            };
            composer.expand(input.to_token_stream())
        },
        Data::Enum(ref data_enum) => from_enum(data_enum, target_name, &input),
        Data::Union(ref _data_union) => panic!("Union is not supported yet")
    }
}

#[proc_macro_attribute]
pub fn impl_ffi_ty_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let (target_name, alias_to) = match &input {
        Item::Type(ItemType { ident, ty, .. }) => (ident, ty),
        _ => panic!("Expected a type alias"),
    };
    let ffi_name = format_ident!("{}FFI", target_name);
    let composer = PresentationComposer::type_alias_composer(
        quote!(#ffi_name),
        quote!(#target_name),
        TYPE_ALIAS_PRESENTER,
        DEFAULT_FIELD_TYPE_PRESENTER,
        |_| quote!(&*ffi),
        |_| quote!(obj),
        FFI_FROM_CONVERSION_PRESENTER,
        FFI_TO_CONVERSION_PRESENTER,
        TYPE_ALIAS_CONVERSION_FROM_PRESENTER,
        TYPE_ALIAS_CONVERSION_TO_PRESENTER,
        SIMPLE_CONVERSION_PRESENTER,
        STRUCT_DESTROY_PRESENTER,
        ROOT_DESTROY_CONTEXT_PRESENTER,
        IntoIterator::into_iter(vec![(&*alias_to.clone(), match &*alias_to.clone() {
            Type::Path(TypePath { path, .. }) => match conversion_type_for_path(path) {
                ConversionType::Simple(..) => obj(),
                _ => usize_to_tokenstream(0),
            },
            Type::Array(_type_array) => usize_to_tokenstream(0),
            Type::Ptr(_type_ptr) => obj(),
            _ => unimplemented!("from_type_alias: not supported {:?}", &alias_to),
        })])
    );
    composer.expand(input.to_token_stream())
}

#[proc_macro_attribute]
pub fn impl_ffi_fn_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let signature = &input_fn.sig;
    let fn_name = &signature.ident;
    let ffi_fn_name = format_ident!("ffi_{}", fn_name);
    let obj = obj();
    let output_type = match &signature.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, field_type) => extract_struct_field(&field_type),
    };

    let args_converted = signature.inputs.iter().map(|arg| match arg {
            FnArg::Typed(PatType { ty, pat, .. }) =>
                NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), extract_struct_field(&ty)),
            _ => panic!("Arg type {:?} not supported", arg)
        }).collect::<Vec<_>>();

    let args_conversions = signature.inputs.iter().map(|arg| match *arg {
        FnArg::Typed(ref pat_type) => match (&*pat_type.ty, &*pat_type.pat) {
            (Type::Path(TypePath { path, .. }), Pat::Ident(PatIdent { ident, .. })) =>
                from_path(quote!(#ident), path, None),
            _ => panic!("error: arg conversion: {:?}", pat_type.ty)
        },
        _ => panic!("Expected typed function argument")
    }).collect::<Vec<_>>();

    let comment = SAFETY_DOC_PRESENTER(fn_name.to_token_stream());
    expansion(
        input_fn.to_token_stream(),
        comment,
        {
            let output_conversion = match &signature.output {
                ReturnType::Default => quote! { ; },
                ReturnType::Type(_, field_type) => match &**field_type {
                    Type::Path(TypePath { path, .. }) => to_path(obj.clone(), path, None),
                    _ => panic!("error: output conversion: {:?}", field_type),
                },
            };
            let name_and_arguments = ROUND_BRACES_FIELDS_PRESENTER((quote!(#ffi_fn_name), args_converted));
            let conversions = ROUND_BRACES_FIELDS_PRESENTER((quote!(#fn_name), args_conversions));
            quote! {
                #[no_mangle]
                pub unsafe extern "C" fn #name_and_arguments -> #output_type {
                    let #obj = #conversions;
                    #output_conversion
                }
            }
        },
        quote!(),
        quote!(),
    )
}

fn doc(target_name: String) -> TokenStream2 {
    let comment = format!("FFI-representation of the {}", target_name);
    parse_quote! { #[doc = #comment] }
}

#[proc_macro_attribute]
pub fn impl_ffi_callback(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    let (ffi_callback_name, output_expr, args_converted) = match &input {
        Item::Type(ItemType { ident, ty, .. }) => match *ty.clone() {
            Type::BareFn(TypeBareFn { inputs, output, .. }) => (
                format_ident!("{}FFI", ident),
                match output {
                    ReturnType::Default => quote!(),
                    ReturnType::Type(_, field_type) => {
                        let field = extract_struct_field(&field_type);
                        quote!(-> #field)
                    },
                }, inputs.iter()
                    .map(|BareFnArg { ty: field_type, name, .. }|
                        NAMED_CONVERSION_PRESENTER(name.clone().unwrap().0.to_token_stream(), extract_struct_field(field_type)))
                    .collect::<Vec<_>>()
            ),
            _ => unimplemented!("Expected a function"),
        },
        _ => unimplemented!("Expected a type alias"),
    };

    let comment = quote! {
        /// FFI Callback
    };
    expansion(
        input.to_token_stream(),
        comment,
        quote!(pub type #ffi_callback_name = unsafe extern "C" fn(#(#args_converted),*) #output_expr;),
        quote!(),
        quote!()
    )
}
