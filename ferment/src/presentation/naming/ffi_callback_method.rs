use quote::{quote, TokenStreamExt, ToTokens};
use syn::__private::TokenStream2;
use ferment_macro::MethodCall;

#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment_interfaces::FFICallback"]
pub enum FFICallbackMethod {
    Get,
}
impl ToTokens for FFICallbackMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            Self::Get => quote!(get),
        }.to_tokens(dst)
    }
}

