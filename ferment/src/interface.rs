use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;


pub fn create_struct<T: ToTokens>(ident: &Ident, attrs: TokenStream2, implementation: T) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #attrs
        pub struct #ident #implementation
    }
}
