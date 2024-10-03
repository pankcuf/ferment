use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Path, PathSegment, Type};
use crate::context::{Scope, ScopeChain, ScopeInfo};
use crate::conversion::ObjectKind;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ScopeTreeExportID {
    Ident(Ident),
    Impl(Type, Option<Path>)
}

impl From<&PathSegment> for ScopeTreeExportID {
    fn from(value: &PathSegment) -> Self {
        ScopeTreeExportID::Ident(value.ident.clone())
    }
}

impl std::fmt::Debug for ScopeTreeExportID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportID::Ident(ident) =>
                f.write_str(format!("{}", ident.to_token_stream()).as_str()),
            ScopeTreeExportID::Impl(ty, path) =>
                f.write_str(format!("Impl({}, {})", ty.to_token_stream(), path.to_token_stream()).as_str())
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
        ScopeTreeExportID::Ident(ident.clone())
    }

    pub fn create_child_scope(&self, scope: &ScopeChain, attrs: Vec<Attribute>) -> ScopeChain {
        match &self {
            ScopeTreeExportID::Ident(ident) => ScopeChain::Mod {
                info: ScopeInfo {
                    attrs,
                    crate_ident: scope.crate_ident_ref().clone(),
                    self_scope: Scope::new(scope.self_path_holder_ref().joined(ident), ObjectKind::Empty),
                },
                parent_scope_chain: Box::new(scope.clone())
            },
            ScopeTreeExportID::Impl(_, _) =>
                panic!("impl not implemented")
        }
    }
}

