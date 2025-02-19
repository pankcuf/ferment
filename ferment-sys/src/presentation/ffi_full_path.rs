use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{parse_quote, Path, Type};
use crate::ext::{SpecialType, ToPath, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentation::{FFIFullDictionaryPath, RustFermentate};

#[derive(Debug)]
pub enum FFIFullPath<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
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
        path: FFIFullDictionaryPath<LANG, SPEC>
    },
}

impl<LANG, SPEC> FFIFullPath<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn external(path: Path) -> Self {
        Self::External { path }
    }
}

impl<LANG, SPEC> From<SpecialType<LANG, SPEC>> for FFIFullPath<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn from(value: SpecialType<LANG, SPEC>) -> Self {
        FFIFullPath::<LANG, SPEC>::external(value.to_path())
    }
}

impl<LANG, SPEC> ToTokens for FFIFullPath<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Self: ToType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}

impl<SPEC> ToType for FFIFullPath<RustFermentate, SPEC>
    where SPEC: RustSpecification {
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
