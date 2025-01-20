use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, Type};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, ObjectKind, TypeModelKind};
use crate::ext::{Accessory, LifetimeProcessor, Resolve, ToPath, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, RustFermentate};

#[derive(Debug)]
pub enum SpecialType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    Custom(Type),
    Opaque(Type),
    Phantom(PhantomData<(LANG, SPEC)>)
}

impl<LANG, SPEC> Display for SpecialType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SpecialType::Custom(ty) => format!("Custom({})", ty.to_token_stream()),
            SpecialType::Opaque(ty) => format!("Opaque({})", ty.to_token_stream()),
            SpecialType::Phantom(..) => "Phantom".to_string(),
        }.as_str())
    }
}

impl<LANG, SPEC> ToTokens for SpecialType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}
impl<LANG, SPEC> ToType for SpecialType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn to_type(&self) -> Type {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.clone(),
            _ => panic!("")
        }
    }
}
impl<LANG, SPEC> ToPath for SpecialType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
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

pub trait FFISpecialTypeResolve<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    /// Types that are exported with [ferment_macro::register] or [ferment_macro::opaque]
    /// so it's custom conversion or opaque pointer therefore we should use direct paths for ffi export
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType<LANG, SPEC>>;
}
impl<LANG, SPEC> FFISpecialTypeResolve<LANG, SPEC> for Type
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType<LANG, SPEC>> {
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

pub trait FFITypeModelKindResolve {
    fn type_model_kind(&self, source: &ScopeContext) -> TypeModelKind;
}

impl FFITypeModelKindResolve for Type {
    fn type_model_kind(&self, source: &ScopeContext) -> TypeModelKind {
        self.resolve(source)
    }
}

pub trait FFIVarResolve<LANG, SPEC>: Clone + LifetimeProcessor + Resolve<FFIFullPath<LANG, SPEC>> + Resolve<SpecialType<LANG, SPEC>> + ToTokens
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          FFIFullPath<LANG, SPEC>: ToType {
    fn ffi_full_path(&self, source: &ScopeContext) -> FFIFullPath<LANG, SPEC> {
        self.resolve(source)
    }
    fn special_or_to_ffi_full_path_type(&self, source: &ScopeContext) -> Type {
        // println!("special_or_to_ffi_full_path_type:: {}", self.to_token_stream());
        let maybe_special_type: Option<SpecialType<LANG, SPEC>> = self.lifetimes_cleaned().maybe_resolve(source);
        // println!("special_or_to_ffi_full_path_type.111:: {}", maybe_special_type.as_ref().map_or("None".to_string(), |t| t.to_string()));
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
impl<LANG, SPEC> FFIVarResolve<LANG, SPEC> for Type
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          FFIFullPath<LANG, SPEC>: ToType,
          FFIFullDictionaryPath<LANG, SPEC>: ToType{}
impl<SPEC> FFIVarResolve<RustFermentate, SPEC> for GenericTypeKind
    where SPEC: RustSpecification {}

#[cfg(feature = "objc")]
impl<SPEC> FFIVarResolve<crate::lang::objc::ObjCFermentate, SPEC> for GenericTypeKind
where SPEC: crate::lang::objc::ObjCSpecification {}

