use proc_macro2::{Ident, TokenStream as TokenStream2};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use crate::conversion::ImportConversion;
use crate::ext::Pop;
use crate::holder::PathHolder;

#[derive(Clone)]
pub struct ImportComposition {
    pub ident: Ident,
    pub scope: PathHolder,
}

impl ImportComposition {
    pub fn new(ident: Ident, scope: PathHolder) -> Self {
        Self { ident, scope }
    }
}

impl<'a> From<(&'a Ident, &'a PathHolder)> for ImportComposition {
    fn from(value: (&'a Ident, &'a PathHolder)) -> Self {
        Self { ident: value.0.clone(), scope: value.1.clone() }
    }
}

// impl<'a> From<&'a GenericConversion> for ImportComposition {
//     fn from(value: &'a GenericConversion) -> Self {
//         ImportComposition {
//             ident: ffi_mangled_ident(value.0.ty()),
//             scope: PathHolder::ffi_generics_scope()
//         }
//     }
//
// }

impl std::fmt::Debug for ImportComposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        f.write_str(&self.scope.to_string())?;
        f.write_str("]: ")?;
        f.write_str(&self.ident.to_token_stream().to_string())
    }
}

impl std::fmt::Display for ImportComposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for ImportComposition {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ident.to_token_stream(), self.scope.to_token_stream()];
        let other_tokens = [other.ident.to_token_stream(), other.scope.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(TokenStream2::to_string))
            .all(|(a, b)| a == b)
    }
}

impl Eq for ImportComposition {}

impl Hash for ImportComposition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.to_token_stream().to_string().hash(state);
        self.scope.to_token_stream().to_string().hash(state);
    }
}

impl ImportComposition {
    pub fn present(&self, import_type: &ImportConversion) -> PathHolder {
        match import_type {
            ImportConversion::External | ImportConversion::Original | ImportConversion::FfiType | ImportConversion::FfiExternal =>
                self.scope.clone(),
            ImportConversion::ExternalChunk =>
                self.scope.popped(),
            _ => self.scope.joined(&self.ident)
        }

    }
}
