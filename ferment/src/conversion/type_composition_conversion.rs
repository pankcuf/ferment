use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use syn::{ParenthesizedGenericArguments, parse_quote, Path, PathArguments, PathSegment, Type, TypePath, TypeReference};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use crate::composer::CommaPunctuatedNestedArguments;
pub use crate::composable::{GenericBoundComposition, TypeComposition, TraitDecompositionPart1};
use crate::ext::{DictionaryType, Pop, ToType};

#[derive(Clone)]
pub enum DictionaryTypeCompositionConversion {
    Primitive(TypeComposition),
    LambdaFn(TypeComposition),
    NonPrimitiveFermentable(TypeComposition),
    NonPrimitiveOpaque(TypeComposition),
}
impl Debug for DictionaryTypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DictionaryTypeCompositionConversion::Primitive(ty) =>
                format!("Primitive({})", ty),
            DictionaryTypeCompositionConversion::NonPrimitiveFermentable(ty) =>
                format!("NonPrimitiveFermentable({})", ty),
            DictionaryTypeCompositionConversion::NonPrimitiveOpaque(ty) =>
                format!("NonPrimitiveOpaque({})", ty),
            DictionaryTypeCompositionConversion::LambdaFn(ty) =>
                format!("LambdaFn({})", ty),
        }.as_str())
    }
}

impl Display for DictionaryTypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub enum TypeCompositionConversion {
    Trait(TypeComposition, TraitDecompositionPart1, Vec<Path>),
    TraitType(TypeComposition),
    // TraitAssociatedType(TypeComposition),
    Object(TypeComposition),
    Optional(TypeComposition),
    Boxed(TypeComposition),
    // Primitive(TypeComposition),
    FnPointer(TypeComposition),
    // LambdaFn(TypeComposition),
    Bounds(GenericBoundComposition),
    // SmartPointer(TypeComposition),
    Fn(TypeComposition),

    Array(TypeComposition),
    Slice(TypeComposition),
    Tuple(TypeComposition),

    Unknown(TypeComposition),
    LocalOrGlobal(TypeComposition),

    Imported(TypeComposition, Path),
    Dictionary(DictionaryTypeCompositionConversion)
}


impl ToTokens for TypeCompositionConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.to_type().to_tokens(tokens)
    }
}

impl TypeCompositionConversion {

