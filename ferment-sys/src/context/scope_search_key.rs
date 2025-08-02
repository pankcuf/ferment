use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, TraitBound, Type, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject};
use crate::ext::ToType;
use crate::lang::Specification;
use crate::presentation::FFIVariable;

#[derive(Clone, Debug)]
pub enum ScopeSearchKey {
    Path(Path, Option<Box<ScopeSearchKey>>),
    // TypeRef(&'static Type, Option<Box<ScopeSearchKey>>),
    Type(Type, Option<Box<ScopeSearchKey>>)
}
impl Display for ScopeSearchKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Path(path, original) => format!("Path({}, {})", path.to_token_stream(), original.as_ref().map(|d| d.to_string()).unwrap_or_default()),
            Self::Type(ty, original) => format!("Type({}, {})", ty.to_token_stream(), original.as_ref().map(|d| d.to_string()).unwrap_or_default()),
        }.as_str())
    }
}

impl ToTokens for ScopeSearchKey {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Path(path, ..) => path.to_tokens(tokens),
            Self::Type(ty, ..) => ty.to_tokens(tokens)
        }
    }
}

impl ToType for ScopeSearchKey {
    fn to_type(&self) -> Type {
        match self {
            ScopeSearchKey::Path(path, ..) => path.to_type(),
            ScopeSearchKey::Type(ty, ..) => ty.clone()
        }
    }
}

impl ScopeSearchKey {
    pub fn ptr_composer<SPEC, T>(&self) -> fn(T) -> FFIVariable<SPEC, T>
    where SPEC: Specification,
          T: ToTokens {
        if self.maybe_originally_is_const_ptr() {
            FFIVariable::const_ptr
        } else {
            FFIVariable::mut_ptr
        }
    }
}


impl ScopeSearchKey {
    pub fn maybe_original_key(&self) -> &Option<Box<ScopeSearchKey>> {
        match self {
            ScopeSearchKey::Path(_, original) |
            ScopeSearchKey::Type(_, original) => original
        }
    }
    pub fn maybe_originally_is_ptr(&self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_const_ptr() || boxed_key.is_mut_ptr(),
            _ => false
        }
    }
    pub fn maybe_originally_is_const_ptr(&self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_const_ptr(),
            _ => false
        }
    }
    pub fn maybe_originally_is_mut_ptr(&self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_mut_ptr(),
            _ => false
        }
    }
    pub fn maybe_originally_is_ref(&self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_ref(),
            _ => false
        }
    }
    pub fn maybe_originally_is_mut_ref(&self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_mut_ref(),
            _ => false
        }
    }
    pub fn maybe_originally_is_dyn(&self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_dyn(),
            _ => false
        }
    }
    pub fn maybe_ptr(&self) -> Option<&TypePtr> {
        match self {
            ScopeSearchKey::Type(Type::Ptr(type_ptr), ..) => Some(type_ptr),
            _ => None
        }
    }
    pub fn maybe_dyn(&self) -> Option<&TypeTraitObject> {
        match self {
            ScopeSearchKey::Type(Type::TraitObject(ty), ..) => Some(ty),
            _ => None
        }
    }
    pub fn maybe_ref(&self) -> Option<&TypeReference> {
        match self {
            ScopeSearchKey::Type(Type::Reference(type_ref), ..) => Some(type_ref),
            _ => None
        }
    }
    pub fn is_const_ptr(&self) -> bool {
        self.maybe_ptr().is_some_and(|ty| ty.const_token.is_some())
    }
    pub fn is_mut_ptr(&self) -> bool {
        self.maybe_ptr().is_some_and(|ty| ty.mutability.is_some())
    }
    pub fn is_ref(&self) -> bool {
        self.maybe_ref().is_some()
    }
    pub fn is_mut_ref(&self) -> bool {
        match self {
            ScopeSearchKey::Type(Type::Reference(TypeReference { mutability: Some(_mutable), .. }), ..) => true,
            _ => false
        }
    }
    pub fn is_dyn(&self) -> bool {
        match self {
            ScopeSearchKey::Type(Type::TraitObject(TypeTraitObject { .. }), ..) => true,
            _ => false
        }
    }
}

impl ScopeSearchKey {
    pub fn maybe_from_ref(ty: &Type) -> Option<Self> {
        let original = ScopeSearchKey::Type(ty.clone(), None);
        match ty {
            Type::TraitObject(TypeTraitObject { bounds , ..}) => match bounds.first()? {
                TypeParamBound::Trait(TraitBound { path, .. }) =>
                    Some(ScopeSearchKey::Path(path.clone(), Some(Box::new(original)))),
                _ =>
                    None
            },
            Type::Reference(TypeReference { elem: ty, .. }) |
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                Some(ScopeSearchKey::Type(*ty.clone(), Some(Box::new(original)))),
            ty =>
                Some(ScopeSearchKey::Type(ty.clone(), Some(Box::new(original)))),
        }

    }
    pub fn maybe_from(ty: Type) -> Option<Self> {
        let original = ScopeSearchKey::Type(ty.clone(), None);
        match ty {
            Type::TraitObject(TypeTraitObject { bounds , .. }) => match bounds.len() {
                1 => match bounds.first()? {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        Some(ScopeSearchKey::Type(path.to_type(), Some(Box::new(original)))),
                    _ =>
                        None
                },
                _ => None
            },
            Type::Reference(TypeReference { elem: ty, .. }) |
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                Some(ScopeSearchKey::Type(*ty, Some(Box::new(original)))),
            ty =>
                Some(ScopeSearchKey::Type(ty, Some(Box::new(original)))),
        }
    }
    pub fn find<K, T: Fn(&Type) -> K>(&self, finder: T) -> K {
        match self {
            ScopeSearchKey::Path(ref path, ..) => finder(&Type::Path(TypePath { qself: None, path: (*path).clone() })),
            ScopeSearchKey::Type(ty, ..) => finder(ty),
        }
    }
}
