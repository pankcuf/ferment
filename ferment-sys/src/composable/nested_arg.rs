use syn::{Generics, TraitBound, Type};
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use crate::composable::TypeModel;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::kind::{ObjectKind, TypeModelKind};
use crate::ext::{AsType, ToType};

#[derive(Clone)]
pub enum NestedArgument {
    Object(ObjectKind),
    Constraint(ObjectKind)
}
impl NestedArgument {
    pub fn trait_bound_object(trait_bound: &TraitBound) -> Self {
        Self::Object(ObjectKind::trait_model_type(TypeModel::new_default_from_trait_bound(trait_bound)))
    }

    pub fn trait_model_constraint(ident: &Ident, generics: &Generics, arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::Constraint(ObjectKind::trait_model_type(TypeModel::new_generic_ident(ident, generics.clone(), arguments)))
    }
    pub fn object_model_constraint(ident: &Ident, generics: &Generics) -> Self {
        Self::Constraint(ObjectKind::object_model_type(TypeModel::new_generic_ident_non_nested(ident, generics)))
    }
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
        match self {
            NestedArgument::Object(obj) =>
                f.write_fmt(format_args!("Object({obj})")),
            NestedArgument::Constraint(obj) =>
                f.write_fmt(format_args!("Object({obj})")),
        }
    }
}

impl Display for NestedArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
