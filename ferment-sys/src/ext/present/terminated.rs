use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

pub trait Terminated {
    fn terminated(&self) -> Self;
}

impl Terminated for TokenStream2 {
    fn terminated(&self) -> Self {
        quote!(#self;)
    }
}

pub trait WrapIntoRoundBraces: ToTokens {
    fn wrap(self) -> TokenStream2;
}

pub trait WrapIntoCurlyBraces: ToTokens {
    fn wrap(self) -> TokenStream2;
}

impl<T: ToTokens> WrapIntoCurlyBraces for T {
    fn wrap(self) -> TokenStream2 {
        quote!({#self})
    }
}

impl<T: ToTokens> WrapIntoRoundBraces for T {
    fn wrap(self) -> TokenStream2 {
        quote!((#self))
    }
}

