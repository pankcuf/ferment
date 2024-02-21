use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::context::ScopeContext;

pub trait ScopeContextPresentable {
    type Presentation: ToTokens;
    fn present(&self, source: &ScopeContext) -> Self::Presentation;
}

impl ScopeContextPresentable for TokenStream2 {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}