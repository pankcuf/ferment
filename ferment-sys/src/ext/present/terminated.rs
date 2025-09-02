use quote::quote;
use syn::__private::TokenStream2;

pub trait Terminated {
    fn terminated(&self) -> Self;
}

impl Terminated for TokenStream2 {
    fn terminated(&self) -> Self {
        quote!(#self;)
    }
}

pub trait WrapInBraces {
    fn wrap_in_braces(&self) -> Self;
    fn wrap_in_rounds(&self) -> Self;
}

impl WrapInBraces for TokenStream2 {
    fn wrap_in_braces(&self) -> Self {
        quote!({#self})
    }

    fn wrap_in_rounds(&self) -> Self {
        quote!((#self))
    }
}

