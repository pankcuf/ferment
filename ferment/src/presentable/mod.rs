use quote::ToTokens;
use syn::punctuated::Punctuated;
use proc_macro2::TokenStream as TokenStream2;
use crate::context::ScopeContext;

mod binding;
mod ctor_presentable;
mod expression;
mod name;
mod owned_item_presenter_context;
mod sequence_output;
pub use binding::*;
// pub use ctor_presentable::*;
pub use expression::*;
pub use name::*;
pub use owned_item_presenter_context::*;
pub use sequence_output::*;

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
    type Presentation = Punctuated<<T as ScopeContextPresentable>::Presentation, SEP>;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        self.iter().map(|presentable| presentable.present(source)).collect()
    }
}
