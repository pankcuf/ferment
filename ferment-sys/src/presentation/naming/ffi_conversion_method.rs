use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use ferment_macro::MethodCall;
#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment::FFIConversionDestroy"]
pub enum FFIConversionDestroyMethod {
    Destroy,
}
impl ToTokens for FFIConversionDestroyMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIConversionDestroyMethod::Destroy => quote!(destroy)
        }.to_tokens(dst)
    }
}
#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment::FFIConversionTo"]
pub enum FFIConversionToMethod {
    Mut,
    Const,
    Opt,
}
impl ToTokens for FFIConversionToMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIConversionToMethod::Const => quote!(ffi_to_const),
            FFIConversionToMethod::Mut => quote!(ffi_to),
            FFIConversionToMethod::Opt => quote!(ffi_to_opt),
        }.to_tokens(dst)
    }
}
#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment::FFIConversionFrom"]
pub enum FFIConversionFromMethod {
    Const,
    Mut,
    Opt,
}
impl ToTokens for FFIConversionFromMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIConversionFromMethod::Const => quote!(ffi_from_const),
            FFIConversionFromMethod::Mut => quote!(ffi_from),
            FFIConversionFromMethod::Opt => quote!(ffi_from_opt),
        }.to_tokens(dst)
    }
}

