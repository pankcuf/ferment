use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::ext::WrapIntoRoundBraces;

pub trait MethodCall {
    fn method(&self) -> TokenStream2;
    fn expr(&self) -> TokenStream2;
}

impl ToTokens for dyn MethodCall {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        self.method().to_tokens(dst);
        WrapIntoRoundBraces::wrap(self.expr()).to_tokens(dst);
    }
}