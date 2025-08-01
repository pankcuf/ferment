use std::marker::PhantomData;
use syn::Type;
use crate::context::{ScopeChain, ScopeSearch, ScopeSearchKey};
use crate::lang::Specification;

#[derive(Clone, Debug)]
pub struct TargetVarComposer<'a, SPEC>
where SPEC: Specification {
    pub search: ScopeSearch<'a>,
    _marker: PhantomData<SPEC>,
}

impl<'a, SPEC> TargetVarComposer<'a, SPEC>
    where SPEC: Specification {
    pub fn new(search: ScopeSearch<'a>) -> Self {
        Self { search, _marker: PhantomData }
    }
    #[allow(unused)]
    pub fn key_in_scope(ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }

    #[allow(unused)]
    pub fn value(ty: &'a Type) -> Self {
        Self::new(ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()))
    }
}
