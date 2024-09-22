use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::ast::{DelimiterTrait, Opposed, SeparatorTrait, Wrapped};
use crate::context::ScopeContext;
use crate::presentation::ScopeContextPresentable;

#[allow(unused)]
pub trait Sequence: ScopeContextPresentable {}

impl<T, P, I> Sequence for Wrapped<T, P, I>
    where
        T: ScopeContextPresentable<Presentation = TokenStream2>,
        P: DelimiterTrait,
        T::Presentation: ToTokens {}
impl<T, P> Sequence for Punctuated<T::Presentation, P>
    where
        T: ScopeContextPresentable,
        P: ToTokens,
        T::Presentation: ToTokens {}
impl<T1, T2, P> Sequence for Opposed<T1::Presentation, T2::Presentation, P>
    where
        T1: ScopeContextPresentable,
        T2: ScopeContextPresentable,
        T1::Presentation: ToTokens,
        T2::Presentation: ToTokens,
        P: ToTokens + SeparatorTrait {}