use quote::ToTokens;
use syn::punctuated::Punctuated;
use crate::opposed::{Opposed, SeparatorTrait};
use crate::wrapped::{DelimiterTrait, Wrapped};

pub trait Sequence: ToTokens {}

impl<T, P> Sequence for Wrapped<T, P>
    where T: ToTokens, P: DelimiterTrait {}
impl<T, P> Sequence for Punctuated<T, P>
    where T: ToTokens, P: ToTokens {}
impl<T1, T2, P> Sequence for Opposed<T1, T2, P>
    where T1: ToTokens, T2: ToTokens, P: ToTokens + SeparatorTrait {}