use std::marker::PhantomData;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::token::{Brace, Bracket, Paren};
use crate::context::ScopeContext;
use crate::presentable::ScopeContextPresentable;

#[derive(Clone, Debug)]
pub struct Void;
impl ToTokens for Void {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        let _ = quote! {};
    }
}
impl Default for Void {
    fn default() -> Self {
        Void
    }
}

pub trait DelimiterTrait {
    fn delimiter() -> Delimiter;
}

impl DelimiterTrait for Brace {
    fn delimiter() -> Delimiter { Delimiter::Brace }
}

impl DelimiterTrait for Paren {
    fn delimiter() -> Delimiter { Delimiter::Parenthesis }
}

impl DelimiterTrait for Bracket {
    fn delimiter() -> Delimiter { Delimiter::Bracket }
}

impl DelimiterTrait for Void {
    fn delimiter() -> Delimiter { Delimiter::None }
}

#[derive(Clone, Debug)]
pub struct Wrapped<S, SP, I>
    where
        S: ScopeContextPresentable<Presentation = SP>,
        SP: ToTokens,
        I: DelimiterTrait + ?Sized,
{
    content: S,
    _marker: PhantomData<I>,
}

impl<S, SP, I> Wrapped<S, SP, I>
    where
        S: ScopeContextPresentable<Presentation = SP>,
        SP: ToTokens,
        I: DelimiterTrait + ?Sized,
{
    pub fn new(content: S) -> Self {
        Wrapped {
            content,
            _marker: PhantomData,
        }
    }
}

impl<S, SP, I> ScopeContextPresentable for Wrapped<S, SP, I>
    where S: ScopeContextPresentable<Presentation = SP>, SP: ToTokens, I: DelimiterTrait + ?Sized {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        TokenTree::Group(Group::new(I::delimiter(), self.content.present(source).to_token_stream()))
            .to_token_stream()
    }
}

// impl<S, SP, I> ToTokens for Wrapped<S, SP, I> {
//
// }