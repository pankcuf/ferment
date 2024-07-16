use std::collections::HashMap;
use syn::Path;
use crate::ast::TypePathHolder;
use crate::context::ScopeChain;
use crate::formatter::format_path_vec;

#[derive(Clone, Default)]
pub struct GenericResolver {
    pub inner: HashMap<ScopeChain, HashMap<TypePathHolder, Vec<Path>>>,
}

impl GenericResolver {
    pub fn scope_mut(&mut self, scope: &ScopeChain) -> &mut HashMap<TypePathHolder, Vec<Path>> {
        self.inner
            .entry(scope.clone())
            .or_default()
    }
    pub fn maybe_generic_bounds(&self, scope: &ScopeChain, ident: &TypePathHolder) -> Option<&Vec<Path>> {
        // println!("maybe_generic_bounds: {}.....", ident);
        let result = self.inner.get(&scope)
            .and_then(|items| items.get(ident));
        if result.is_some() {
            println!("maybe_generic_bounds: FOUND {}.....", format_path_vec(result.unwrap()));
        }
        result
    }

    pub fn extend_in_scope(&mut self, scope: &ScopeChain, generics: HashMap<TypePathHolder, Vec<Path>>) {
        self.scope_mut(&scope)
            .extend(generics);

    }
}