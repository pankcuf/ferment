use crate::composer::FieldComposers;
use crate::lang::Specification;
use crate::presentable::{Aspect, ScopeContextPresentable};

pub struct ArgsComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable  {
    pub fields: FieldComposers<LANG, SPEC>
}
