use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, Path, PathSegment, Type};
use syn::punctuated::Punctuated;
use crate::ast::PathHolder;
use crate::composable::TypeModel;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, TypeKind};
use crate::ext::{path_arguments_to_types, Resolve, ToPath};
use crate::presentation::FFIFullPath;

pub mod composing;
pub mod mangling;
mod lookup;

impl TypeModel {
    pub fn new_default(ty: Type) -> Self {
        Self::new(ty, None, Punctuated::new())
    }
}
impl TypeKind {
    fn mangled_generic_arguments_types_strings(&self, context: &ScopeContext) -> Vec<String> {
        let path: Path = parse_quote!(#self);
        path
            .segments
            .iter()
            .flat_map(|PathSegment { arguments, .. }| {
                path_arguments_to_types(&arguments)
                    .into_iter()
                    .map(Self::from)
                    .map(|arg| arg.as_generic_arg_type(context).to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn as_generic_arg_type(&self, source: &ScopeContext) -> TokenStream2 {
        match self {
            TypeKind::Primitive(path) =>
                quote!(#path),
            TypeKind::Complex(ty) =>
                <Type as Resolve<FFIFullPath>>::resolve(ty, source).to_token_stream(),
            TypeKind::Generic(conversion) =>
                <GenericTypeKind as Resolve<FFIFullPath>>::resolve(conversion, source).to_token_stream(),
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
        Self::crate_root().joined_path(path.to_path())
    }
    pub fn ffi_types_and(path: TokenStream2) -> Self {
        Self::ffi_types_scope().joined_path(path.to_path())
    }
    pub fn joined_path(&self, path: Path) -> PathHolder {
        parse_quote!(#self::#path)
    }

}