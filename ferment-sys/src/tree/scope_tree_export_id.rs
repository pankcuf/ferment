use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Path, PathSegment, Type};
use crate::context::{Scope, ScopeChain, ScopeInfo};
use crate::conversion::ObjectKind;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ScopeTreeExportID {
    Ident(Ident),
    Impl(Type, Option<Path>, Generics)
}

impl From<&PathSegment> for ScopeTreeExportID {
    fn from(value: &PathSegment) -> Self {
        Self::from_ident(&value.ident)
    }
}

impl std::fmt::Debug for ScopeTreeExportID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident) =>
                f.write_str(format!("{ident}").as_str()),
            Self::Impl(ty, path, generics) =>
                f.write_str(format!("Impl({}, {}, {})", ty.to_token_stream(), path.to_token_stream(), generics.to_token_stream().to_string()).as_str())
        }
    }
}

impl std::fmt::Display for ScopeTreeExportID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeTreeExportID {
    pub fn from_ident(ident: &Ident) -> Self {
        Self::Ident(ident.clone())
    }

    pub fn create_child_scope(&self, scope: &ScopeChain, attrs: Vec<Attribute>) -> ScopeChain {
        match &self {
            Self::Ident(ident) => ScopeChain::Mod {
                info: ScopeInfo {
                    attrs,
                    crate_ident: scope.crate_ident_ref().clone(),
                    self_scope: Scope::new(scope.self_path_holder_ref().joined(ident), ObjectKind::Empty),
                },
                parent_scope_chain: Box::new(scope.clone())
            },
            Self::Impl(..) =>
                panic!("impl not implemented")
        }
    }
}

