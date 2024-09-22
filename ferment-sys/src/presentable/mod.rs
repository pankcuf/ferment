use quote::ToTokens;
use syn::punctuated::Punctuated;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use crate::context::ScopeContext;

mod binding;
mod expression;
mod name;
mod argument;
mod sequence;
mod aspect;
mod ty_context;

pub use aspect::*;
pub use binding::*;
// pub use ctor_presentable::*;
pub use expression::*;
pub use name::*;
pub use argument::*;
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
    where T: ScopeContextPresentable, SEP: ToTokens + Clone + Default {
    type Presentation = Punctuated<<T as ScopeContextPresentable>::Presentation, SEP>;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        self.iter().map(|presentable| presentable.present(source)).collect()
    }
}
// impl<T, SEP> ScopeContextPresentable for Punctuated<T, SEP>
//     where T: ScopeContextPresentable, SEP: ToTokens + Clone + Default {
//     type Presentation = Punctuated<<T as ScopeContextPresentable>::Presentation, SEP>;
//
//     fn present(&self, source: &ScopeContext) -> Self::Presentation {
//         self.iter().map(|presentable| presentable.present(source)).collect()
//     }
// }

pub trait NameTreeContext: Clone {
    // fn maybe_parent(&self) -> Option<&Box<Self>>;
    fn join_fn(&self, path: Path, sig_context: FnSignatureContext, attrs: Vec<Attribute>) -> Self;
    fn join_variant(&self, ident: Ident, variant_ident: Ident, attrs: Vec<Attribute>) -> Self;
}


