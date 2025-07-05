use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, Type};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, ObjectKind};
use crate::ext::{Accessory, LifetimeProcessor, Resolve, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

#[derive(Debug)]
pub enum SpecialType<SPEC>
    where SPEC: Specification {
    Custom(Type),
    Opaque(Type),
    Phantom(PhantomData<SPEC>)
}

impl<SPEC> Display for SpecialType<SPEC>
    where SPEC: Specification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SpecialType::Custom(ty) => format!("Custom({})", ty.to_token_stream()),
            SpecialType::Opaque(ty) => format!("Opaque({})", ty.to_token_stream()),
            SpecialType::Phantom(..) => "Phantom".to_string(),
        }.as_str())
    }
}

impl<SPEC> ToTokens for SpecialType<SPEC>
    where SPEC: Specification {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}
impl<SPEC> ToType for SpecialType<SPEC>
    where SPEC: Specification {
    fn to_type(&self) -> Type {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.clone(),
            _ => panic!("")
        }
    }
}
impl<SPEC> ToPath for SpecialType<SPEC>
    where SPEC: Specification {
    fn to_path(&self) -> Path {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.to_path(),
            _ => panic!()
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

pub trait FFISpecialTypeResolve<SPEC>
    where SPEC: Specification {
    /// Types that are exported with [ferment_macro::register] or [ferment_macro::opaque]
    /// so it's custom conversion or opaque pointer therefore we should use direct paths for ffi export
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>>;
}
impl<SPEC> FFISpecialTypeResolve<SPEC> for Type
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        self.maybe_resolve(source)
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

// pub trait FFITypeModelKindResolve {
//     fn type_model_kind(&self, source: &ScopeContext) -> TypeModelKind;
// }
//
// impl FFITypeModelKindResolve for Type {
//     fn type_model_kind(&self, source: &ScopeContext) -> TypeModelKind {
//         self.resolve(source)
//     }
// }

pub trait FFIVarResolve<SPEC>: Clone + LifetimeProcessor + Resolve<FFIFullPath<SPEC>> + Resolve<SpecialType<SPEC>> + ToTokens
    where SPEC: Specification,
          FFIFullPath<SPEC>: ToType {
    fn ffi_full_path(&self, source: &ScopeContext) -> FFIFullPath<SPEC> {
        self.resolve(source)
    }
    fn special_or_to_ffi_full_path_type(&self, source: &ScopeContext) -> Type {
        let maybe_special_type: Option<SpecialType<SPEC>> = self.lifetimes_cleaned().maybe_resolve(source);
        maybe_special_type
            .map(|special| special.to_type())
            .unwrap_or_else(|| self.ffi_full_path(source).to_type())
    }
    fn special_or_to_ffi_full_path_variable_type(&self, source: &ScopeContext) -> Type {
        self.special_or_to_ffi_full_path_type(source)
            .joined_mut()
    }
}
impl<SPEC> FFIVarResolve<SPEC> for Type
    where SPEC: Specification,
          FFIFullPath<SPEC>: ToType,
          FFIFullDictionaryPath<SPEC>: ToType{}
impl FFIVarResolve<RustSpecification> for GenericTypeKind {}

#[cfg(feature = "objc")]
impl FFIVarResolve<crate::lang::objc::ObjCSpecification> for GenericTypeKind {}

