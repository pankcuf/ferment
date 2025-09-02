use syn::Type;
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use crate::kind::{ObjectKind, TypeModelKind};
use crate::ext::{AsType, ToType};

#[derive(Clone)]
pub enum NestedArgument {
    Object(ObjectKind),
    Constraint(ObjectKind)
}

impl NestedArgument {
    pub fn is_refined(&self) -> bool {
        match self.object() {
            ObjectKind::Type(ty) => !ty.is_refined(),
            _ => false
        }
    }
    pub fn object(&self) -> &ObjectKind {
        match self {
            NestedArgument::Object(obj) |
            NestedArgument::Constraint(obj) => obj
        }
    }
    pub fn object_mut(&mut self) -> &mut ObjectKind {
        match self {
            NestedArgument::Object(obj) |
            NestedArgument::Constraint(obj) => obj
        }
    }
    pub fn maybe_type_model_kind_ref(&self) -> Option<&TypeModelKind> {
        self.object().maybe_type_model_kind_ref()
    }

    pub fn ty(&self) -> Option<&Type> {
        self.maybe_type_model_kind_ref()
            .map(TypeModelKind::as_type)
    }
    pub fn maybe_type(&self) -> Option<Type> {
        self.maybe_type_model_kind_ref()
            .map(TypeModelKind::to_type)
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
