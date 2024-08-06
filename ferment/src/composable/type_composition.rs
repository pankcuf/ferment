use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Generics, Path, Type, TypePtr, TypeReference};
use syn::punctuated::Punctuated;
use crate::composable::nested_arg::NestedArgument;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::ext::{ToPath, ToType};

#[derive(Clone)]
pub struct TypeComposition {
    pub ty: Type,
    pub generics: Option<Generics>,
    pub nested_arguments: CommaPunctuatedNestedArguments,
}

impl TypeComposition {
    pub fn new_non_gen(ty: Type, generics: Option<Generics>) -> Self {
        Self { ty, generics, nested_arguments: Punctuated::new() }
    }
    pub fn new(ty: Type, generics: Option<Generics>, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self { ty, generics, nested_arguments }
    }
    pub fn nested_argument_at_index(&self, index: usize) -> &NestedArgument {
        &self.nested_arguments[index]
    }

    pub fn pointer_less(&self) -> Path {
        match &self.ty {
            Type::Reference(TypeReference { elem, ..}) |
            Type::Ptr(TypePtr { elem, ..}) =>
                elem.to_path(),
            other =>
                other.to_path()
        }
    }
}

impl ToType for TypeComposition {
    fn to_type(&self) -> Type {
        self.ty.clone()
    }
}

impl Debug for TypeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("$Ty({}, {:?})",
                    self.ty.to_token_stream(),
                    self.nested_arguments,
                    // self.generics.as_ref().map_or(format!("None"), format_token_stream)
                ).as_str())
    }
}

impl Display for TypeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
