use std::collections::HashMap;
use syn::Path;
use crate::context::ScopeChain;
use crate::holder::PathHolder;

#[derive(Clone, Default)]
pub struct GenericResolver {
    pub inner: HashMap<ScopeChain, HashMap<PathHolder, Vec<Path>>>,
}

impl GenericResolver {
    pub fn scope_mut(&mut self, scope: &ScopeChain) -> &mut HashMap<PathHolder, Vec<Path>> {
        self.inner
            .entry(scope.clone())
            .or_default()
    }
    pub fn maybe_generic_bounds(&self, scope: &ScopeChain, ident: &PathHolder) -> Option<&Vec<Path>> {
        self.inner.get(&scope)
            .and_then(|items| items.get(ident))
    }

    pub fn extend_in_scope(&mut self, scope: &ScopeChain, generics: HashMap<PathHolder, Vec<Path>>) {
        self.scope_mut(&scope)
            .extend(generics);

    }
}