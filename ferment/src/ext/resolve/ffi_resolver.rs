use std::fmt::{Debug, Display, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, Type};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, ObjectKind, TypeModelKind};
use crate::ext::{Accessory, Resolve, ToPath, ToType};
use crate::presentation::FFIFullPath;

#[derive(Debug)]
pub enum SpecialType {
    Custom(Type),
    Opaque(Type),
}

impl Display for SpecialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SpecialType::Custom(ty) => format!("Custom({})", ty.to_token_stream()),
            SpecialType::Opaque(ty) => format!("Opaque({})", ty.to_token_stream()),
        }.as_str())
    }
}

impl ToTokens for SpecialType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}
impl ToType for SpecialType {
    fn to_type(&self) -> Type {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.clone()
        }
    }
}
impl ToPath for SpecialType {
    fn to_path(&self) -> Path {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.to_path()
        }
    }
}

pub trait FFITypeResolve {
    fn full_type(&self, source: &ScopeContext) -> Type;
}

impl FFITypeResolve for Type {
    fn full_type(&self, source: &ScopeContext) -> Type {
        self.resolve(source)
    }
}

pub trait FFISpecialTypeResolve {
    /// Types that are exported with [ferment_macro::register] or [ferment_macro::opaque]
    /// so it's custom conversion or opaque pointer therefore we should use direct paths for ffi export
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType>;
}
impl FFISpecialTypeResolve for Type {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType> {
        self.resolve(source)
    }
}

pub trait FFIObjectResolve {
    fn maybe_object(&self, source: &ScopeContext) -> Option<ObjectKind>;
}

impl FFIObjectResolve for Type {
    fn maybe_object(&self, source: &ScopeContext) -> Option<ObjectKind> {
        source.maybe_object_by_key(self)
    }
}

pub trait FFITypeModelKindResolve {
    fn type_model_kind(&self, source: &ScopeContext) -> TypeModelKind;
}

impl FFITypeModelKindResolve for Type {
    fn type_model_kind(&self, source: &ScopeContext) -> TypeModelKind {
        self.resolve(source)
    }
}

pub trait FFIVarResolve: Resolve<FFIFullPath> + Resolve<Option<SpecialType>> + ToTokens {
    fn ffi_full_path(&self, source: &ScopeContext) -> FFIFullPath {
        self.resolve(source)
    }
    fn special_or_to_ffi_full_path_type(&self, source: &ScopeContext) -> Type {
        // println!("special_or_to_ffi_full_path_type:: {}", self.to_token_stream());
        let maybe_special_type: Option<SpecialType> = self.resolve(source);
        let res = maybe_special_type
            .map(|special| special.to_type())
            .unwrap_or_else(|| self.ffi_full_path(source).to_type());
        // println!("special_or_to_ffi_full_path_type.222:: {}", res.to_type().to_token_stream());
        res
    }
    fn special_or_to_ffi_full_path_variable_type(&self, source: &ScopeContext) -> Type {
        self.special_or_to_ffi_full_path_type(source)
            .joined_mut()
    }
}
impl FFIVarResolve for Type {}
impl FFIVarResolve for GenericTypeKind {}

