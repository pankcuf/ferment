use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{parse_quote, Path, Type};
use crate::ext::{ToPath, ToType};
use crate::presentation::FFIFullDictionaryPath;

pub enum FFIFullPath {
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
        path: FFIFullDictionaryPath
    },
}

impl ToTokens for FFIFullPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_path().to_tokens(tokens)
    }
}

impl ToPath for FFIFullPath {
    fn to_path(&self) -> Path {
        match self {
            FFIFullPath::Type { crate_ident, ffi_name } =>
                parse_quote!(crate::fermented::types::#crate_ident::#ffi_name),
            FFIFullPath::Generic { ffi_name } =>
                parse_quote!(crate::fermented::generics::#ffi_name),
            FFIFullPath::External { path } =>
                parse_quote!(#path),
            FFIFullPath::Dictionary { path } =>
                path.to_path(),
        }
    }
}
impl ToType for FFIFullPath {
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
