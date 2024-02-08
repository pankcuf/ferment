use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use syn::{parse_quote, Type};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
pub use crate::composition::{GenericBoundComposition, TypeComposition, TraitDecompositionPart1};
use crate::holder::PathHolder;

#[derive(Clone)]
pub enum TypeConversion {
    Trait(TypeComposition, TraitDecompositionPart1),
    TraitType(TypeComposition),
    TraitAssociatedType(TypeComposition),
    Object(TypeComposition),
    Primitive(TypeComposition),
    Bounds(GenericBoundComposition),
    SmartPointer(TypeComposition),
    FnPointer(TypeComposition),
    Unknown(TypeComposition),
    // Trait(TypeComposition, TraitDecompositionPart1),
    // Object(TypeComposition),
    // Primitive(TypeComposition),
    // Unknown(TypeComposition),
    // Trait(TypeComposition, TraitDecompositionPart1, Option<Generics>),
    // Object(TypeComposition, Option<Generics>),
    // Primitive(TypeComposition),
    // Unknown(TypeComposition, Option<Generics>),
}

impl ToTokens for TypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty().to_tokens(tokens)
    }
}

impl TypeConversion {
    pub fn ty_composition(&self) -> &TypeComposition {
        match self {
            TypeConversion::Trait(ty, ..) |
            TypeConversion::TraitType(ty) |
            TypeConversion::TraitAssociatedType(ty) |
            TypeConversion::Object(ty, ..) |
            TypeConversion::Primitive(ty) |
            TypeConversion::Bounds(GenericBoundComposition { type_composition: ty, .. }) |
            TypeConversion::SmartPointer(ty, ..) |
            TypeConversion::Unknown(ty, ..) |
            TypeConversion::FnPointer(ty, ..) => ty
        }
    }
    pub fn ty(&self) -> &Type {
        &self.ty_composition().ty
    }
    pub fn as_scope(&self) -> PathHolder {
        let ty = self.ty();
        parse_quote!(#ty)
    }
}

impl Debug for TypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeConversion::Trait(ty, _decomposition) =>
                f.write_str(format!("Trait({})", ty).as_str()),
            TypeConversion::Object(ty) =>
                f.write_str(format!("Object({})", ty).as_str()),
            TypeConversion::Unknown(ty) =>
                f.write_str(format!("Unknown({})", ty).as_str()),
            TypeConversion::Primitive(ty) =>
                f.write_str(format!("Primitive({})", ty).as_str()),
            TypeConversion::TraitType(ty) =>
                f.write_str(format!("TraitType({})", ty).as_str()),
            TypeConversion::Bounds(gbc) =>
                f.write_str(format!("Bounds({})", gbc).as_str()),
            TypeConversion::SmartPointer(ty) =>
                f.write_str(format!("SmartPointer({})", ty).as_str()),
            TypeConversion::TraitAssociatedType(ty) =>
                f.write_str(format!("TraitAssociatedType({})", ty).as_str()),
            TypeConversion::FnPointer(ty) =>
                f.write_str(format!("FnPointer({})", ty).as_str()),
        }
    }
}

impl Display for TypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for TypeConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ty().to_token_stream()];
        let other_tokens = [other.ty().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypeConversion {}

impl Hash for TypeConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty().to_token_stream().to_string().hash(state);
    }
}

