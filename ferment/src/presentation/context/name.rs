use proc_macro2::Ident;
use syn::{parse_quote, Path, Type};
use crate::composition::FnSignatureContext;
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ResolveTrait, ToType};
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
        // self_ty: Option<Type>,
        // trait_ty: Option<Type>,
        sig_context: FnSignatureContext
    },
    Trait {
        path: Path,
    }
}

impl ScopeContextPresentable for Aspect {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(context) => {
                match context {
                    Context::Enum { ident } |
                    Context::Struct { ident } =>
                        ident.to_type()
                            .resolve(source),
                    Context::EnumVariant { ident, variant_ident } => {
                        let full_ty = ident.to_type().resolve(source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path, .. } => {

                        path.to_type()
                    }
                    Context::Trait { path } => path.to_type().resolve(source)
                }
            },
            Aspect::FFI(context) => {
                match context {
                    Context::Enum { ident } |
                    Context::Struct { ident } => {
                        ident.to_type()
                            .resolve(source)
                            .mangle_ident_default()
                            .to_type()
                    }
                    Context::Trait { path } =>
                        path.to_type()
                            .resolve(source)
                            .mangle_ident_default()
                            .to_type(),
                    Context::EnumVariant { ident, variant_ident } => {
                        let mangled_ty = ident.to_type().resolve(source).mangle_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    },
                    Context::Fn { path, sig_context } => {
                        match sig_context {
                            FnSignatureContext::ModFn(item_fn) => {
                                item_fn
                                    .sig
                                    .ident
                                    .to_type()
                                    .resolve(source)
                                    .mangle_ident_default()
                                    .to_type()
                            }
                            FnSignatureContext::TraitInner(self_ty, trait_ty, sig) => {
                                self_ty.resolve(source).mangle_ident_default().to_type()
                            },
                            FnSignatureContext::Impl(self_ty, trait_ty, sig) => {
                                let self_ty = self_ty.resolve(source);
                                let trait_ty = trait_ty.as_ref()
                                    .and_then(|trait_ty|
                                        trait_ty.resolve(source)
                                            .to_trait_ty(source));

                                match trait_ty {
                                    Some(trait_ty) => {
                                        let fn_name = &path.segments.last().unwrap().ident;
                                        parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                                    }
                                    None => path.to_type()
                                }
                            }
                            FnSignatureContext::Bare(ident, type_bare_fn) => {
                                ident.to_type().resolve(source).mangle_ident_default().to_type()
                                // let TypeBareFn { inputs, output, .. } = type_bare_fn;
                                // let arguments = inputs.compose(source);
                                // let return_type = output.compose(&(true, source));

                            }
                        }
                        // match (self_ty, trait_ty) {
                        //     (Some(self_ty), Some(trait_ty)) => {
                        //         let fn_name = &path.segments.last().unwrap().ident;
                        //         parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                        //     },
                        //     // (Some(self_ty), None) => {
                        //     //     parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                        //     //
                        //     // },
                        //     _ => path.to_type()
                        // }
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
                        let full_ty = ident.to_type().resolve(source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path, .. } => path.to_type(),
                    Context::Trait { path } => path.to_type()
                }
            }
        }
    }
}