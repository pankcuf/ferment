use quote::ToTokens;
use syn::punctuated::Punctuated;
use crate::opposed::{Opposed, SeparatorTrait};
use crate::presentation::ScopeContextPresentable;
use crate::wrapped::{DelimiterTrait, Wrapped};

#[allow(unused)]
pub trait Sequence: ScopeContextPresentable {}

impl<T, P> Sequence for Wrapped<T, P>
    where
        T: ScopeContextPresentable<Presentation = Tokenstrea>,
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