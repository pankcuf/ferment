use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, TraitBound, Type, TypePtr, TypeReference, TypeTraitObject};
use crate::ext::{MaybeTraitBound, ToType};
use crate::lang::Specification;
use crate::presentation::FFIVariable;

#[derive(Clone, Debug)]
pub enum ScopeSearchKey {
    Path(Path, Option<Box<ScopeSearchKey>>),
    Type(Type, Option<Box<ScopeSearchKey>>)
}
impl Display for ScopeSearchKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(path, original) =>
                f.write_fmt(format_args!("Path({}, {})", path.to_token_stream(), original.as_ref().map(|d| d.to_string()).unwrap_or_default())),
            Self::Type(ty, original) =>
                f.write_fmt(format_args!("Type({}, {})", ty.to_token_stream(), original.as_ref().map(|d| d.to_string()).unwrap_or_default())),
        }
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
        self.maybe_original_key().as_ref().is_some_and(|boxed_key| boxed_key.is_const_ptr() || boxed_key.is_mut_ptr())
    }
    pub fn maybe_originally_is_const_ptr(&self) -> bool {
        self.maybe_original_key().as_ref().is_some_and(|boxed_key| boxed_key.is_const_ptr())
    }
    pub fn maybe_originally_is_mut_ptr(&self) -> bool {
        self.maybe_original_key().as_ref().is_some_and(|boxed_key| boxed_key.is_mut_ptr())
    }
    pub fn maybe_originally_is_ref(&self) -> bool {
        self.maybe_original_key().as_ref().is_some_and(|boxed_key| boxed_key.is_ref())
    }
    pub fn maybe_originally_is_mut_ref(&self) -> bool {
        self.maybe_original_key().as_ref().is_some_and(|boxed_key| boxed_key.is_mut_ref())
    }
    pub fn maybe_originally_is_dyn(&self) -> bool {
        self.maybe_original_key().as_ref().is_some_and(|boxed_key| boxed_key.is_dyn())
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
        matches!(self, ScopeSearchKey::Type(Type::Reference(TypeReference { mutability: Some(_), .. }), ..))
    }
    pub fn is_dyn(&self) -> bool {
        matches!(self, ScopeSearchKey::Type(Type::TraitObject(TypeTraitObject { .. }), ..))
    }
}

impl ScopeSearchKey {
    pub fn trait_bound(trait_bound: &TraitBound, original: Self) -> Self {
        Self::Path(trait_bound.path.clone(), Some(Box::new(original)))
    }
    pub fn r#type(ty: &Type, original: Self) -> Self {
        Self::Type(ty.clone(), Some(Box::new(original)))
    }
    pub fn maybe_from_ref(ty: &Type) -> Option<Self> {
        let original = ScopeSearchKey::Type(ty.clone(), None);
        match ty {
            Type::TraitObject(TypeTraitObject { bounds , ..}) =>
                bounds.first()
                    .and_then(MaybeTraitBound::maybe_trait_bound)
                    .map(|trait_bound| ScopeSearchKey::trait_bound(trait_bound, original)),
            Type::Reference(TypeReference { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) =>
                Some(ScopeSearchKey::r#type(elem, original)),
            ty =>
                Some(ScopeSearchKey::r#type(ty, original)),
        }

    }
    pub fn maybe_from(ty: Type) -> Option<Self> {
        let original = ScopeSearchKey::Type(ty.clone(), None);
        match ty {
            Type::TraitObject(TypeTraitObject { bounds , .. }) => match bounds.len() {
                1 => bounds.first()
                    .and_then(MaybeTraitBound::maybe_trait_bound)
                    .map(|trait_bound| ScopeSearchKey::trait_bound(trait_bound, original)),
                _ => None
            },
            Type::Reference(TypeReference { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) =>
                Some(ScopeSearchKey::Type(*elem, Some(Box::new(original)))),
            ty =>
                Some(ScopeSearchKey::Type(ty, Some(Box::new(original)))),
        }
    }
    pub fn find<K, T: Fn(&Type) -> K>(&self, finder: T) -> K {
        match self {
            ScopeSearchKey::Path(ref path, ..) => finder(&path.to_type()),
            ScopeSearchKey::Type(ty, ..) => finder(ty),
        }
    }
}
