use syn::Type;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable};

#[allow(unused)]
pub trait AttrTypeComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn new(ty: &'a Type, attrs: &'a SPEC::Attr) -> Self;
    fn attrs(&self) -> &'a SPEC::Attr;
    fn ty(&self) -> &'a Type;
}
