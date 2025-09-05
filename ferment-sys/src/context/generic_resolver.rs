use indexmap::IndexMap;
use syn::{Path, Type};
use crate::context::ScopeChain;

#[derive(Clone, Default)]
pub struct GenericResolver {
    pub inner: IndexMap<ScopeChain, IndexMap<Type, Vec<Path>>>,
}

impl GenericResolver {
    fn scope_mut(&mut self, scope: &ScopeChain) -> &mut IndexMap<Type, Vec<Path>> {
        self.inner
            .entry(scope.clone())
            .or_default()
    }
    pub fn maybe_generic_bounds(&self, scope: &ScopeChain, ident: &Type) -> Option<&Vec<Path>> {
        self.inner.get(scope)
            .and_then(|items| items.get(ident))
    }

    pub fn extend_in_scope(&mut self, scope: &ScopeChain, generics: IndexMap<Type, Vec<Path>>) {
        self.scope_mut(&scope)
            .extend(generics);

    }
}