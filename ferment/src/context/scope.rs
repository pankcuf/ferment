use syn::{Item, Path};
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use crate::composition::GenericBoundComposition;
use crate::conversion::ObjectConversion;
use crate::helper::ItemExtension;
use crate::holder::PathHolder;

#[derive(Clone, Eq)]
pub struct Scope {
    pub self_scope: PathHolder,
    pub object: ObjectConversion,
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Scope({}, {})", self.self_scope, self.object).as_str())
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl PartialEq<Self> for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.self_scope.0.to_token_stream().to_string() ==
            other.self_scope.0.to_token_stream().to_string()
        //&& self.object.
    }
}

impl Hash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.self_scope.to_string().hash(state);
        self.object.to_string().hash(state);
    }
}

impl Scope {
    pub fn new(self_scope: PathHolder, object: ObjectConversion) -> Self {
        Scope { self_scope, object }
    }
    pub fn joined(&self, item: &Item) -> Self {
        //println!("Scope::joined: {} + {}", self, item.ident_string());
        let child_self_scope = item.maybe_ident()
            .map(|ident| self.self_scope.joined(ident))
            .unwrap_or(self.self_scope.clone());
        // println!(":::: joined: {} in [{}] --> [{}] ", item.ident_string(), self, child_self_scope);
        let object = ObjectConversion::try_from(item).unwrap();
        Scope::new(child_self_scope, object)
    }

    pub fn maybe_generic_bound_for_path(&self, path: &Path) -> Option<GenericBoundComposition> {
        // println!("Scope::maybe_generic_bound_for_path: {} in [{}]", format_token_stream(path), self);
        match &self.object {
            ObjectConversion::Item(_, item) => item.maybe_generic_bound_for_path(path),
            _ => None
        }
    }
}
