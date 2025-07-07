use std::fmt::Debug;
use quote::ToTokens;
use syn::Type;
use crate::ext::{Accessory, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentation::FFIVariable;

pub trait VarComposable<SPEC>: Clone + Debug + ToTokens + ToType
where SPEC: Specification {}
impl<SPEC, T> VarComposable<SPEC> for FFIVariable<SPEC, T>
where Self: ToTokens + ToType,
      T: Clone + Debug + ToTokens,
      SPEC: Specification {}


// impl<SPEC: Specification, T: VarComposable<SPEC>> Accessory for T {
//     fn joined_mut(&self) -> Self {
//         let ty = self.to_type();
//         parse_quote!(*mut #ty)
//     }
//     fn joined_const(&self) -> Self {
//         let ty = self.to_type();
//         parse_quote!(*const #ty)
//     }
//     fn joined_dyn(&self) -> Self {
//         let ty = self.to_type();
//         parse_quote!(dyn #ty)
//     }
//     fn joined_ref(&self) -> Self {
//         let ty = self.to_type();
//         parse_quote!(&#ty)
//     }
//     fn joined_mut_ref(&self) -> Self {
//         let ty = self.to_type();
//         parse_quote!(&mut #ty)
//     }
// }

impl Accessory for FFIVariable<RustSpecification, Type> {
    fn joined_mut(&self) -> Self {
        FFIVariable::mut_ptr(self.to_type())
    }
    fn joined_const(&self) -> Self {
        FFIVariable::const_ptr(self.to_type())
    }
    fn joined_dyn(&self) -> Self {
        FFIVariable::r#dyn(self.to_type())
    }
    fn joined_ref(&self) -> Self {
        FFIVariable::r#ref(self.to_type())
    }
    fn joined_mut_ref(&self) -> Self {
        FFIVariable::mut_ref(self.to_type())
    }
}
