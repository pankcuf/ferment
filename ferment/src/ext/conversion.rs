use quote::quote;
use syn::__private::TokenStream2;
use syn::{Type, TypePath};
use crate::conversion::FieldTypeConversion;
use crate::helper::{destroy_path, destroy_ptr, destroy_reference, from_array, from_path, from_ptr, from_reference, to_array, to_path, to_ptr, to_reference};
use crate::presentation::context::FieldTypePresentationContext;

pub trait Conversion {
    type Item;
    fn destroy(&self, field_path: TokenStream2) -> Self::Item;
    fn from(&self, field_path: TokenStream2) -> Self::Item;
    fn to(&self, field_path: TokenStream2) -> Self::Item;
}

impl Conversion for Type {
    type Item = FieldTypePresentationContext;

    fn destroy(&self, field_path: TokenStream2) -> Self::Item {
        match self {
            Type::Array(..) =>
                FieldTypePresentationContext::UnboxAny(field_path),
            Type::Path(TypePath { path, .. }) =>
                destroy_path(field_path, path),
            Type::Ptr(type_ptr) =>
                destroy_ptr(field_path, type_ptr),
            Type::Reference(type_reference) =>
                destroy_reference(field_path, type_reference),
            _ => panic!("add_conversion: Unknown field {}", quote!(#self)),
        }
    }

    fn from(&self, field_path: TokenStream2) -> Self::Item {
        match self {
            Type::Array(type_array) =>
                from_array(field_path, type_array),
            Type::Path(TypePath { path, .. }) =>
                from_path(field_path, path),
            Type::Ptr(type_ptr) =>
                from_ptr(field_path, type_ptr),
            Type::Reference(type_reference) =>
                from_reference(field_path, type_reference),
            _ => panic!("add_conversion: Unknown field {}", quote!(#self)),
        }
    }

    fn to(&self, field_path: TokenStream2) -> Self::Item {
        match self {
            Type::Array(type_array) =>
                to_array(field_path, type_array),
            Type::Path(TypePath { path, .. }) =>
                to_path(field_path, path),
            Type::Ptr(type_ptr) =>
                to_ptr(field_path, type_ptr),
            Type::Reference(type_reference) =>
                to_reference(field_path, type_reference),
            _ => panic!("add_conversion: Unknown field {}", quote!(#self)),
        }
    }
}

impl Conversion for FieldTypeConversion {
    type Item = FieldTypePresentationContext;

    fn destroy(&self, field_path: TokenStream2) -> Self::Item {
        self.ty().destroy(field_path)
    }

    fn from(&self, field_path: TokenStream2) -> Self::Item {
        self.ty().from(field_path)
    }
    fn to(&self, field_path: TokenStream2) -> Self::Item {
        self.ty().to(field_path)
    }
}