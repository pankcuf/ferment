use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use syn::{Path, Type};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
pub use crate::composition::{GenericBoundComposition, TypeComposition, TraitDecompositionPart1};

#[derive(Clone)]
pub enum TypeCompositionConversion {
    Trait(TypeComposition, TraitDecompositionPart1, Vec<Path>),
    TraitType(TypeComposition),
    // TraitAssociatedType(TypeComposition),
    Object(TypeComposition),
    Primitive(TypeComposition),
    Bounds(GenericBoundComposition),
    SmartPointer(TypeComposition),
    Fn(TypeComposition),
    Tuple(TypeComposition),
    Unknown(TypeComposition),
}

impl ToTokens for TypeCompositionConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty().to_tokens(tokens)
    }
}

impl TypeCompositionConversion {
    pub fn ty_composition(&self) -> &TypeComposition {
        match self {
            TypeCompositionConversion::Trait(ty, ..) |
            TypeCompositionConversion::TraitType(ty) |
            // TypeCompositionConversion::TraitAssociatedType(ty) |
            TypeCompositionConversion::Object(ty, ..) |
            TypeCompositionConversion::Primitive(ty) |
            TypeCompositionConversion::Bounds(GenericBoundComposition { type_composition: ty, .. }) |
            TypeCompositionConversion::SmartPointer(ty, ..) |
            TypeCompositionConversion::Unknown(ty, ..) |
            TypeCompositionConversion::Tuple(ty) |
            TypeCompositionConversion::Fn(ty, ..) => ty,
        }
    }
    pub fn ty(&self) -> &Type {
        &self.ty_composition().ty
    }
}

impl Debug for TypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) =>
                f.write_str(format!("Trait({})", ty).as_str()),
            TypeCompositionConversion::Object(ty) =>
                f.write_str(format!("Object({})", ty).as_str()),
            TypeCompositionConversion::Unknown(ty) =>
                f.write_str(format!("Unknown({})", ty).as_str()),
            TypeCompositionConversion::Primitive(ty) =>
                f.write_str(format!("Primitive({})", ty).as_str()),
            TypeCompositionConversion::TraitType(ty) =>
                f.write_str(format!("TraitType({})", ty).as_str()),
            TypeCompositionConversion::Bounds(gbc) =>
                f.write_str(format!("Bounds({})", gbc).as_str()),
            TypeCompositionConversion::SmartPointer(ty) =>
                f.write_str(format!("SmartPointer({})", ty).as_str()),
            // TypeCompositionConversion::TraitAssociatedType(ty) =>
            //     f.write_str(format!("TraitAssociatedType({})", ty).as_str()),
            TypeCompositionConversion::Fn(ty) =>
                f.write_str(format!("Fn({})", ty).as_str()),
            TypeCompositionConversion::Tuple(ty) =>
                f.write_str(format!("Tuple({})", ty).as_str()),

        }
    }
}

impl Display for TypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for TypeCompositionConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ty().to_token_stream()];
        let other_tokens = [other.ty().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypeCompositionConversion {}

impl Hash for TypeCompositionConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty().to_token_stream().to_string().hash(state);
    }
}

