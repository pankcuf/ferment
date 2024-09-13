use syn::Type;
use crate::lang::LangAttrSpecification;

#[allow(unused)]
pub trait AttrTypeComposer<'a, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    // pub ty: &'a Type,
    // pub attrs: &'a SPEC,
    // phantom_data: PhantomData<LANG>,
    fn new(ty: &'a Type, attrs: &'a SPEC) -> Self;
    fn attrs(&self) -> &'a SPEC;
    fn ty(&self) -> &'a Type;
}

// impl<'a, LANG, SPEC> AttrTypeComposer<'a, LANG, SPEC>
//     where LANG: Clone,
//           SPEC: LangAttrSpecification<LANG>{
//     pub fn new(ty: &'a Type, attrs: &'a SPEC) -> Self {
//         Self { ty, attrs, phantom_data: Default::default() }
//     }
// }
