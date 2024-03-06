use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
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

impl<T, SEP> ScopeContextPresentable for Punctuated<T, SEP>
    where T: ScopeContextPresentable, SEP: ToTokens + Default {
    type Presentation = Punctuated<T::Presentation, SEP>;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        self.iter().map(|presentable| presentable.present(source)).collect()
    }
}
