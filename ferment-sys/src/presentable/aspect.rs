use syn::{Attribute, parse_quote, Type, TypeSlice};
use std::fmt::{Debug, Display};
use proc_macro2::{Group, TokenTree};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::token::Comma;
use crate::ast::{DelimiterTrait, Wrapped};
use crate::composable::{FnSignatureContext, TypeModeled};
use crate::composer::{AspectArgComposers, AttrComposable, ComposerLinkRef, FieldsContext, GenericsComposable, NameKindComposable, PunctuatedArgKinds, TypeAspect};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, MixinKind};
use crate::ext::{AsType, Mangle, Resolve, ResolveTrait, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{TypeContext, ScopeContextPresentable, NameTreeContext};
use crate::presentation::{DictionaryName, RustFermentate};

#[derive(Clone, Debug)]
pub enum Aspect<T> {
    Target(T),
    FFI(T),
    RawTarget(T),
}

impl<T> Aspect<T> where T: NameTreeContext {
    pub fn ffi<LANG, SPEC, C>(by_ref: &ComposerLinkRef<C>) -> AspectArgComposers<LANG, SPEC>
    where C: AttrComposable<SPEC::Attr> + GenericsComposable<SPEC::Gen> + TypeAspect<SPEC::TYC> + FieldsContext<LANG, SPEC> + NameKindComposable,
          LANG: LangFermentable,
          SPEC: Specification<LANG, TYC=T> {
        ((Aspect::FFI(C::type_context(by_ref)), C::compose_attributes(by_ref), C::compose_generics(by_ref), C::compose_name_kind(by_ref)), C::field_composers(by_ref))
    }
    pub fn target<LANG, SPEC, C>(by_ref: &ComposerLinkRef<C>) -> AspectArgComposers<LANG, SPEC>
    where C: AttrComposable<SPEC::Attr> + GenericsComposable<SPEC::Gen> + TypeAspect<SPEC::TYC> + FieldsContext<LANG, SPEC> + NameKindComposable,
          LANG: LangFermentable,
          SPEC: Specification<LANG, TYC=T> {
        ((Aspect::Target(C::type_context(by_ref)), C::compose_attributes(by_ref), C::compose_generics(by_ref), C::compose_name_kind(by_ref)), C::field_composers(by_ref))
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
    #[allow(unused)]
    pub fn allocate<I, SPEC>(&self, fields: Wrapped<PunctuatedArgKinds<RustFermentate, SPEC, Comma>, Comma, I>, source: &ScopeContext) -> TokenStream2
    where I: DelimiterTrait,
          SPEC: RustSpecification {
        let aspect_presentation = self.present(source);
        match self {
            Aspect::Target(_context) => {
                let fields_presentation = TokenTree::Group(Group::new(I::delimiter(), fields.content.present(source).to_token_stream()));
                quote! {
                    #aspect_presentation #fields_presentation
                }
            }
            Aspect::FFI(_context) | Aspect::RawTarget(_context) => {
                let fields_presentation = TokenTree::Group(Group::new(I::delimiter(), fields.content.present(source).to_token_stream()));
                quote! {
                    #aspect_presentation #fields_presentation
                }
            }
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

impl ScopeContextPresentable for Aspect<TypeContext> {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(context) => {
                match context {
                    TypeContext::Enum { ident, .. } |
                    TypeContext::Struct { ident , .. } =>
                        ident.to_type()
                            .resolve(source),
                    TypeContext::EnumVariant { parent: _, ident, variant_ident, attrs: _ } => {
                        let full_ty = <Type as Resolve<Type>>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
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
                    // model.type_model_ref().ty.clone(),
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
                        <Type as Resolve<Type>>::resolve(&ident.to_type(), source)
                            .mangle_ident_default()
                            .to_type()
                    }
                    TypeContext::Trait { path , .. } =>
                        <Type as Resolve<Type>>::resolve(&path.to_type(), source)
                            .mangle_ident_default()
                            .to_type(),
                    TypeContext::Impl { path , .. } =>
                        <Type as Resolve<Type>>::resolve(&path.to_type(), source)
                            .mangle_ident_default()
                            .to_type(),
                    TypeContext::EnumVariant { parent: _, ident, variant_ident, attrs: _ } => {
                        let mangled_ty = <Type as Resolve<Type>>::resolve(&ident.to_type(), source).mangle_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    },
                    TypeContext::Fn { path, sig_context, .. } => {
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
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. } =>
                        kind.ty().cloned().unwrap(),
                    TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. } =>
                        model.type_model_ref().ty.clone(),
                    TypeContext::Enum { ident , attrs: _, } |
                    TypeContext::Struct { ident , attrs: _, } =>
                        ident.to_type(),
                    TypeContext::EnumVariant { parent: _, ident, variant_ident, attrs: _ } => {
                        let full_ty = <Type as Resolve<Type>>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    TypeContext::Fn { path, .. } => path.to_type(),
                    TypeContext::Trait { path , attrs: _ } => path.to_type(),
                    TypeContext::Impl { path , trait_, attrs: _ } =>
                        trait_.as_ref()
                            .map(|trait_| trait_.to_type())
                            .unwrap_or(path.to_type())
                            .resolve(source)
                }
            }
        }
    }
}
