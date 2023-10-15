use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{Ident, parse_quote, Path};
use syn::__private::TokenStream2;
use crate::generics::GenericConversion;
use crate::helper::ffi_mangled_ident;
use crate::scope::Scope;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ImportType {
    Original,
    External,
    // full or partial import
    ExternalChunk,
    FfiType,
    FfiGeneric,
    Inner,
    None,
}

impl ImportType {

    pub fn as_path(&self) -> Path {
        match self {
            ImportType::Original => parse_quote!(ImportType::Original),
            ImportType::External => parse_quote!(ImportType::External),
            ImportType::ExternalChunk => parse_quote!(ImportType::ExternalChunk),
            ImportType::FfiType => parse_quote!(ImportType::FfiType),
            ImportType::FfiGeneric => parse_quote!(ImportType::FfiGeneric),
            ImportType::Inner => parse_quote!(ImportType::Inner),
            ImportType::None => parse_quote!(ImportType::None),
        }
    }

    pub fn get_imports_for(self, used_imports: HashSet<ImportConversion>) -> Option<(ImportType, HashSet<ImportConversion>)> {
        match self {
            ImportType::Inner | ImportType::None => None,
            _ => Some((self, used_imports))
        }
    }
}

#[derive(Clone)]
pub struct ImportConversion {
    pub ident: Ident,
    pub scope: Scope,
}

impl ImportConversion {
    pub fn new(ident: Ident, scope: Scope) -> Self {
        Self { ident, scope }
    }
}

impl<'a> From<(&'a Ident, &'a Scope)> for ImportConversion {
    fn from(value: (&'a Ident, &'a Scope)) -> Self {
        Self { ident: value.0.clone(), scope: value.1.clone() }
    }
}

impl<'a> From<&'a GenericConversion> for ImportConversion {
    fn from(value: &'a GenericConversion) -> Self {
        ImportConversion {
            ident: ffi_mangled_ident(&value.full_type),
            scope: Scope::ffi_generics_scope()
        }
    }

}

impl std::fmt::Debug for ImportConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        f.write_str(&self.scope.to_string())?;
        f.write_str("]: ")?;
        f.write_str(&self.ident.to_token_stream().to_string())
    }
}

impl std::fmt::Display for ImportConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for ImportConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ident.to_token_stream(), self.scope.to_token_stream()];
        let other_tokens = [other.ident.to_token_stream(), other.scope.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(TokenStream2::to_string))
            .all(|(a, b)| a == b)
    }
}

impl Eq for ImportConversion {}

impl Hash for ImportConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.to_token_stream().to_string().hash(state);
        self.scope.to_token_stream().to_string().hash(state);
    }
}

impl ImportConversion {
    pub fn present(&self, import_type: &ImportType) -> Scope {
        match import_type {
            ImportType::External | ImportType::Original | ImportType::FfiType =>
                self.scope.clone(),
            ImportType::ExternalChunk =>
                self.scope.popped(),
            _ => self.scope.joined(&self.ident)
        }

    }
}
