use proc_macro2::Ident;
use syn::{parse_quote, Path, Type};
use crate::context::ScopeContext;
use crate::ext::Mangle;
use crate::presentation::ScopeContextPresentable;


#[derive(Clone, Debug)]
pub enum Aspect {
    Target(Context),
    FFI(Context),
    RawTarget(Context),
}

#[derive(Clone, Debug)]
pub enum Context {
    Enum {
        ident: Ident,
    },
    EnumVariant {
        ident: Ident,
        variant_ident: Ident
    },
    Struct {
        ident: Ident,
    },
    Fn {
        path: Path,
    },
}

impl ScopeContextPresentable for Aspect {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(context) => {
                match context {
                    Context::Enum { ident } |
                    Context::Struct { ident } => {
                        let ty = parse_quote!(#ident);
                        source.full_type_for(&ty)
                    }
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path } => parse_quote!(#path)
                }
            },
            Aspect::FFI(context) => {
                match context {
                    Context::Enum { ident } |
                    Context::Struct { ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        let mangled_ty = full_ty.mangle_ident_default();
                        parse_quote!(#mangled_ty)
                    }
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        let mangled_ty = full_ty.mangle_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    },
                    Context::Fn { path } => parse_quote!(#path)
                }
            },
            Aspect::RawTarget(context) => {
                match context {
                    Context::Enum { ident } |
                    Context::Struct { ident } =>
                        parse_quote!(#ident),
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path } => parse_quote!(#path)
                }
            }
        }
    }
}