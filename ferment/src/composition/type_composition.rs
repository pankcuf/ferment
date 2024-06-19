use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Generics, Type};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::conversion::ObjectConversion;

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
