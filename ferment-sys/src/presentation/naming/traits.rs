use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

pub trait MethodCall {
    fn method(&self) -> TokenStream2;
    fn expr(&self) -> &TokenStream2;
}

impl ToTokens for dyn MethodCall {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        let method = self.method();
        let expr = self.expr();
        quote!(#method(#expr)).to_tokens(dst)
    }
}