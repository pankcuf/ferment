use std::fmt::{Debug, Display, Formatter};
use quote::{quote, ToTokens};
use crate::composer::DestructorContext;

#[derive(Clone)]
pub enum ConstructorPresentableContext {
    EnumVariant(DestructorContext),
    Default(DestructorContext)
}

impl Debug for ConstructorPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnumVariant((ty, attrs, generics)) =>
                f.write_str(format!("EnumVariant({}, {}, {})", ty.to_token_stream(), quote!(#(#attrs)*), generics.to_token_stream()).as_str()),
            Self::Default((ty, attrs, generics)) =>
                f.write_str(format!("Default({}, {}, {})", ty.to_token_stream(), quote!(#(#attrs)*), generics.to_token_stream()).as_str()),
        }
    }
}
impl Display for ConstructorPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
