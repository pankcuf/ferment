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
    RawFFI(Context)
}

impl Aspect {
    pub fn context(&self) -> &Context {
        match self {
            Aspect::Target(context) => context,
            Aspect::FFI(context) => context,
            Aspect::RawTarget(context) => context,
            Aspect::RawFFI(context) => context,
        }
    }
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
    }
}

// impl Context {
    // pub fn ffi(&self) -> Type {
    //     match self {
    //         Context::Enum { ident } => parse_quote!(#ident),
    //         Context::EnumVariant { ident, variant_ident } =>
    //             parse_quote!("{}::{}", ident, variant_ident),
    //         Context::Struct { ident } => parse_quote!(#ident),
    //         Context::Fn { path } => {
    //             let path = path.to_mangled_ident_default();
    //             parse_quote!(#path)
    //         }
    //     }
    // }
    // pub fn target(&self) -> Type {
    //     match self {
    //         Context::Enum { ident } => ident.clone(),
    //         Context::EnumVariant { ident, variant_ident } =>
    //             format_ident!("{}_{}", ident, variant_ident),
    //         Context::Struct { ident } => ident.clone(),
    //         Context::Fn { path } => path.to_mangled_ident_default(),
    //     }
    //
    // }
// }

impl ScopeContextPresentable for Aspect {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(context) => {
                match context {
                    Context::Enum { ident } => {
                        let ty = parse_quote!(#ident);
                        source.full_type_for(&ty)
                    }
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Struct { ident } => {
                        let ty = parse_quote!(#ident);
                        source.full_type_for(&ty)
                    },
                    Context::Fn { path } => parse_quote!(#path)
                }
            },
            Aspect::FFI(context) => {
                match context {
                    Context::Enum { ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        let mangled_ty = full_ty.to_mangled_ident_default();
                        // println!("NamePresentationAspect::FFI -> NamePresentationContext::Enum -> {} ({}) -> ({})", ty.to_token_stream(), full_ty.to_token_stream(), mangled_ty.to_token_stream());
                        parse_quote!(#mangled_ty)
                        // source.ffi_full_dictionary_type_presenter(&ty)
                    }
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        let mangled_ty = full_ty.to_mangled_ident_default();
                        // let full_ty_with_variant = format_ident!("{}::{}", mangled_ty, variant_ident);
                        // println!("NamePresentationAspect::FFI -> NamePresentationContext::EnumVariant -> {} ({}) -> ({})", ty.to_token_stream(), full_ty.to_token_stream(), mangled_ty.to_token_stream());

                        // TODO: differentiante for methods and for conversions:
                        // crate_nested_ProtocolError::IdentifierError for conversions
                        // but crate_nested_ProtocolError_IdentifierError in methods
                        parse_quote!(#mangled_ty::#variant_ident)
                        // source.ffi_full_dictionary_type_presenter(&ty)
                    },
                    Context::Struct { ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        let mangled_ty = full_ty.to_mangled_ident_default();
                        // println!("NamePresentationAspect::FFI -> NamePresentationContext::Struct -> {} ({}) -> ({})", ty.to_token_stream(), full_ty.to_token_stream(), mangled_ty.to_token_stream());
                        parse_quote!(#mangled_ty)
                    }
                    Context::Fn { path } => parse_quote!(#path)
                }
            }
            Aspect::RawTarget(context) => {
                match context {
                    Context::Enum { ident } =>
                        parse_quote!(#ident),
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Struct { ident } =>
                        parse_quote!(#ident),
                    Context::Fn { path } => parse_quote!(#path)
                }

            }
            Aspect::RawFFI(context) => {
                match context {
                    Context::Enum { ident } =>
                        parse_quote!(#ident),
                    Context::EnumVariant { ident, variant_ident } => {
                        let ty = parse_quote!(#ident);
                        let full_ty = source.full_type_for(&ty);
                        let mangled_ty = full_ty.to_mangled_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    }
                    Context::Struct { ident } =>
                        parse_quote!(#ident),
                    Context::Fn { path } => parse_quote!(#path)
                }

            }
        }
    }
}