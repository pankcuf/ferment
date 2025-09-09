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
        match self {
            ScopeSearch::KeyInScope(key, scope) =>
                f.write_fmt(format_args!("KeyInScope({key} in {})", scope.fmt_short())),
            ScopeSearch::KeyInComposerScope(key) =>
                f.write_fmt(format_args!("KeyInComposerScope({key})")),
            ScopeSearch::Value(key) =>
                f.write_fmt(format_args!("Value({key})")),
        }
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