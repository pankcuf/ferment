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