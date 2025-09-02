use std::fmt::{Debug, Display, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Type;
use crate::composable::{TypeModel, TypeModeled};
use crate::kind::{GroupModelKind, SmartPointerModelKind};
use crate::ext::{AsType, ToType};

#[derive(Clone)]
pub enum DictFermentableModelKind {
    SmartPointer(SmartPointerModelKind),
    Group(GroupModelKind),
    String(TypeModel),
    Str(TypeModel),
    Other(TypeModel),
    Cow(TypeModel),
    I128(TypeModel),
    U128(TypeModel),
}
impl ToType for DictFermentableModelKind {
    fn to_type(&self) -> Type {
        self.as_type().clone()
    }
}
impl<'a> AsType<'a> for DictFermentableModelKind {
    fn as_type(&'a self) -> &'a Type {
        match self {
            DictFermentableModelKind::SmartPointer(kind) => kind.as_type(),
            DictFermentableModelKind::Group(kind) => kind.as_type(),
            DictFermentableModelKind::Str(model) |
            DictFermentableModelKind::String(model) |
            DictFermentableModelKind::Other(model) |
            DictFermentableModelKind::Cow(model) |
            DictFermentableModelKind::I128(model) |
            DictFermentableModelKind::U128(model) => model.as_type(),
        }
    }
}

impl TypeModeled for DictFermentableModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            DictFermentableModelKind::SmartPointer(kind) => kind.type_model_mut(),
            DictFermentableModelKind::Group(kind) => kind.type_model_mut(),
            DictFermentableModelKind::Str(model) |
            DictFermentableModelKind::String(model) |
            DictFermentableModelKind::I128(model) |
            DictFermentableModelKind::U128(model) |
            DictFermentableModelKind::Cow(model) |
            DictFermentableModelKind::Other(model) => model
        }
    }
    fn type_model_ref(&self) -> &TypeModel {
        match self {
            DictFermentableModelKind::SmartPointer(kind) => kind.type_model_ref(),
            DictFermentableModelKind::Group(kind) => kind.type_model_ref(),
            DictFermentableModelKind::Str(model) |
            DictFermentableModelKind::String(model) |
            DictFermentableModelKind::I128(model) |
            DictFermentableModelKind::U128(model) |
            DictFermentableModelKind::Cow(model) |
            DictFermentableModelKind::Other(model) => model
        }
    }
}

impl Debug for DictFermentableModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DictFermentableModelKind::SmartPointer(model) =>
                format!("SmartPointer({})", model),
            DictFermentableModelKind::Group(model) =>
                format!("Group({})", model),
            DictFermentableModelKind::Str(model) =>
                format!("Str({})", model),
            DictFermentableModelKind::String(model) =>
                format!("String({})", model),
            DictFermentableModelKind::Cow(model) =>
                format!("Cow({})", model),
            DictFermentableModelKind::Other(model) =>
                format!("Other({})", model),
            DictFermentableModelKind::I128(model) =>
                format!("Digit128({})", model),
            DictFermentableModelKind::U128(model) =>
                format!("Digit128({})", model),
        }.as_str())
    }
}

impl Display for DictFermentableModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for DictFermentableModelKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_type()
            .to_tokens(tokens)
    }
}
