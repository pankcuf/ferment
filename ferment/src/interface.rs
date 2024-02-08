use syn::{parse_quote, Path, Type};
use quote::quote;
use syn::__private::TokenStream2;

use crate::conversion::FieldTypeConversion;
use crate::formatter::format_token_stream;
use crate::presentation::context::{OwnedItemPresenterContext, IteratorPresentationContext, OwnerIteratorPresentationContext};

/// token -> token
pub type MapPresenter = fn(field_name: TokenStream2) -> TokenStream2;
/// token + token -> token
pub type MapPairPresenter = fn(field_name: TokenStream2, conversion: TokenStream2) -> TokenStream2;

/// token + type + dictionary -> token
pub type ScopeTreeFieldTypedPresenter = fn(field_type: FieldTypeConversion) -> OwnedItemPresenterContext;
/// [token] -> token
pub type IteratorPresenter = fn(items: Vec<OwnedItemPresenterContext>) -> IteratorPresentationContext;

/// token + [token] -> token
pub type OwnerIteratorPresenter = fn((TokenStream2, Vec<OwnedItemPresenterContext>)) -> OwnerIteratorPresentationContext;

/// Map Presenters
pub const FFI_DEREF_FIELD_NAME: MapPresenter = |field_name| quote!(ffi_ref.#field_name);
pub const DEREF_FIELD_PATH: MapPresenter = |field_path| quote!(*#field_path);

pub const FROM_OFFSET_MAP_PRESENTER: MapPresenter = |field_path| quote!(#field_path.add(i));

pub const OBJ_FIELD_NAME: MapPresenter = |field_name| quote!(obj.#field_name);
pub const SIMPLE_PRESENTER: MapPresenter = |name| quote!(#name);
pub const SIMPLE_TERMINATED_PRESENTER: MapPresenter = |name| quote!(#name;);
pub const ROOT_DESTROY_CONTEXT_PRESENTER: MapPresenter = |_| package_unboxed_root();
pub const EMPTY_DESTROY_PRESENTER: MapPresenter = |_| quote!({});
pub const DEFAULT_DOC_PRESENTER: MapPresenter = |target_name: TokenStream2| {
    let comment = format!("FFI-representation of the [`{}`]", format_token_stream(&target_name));
    // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
    parse_quote! { #[doc = #comment] }
};


/// Map Pair Presenters
pub const SIMPLE_PAIR_PRESENTER: MapPairPresenter = |name, presentation|
    quote!(#name #presentation);
pub const SIMPLE_CONVERSION_PRESENTER: MapPairPresenter = |_, conversion|
    quote!(#conversion);
pub const NAMED_CONVERSION_PRESENTER: MapPairPresenter = |l_value, r_value|
    quote!(#l_value: #r_value);
pub const LAMBDA_CONVERSION_PRESENTER: MapPairPresenter = |l_value, r_value|
    quote!(#l_value => #r_value);
pub const FFI_FROM_ROOT_PRESENTER: MapPairPresenter = |field_path: TokenStream2, conversions: TokenStream2|
    quote!(let ffi_ref = #field_path; #conversions);
pub const FFI_TO_ROOT_PRESENTER: MapPairPresenter = |_, conversions: TokenStream2|
    package_boxed_expression(conversions);



/// Owner Iterator Presenters
///
pub const CURLY_BRACES_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    OwnerIteratorPresentationContext::CurlyBracesFields(name, fields);
pub const ROUND_BRACES_FIELDS_PRESENTER: OwnerIteratorPresenter = |(name, fields)|
    OwnerIteratorPresentationContext::RoundBracesFields(name, fields);

/// PathArguments Presenters

pub fn create_struct(name: TokenStream2, implementation: TokenStream2) -> TokenStream2 {
    let path: Path = parse_quote!(#name);
    let ident = &path.segments.last().unwrap().ident;
    quote! {
        #[repr(C)]
        #[derive(Clone)]
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

pub fn destroy_conversion(field_value: TokenStream2, ffi_type: Type, field_type: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let destroy = destroy();
    quote!(<#ffi_type as #package::#interface<#field_type>>::#destroy(#field_value))
}
