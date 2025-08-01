use std::marker::PhantomData;
use quote::ToTokens;
use syn::Type;
use ferment_macro::Display;
use crate::kind::SpecialType;
use crate::ext::ToType;
use crate::lang::Specification;

pub trait ToFFIVariable<SPEC, T>
    where T: ToTokens,
          SPEC: Specification {
    fn to_direct_var(&self) -> FFIVariable<SPEC, T>;
    fn to_dyn_var(&self) -> FFIVariable<SPEC, T>;
}

impl<SPEC> ToFFIVariable<SPEC, Type> for Type where SPEC: Specification {
    fn to_direct_var(&self) -> FFIVariable<SPEC, Type> {
        FFIVariable::direct(self.clone())
    }

    fn to_dyn_var(&self) -> FFIVariable<SPEC, Type> {
        FFIVariable::r#dyn(self.clone())
    }
}

impl<SPEC> ToFFIVariable<SPEC, Type> for SpecialType<SPEC> where SPEC: Specification {
    fn to_direct_var(&self) -> FFIVariable<SPEC, Type> {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.to_direct_var(),
            _ => panic!("")
        }
    }

    fn to_dyn_var(&self) -> FFIVariable<SPEC, Type> {
        self.to_type().to_dyn_var()
    }
}

#[derive(Clone, Display, Debug)]
pub enum FFIVariable<SPEC, T>
    where T: ToTokens,
          SPEC: Specification {
    Direct { ty: T, _marker: PhantomData<SPEC> },
    ConstPtr { ty: T, _marker: PhantomData<SPEC> },
    MutPtr { ty: T, _marker: PhantomData<SPEC> },
    Ref { ty: T, _marker: PhantomData<SPEC> },
    MutRef { ty: T, _marker: PhantomData<SPEC> },
    Dyn { ty: T, _marker: PhantomData<SPEC> },
}

impl<SPEC, T> FFIVariable<SPEC, T>
    where T: ToTokens,
          SPEC: Specification {
    pub(crate) fn direct(ty: T) -> Self {
        Self::Direct { ty, _marker: PhantomData }
    }
    pub(crate) fn const_ptr(ty: T) -> Self {
        Self::ConstPtr { ty, _marker: PhantomData }
    }
    pub(crate) fn mut_ptr(ty: T) -> Self {
        Self::MutPtr { ty, _marker: PhantomData }
    }
    pub(crate) fn r#ref(ty: T) -> Self {
        Self::Ref { ty, _marker: PhantomData }
    }
    pub(crate) fn mut_ref(ty: T) -> Self {
        Self::MutRef { ty, _marker: PhantomData }
    }
    pub(crate) fn r#dyn(ty: T) -> Self {
        Self::Dyn { ty, _marker: PhantomData }
    }
}

