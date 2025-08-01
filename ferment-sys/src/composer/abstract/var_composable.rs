use std::fmt::Debug;
use quote::ToTokens;
use crate::ext::ToType;
use crate::lang::Specification;
use crate::presentation::FFIVariable;

pub trait VarComposable<SPEC>: Clone + Debug + ToTokens + ToType
where SPEC: Specification {}
impl<SPEC, T> VarComposable<SPEC> for FFIVariable<SPEC, T>
where Self: ToTokens + ToType,
      T: Clone + Debug + ToTokens,
      SPEC: Specification {}
