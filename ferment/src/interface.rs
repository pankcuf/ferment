use syn::{parse_quote, Path};
use quote::quote;
use syn::__private::TokenStream2;
use crate::composer::{OwnerIteratorConversionComposer, SimplePairConversionComposer, SimpleComposerPresenter, ComposerPresenter};

use crate::formatter::format_token_stream;
use crate::presentation::context::{IteratorPresentationContext, OwnerIteratorPresentationContext};

pub const EMPTY_PRESENTER: SimpleComposerPresenter = |_| quote!();
pub const SIMPLE_PRESENTER: SimpleComposerPresenter = |name| quote!(#name);
pub const SIMPLE_TERMINATED_PRESENTER: SimpleComposerPresenter = |name| quote!(#name;);
pub const ROOT_DESTROY_CONTEXT_COMPOSER: SimpleComposerPresenter = |_| package_unboxed_root();
pub const DEFAULT_DOC_PRESENTER: SimpleComposerPresenter = |target_name| {
    let comment = format!("FFI-representation of the [`{}`]", format_token_stream(&target_name));
    // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
    parse_quote! { #[doc = #comment] }
};


/// Map Pair Presenters
pub const SIMPLE_PAIR_PRESENTER: SimplePairConversionComposer = |(name, presentation)|
    quote!(#name #presentation);
// pub const SIMPLE_CONVERSION_PRESENTER: SimplePairConversionComposer = |(_, conversion)|
//     quote!(#conversion);
//
// pub const SIMPLE_CONVERSION_PRESENTER2: ComposerPresenter<(TokenStream2, IteratorPresentationContext), TokenStream2> = |(_, conversion)|
//     conversion;
// pub const SIMPLE_CONVERSION_PRESENTER3: ComposerPresenter<(TokenStream2, FieldTypePresentationContext), TokenStream2> = |(_, conversion)|
//     conversion;

// pub const NAMED_CONVERSION_PRESENTER: SimplePairConversionComposer = |(l_value, r_value)|
//     quote!(#l_value: #r_value);
// pub const NAMED_CONVERSION_PRESENTER_PASS: FieldTypePresentationContextPass = |(l_value, r_value)|
//     FieldTypePresentationContext::Named((l_value, Box::new(r_value)));
// pub const LAMBDA_CONVERSION_PRESENTER: SimplePairConversionComposer = |(l_value, r_value)|
//     quote!(#l_value => #r_value);
pub const LAMBDA_CONVERSION_PRESENTER2: ComposerPresenter<(TokenStream2, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> = |(l_value, r_value)|
    OwnerIteratorPresentationContext::Lambda(l_value, Box::new(r_value));
pub const LAMBDA_CONVERSION_PRESENTER3: ComposerPresenter<(TokenStream2, IteratorPresentationContext), IteratorPresentationContext> = |(l_value, r_value)|
    IteratorPresentationContext::Lambda(l_value, Box::new(r_value));
// pub const FFI_FROM_ROOT_PRESENTER: SimplePairConversionComposer = |(field_path, conversions)|
//     quote!(let ffi_ref = #field_path; #conversions);
pub const FFI_FROM_ROOT_PRESENTER: ComposerPresenter<(TokenStream2, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> = |(field_path, conversions)|
    OwnerIteratorPresentationContext::FromRoot(field_path, Box::new(conversions));

// pub const FFI_TO_ROOT_PRESENTER: SimplePairConversionComposer = |(_, conversions)|
//     package_boxed_expression(conversions);

pub const FFI_TO_ROOT_PRESENTER: ComposerPresenter<(TokenStream2, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> = |(_, conversions)|
    OwnerIteratorPresentationContext::Boxed(Box::new(conversions));



/// Owner Iterator Presenters
///
pub const CURLY_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer = |local_context|
    OwnerIteratorPresentationContext::CurlyBracesFields(local_context);
pub const ROUND_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer = |local_context|
    OwnerIteratorPresentationContext::RoundBracesFields(local_context);

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
