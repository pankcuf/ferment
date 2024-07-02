use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, Type};
use crate::context::ScopeContext;
use crate::conversion::GenericTypeConversion;
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

// pub trait FFISpecialTypeResolve {
//     /// Types that are exported with [ferment_macro::register] or [ferment_macro::opaque]
//     /// so it's custom conversion or opaque pointer therefore we should use direct paths for ffi export
//     fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType>;
// }
// impl FFISpecialTypeResolve for Type {
//     fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType> {
//         // println!("Type::maybe_special_type: {}", self.to_token_stream());
//         source.maybe_custom_conversion(self)
//             .map(SpecialType::Custom)
//             .or_else(|| source.maybe_opaque_object(self)
//                 .map(SpecialType::Opaque))
//     }
// }
// impl FFISpecialTypeResolve for GenericTypeConversion {
//     fn maybe_special_type(&self, source: &ScopeContext) -> Option<SpecialType> {
//         // println!("GenericTypeConversion::maybe_special_type: {}", self.to_token_stream());
//         self.ty()
//             .and_then(|ty| ty.maybe_special_type(source))
//     }
// }

pub trait FFITypeResolve: Resolve<FFIFullPath> + Resolve<Option<SpecialType>> {
    fn special_or_to_ffi_full_path_type(&self, source: &ScopeContext) -> Type {
        <Self as Resolve<Option<SpecialType>>>::resolve(self, source)
            .map(|special| special.to_type())
            .unwrap_or(<Self as Resolve::<FFIFullPath>>::resolve(self, source).to_type())
    }
    fn special_or_to_ffi_full_path_variable_type(&self, source: &ScopeContext) -> Type {
        self.special_or_to_ffi_full_path_type(source)
            .joined_mut()
    }
}
impl FFITypeResolve for Type {}
impl FFITypeResolve for GenericTypeConversion {}
