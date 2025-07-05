use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{parse_quote, Path, Type};
use crate::ext::{SpecialType, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
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
    pub fn external(path: Path) -> Self {
        Self::External { path }
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

impl ToType for FFIFullPath<RustSpecification> {
    fn to_type(&self) -> Type {
        match self {
            FFIFullPath::Type { crate_ident, ffi_name } =>
                parse_quote!(crate::fermented::types::#crate_ident::#ffi_name),
            FFIFullPath::Generic { ffi_name } =>
                parse_quote!(crate::fermented::generics::#ffi_name),
            FFIFullPath::External { path } =>
                parse_quote!(#path),
            FFIFullPath::Dictionary { path } =>
                path.to_type(),
        }
    }
}
