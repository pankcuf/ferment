use quote::ToTokens;
use syn::Type;
use crate::conversion::TypeConversion;
use crate::holder::Holder;
use crate::impl_holder;

impl_holder!(TypeHolder, Type);

impl<'a> From<&'a TypeConversion> for TypeHolder {
    fn from(value: &'a TypeConversion) -> Self {
        TypeHolder(value.ty().clone())
    }
}
impl<'a> From<&'a Box<Type>> for TypeHolder {
    fn from(value: &'a Box<Type>) -> Self {
        TypeHolder(*value.clone())
    }
}
