use std::fmt::Debug;
use syn::Type;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use crate::ast::Depunctuated;
use crate::context::ScopeContext;
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{Expansion, Name};

#[derive(Clone, Debug)]
pub enum FieldTypeConversionKind {
    Type(Type),
    Conversion(TokenStream2)
}
impl ToTokens for FieldTypeConversionKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeConversionKind::Type(ty) => quote!(#ty),
            FieldTypeConversionKind::Conversion(conversion) => quote!(#conversion),
        }.to_tokens(tokens)
    }
}
impl FieldTypeConversionKind {
    pub fn ty(&self) -> &Type {
        if let FieldTypeConversionKind::Type(ty) = self {
            ty
        } else {
            panic!("improper use of conversion as type")
        }
    }

}

#[derive(Clone, Debug)]
pub struct FieldTypeComposition {
    pub name: Name,
    pub kind: FieldTypeConversionKind,
    pub attrs: Depunctuated<Expansion>,
    pub named: bool,
}

impl FieldTypeComposition {
    pub fn new(name: Name, kind: FieldTypeConversionKind, named: bool, attrs: Depunctuated<Expansion>) -> Self {
        Self { name, kind, named, attrs }
    }
    pub fn no_attrs(name: Name, kind: FieldTypeConversionKind, named: bool) -> Self {
        Self { name, kind, named, attrs: Depunctuated::new() }
    }
    pub fn named(name: Name, kind: FieldTypeConversionKind) -> Self {
        Self::no_attrs(name, kind, true)
    }
    pub fn unnamed(name: Name, kind: FieldTypeConversionKind) -> Self {
        Self { name, kind, named: false, attrs: Depunctuated::new() }
    }
    pub fn ty(&self) -> &Type {
        if let FieldTypeConversionKind::Type(ty) = &self.kind {
            ty
        } else {
            panic!("improper use of conversion as type")
        }
    }

}
impl ToTokens for FieldTypeComposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, kind, attrs, .. } = self;
        let template = quote! { #attrs #name: #kind };
        template.to_tokens(tokens)
    }
}
impl ScopeContextPresentable for FieldTypeComposition {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}
