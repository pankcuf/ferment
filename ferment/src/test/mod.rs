use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, Path, PathSegment, Type};
use crate::composition::{ImportComposition, TypeComposition};
use crate::context::ScopeContext;
use crate::conversion::PathConversion;
use crate::helper::path_arguments_to_paths;
use crate::holder::PathHolder;

pub mod composing;
pub mod mangling;

impl ImportComposition {
    pub fn new(ident: Ident, scope: PathHolder) -> Self {
        Self { ident, scope }
    }
}
impl TypeComposition {
    pub fn new_default(ty: Type) -> Self {
        Self::new(ty, None)
    }
}
impl PathConversion {
    fn mangled_generic_arguments_types_strings(&self, context: &ScopeContext) -> Vec<String> {
        self.as_path()
            .segments
            .iter()
            .flat_map(|PathSegment { arguments, .. }| {
                path_arguments_to_paths(arguments)
                    .into_iter()
                    .map(Self::from)
                    .map(|arg| arg.as_generic_arg_type(context).to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn as_generic_arg_type(&self, context: &ScopeContext) -> TokenStream2 {
        match self {
            PathConversion::Primitive(path) =>
                quote!(#path),
            PathConversion::Complex(path) =>
                context.ffi_path_converted_or_same(path)
                    .to_token_stream(),
            PathConversion::Generic(conversion) =>
                context.convert_to_ffi_path(conversion)
                    .to_token_stream()
        }
    }

}

impl PathHolder {
    pub fn ffi_expansion_scope() -> Self {
        Self::crate_and(quote!(fermented))
    }
    pub fn ffi_generics_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(generics))
    }
    pub fn ffi_types_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(types))
    }

    pub fn crate_and(path: TokenStream2) -> Self {
        Self::crate_root().joined_path(parse_quote!(#path))
    }
    pub fn ffi_types_and(path: TokenStream2) -> Self {
        Self::ffi_types_scope().joined_path(parse_quote!(#path))
    }
    pub fn joined_path(&self, path: Path) -> PathHolder {
        parse_quote!(#self::#path)
    }

}