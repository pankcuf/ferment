use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Type;
use crate::conversion::TypeCompositionConversion;
use crate::formatter::format_ident_types_dict;
use crate::holder::{PathHolder, TypeHolder};

#[derive(Clone)]
#[allow(unused)]
pub enum TraitLink {
    Bounds(HashMap<Ident, Type>),
    Generics(HashMap<Ident, Type>)
}
impl Debug for TraitLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TraitLink::Bounds(bounds) =>
                f.write_str(format!("Bounds({})", format_ident_types_dict(bounds)).as_str()),
            TraitLink::Generics(bounds) =>
                f.write_str(format!("Generics({})", format_ident_types_dict(bounds)).as_str()),
        }
    }
}
impl Display for TraitLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq)]
pub enum InitialType {
    Unknown(TypeHolder),
    Local(TypeHolder, TypeCompositionConversion, PathHolder),
    Crate(TypeHolder, TypeCompositionConversion, PathHolder),
    Global(TypeHolder, TypeCompositionConversion)
}

impl Hash for InitialType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            InitialType::Unknown(ty) =>
                ty.to_string().hash(state),
            InitialType::Local(ty, tt, scope) |
            InitialType::Crate(ty, tt, scope) => {
                ty.to_string().hash(state);
                format!("{:?}", tt).hash(state);
                scope.to_string().hash(state);
            },
            InitialType::Global(ty, tt) => {
                ty.to_string().hash(state);
                format!("{:?}", tt).hash(state);
            }
        }
    }
}


impl Debug for InitialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            InitialType::Unknown(ty) =>
                format!("InitialType::Unknown({})", ty.to_token_stream()),
            InitialType::Local(ty, tt, scope) |
            InitialType::Crate(ty, tt, scope) =>
                format!("InitialType::{:?}({}, {:?}, {})", self, ty.to_token_stream(), tt, scope.to_token_stream()),
            InitialType::Global(ty, tt) =>
                format!("InitialType::Global({}, {:?})", ty.to_token_stream(), tt),
        }.as_str())
    }
}

impl Display for InitialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for InitialType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            InitialType::Unknown(ty) => quote!(InitialType::Unknown(#ty)),
            InitialType::Local(ty, tt, scope) => quote!(InitialType::Local(#ty, #tt, #scope)),
            InitialType::Crate(ty, tt, scope) => quote!(InitialType::Crate(#ty, #tt, #scope)),
                // format!("InitialType::{:?}({}, {:?}, {})", self, ty.to_token_stream(), tt, scope.to_token_stream()),
            InitialType::Global(ty, tt) => quote!(InitialType::Crate(#ty, #tt)),
                // format!("InitialType::Global({}, {:?})", ty.to_token_stream(), tt),
        }.to_tokens(tokens)
    }
}
