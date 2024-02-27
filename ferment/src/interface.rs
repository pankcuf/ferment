use std::convert::Into;
use syn::{parse_quote, Path};
use quote::quote;
use syn::__private::TokenStream2;
use syn::token::Comma;
use crate::composer::{OwnerIteratorConversionComposer, SimplePairConversionComposer, SimpleComposerPresenter, ComposerPresenter};

use crate::formatter::format_token_stream;
use crate::presentation::context::OwnerIteratorPresentationContext;

pub const ROOT_DESTROY_CONTEXT_COMPOSER: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext> =
    |_| OwnerIteratorPresentationContext::UnboxedRoot;
pub const DEFAULT_DOC_PRESENTER: SimpleComposerPresenter = |target_name| {
    let comment = format!("FFI-representation of the [`{}`]", format_token_stream(&target_name));
    // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
    parse_quote! { #[doc = #comment] }
};
pub const SIMPLE_PAIR_PRESENTER: SimplePairConversionComposer = |(name, presentation)|
    quote!(#name #presentation);
pub const FFI_FROM_ROOT_PRESENTER: ComposerPresenter<(OwnerIteratorPresentationContext, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> = |(field_path, conversions)|
    OwnerIteratorPresentationContext::FromRoot(field_path.into(), conversions.into());
pub const FFI_TO_ROOT_PRESENTER: ComposerPresenter<(OwnerIteratorPresentationContext, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> = |(_, conversions)|
    OwnerIteratorPresentationContext::Boxed(conversions.into());
pub const CURLY_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer<Comma> = |local_context|
    OwnerIteratorPresentationContext::CurlyBracesFields(local_context);
pub const ROUND_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer<Comma> = |local_context|
    OwnerIteratorPresentationContext::RoundBracesFields(local_context);

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

pub fn ffi_from_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    quote!(#package::#interface::ffi_from(#field_value))
}

pub fn ffi_to_conversion(field_path: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    quote!(#package::#interface::ffi_to(#field_path))
}
