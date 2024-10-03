use crate::composer::FieldComposers;
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable};

pub struct ArgsComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable  {
    pub fields: FieldComposers<LANG, SPEC>
}
