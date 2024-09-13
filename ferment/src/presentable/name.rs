use std::fmt::{Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, parse_quote, Path, Type, TypeSlice};
use crate::composable::{FnSignatureContext, TypeModeled};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, MixinKind};
use crate::ext::{AsType, Mangle, Resolve, ResolveTrait, ToType};
use crate::presentable::ScopeContextPresentable;


#[derive(Clone, Debug)]
pub enum Aspect<T> {
    Target(T),
    FFI(T),
    RawTarget(T),
}

impl Aspect<Context> {
    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Aspect::Target(context) => context.attrs(),
            Aspect::FFI(context) => context.attrs(),
            Aspect::RawTarget(context) => context.attrs(),
        }
    }
}
impl<T> Display for Aspect<T> where T: ToString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Target(context) => format!("Target({})", context.to_string()),
            Self::FFI(context) => format!("FFI({})", context.to_string()),
            Self::RawTarget(context) => format!("RawTarget({})", context.to_string()),
        }.as_str())
    }
}

#[derive(Clone, Debug)]
pub enum Context {
    Enum {
        ident: Ident,
        attrs: Vec<Attribute>,
    },
    EnumVariant {
        ident: Ident,
        variant_ident: Ident,
        attrs: Vec<Attribute>
    },
    Struct {
        ident: Ident,
        attrs: Vec<Attribute>,
    },
    Fn {
        path: Path,
        sig_context: FnSignatureContext,
        attrs: Vec<Attribute>,
    },
    Trait {
        path: Path,
        attrs: Vec<Attribute>,
    },
    Impl {
        path: Path,
        attrs: Vec<Attribute>,
    },
    Mixin {
        mixin_kind: MixinKind,
        attrs: Vec<Attribute>,
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Context::Enum { ident, .. } |
            Context::Struct { ident, .. } =>
                ident.to_string(),
            Context::EnumVariant { ident, variant_ident, .. } =>
                format!("{ident}_{variant_ident}"),
            Context::Fn { path, .. } |
            Context::Trait { path, .. } =>
                path.to_token_stream().to_string(),
            Context::Impl { path, .. } =>
                path.to_token_stream().to_string(),
            Context::Mixin { mixin_kind: MixinKind::Generic(kind), .. } =>
                kind.to_token_stream().to_string(),
            Context::Mixin { mixin_kind: MixinKind::Bounds(model), .. } =>
                model.to_string(),
        }.as_str())
    }
}

impl Context {
    fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Context::Mixin { attrs, .. } |
            Context::Enum { attrs, .. } |
            Context::EnumVariant { attrs, .. } |
            Context::Struct { attrs, .. } |
            Context::Fn { attrs, .. } |
            Context::Trait { attrs, .. } |
            Context::Impl { attrs, .. } => attrs,
        }
    }

    pub fn r#struct(ident: &Ident, attrs: Vec<Attribute>) -> Self {
        Self::Struct { ident: ident.clone(), attrs }
    }
    pub fn r#enum(ident: &Ident, attrs: Vec<Attribute>) -> Self {
        Self::Enum { ident: ident.clone(), attrs }
    }
    pub fn variant(ident: &Ident, variant: &Ident, attrs: Vec<Attribute>) -> Self {
        Self::EnumVariant { ident: ident.clone(), variant_ident: variant.clone(), attrs }
    }

    pub fn mixin(kind: &MixinKind, attrs: Vec<Attribute>) -> Self {
        Self::Mixin { mixin_kind: kind.clone(), attrs }
    }
}

impl ScopeContextPresentable for Aspect<Context> {
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
                    Context::Trait { path , ..} |
                    Context::Impl { path , ..} =>
                        path.to_type().resolve(source),
                    Context::Mixin { mixin_kind: MixinKind::Generic(GenericTypeKind::Slice(ty)), ..} => {
                        let type_slice: TypeSlice = parse_quote!(#ty);
                        let elem_type = &type_slice.elem;
                        parse_quote!(Vec<#elem_type>)
                    }
                    Context::Mixin { mixin_kind: MixinKind::Generic(kind), ..} =>
                        kind.ty().cloned().unwrap(),
                    Context::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.as_type().clone()
                    // model.type_model_ref().ty.clone(),
                }
            },
            Aspect::FFI(context) => {
                match context {
                    Context::Mixin { mixin_kind: MixinKind::Generic(kind), ..} =>
                        kind.ty().cloned().unwrap().mangle_ident_default().to_type(),
                    Context::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.mangle_ident_default().to_type(),
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
                    Context::Impl { path , .. } =>
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
                    Context::Mixin { mixin_kind: MixinKind::Generic(kind), ..} =>
                        kind.ty().cloned().unwrap(),
                    Context::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.type_model_ref().ty.clone(),
                    Context::Enum { ident , attrs: _, } |
                    Context::Struct { ident , attrs: _, } =>
                        ident.to_type(),
                    Context::EnumVariant { ident, variant_ident, attrs: _ } => {
                        let full_ty = <Type as Resolve<Type>>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    Context::Fn { path, .. } => path.to_type(),
                    Context::Trait { path , attrs: _ } => path.to_type(),
                    Context::Impl { path , attrs: _ } => path.to_type()
                }
            }
        }
    }
}