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
    FfiTo,
    FfiToConst,
    FfiToOpt,
    // FfiToByRef,
    // FfiToConstByRef,
    // FfiToOptByRef,
}
impl ToTokens for FFIConversionToMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIConversionToMethod::FfiToConst => quote!(ffi_to_const),
            FFIConversionToMethod::FfiTo => quote!(ffi_to),
            FFIConversionToMethod::FfiToOpt => quote!(ffi_to_opt),
            // FFIConversionToMethod::FfiToByRef => quote!(ffi_to_by_ref),
            // FFIConversionToMethod::FfiToConstByRef => quote!(ffi_to_const_by_ref),
            // FFIConversionToMethod::FfiToOptByRef => quote!(ffi_to_opt_by_ref),
        }.to_tokens(dst)
    }
}
#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment::FFIConversionFrom"]
pub enum FFIConversionFromMethod {
    FfiFromConst,
    FfiFrom,
    FfiFromOpt,
}
impl ToTokens for FFIConversionFromMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIConversionFromMethod::FfiFromConst => quote!(ffi_from_const),
            FFIConversionFromMethod::FfiFrom => quote!(ffi_from),
            FFIConversionFromMethod::FfiFromOpt => quote!(ffi_from_opt),
        }.to_tokens(dst)
    }
}

