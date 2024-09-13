use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use syn::{Attribute, Type};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use ferment_macro::Display;
use crate::context::ScopeContext;
use crate::lang::LangAttrSpecification;
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{Name, RustFermentate};

#[derive(Clone, Debug, Display)]
pub enum FieldTypeKind {
    Type(Type),
    Conversion(TokenStream2)
}

impl ToTokens for FieldTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeKind::Type(ty) => quote!(#ty),
            FieldTypeKind::Conversion(conversion) => quote!(#conversion),
        }.to_tokens(tokens)
    }
}
impl FieldTypeKind {
    pub fn ty(&self) -> &Type {
        if let FieldTypeKind::Type(ty) = self {
            ty
        } else {
            panic!("improper use of conversion as type")
        }
    }
    pub fn r#type(ty: &Type) -> Self {
        Self::Type(ty.clone())
    }
}

pub type RustFieldComposer = FieldComposer<RustFermentate, Vec<Attribute>>;

#[derive(Clone, Debug)]
pub struct FieldComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub name: Name,
    pub kind: FieldTypeKind,
    pub attrs: SPEC,
    pub named: bool,
    _marker: PhantomData<LANG>,
}

impl<LANG, SPEC> Display for FieldComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let attrs = &self.attrs;
        f.write_str(
            format!(
                "FieldComposer({}({}), {}({}), {}, {}",
                self.name,
                self.name.to_token_stream(),
                self.kind,
                self.kind.to_token_stream(),
                self.named,
                self.attrs,
                // quote!(#(#attrs),*)
            ).as_str())
    }
}

impl<LANG, SPEC> FieldComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub fn new(name: Name, kind: FieldTypeKind, named: bool, attrs: SPEC) -> Self {
        Self { name, kind, named, attrs, _marker: PhantomData::default() }
    }
    pub fn named(name: Name, kind: FieldTypeKind) -> Self {
        Self::no_attrs(name, kind, true)
    }
    pub fn unnamed(name: Name, kind: FieldTypeKind) -> Self {
        Self { name, kind, named: false, attrs: SPEC::default(), _marker: PhantomData::default() }
    }
    pub fn no_attrs(name: Name, kind: FieldTypeKind, named: bool) -> Self {
        Self { name, kind, named, attrs: SPEC::default(), _marker: PhantomData::default() }
    }
    pub fn tokenized_name(&self) -> TokenStream2 {
        self.name.to_token_stream()
    }
    pub fn ty(&self) -> &Type {
        if let FieldTypeKind::Type(ty) = &self.kind {
            ty
        } else {
            panic!("improper use of conversion as type")
        }
    }
}

impl ToTokens for FieldComposer<RustFermentate, Vec<Attribute>> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, kind, attrs, .. } = self;
        let template = quote! { #(#attrs)* #name: #kind };
        template.to_tokens(tokens)
    }
}



impl<LANG, SPEC> ScopeContextPresentable for FieldComposer<LANG, SPEC>
    where Self: ToTokens,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}
