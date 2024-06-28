use quote::ToTokens;
use syn::Type;
use crate::ast::Holder;
use crate::conversion::TypeCompositionConversion;
use crate::ext::ToType;
use crate::impl_holder;

impl_holder!(TypeHolder, Type);

impl<'a> From<&'a TypeCompositionConversion> for TypeHolder {
    fn from(value: &'a TypeCompositionConversion) -> Self {
        TypeHolder(value.to_type())
    }
}
impl<'a> From<&'a Box<Type>> for TypeHolder {
    fn from(value: &'a Box<Type>) -> Self {
        TypeHolder(*value.clone())
    }
}