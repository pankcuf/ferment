use std::marker::PhantomData;
use syn::Type;
use crate::context::ScopeSearch;
use crate::lang::Specification;

// Dictionary generics and strings should be fermented
// Others should be treated as opaque

#[derive(Clone, Debug)]
pub struct VarComposer<SPEC>
    where SPEC: Specification {
    pub search: ScopeSearch,
    _marker: PhantomData<SPEC>,
}

impl<SPEC> VarComposer<SPEC>
    where SPEC: Specification {
    fn new(search: ScopeSearch) -> Self {
        Self { search, _marker: PhantomData }
    }
    pub fn key_ref_in_composer_scope(ty: &Type) -> Self {
        Self::new(ScopeSearch::type_ref_key_in_composer_scope(ty))
    }
    pub fn value(ty: &Type) -> Self {
        Self::new(ScopeSearch::type_ref_value(ty))
    }
}
