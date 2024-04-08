use proc_macro2::Ident;
use syn::{parse_quote, Path, Type};
use crate::context::ScopeContext;
use crate::ext::{Mangle, ToType};
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
        self_ty: Option<Type>,
        trait_ty: Option<Type>,
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
                        source.full_type_for(&ident.to_type())
                    }
                    Context::EnumVariant { ident, variant_ident } => {
                        let full_ty = source.full_type_for(&ident.to_type());
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path, .. } => {

                        path.to_type()
                    }
                }
            },
            Aspect::FFI(context) => {
                match context {
                    Context::Enum { ident } |
                    Context::Struct { ident } => {
                        source.full_type_for(&ident.to_type())
                            .mangle_ident_default()
                            .to_type()
                    }
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = ident.to_type();
                        let full_ty = source.full_type_for(&ty);
                        let mangled_ty = full_ty.mangle_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    },
                    Context::Fn { path, self_ty, trait_ty } => {
                        match (self_ty, trait_ty) {
                            (Some(self_ty), Some(trait_ty)) => {
                                let fn_name = &path.segments.last().unwrap().ident;
                                parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                            },
                            // (Some(self_ty), None) => {
                            //     parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                            //
                            // },
                            _ => path.to_type()
                        }
                        // if trait_ty.is_some() {
                        //     println!("Context::Fn {} ---- {} ---- {}", path.to_token_stream(), self_ty.to_token_stream(), trait_ty.to_token_stream());
                        // }
                        // path.to_type()
                    }
                }
            },
            Aspect::RawTarget(context) => {
                match context {
                    Context::Enum { ident } |
                    Context::Struct { ident } =>
                        ident.to_type(),
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = ident.to_type();
                        let full_ty = source.full_type_for(&ty);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path, .. } => path.to_type()
                }
            }
        }
    }
}