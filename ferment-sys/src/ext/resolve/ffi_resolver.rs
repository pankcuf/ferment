use quote::ToTokens;
use syn::Type;
use crate::context::ScopeContext;
use crate::kind::{ObjectKind, SpecialType, TypeKind};
use crate::ext::{Accessory, LifetimeProcessor, Resolve, ToType};
use crate::lang::Specification;
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};


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
    /// so it's custom kind or opaque pointer therefore we should use direct paths for ffi export
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>>;

    #[allow(unused)]
    fn is_opaque(&self, source: &ScopeContext) -> bool {
        self.maybe_special_type(source)
            .map(|special| matches!(special, SpecialType::Opaque(..)))
            .unwrap_or(false)
    }
    #[allow(unused)]
    fn is_custom(&self, source: &ScopeContext) -> bool {
        self.maybe_special_type(source)
            .map(|special| matches!(special, SpecialType::Custom(..)))
            .unwrap_or(false)
    }
}
impl<SPEC> FFISpecialTypeResolve<SPEC> for Type
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        self.maybe_resolve(source)
    }
}

impl<SPEC> FFISpecialTypeResolve<SPEC> for TypeKind
where SPEC: Specification,
      FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        let ty = self.to_type();
        FFISpecialTypeResolve::<SPEC>::maybe_special_type(&ty, source)
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
    #[allow(unused)]
    fn special_or_to_ffi_full_path_variable_type(&self, source: &ScopeContext) -> Type {
        self.special_or_to_ffi_full_path_type(source)
            .joined_mut()
    }
}
impl<SPEC> FFIVarResolve<SPEC> for Type
    where SPEC: Specification,
          FFIFullPath<SPEC>: ToType,
          FFIFullDictionaryPath<SPEC>: ToType{}
