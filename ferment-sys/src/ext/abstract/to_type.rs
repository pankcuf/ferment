use proc_macro2::Ident;
use syn::{parse_quote, Path, Type, TypePath};
use syn::__private::TokenStream2;
use crate::ast::{AddPunctuated, Colon2Punctuated};

pub trait ToType {
    fn to_type(&self) -> Type;
}
pub trait AsType<'a> {
    fn as_type(&'a self) -> &'a Type;
}
pub trait ToPath {
    fn to_path(&self) -> Path;
}
pub trait ToPathSepSegments {
    fn to_segments(&self) -> Colon2Punctuated<syn::PathSegment>;
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
#[macro_export]
macro_rules! impl_to_segments {
    ($holder_name:ty) => {
        impl ToPathSepSegments for $holder_name {
            fn to_segments(&self) -> Colon2Punctuated<syn::PathSegment> {
                parse_quote!(#self)
            }
        }
    };
}

impl_to_type!(Ident);
// impl_to_type!(Path);
impl_to_type!(Type);
impl_to_type!(TokenStream2);
impl_to_type!(crate::composable::TraitBoundDecomposition);
impl_to_type!(crate::kind::TypeKind);
impl_to_type!(AddPunctuated<syn::TypeParamBound>);

impl_to_path!(Ident);
impl_to_path!(Type);
impl_to_path!(TokenStream2);
impl_to_path!(Colon2Punctuated<syn::PathSegment>);
impl_to_path!(AddPunctuated<syn::TypeParamBound>);
impl_to_path!(crate::kind::ObjectKind);
impl_to_path!(syn::TypePath);

impl_to_segments!(Ident);
impl_to_segments!(Path);

impl ToType for Path {
    fn to_type(&self) -> Type {
        Type::Path(TypePath { qself: None, path: self.clone() })
    }
}

