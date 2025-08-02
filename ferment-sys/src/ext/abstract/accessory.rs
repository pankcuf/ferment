use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_quote, Type};
use syn::__private::TokenStream2;

pub trait Accessory: ToTokens {
    fn joined_mut(&self) -> Self;
    #[allow(unused)]
    fn joined_const(&self) -> Self;
    #[allow(unused)]
    fn joined_dyn(&self) -> Self;

    #[allow(unused)]
    fn joined_ref(&self) -> Self;

    #[allow(unused)]
    fn joined_mut_ref(&self) -> Self;
    #[allow(unused)]
    fn joined_ident(&self, ident: &Ident) -> Self;
}
#[macro_export]
macro_rules! impl_accessory {
    ($ty:ty) => {
        impl crate::ext::Accessory for $ty {
            fn joined_mut(&self) -> Self {
                parse_quote!(*mut #self)
            }
            fn joined_const(&self) -> Self {
                parse_quote!(*const #self)
            }
            fn joined_dyn(&self) -> Self {
                parse_quote!(dyn #self)
            }
            fn joined_ref(&self) -> Self {
                parse_quote!(&#self)
            }
            fn joined_mut_ref(&self) -> Self {
                parse_quote!(&mut #self)
            }
            fn joined_ident(&self, ident: &Ident) -> Self {
                parse_quote!(#self::#ident)
            }
        }
    };
}
impl_accessory!(Type);
impl_accessory!(TokenStream2);
