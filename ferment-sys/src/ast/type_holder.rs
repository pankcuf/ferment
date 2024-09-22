use quote::ToTokens;
use syn::Type;
use crate::ast::Holder;
use crate::conversion::TypeModelKind;
use crate::ext::ToType;
use crate::impl_holder;

impl_holder!(TypeHolder, Type);

impl<'a> From<&'a TypeModelKind> for TypeHolder {
    fn from(value: &'a TypeModelKind) -> Self {
        TypeHolder(value.to_type())
    }
}
impl<'a> From<&'a Box<Type>> for TypeHolder {
    fn from(value: &'a Box<Type>) -> Self {
        TypeHolder(*value.clone())
    }
}