use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use ferment_macro::MethodCall;
#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment_interfaces::FFIConversion"]
pub enum FFIConversionMethod {
    FfiFromConst,
    FfiToConst,
    FfiFrom,
    FfiTo,
    FfiFromOpt,
    FfiToOpt,
    Destroy,
}
impl ToTokens for FFIConversionMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIConversionMethod::FfiFromConst => quote!(ffi_from_const),
            FFIConversionMethod::FfiToConst => quote!(ffi_to_const),
            FFIConversionMethod::FfiFrom => quote!(ffi_from),
            FFIConversionMethod::FfiTo => quote!(ffi_to),
            FFIConversionMethod::FfiFromOpt => quote!(ffi_from_opt),
            FFIConversionMethod::FfiToOpt => quote!(ffi_to_opt),
            FFIConversionMethod::Destroy => quote!(destroy)
        }.to_tokens(dst)
    }
}

