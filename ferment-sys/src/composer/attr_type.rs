use syn::Type;
use crate::lang::Specification;
use crate::presentable::{Aspect, ScopeContextPresentable};

#[allow(unused)]
pub trait AttrTypeComposer<'a, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn new(ty: &'a Type, attrs: &'a SPEC::Attr) -> Self;
    fn attrs(&self) -> &'a SPEC::Attr;
    fn ty(&self) -> &'a Type;
}

// impl<'a, LANG, SPEC> AttrTypeComposer<'a, LANG, SPEC>
//     where LANG: Clone,
//           SPEC: LangAttrSpecification<LANG>{
//     pub fn new(ty: &'a Type, attrs: &'a SPEC) -> Self {
//         Self { ty, attrs, phantom_data: Default::default() }
//     }
// }