    pub fn is_unknown(&self) -> bool {
        match self {
            TypeCompositionConversion::Unknown(..) => true,
            _ => false
        }
    }
    pub fn is_dictionary_opaque(&self) -> bool {
        match self {
            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::NonPrimitiveOpaque(..)) => true,
            _ => false
        }
    }
    pub fn is_imported(&self) -> bool {
        match self {
            TypeCompositionConversion::Imported(..) => true,
            _ => false
        }
    }
    pub fn is_bounds(&self) -> bool {
        match self {
            TypeCompositionConversion::Bounds(..) => true,
            _ => false
        }
    }
    pub fn is_lambda(&self) -> bool {
        match self {
            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::LambdaFn(..)) => true,
            _ => false
        }
    }
    pub fn is_refined(&self) -> bool {
        match self {
            TypeCompositionConversion::Imported(..) |
            TypeCompositionConversion::Unknown(..) => false,
            other => {
                !other.nested_arguments()
                    .iter()
                    .find(|arg| arg.is_refined())
                    .is_some()
            },
        }
    }
    pub fn nested_arguments(&self) -> &CommaPunctuatedNestedArguments {
        &self.ty_composition().nested_arguments
    }
    pub fn replace_composition_type(&mut self, with_ty: Type) {
        match self {
            TypeCompositionConversion::Trait(ty, ..) |
            TypeCompositionConversion::TraitType(ty) |
            TypeCompositionConversion::Object(ty, ..) |
            TypeCompositionConversion::Boxed(ty, ..) |
            TypeCompositionConversion::Optional(ty, ..) |
            TypeCompositionConversion::FnPointer(ty) |
            TypeCompositionConversion::Bounds(GenericBoundComposition { type_composition: ty, .. }) |
            TypeCompositionConversion::Unknown(ty, ..) |
            TypeCompositionConversion::LocalOrGlobal(ty, ..) |
            TypeCompositionConversion::Array(ty) |
            TypeCompositionConversion::Slice(ty) |
            TypeCompositionConversion::Tuple(ty) |
            TypeCompositionConversion::Imported(ty, ..) |
            TypeCompositionConversion::Fn(ty, ..) |
            TypeCompositionConversion::Dictionary(
                DictionaryTypeCompositionConversion::Primitive(ty) |
                DictionaryTypeCompositionConversion::LambdaFn(ty) |
                DictionaryTypeCompositionConversion::NonPrimitiveFermentable(ty) |
                DictionaryTypeCompositionConversion::NonPrimitiveOpaque(ty)) => ty.ty = with_ty,
        }

    }
    pub fn ty_composition(&self) -> &TypeComposition {
        match self {
            TypeCompositionConversion::Trait(ty, ..) |
            TypeCompositionConversion::TraitType(ty) |
            TypeCompositionConversion::Object(ty, ..) |
            TypeCompositionConversion::Optional(ty, ..) |
            TypeCompositionConversion::Boxed(ty, ..) |
            TypeCompositionConversion::FnPointer(ty) |
            TypeCompositionConversion::Bounds(GenericBoundComposition { type_composition: ty, .. }) |
            TypeCompositionConversion::Unknown(ty, ..) |
            TypeCompositionConversion::LocalOrGlobal(ty, ..) |
            TypeCompositionConversion::Array(ty) |
            TypeCompositionConversion::Slice(ty) |
            TypeCompositionConversion::Tuple(ty) |
            TypeCompositionConversion::Imported(ty, ..) |
            TypeCompositionConversion::Fn(ty, ..) |
            TypeCompositionConversion::Dictionary(
                DictionaryTypeCompositionConversion::Primitive(ty) |
                DictionaryTypeCompositionConversion::LambdaFn(ty) |
                DictionaryTypeCompositionConversion::NonPrimitiveFermentable(ty) |
                DictionaryTypeCompositionConversion::NonPrimitiveOpaque(ty)) => ty,
        }
    }
    pub fn ty(&self) -> &Type {
        &self.ty_composition().ty
    }
    pub fn maybe_callback<'a>(&'a self) -> Option<&'a ParenthesizedGenericArguments> {
        if let TypeCompositionConversion::FnPointer(ty) | TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::LambdaFn(ty)) = self {
            if let Type::Path(TypePath { path, .. }) = &ty.ty {
                if let Some(PathSegment { arguments, ident: last_ident, ..}) = &path.segments.last() {
                    if last_ident.is_lambda_fn() {
                        if let PathArguments::Parenthesized(args) = arguments {
                            return Some(args)
                        }
                    }
                }
            }
        }
        None
    }

}

impl ToType for TypeCompositionConversion {
    fn to_type(&self) -> Type {
        match self {
            TypeCompositionConversion::Imported(ty, import_path) => {
                let ty = &ty.ty;
                let path = import_path.popped();
                match ty {
                    Type::Reference(TypeReference { elem, mutability, lifetime, .. }) => {
                        parse_quote!(&#mutability #path::#elem)
                    },
                    _ => parse_quote!(#path::#ty)
                }
            },
            _ => self.ty_composition().ty.clone()
        }
    }
}

impl Debug for TypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) =>
                format!("Trait({})", ty),
            TypeCompositionConversion::Object(ty) =>
                format!("Object({})", ty),
            TypeCompositionConversion::Optional(ty) =>
                format!("Optional({})", ty),
            TypeCompositionConversion::Boxed(ty) =>
                format!("Boxed({})", ty),
            TypeCompositionConversion::Unknown(ty) =>
               format!("Unknown({})", ty),
            TypeCompositionConversion::TraitType(ty) =>
                format!("TraitType({})", ty),
            TypeCompositionConversion::Bounds(gbc) =>
                format!("Bounds({})", gbc),
            TypeCompositionConversion::Fn(ty) =>
                format!("Fn({})", ty),
            TypeCompositionConversion::Imported(ty, import_path) =>
                format!("Imported({}, {})", ty, import_path.to_token_stream()),
            TypeCompositionConversion::Array(ty) =>
                format!("Array({})", ty),
            TypeCompositionConversion::Slice(ty) =>
                format!("Slice({})", ty),
            TypeCompositionConversion::Tuple(ty) =>
                format!("Tuple({})", ty),
            TypeCompositionConversion::LocalOrGlobal(ty) =>
                format!("LocalOrGlobal({})", ty),
            TypeCompositionConversion::FnPointer(ty) =>
                format!("FnPointer({})", ty),
            TypeCompositionConversion::Dictionary(ty) =>
                format!("Dictionary({})", ty),
        }.as_str())
    }
}

impl Display for TypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for TypeCompositionConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.to_type().to_token_stream()];
        let other_tokens = [other.to_type().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypeCompositionConversion {}

impl Hash for TypeCompositionConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_type().to_token_stream().to_string().hash(state);
    }
}
