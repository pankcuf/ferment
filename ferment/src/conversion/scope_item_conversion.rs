use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Item, Signature};
use syn::__private::TokenStream2;
use crate::formatter::format_token_stream;
use crate::helper::ItemExtension;

#[derive(Clone, PartialEq)]
pub enum ScopeItemConversion {
    Item(Item),
    Fn(Signature),
}

impl ToTokens for ScopeItemConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ScopeItemConversion::Item(item) => item.to_tokens(tokens),
            ScopeItemConversion::Fn(sig) => sig.to_tokens(tokens)
        }
    }
}
impl Debug for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItemConversion::Item(item) =>
                f.write_str(format!("Item({})", format_token_stream(item.maybe_ident())).as_str()),
            ScopeItemConversion::Fn(sig) =>
                f.write_str(format!("Fn({})", format_token_stream(&sig.ident)).as_str()),
        }
    }
}

impl Display for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ItemExtension for ScopeItemConversion {
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_attrs(),
            ScopeItemConversion::Fn(..) => None
        }
    }

    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_ident(),
            ScopeItemConversion::Fn(sig) => Some(&sig.ident)
        }
    }

    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_generics(),
            ScopeItemConversion::Fn(sig) => Some(&sig.generics)
        }
    }
}