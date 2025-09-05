use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use syn::{Path, Type};
use crate::context::{ScopeChain, ScopeSearchKey, TypeChain};
use crate::kind::ObjectKind;
use crate::ext::{LifetimeProcessor, RefineMut};
use crate::formatter::types_dict;

pub type ScopeRefinement = Vec<(ScopeChain, HashMap<Type, ObjectKind>)>;

#[derive(Clone, Default)]
pub struct ScopeResolver {
    pub inner: HashMap<ScopeChain, TypeChain>,
}

impl Debug for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.inner.iter()
            .map(|(key, value)| format!("\t{}:\n\t{}", key.fmt_short(), types_dict(&value.inner).join("\n\t")))
            .collect::<Vec<String>>();
        iter.sort();
        f.write_str( iter.join("\n\n").as_str())
    }
}

impl Display for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ScopeResolver {

    pub(crate) fn maybe_scope(&self, path: &Path) -> Option<&ScopeChain> {
        self.inner.keys()
            .find_map(|scope_chain| path.eq(scope_chain.self_path()).then_some(scope_chain))
    }
    pub(crate) fn maybe_first_obj_scope(&self, path: &Path) -> Option<&ScopeChain> {
        let mut scopes = self.inner.keys()
            .filter(|scope_chain| path.eq(scope_chain.self_path()))
            .collect::<Vec<_>>();
        scopes.sort_by(|c1, c2| {
            if c1.obj_scope_priority() == c2.obj_scope_priority() {
                Ordering::Equal
            } else if c1.obj_scope_priority() < c2.obj_scope_priority() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        scopes.first().cloned()
    }
    pub fn type_chain_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        let maybe_entry = self.inner.entry(scope.clone());
        maybe_entry.or_default()
    }

    pub fn maybe_object_ref_by_key_in_scope<'a>(&'a self, search_key: ScopeSearchKey, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        self.get(scope)
            .and_then(|chain|
                search_key.find(|ty|
                    chain.get(ty)
                        .or_else(|| chain.get(&ty.lifetimes_cleaned()))))
    }
    pub fn maybe_object_ref_by_value<'a>(&'a self, search_key: ScopeSearchKey) -> Option<&'a ObjectKind> {
        self.inner.values()
            .find_map(|chain|
                search_key.find(|ty|
                    chain.get_by_value(ty)
                        .or_else(|| chain.get_by_value(&ty.lifetimes_cleaned()))))
    }

    pub fn get(&self, scope: &ScopeChain) -> Option<&TypeChain> {
        self.inner.get(scope)
    }
    pub fn scope_key_type_for_path(&self, path: &Path, scope: &ScopeChain) -> Option<Type> {
        self.get(scope)
            .and_then(|chain| chain.get_by_path(path))
    }
}

impl RefineMut for ScopeResolver {
    type Refinement = ScopeRefinement;

    fn refine_with(&mut self, refined: Self::Refinement) {
        refined.into_iter()
            .for_each(|(scope, updates)|
                self.type_chain_mut(&scope)
                    .add_many(updates.into_iter())
            );
    }
}