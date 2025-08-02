use std::fmt::{Display, Formatter};
use syn::Type;
use crate::context::{ScopeChain, ScopeSearchKey};

#[derive(Clone, Debug)]
pub enum ScopeSearch {
    KeyInScope(ScopeSearchKey, ScopeChain),
    KeyInComposerScope(ScopeSearchKey),
    Value(ScopeSearchKey),
}
impl Display for ScopeSearch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::KeyInScope(key, scope) => format!("KeyInScope({} in {})", key, scope.fmt_short()),
            Self::Value(key) => format!("Value({})", key),
            Self::KeyInComposerScope(key) => format!("KeyInComposerScope({})", key),
        }.as_str())
    }
}

impl ScopeSearch {
    pub fn type_ref_key_in_composer_scope(ty: &Type) -> Self {
        Self::KeyInComposerScope(ScopeSearchKey::maybe_from_ref(ty).unwrap())
    }
    pub fn type_ref_value(ty: &Type) -> Self {
        Self::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap())
    }
    pub fn search_key(&self) -> &ScopeSearchKey {
        match self {
            Self::KeyInScope(search_key, _) |
            Self::Value(search_key) |
            Self::KeyInComposerScope(search_key) => search_key
        }
    }
}