use proc_macro2::Ident;
use syn::{parse_quote, Path, Type};
use crate::ast::Depunctuated;
use crate::composable::FnSignatureContext;
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ResolveTrait, ToType};
use crate::presentable::ScopeContextPresentable;
use crate::presentation::Expansion;


#[derive(Clone, Debug)]
pub enum Aspect {
    Target(Context),
    FFI(Context),
    RawTarget(Context),
}

impl Aspect {
    pub fn attrs(&self) -> &Depunctuated<Expansion> {
        match self {
            Aspect::Target(context) => context.attrs(),
            Aspect::FFI(context) => context.attrs(),
            Aspect::RawTarget(context) => context.attrs(),
        }
    }
}

// #[derive(Clone, Debug)]
// pub enum ContextID {
//     Ident(Ident),
//     Variant(Ident, Ident),
//     Path(Path),
//     SignaturePath(FnSignatureContext, Path)
// }

#[derive(Clone, Debug)]
pub enum Context {
    Enum {
        ident: Ident,
        attrs: Depunctuated<Expansion>,
    },
    EnumVariant {
        ident: Ident,
        variant_ident: Ident,
        attrs: Depunctuated<Expansion>
    },
    Struct {
        ident: Ident,
        attrs: Depunctuated<Expansion>,
    },
    Fn {
        path: Path,
        sig_context: FnSignatureContext,
        attrs: Depunctuated<Expansion>,
    },
    Trait {
        path: Path,
        attrs: Depunctuated<Expansion>,
    }
}

impl Context {
    fn attrs(&self) -> &Depunctuated<Expansion> {
        match self {
            Context::Enum { attrs, .. } => attrs,
            Context::EnumVariant { attrs, .. } => attrs,
            Context::Struct { attrs, .. } => attrs,
            Context::Fn { attrs, .. } => attrs,
            Context::Trait { attrs, .. } => attrs
        }
    }
}

impl ScopeContextPresentable for Aspect {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(context) => {
                match context {
                    Context::Enum { ident, .. } |
                    Context::Struct { ident , .. } =>
                        ident.to_type()
                            .resolve(source),
                    Context::EnumVariant { ident, variant_ident, attrs: _ } => {
                        let full_ty = <Type as Resolve<Type>>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path, .. } => {
                        path.to_type()
                    }
                    Context::Trait { path , ..} => path.to_type().resolve(source)
                }
            },
            Aspect::FFI(context) => {
                match context {
                    Context::Enum { ident , .. } |
                    Context::Struct { ident , .. } => {
                        <Type as Resolve<Type>>::resolve(&ident.to_type(), source)
                            .mangle_ident_default()
                            .to_type()
                    }
                    Context::Trait { path , .. } =>
                        <Type as Resolve<Type>>::resolve(&path.to_type(), source)
                            .mangle_ident_default()
                            .to_type(),
                    Context::EnumVariant { ident, variant_ident, attrs: _ } => {
                        let mangled_ty = <Type as Resolve<Type>>::resolve(&ident.to_type(), source).mangle_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    },
                    Context::Fn { path, sig_context, .. } => {
                        match sig_context {
                            FnSignatureContext::ModFn(item_fn) => {
                                <Type as Resolve<Type>>::resolve(&item_fn.sig.ident.to_type(), source)
                                    .mangle_ident_default()
                                    .to_type()
                            }
                            FnSignatureContext::TraitInner(self_ty, _trait_ty, _sig) => {
                                <Type as Resolve<Type>>::resolve(self_ty, source)
                                    .mangle_ident_default()
                                    .to_type()
                            },
                            FnSignatureContext::Impl(self_ty, trait_ty, _sig) => {
                                let self_ty = <Type as Resolve<Type>>::resolve(&self_ty, source);
                                let trait_ty = trait_ty.as_ref()
                                    .and_then(|trait_ty|
                                        <Type as Resolve<Type>>::resolve(trait_ty, source)
                                            .maybe_trait_ty(source));

                                match trait_ty {
                                    Some(trait_ty) => {
                                        let fn_name = &path.segments.last().unwrap().ident;
                                        parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                                    }
                                    None => path.to_type()
                                }
                            }
                            FnSignatureContext::Bare(ident, _type_bare_fn) => {
                                <Type as Resolve<Type>>::resolve(&ident.to_type(), source)
                                    .mangle_ident_default()
                                    .to_type()
                            }
                        }
                    }
                }
            },
            Aspect::RawTarget(context) => {
                match context {
                    Context::Enum { ident , attrs: _, } |
                    Context::Struct { ident , attrs: _, } =>
                        ident.to_type(),
                    Context::EnumVariant { ident, variant_ident, attrs: _ } => {
                        let full_ty = <Type as Resolve<Type>>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path, .. } => path.to_type(),
                    Context::Trait { path , attrs: _ } => path.to_type()
                }
            }
        }
    }
}