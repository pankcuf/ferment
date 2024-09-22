use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use syn::Type;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use ferment_macro::Display;
use crate::composer::SourceFermentable;
use crate::context::ScopeContext;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable};
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

#[derive(Clone, Debug)]
pub struct FieldComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub attrs: SPEC::Attr,
    pub name: Name,
    pub kind: FieldTypeKind,
    pub named: bool,
    _marker: PhantomData<LANG>,
}

impl<LANG, SPEC> Display for FieldComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Attr: std::fmt::Display> + Display,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("FieldComposer({}({}), {}({}), {}, {}",
                            self.name, self.name.to_token_stream(),
                            self.kind, self.kind.to_token_stream(),
                            self.named, self.attrs).as_str())
    }
}

impl<LANG, SPEC> FieldComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(name: Name, kind: FieldTypeKind, named: bool, attrs: SPEC::Attr) -> Self {
        Self { name, kind, named, attrs, _marker: PhantomData }
    }
    pub fn named(name: Name, kind: FieldTypeKind) -> Self {
        Self::no_attrs(name, kind, true)
    }
    pub fn unnamed(name: Name, kind: FieldTypeKind) -> Self {
        Self { name, kind, named: false, attrs: SPEC::Attr::default(), _marker: PhantomData }
    }
    pub fn no_attrs(name: Name, kind: FieldTypeKind, named: bool) -> Self {
        Self { name, kind, named, attrs: SPEC::Attr::default(), _marker: PhantomData }
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

impl<LANG, SPEC> ToTokens for FieldComposer<LANG, SPEC>
    where LANG: Clone + ToTokens,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Self: SourceFermentable<LANG> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ferment().to_tokens(tokens);
        // let Self { name, kind, attrs, .. } = self;
        // quote!(#(#attrs)* #name: #kind).to_tokens(tokens)
    }
}

impl<SPEC> SourceFermentable<RustFermentate> for FieldComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        let Self { name, kind, attrs, .. } = self;
        RustFermentate::TokenStream(quote!(#(#attrs)* #name: #kind))
    }
}

impl<LANG, SPEC> ScopeContextPresentable for FieldComposer<LANG, SPEC>
    where Self: ToTokens,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}
