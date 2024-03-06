use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::marker::PhantomData;

pub trait SeparatorTrait {
    fn separator() -> TokenStream;
}
pub struct Opposed<T1, T2, D>
    where
        T1: ToTokens,
        T2: ToTokens,
        D: SeparatorTrait,
{
    left: T1,
    right: T2,
    _marker: PhantomData<D>,
}

impl<T1, T2, D> Opposed<T1, T2, D>
    where
        T1: ToTokens,
        T2: ToTokens,
        D: SeparatorTrait,
{
    pub fn new(left: T1, right: T2) -> Self {
        Opposed {
            left,
            right,
            _marker: PhantomData,
        }
    }
}

impl<T1, T2, D> ToTokens for Opposed<T1, T2, D>
    where
        T1: ToTokens,
        T2: ToTokens,
        D: SeparatorTrait,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let left = self.left.to_token_stream();
        let separator = D::separator();
        let right = self.right.to_token_stream();
        tokens.extend(quote! { #left #separator #right });
    }
}

impl SeparatorTrait for syn::token::Eq {
    fn separator() -> TokenStream { quote! { = } }
}

impl SeparatorTrait for syn::token::FatArrow {
    fn separator() -> TokenStream { quote! { => } }
}

impl SeparatorTrait for syn::token::Colon {
    fn separator() -> TokenStream { quote! { : } }
}

// impl<T1, T2, D> ScopeContextPresentable for Opposed<T1, T2, D>
//     where
//         T1: ScopeContextPresentable + ToTokens,
//         T2: ScopeContextPresentable + ToTokens,
//         D: SeparatorTrait {
//     type Presentation = TokenStream2;
//
//     fn present(&self, source: &ScopeContext) -> Self::Presentation {
//         let left = self.left.present(source);
//         let separator = D::separator();
//         let right = self.right.present(source);
//         quote! { #left #separator #right }
//     }
// }
//
