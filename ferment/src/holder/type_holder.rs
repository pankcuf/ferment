use quote::ToTokens;
use syn::Type;
use crate::conversion::TypeCompositionConversion;
use crate::holder::Holder;
use crate::impl_holder;

impl_holder!(TypeHolder, Type);

impl<'a> From<&'a TypeCompositionConversion> for TypeHolder {
    fn from(value: &'a TypeCompositionConversion) -> Self {
        TypeHolder(value.ty().clone())
    }
}
impl<'a> From<&'a Box<Type>> for TypeHolder {
    fn from(value: &'a Box<Type>) -> Self {
        TypeHolder(*value.clone())
    }
}
