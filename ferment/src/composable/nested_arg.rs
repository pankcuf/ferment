use syn::Type;
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::ToType;

#[derive(Clone)]
pub enum NestedArgument {
    Object(ObjectConversion),
    Constraint(ObjectConversion)
}

impl NestedArgument {
    pub fn is_refined(&self) -> bool {
        match self.object() {
            ObjectConversion::Type(ty) => !ty.is_refined(),
            _ => false
        }
    }
    pub fn object(&self) -> &ObjectConversion {
        match self {
            NestedArgument::Object(obj) |
            NestedArgument::Constraint(obj) => obj
        }
    }
    pub fn object_mut(&mut self) -> &mut ObjectConversion {
        match self {
            NestedArgument::Object(obj) |
            NestedArgument::Constraint(obj) => obj
        }
    }
    pub fn type_conversion(&self) -> Option<&TypeCompositionConversion> {
        self.object().type_conversion()
    }

    pub fn ty(&self) -> Option<&Type> {
        self.type_conversion()
            .map(TypeCompositionConversion::ty)
    }
    pub fn maybe_type(&self) -> Option<Type> {
        self.type_conversion()
            .map(TypeCompositionConversion::to_type)
    }
}

impl ToTokens for NestedArgument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            NestedArgument::Object(obj) |
            NestedArgument::Constraint(obj) => obj.to_tokens(tokens),
        }
    }
}

impl Debug for NestedArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NestedArgument::Object(obj) => format!("Object({})", obj),
            NestedArgument::Constraint(obj) => format!("Constraint({})", obj)
        }.as_str())
    }
}

impl Display for NestedArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
