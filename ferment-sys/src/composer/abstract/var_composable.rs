use std::fmt::Debug;
use quote::ToTokens;
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentation::FFIVariable;

pub trait VarComposable<LANG, SPEC>: Clone + Debug + ToTokens + ToType
where LANG: LangFermentable,
      SPEC: Specification<LANG> {}
impl<LANG, SPEC, T> VarComposable<LANG, SPEC> for FFIVariable<LANG, SPEC, T>
where Self: ToTokens + ToType,
      T: Clone + Debug + ToTokens,
      LANG: LangFermentable,
      SPEC: Specification<LANG> {}
