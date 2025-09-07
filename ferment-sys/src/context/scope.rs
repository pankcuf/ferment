use syn::{Generics, Item, Path, TypeParam};
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use crate::kind::ObjectKind;
use crate::ext::{GenericBoundKey, MaybeIdent, Join, MaybeGenerics};

#[derive(Clone, Eq)]
pub struct Scope {
    pub self_scope: Path,
    pub object: ObjectKind,
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Scope({}, {})", self.self_scope.to_token_stream(), self.object).as_str())
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl PartialEq<Self> for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.self_scope.to_token_stream().to_string() ==
            other.self_scope.to_token_stream().to_string()
        && self.object.eq(&other.object)
    }
}

impl Hash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.self_scope.to_token_stream().to_string().hash(state);
        self.object.to_string().hash(state);
    }
}

impl Scope {
    pub fn new(self_scope: Path, object: ObjectKind) -> Self {
        Self { self_scope, object }
    }
    pub fn empty(self_scope: Path) -> Self {
        Self::new(self_scope, ObjectKind::Empty)
    }
    pub fn joined(&self, item: &Item) -> Self {
        let child_self_scope = item.maybe_ident()
            .map(|ident| self.self_scope.joined(ident))
            .unwrap_or_else(|| self.self_scope.clone());
        let object = ObjectKind::try_from((item, &child_self_scope)).expect("Can't obtain ObjectKind for Item for child scope");
        Self::new(child_self_scope, object)
    }

    pub fn maybe_generic_bound_for_path(&self, path: &GenericBoundKey) -> Option<(Generics, TypeParam)> {
        self.object.maybe_scope_item()
            .and_then(|item| item.maybe_generic_bound_for_path(path))
    }
}
