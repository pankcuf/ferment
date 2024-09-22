use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use ferment_macro::MethodCall;

#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment::FFIMapConversion"]
pub enum FFIMapConversionMethod {
    New,
    Insert,
}
impl ToTokens for FFIMapConversionMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIMapConversionMethod::New => quote!(new),
            FFIMapConversionMethod::Insert => quote!(insert),
        }.to_tokens(dst)
    }
}
