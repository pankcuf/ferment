use std::fmt::Debug;
use std::marker::PhantomData;
use syn::Type;
use crate::lang::Specification;

// Dictionary generics and strings should be fermented
// Others should be treated as opaque

#[derive(Clone, Debug)]
pub struct VariableComposer<SPEC>
    where SPEC: Specification {
    pub ty: Type,
    _marker: PhantomData<SPEC>
}

impl<SPEC> VariableComposer<SPEC>
    where SPEC: Specification {
    pub fn new(ty: Type) -> Self {
        Self { ty, _marker: PhantomData }
    }
}
impl<SPEC> From<&Type> for VariableComposer<SPEC>
    where SPEC: Specification {
    fn from(value: &Type) -> Self {
        Self { ty: value.clone(), _marker: PhantomData }
    }
}
