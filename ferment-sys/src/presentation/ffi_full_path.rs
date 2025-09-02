use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::Path;
use crate::ext::{ToPath, ToType};
use crate::kind::SpecialType;
use crate::lang::Specification;
use crate::presentation::FFIFullDictionaryPath;

#[derive(Debug)]
pub enum FFIFullPath<SPEC>
    where SPEC: Specification {
    Type {
        crate_ident: Ident,
        ffi_name: Path,
    },
    Generic {
        ffi_name: Path
    },
    External {
        path: Path,
    },
    Dictionary {
        path: FFIFullDictionaryPath<SPEC>
    },
}

impl<SPEC> FFIFullPath<SPEC>
    where SPEC: Specification {
    pub fn r#type(crate_ident: Ident, ffi_name: Path) -> Self {
        Self::Type { crate_ident, ffi_name }
    }
    pub fn generic(ffi_name: Path) -> Self {
        Self::Generic { ffi_name }
    }
    pub fn external(path: Path) -> Self {
        Self::External { path }
    }
    pub fn c_char() -> Self {
        Self::Dictionary { path: FFIFullDictionaryPath::CChar }
    }
    pub fn void() -> Self {
        Self::Dictionary { path: FFIFullDictionaryPath::Void }
    }
}

impl<SPEC> From<SpecialType<SPEC>> for FFIFullPath<SPEC>
    where SPEC: Specification {
    fn from(value: SpecialType<SPEC>) -> Self {
        FFIFullPath::<SPEC>::external(value.to_path())
    }
}

impl<SPEC> ToTokens for FFIFullPath<SPEC>
    where SPEC: Specification,
          Self: ToType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}

