use proc_macro2::Ident;
use syn::{parse_quote, Path, Type};
use syn::__private::TokenStream2;
use crate::composer::Colon2Punctuated;

pub trait ToType {
    fn to_type(&self) -> Type;
}
pub trait ToPath {
    fn to_path(&self) -> Path;
}

#[macro_export]
macro_rules! impl_to_type {
    ($holder_name:ty) => {
        impl ToType for $holder_name {
            fn to_type(&self) -> syn::Type {
                parse_quote!(#self)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_to_path {
    ($holder_name:ty) => {
        impl ToPath for $holder_name {
            fn to_path(&self) -> syn::Path {
                parse_quote!(#self)
            }
        }
    };
}
impl_to_type!(Ident);
impl_to_type!(Path);
impl_to_type!(Type);
impl_to_type!(TokenStream2);
impl_to_type!(crate::composition::TraitBoundDecomposition);
impl_to_type!(crate::holder::PathHolder);
impl_to_type!(crate::holder::TypeHolder);
impl_to_type!(crate::naming::Name);

impl_to_path!(Ident);
impl_to_path!(Type);
impl_to_path!(TokenStream2);
impl_to_path!(Colon2Punctuated<syn::PathSegment>);
impl_to_path!(crate::conversion::ObjectConversion);
impl_to_path!(crate::naming::Name);
impl_to_path!(syn::TypePath);
