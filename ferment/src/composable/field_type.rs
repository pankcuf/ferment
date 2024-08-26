use std::fmt::{Debug, Display, Formatter};
use syn::{Attribute, Type};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use ferment_macro::Display;
use crate::context::ScopeContext;
use crate::presentable::ScopeContextPresentable;
use crate::presentation::Name;

#[derive(Clone, Debug, Display)]
pub enum FieldTypeKind {
    Type(Type),
    Conversion(TokenStream2)
}

// impl Display for FieldTypeKind {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             FieldTypeKind::Type(ty) => {}
//             FieldTypeKind::Conversion(conv) => {}
//         }
//         f.write_str(format!("Kind::{}({})").as_str())
//     }
// }
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

}

#[derive(Clone, Debug)]
pub struct FieldComposer {
    pub name: Name,
    pub kind: FieldTypeKind,
    pub attrs: Vec<Attribute>,
    // pub attrs: Directives,
    pub named: bool,
}

impl Display for FieldComposer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let attrs = &self.attrs;
        f.write_str(
            format!(
                "FieldComposer({}({}), {}({}), {}, {}",
                self.name,
                self.name.to_token_stream(),
                self.kind,
                self.kind.to_token_stream(),
                self.named,
                quote!(#(#attrs),*)
            ).as_str())
    }
}

impl FieldComposer {
    pub fn new(name: Name, kind: FieldTypeKind, named: bool, attrs: Vec<Attribute>) -> Self {
        Self { name, kind, named, attrs }
    }
    pub fn no_attrs(name: Name, kind: FieldTypeKind, named: bool) -> Self {
        Self { name, kind, named, attrs: Vec::new() }
    }
    pub fn named(name: Name, kind: FieldTypeKind) -> Self {
        Self::no_attrs(name, kind, true)
    }
    pub fn unnamed(name: Name, kind: FieldTypeKind) -> Self {
        Self { name, kind, named: false, attrs: Vec::new() }
    }
    pub fn ty(&self) -> &Type {
        if let FieldTypeKind::Type(ty) = &self.kind {
            ty
        } else {
            panic!("improper use of conversion as type")
        }
    }
    pub fn tokenized_name(&self) -> TokenStream2 {
        self.name.to_token_stream()
    }
    pub fn to_attrs(&self) -> Vec<Attribute> {
        self.attrs.clone()
    }
}
impl ToTokens for FieldComposer {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, kind, attrs, .. } = self;
        let template = quote! { #(#attrs)* #name: #kind };
        template.to_tokens(tokens)
    }
}
impl ScopeContextPresentable for FieldComposer {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}
