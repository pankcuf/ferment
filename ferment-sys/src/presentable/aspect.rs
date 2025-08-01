use syn::{Attribute, parse_quote, Type, TypeSlice};
use std::fmt::{Debug, Display};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::composable::{FnSignatureContext, TypeModeled};
use crate::composer::{AspectArgComposers, AttrComposable, ComposerLinkRef, FieldsContext, GenericsComposable, LifetimesComposable, NameKindComposable, TypeAspect};
use crate::context::ScopeContext;
use crate::kind::{GenericTypeKind, MixinKind};
use crate::ext::{AsType, LifetimeProcessor, Mangle, Resolve, ResolveTrait, ToType};
use crate::lang::Specification;
use crate::presentable::{TypeContext, ScopeContextPresentable, NameTreeContext};
use crate::presentation::DictionaryName;

#[derive(Clone, Debug)]
pub enum Aspect<T> {
    Target(T),
    FFI(T),
    RawTarget(T),
}

impl<T> Aspect<T> where T: NameTreeContext {
    pub fn ffi<SPEC, C>(by_ref: &ComposerLinkRef<C>) -> AspectArgComposers<SPEC>
    where C: AttrComposable<SPEC::Attr> + LifetimesComposable<SPEC::Lt> + GenericsComposable<SPEC::Gen> + TypeAspect<SPEC::TYC> + FieldsContext<SPEC> + NameKindComposable,
          SPEC: Specification<TYC=T> {
        ((Aspect::FFI(C::type_context(by_ref)), (C::compose_attributes(by_ref), C::compose_lifetimes(by_ref), C::compose_generics(by_ref)), C::compose_name_kind(by_ref)), C::field_composers(by_ref))
    }
    pub fn target<SPEC, C>(by_ref: &ComposerLinkRef<C>) -> AspectArgComposers<SPEC>
    where C: AttrComposable<SPEC::Attr> + LifetimesComposable<SPEC::Lt> + GenericsComposable<SPEC::Gen> + TypeAspect<SPEC::TYC> + FieldsContext<SPEC> + NameKindComposable,
          SPEC: Specification<TYC=T> {
        ((Aspect::Target(C::type_context(by_ref)), (C::compose_attributes(by_ref), C::compose_lifetimes(by_ref), C::compose_generics(by_ref)), C::compose_name_kind(by_ref)), C::field_composers(by_ref))
    }
}

