use std::marker::PhantomData;
use syn::Type;
use crate::context::{ScopeChain, ScopeSearch, ScopeSearchKey};
use crate::lang::Specification;

// Dictionary generics and strings should be fermented
// Others should be treated as opaque

#[derive(Clone, Debug)]
pub struct VarComposer<'a, SPEC>
    where SPEC: Specification {
    pub search: ScopeSearch<'a>,
    _marker: PhantomData<SPEC>,
}

impl<'a, SPEC> VarComposer<'a, SPEC>
    where SPEC: Specification {
    fn new(search: ScopeSearch<'a>) -> Self {
        Self { search, _marker: PhantomData }
    }
    pub fn key_in_scope(ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }

    pub fn value(ty: &'a Type) -> Self {
        Self::new(ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()))
    }
}
