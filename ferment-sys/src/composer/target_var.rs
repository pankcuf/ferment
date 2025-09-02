use std::marker::PhantomData;
use syn::Type;
use crate::context::ScopeSearch;
use crate::lang::Specification;

#[derive(Clone, Debug)]
pub struct TargetVarComposer<SPEC>
where SPEC: Specification {
    pub search: ScopeSearch,
    _marker: PhantomData<SPEC>,
}

impl<SPEC> TargetVarComposer<SPEC>
    where SPEC: Specification {
    pub fn new(search: ScopeSearch) -> Self {
        Self { search, _marker: PhantomData }
    }
    #[allow(unused)]
    pub fn value(ty: &Type) -> Self {
        Self::new(ScopeSearch::type_ref_value(ty))
    }
}
