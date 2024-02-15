use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Type, TypePath};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::helper::{destroy_path, destroy_ptr, destroy_reference, from_array, from_path, from_ptr, from_reference, to_array, to_path, to_ptr, to_reference};
use crate::interface::package_unbox_any_expression;

pub trait Conversion {
    type Item: ToTokens;
    fn destroy(&self, field_path: TokenStream2, context: &ScopeContext) -> Self::Item;
    fn from(&self, field_path: TokenStream2, context: &ScopeContext) -> Self::Item;
    fn to(&self, field_path: TokenStream2, context: &ScopeContext) -> Self::Item;
}

impl Conversion for Type {
    type Item = TokenStream2;

    fn destroy(&self, field_path: TokenStream2, _context: &ScopeContext) -> Self::Item {
        match self {
            Type::Array(..) =>
                package_unbox_any_expression(field_path),
            Type::Path(TypePath { path, .. }) =>
                destroy_path(field_path, path),
            Type::Ptr(type_ptr) =>
                destroy_ptr(field_path, type_ptr),
            Type::Reference(type_reference) =>
                destroy_reference(field_path, type_reference),
            _ => panic!("add_conversion: Unknown field {}", quote!(#self)),
        }
    }

    fn from(&self, field_path: TokenStream2, _context: &ScopeContext) -> Self::Item {
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

    fn to(&self, field_path: TokenStream2, context: &ScopeContext) -> Self::Item {
        println!("to: {}: {}", quote!(#field_path), self.to_token_stream());
        match self {
            Type::Array(type_array) =>
                to_array(field_path, type_array, &context),
            Type::Path(TypePath { path, .. }) =>
                to_path(field_path, path, &context),
            Type::Ptr(type_ptr) =>
                to_ptr(field_path, type_ptr, &context),
            Type::Reference(type_reference) =>
                to_reference(field_path, type_reference, &context),
            _ => panic!("add_conversion: Unknown field {}", quote!(#self)),
        }
    }
}

impl Conversion for FieldTypeConversion {
    type Item = TokenStream2;

    fn destroy(&self, field_path: TokenStream2, context: &ScopeContext) -> Self::Item {
        self.ty().destroy(field_path, context)
    }

    fn from(&self, field_path: TokenStream2, context: &ScopeContext) -> Self::Item {
        self.ty().from(field_path, context)
    }
    fn to(&self, field_path: TokenStream2, context: &ScopeContext) -> Self::Item {
        println!("FieldTypeConversion:to: {}", quote!(#field_path));
        self.ty().to(field_path, context)
    }
}