impl Aspect<TypeContext> {
    #[allow(unused)]
    pub fn alloc_field_name(&self) -> TokenStream2 {
        match self {
            Aspect::Target(_) => DictionaryName::Obj.to_token_stream(),
            Aspect::FFI(_) => DictionaryName::FfiRef.to_token_stream(),
            Aspect::RawTarget(_) => DictionaryName::Obj.to_token_stream(),
        }
    }

    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Aspect::Target(context) => context.attrs(),
            Aspect::FFI(context) => context.attrs(),
            Aspect::RawTarget(context) => context.attrs(),
        }
    }
    pub fn raw_struct_ident(ident: Ident) -> Self {
        Aspect::RawTarget(TypeContext::struct_ident(ident))
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

impl ScopeContextPresentable for Aspect<TypeContext> {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(context) => {
                match context {
                    TypeContext::Enum { ident, .. } |
                    TypeContext::Struct { ident, .. } => {
                        let result: Type = ident.to_type()
                            .resolve(source);
                        result
                    },
                    TypeContext::EnumVariant { parent: _, ident, variant_ident, attrs: _ } => {
                        let full_ty = Resolve::<Type>::resolve(&ident.to_type(), source);
                        let ty = full_ty.lifetimes_cleaned();
                        parse_quote!(#ty::#variant_ident)
                    },
                    TypeContext::Fn { path, .. } => {
                        path.to_type()
                    }
                    TypeContext::Trait { path , ..} |
                    TypeContext::Impl { path , ..} =>
                        path.to_type().resolve(source),
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(GenericTypeKind::Slice(ty)), ..} => {
                        let type_slice: TypeSlice = parse_quote!(#ty);
                        let elem_type = &type_slice.elem;
                        parse_quote!(Vec<#elem_type>)
                    }
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), ..} =>
                        kind.ty().cloned().unwrap(),
                    TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.as_type().clone()
                }
            },
            Aspect::FFI(context) => {
                match context {
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), ..} =>
                        kind.ty().cloned().unwrap().mangle_ident_default().to_type(),
                    TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.mangle_ident_default().to_type(),
                    TypeContext::Enum { ident , .. } |
                    TypeContext::Struct { ident , .. } => {
                        Resolve::<Type>::resolve(&ident.to_type(), source)
                            .mangle_ident_default()
                            .to_type()
                    }
                    TypeContext::Trait { path , .. } =>
                        Resolve::<Type>::resolve(&path.to_type(), source)
                            .mangle_ident_default()
                            .to_type(),
                    TypeContext::Impl { path , .. } =>
                        Resolve::<Type>::resolve(&path.to_type(), source)
                            .mangle_ident_default()
                            .to_type(),
                    TypeContext::EnumVariant { parent: _, ident, variant_ident, attrs: _ } => {
                        let mangled_ty = Resolve::<Type>::resolve(&ident.to_type(), source).mangle_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    },
                    TypeContext::Fn { path, sig_context, .. } => {
                        match sig_context {
                            FnSignatureContext::ModFn(item_fn) => {
                                Resolve::<Type>::resolve(&item_fn.sig.ident.to_type(), source)
                                    .mangle_ident_default()
                                    .to_type()
                            }
                            FnSignatureContext::TraitInner(self_ty, _trait_ty, _sig) => {
                                Resolve::<Type>::resolve(self_ty, source)
                                    .mangle_ident_default()
                                    .to_type()
                            },
                            FnSignatureContext::Impl(self_ty, trait_ty, _sig) => {
                                let self_ty = Resolve::<Type>::resolve(self_ty, source);
                                let trait_ty = trait_ty.as_ref()
                                    .and_then(|trait_ty|
                                        Resolve::<Type>::resolve(trait_ty, source)
                                            .maybe_trait_ty(source));

                                match trait_ty {
                                    Some(trait_ty) => {
                                        let fn_name = &path.segments.last().unwrap().ident;
                                        parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                                    }
                                    None => path.to_type()
                                }
                            }
                            FnSignatureContext::TraitAsType(self_ty, trait_ty, _sig) => {
                                let self_ty = Resolve::<Type>::resolve(self_ty, source);
                                let trait_ty = Resolve::<Type>::resolve(trait_ty, source)
                                    .maybe_trait_ty(source);

                                match trait_ty {
                                    Some(trait_ty) => {
                                        let fn_name = &path.segments.last().unwrap().ident;
                                        parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                                    }
                                    None => path.to_type()
                                }
                            }
                            FnSignatureContext::Bare(ident, _type_bare_fn) => {
                                Resolve::<Type>::resolve(&ident.to_type(), source)
                                    .mangle_ident_default()
                                    .to_type()
                            }
                        }
                    }
                }
            },
            Aspect::RawTarget(context) => {
                match context {
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. } =>
                        kind.ty().cloned().unwrap(),
                    TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. } =>
                        model.type_model_ref().ty.clone(),
                    TypeContext::Enum { ident , attrs: _, generics: _ } |
                    TypeContext::Struct { ident , attrs: _, generics: _ } =>
                        ident.to_type(),
                    TypeContext::EnumVariant { parent: _, ident, variant_ident, attrs: _ } => {
                        let full_ty = Resolve::<Type>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    TypeContext::Fn { path, .. } => path.to_type(),
                    TypeContext::Trait { path , attrs: _ } => path.to_type(),
                    TypeContext::Impl { path , trait_, attrs: _ } =>
                        trait_.as_ref()
                            .map(|trait_| trait_.to_type())
                            .unwrap_or_else(|| path.to_type())
                            .resolve(source)
                }
            }
        }
    }
}
