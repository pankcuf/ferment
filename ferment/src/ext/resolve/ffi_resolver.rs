use std::fmt::Debug;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, Type};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion};
use crate::ext::{Accessory, Resolve, ToPath, ToType};
use crate::presentation::FFIFullPath;

#[derive(Debug)]
pub enum SpecialType {
    Custom(Type),
    Opaque(Type),
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
    fn maybe_object(&self, source: &ScopeContext) -> Option<ObjectConversion>;
}

impl FFIObjectResolve for Type {
    fn maybe_object(&self, source: &ScopeContext) -> Option<ObjectConversion> {
        source.maybe_object(self)
    }
}

pub trait FFICompositionResolve {
    fn composition(&self, source: &ScopeContext) -> TypeCompositionConversion;
}

impl FFICompositionResolve for Type {
    fn composition(&self, source: &ScopeContext) -> TypeCompositionConversion {
        self.resolve(source)
    }
}

pub trait FFIVarResolve: Resolve<FFIFullPath> + Resolve<Option<SpecialType>> {
    fn special_or_to_ffi_full_path_type(&self, source: &ScopeContext) -> Type where Self: Debug {
        println!("special_or_to_ffi_full_path_type:: {:?}", self);
        let res = <Self as Resolve<Option<SpecialType>>>::resolve(self, source)
            .map(|special| {
                println!("spec:: {:?}", special);

                special.to_type()
            })
            .unwrap_or_else(|| {
                println!("else:");
                <Self as Resolve::<FFIFullPath>>::resolve(self, source).to_type()
            });
        println!("special_or_to_ffi_full_path_type.222:: {:?}", self);
        res
    }
    fn special_or_to_ffi_full_path_variable_type(&self, source: &ScopeContext) -> Type where Self: Debug {
        self.special_or_to_ffi_full_path_type(source)
            .joined_mut()
    }
}
impl FFIVarResolve for Type {}
impl FFIVarResolve for GenericTypeConversion {}

