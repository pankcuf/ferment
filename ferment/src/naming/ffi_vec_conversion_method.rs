use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use ferment_macro::MethodCall;

#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment_interfaces::FFIVecConversion"]
pub enum FFIVecConversionMethod {
    Encode,
    Decode,
}
impl ToTokens for FFIVecConversionMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            FFIVecConversionMethod::Encode => quote!(encode),
            FFIVecConversionMethod::Decode => quote!(decode),
        }.to_tokens(dst)
    }
}
