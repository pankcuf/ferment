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
    // fn wrap<S, SP>(content: S) -> Wrapped<S, SP, Self> where SP: ToTokens {
    //     Wrapped::new(content)
    // }
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
    where SP: ToTokens,
          I: DelimiterTrait + ?Sized {
    pub content: S,
    _marker: PhantomData<(SP, I)>,
}

impl<S, SP, I> Wrapped<S, SP, I>
    where SP: ToTokens,
          I: DelimiterTrait + ?Sized {
    pub fn new(content: S) -> Self {
        Wrapped {
            content,
            _marker: PhantomData,
        }
    }
    pub fn token_tree<T: ToTokens>(content: T) -> TokenTree {
        TokenTree::Group(Group::new(I::delimiter(), content.to_token_stream()))
    }
}

impl<S, SP, I> ScopeContextPresentable for Wrapped<S, SP, I>
    where S: ScopeContextPresentable<Presentation = SP>,
          SP: ToTokens,
          I: DelimiterTrait + ?Sized {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        Self::token_tree(self.content.present(source))
            .to_token_stream()
    }
}

impl<S, SP, I> ToTokens for Wrapped<S, SP, I>
    where S: ToTokens,
          SP: ToTokens,
          I: DelimiterTrait {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        Self::token_tree(&self.content)
            .to_tokens(tokens)
    }
}
