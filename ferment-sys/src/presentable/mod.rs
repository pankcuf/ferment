use std::fmt::Debug;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use crate::context::ScopeContext;

mod arg;
mod aspect;
mod binding;
mod conversion_expression_kind;
mod expression;
#[allow(unused)]
mod interface;
mod name;
mod sequence;

pub use arg::*;
pub use aspect::*;
pub use binding::*;
pub use conversion_expression_kind::*;
pub use expression::*;
pub use interface::*;
pub use name::*;
pub use sequence::*;
use syn::{Attribute, Path};
use crate::composable::FnSignatureContext;

pub trait ScopeContextPresentable {
    type Presentation: Clone + ToTokens;
    fn present(&self, source: &ScopeContext) -> Self::Presentation;
}

impl ScopeContextPresentable for TokenStream2 {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}

impl<T, SEP> ScopeContextPresentable for Punctuated<T, SEP>
    where T: ScopeContextPresentable,
          SEP: ToTokens + Clone + Default {
    type Presentation = Punctuated<<T as ScopeContextPresentable>::Presentation, SEP>;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        self.iter().map(|presentable| presentable.present(source)).collect()
    }
}

pub trait NameTreeContext: Clone + Debug + ToTokens {
    fn join_fn(&self, path: Path, sig_context: FnSignatureContext, attrs: Vec<Attribute>) -> Self;
    fn join_variant(&self, ident: Ident, variant_ident: Ident, attrs: Vec<Attribute>) -> Self;
}


