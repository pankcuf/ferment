use syn::{Attribute, parse_quote, Type, TypeSlice, ItemFn, Signature, TypePath};
use std::fmt::{Debug, Display};
use proc_macro2::Ident;
use crate::composable::FnSignatureContext;
use crate::composer::{AspectArgComposers, AttrComposable, ComposerLinkRef, FieldsContext, GenericsComposable, LifetimesComposable, NameKindComposable, TypeAspect};
use crate::context::ScopeContext;
use crate::kind::{GenericTypeKind, MixinKind};
use crate::ext::{Accessory, Join, LifetimeProcessor, Mangle, Resolve, ResolveTrait, ToType};
use crate::lang::Specification;
use crate::presentable::{TypeContext, ScopeContextPresentable, NameTreeContext};

#[derive(Clone, Debug)]
pub enum Aspect<T> {
    Target(T),
    Ffi(T),
    RawTarget(T),
}

impl<T> Aspect<T> where T: NameTreeContext {
    pub fn ffi<SPEC, C>(by_ref: &ComposerLinkRef<C>) -> AspectArgComposers<SPEC>
    where C: AttrComposable<SPEC::Attr> + LifetimesComposable<SPEC::Lt> + GenericsComposable<SPEC::Gen> + TypeAspect<SPEC::TYC> + FieldsContext<SPEC> + NameKindComposable,
          SPEC: Specification<TYC=T> {
        ((Aspect::Ffi(C::type_context(by_ref)), (C::compose_attributes(by_ref), C::compose_lifetimes(by_ref), C::compose_generics(by_ref)), C::compose_name_kind(by_ref)), C::field_composers(by_ref))
    }
    pub fn target<SPEC, C>(by_ref: &ComposerLinkRef<C>) -> AspectArgComposers<SPEC>
    where C: AttrComposable<SPEC::Attr> + LifetimesComposable<SPEC::Lt> + GenericsComposable<SPEC::Gen> + TypeAspect<SPEC::TYC> + FieldsContext<SPEC> + NameKindComposable,
          SPEC: Specification<TYC=T> {
        ((Aspect::Target(C::type_context(by_ref)), (C::compose_attributes(by_ref), C::compose_lifetimes(by_ref), C::compose_generics(by_ref)), C::compose_name_kind(by_ref)), C::field_composers(by_ref))
    }
}

impl Aspect<TypeContext> {
    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Aspect::Target(context) => context.attrs(),
            Aspect::Ffi(context) => context.attrs(),
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
            Self::Ffi(context) => format!("FFI({})", context.to_string()),
            Self::RawTarget(context) => format!("RawTarget({})", context.to_string()),
        }.as_str())
    }
}

impl ScopeContextPresentable for Aspect<TypeContext> {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(TypeContext::Fn { path, .. }) |
            Aspect::Ffi(TypeContext::Fn { path, sig_context: FnSignatureContext::Impl(..), .. }) |
            Aspect::RawTarget(TypeContext::Fn { path, .. } |
                              TypeContext::Trait { path, .. }) =>
                path.to_type(),
            Aspect::Target(TypeContext::Enum { ident, .. } |
                           TypeContext::Struct { ident, .. }) |
            Aspect::RawTarget(TypeContext::Enum { ident, .. } |
                              TypeContext::Struct { ident, .. }) =>
                Resolve::<Type>::resolve(ident, source),
            Aspect::Target(TypeContext::EnumVariant { ident, variant_ident, .. }) =>
                Resolve::<Type>::resolve(ident, source)
                    .lifetimes_cleaned()
                    .joined_ident(variant_ident),
            Aspect::Target(TypeContext::Trait { path, .. } | TypeContext::Impl { path, .. }) |
            Aspect::RawTarget(TypeContext::Impl { trait_: Some(path), .. } | TypeContext::Impl { path, .. }) =>
                path.to_type()
                    .resolve(source),
            Aspect::Target(TypeContext::Mixin { mixin_kind: MixinKind::Generic(GenericTypeKind::Slice(Type::Slice(TypeSlice { elem, ..}))), .. }) =>
                parse_quote!(Vec<#elem>),
            Aspect::Target(TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. }) |
            Aspect::RawTarget(TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. }) =>
                kind.ty()
                    .cloned()
                    .unwrap(),
            Aspect::Target(TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. }) |
            Aspect::RawTarget(TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. }) =>
                model.to_type(),
            Aspect::Ffi(TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. }) =>
                kind.ty()
                    .cloned()
                    .unwrap()
                    .mangle_ident_default()
                    .to_type(),
            Aspect::Ffi(TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. }) =>
                model.mangle_ident_default()
                    .to_type(),
            Aspect::Ffi(TypeContext::Enum { ident , .. } |
                        TypeContext::Struct { ident , .. } |
                        TypeContext::Fn { sig_context:
                            FnSignatureContext::ModFn(ItemFn { sig: Signature { ident, .. }, .. }) |
                            FnSignatureContext::Bare(ident, _), .. }) =>
                Resolve::<Type>::resolve(ident, source)
                    .mangle_ident_default()
                    .to_type(),
            Aspect::Ffi(TypeContext::Trait { path , .. } |
                        TypeContext::Impl { path , .. }) =>
                Resolve::<Type>::resolve(path, source)
                    .mangle_ident_default()
                    .to_type(),
            Aspect::Ffi(TypeContext::EnumVariant { ident, variant_ident, .. }) =>
                Resolve::<Type>::resolve(ident, source)
                    .mangle_ident_default()
                    .to_type()
                    .joined_ident(variant_ident),
            Aspect::Ffi(TypeContext::Fn { sig_context: FnSignatureContext::TraitInner(_, self_ty, _), .. }) =>
                Resolve::<Type>::resolve(self_ty, source)
                    .mangle_ident_default()
                    .to_type(),
            Aspect::Ffi(TypeContext::Fn { path, sig_context: FnSignatureContext::TraitImpl(_, self_ty, trait_ty) | FnSignatureContext::TraitAsType(_, self_ty, trait_ty), .. }) =>
                Resolve::<Type>::resolve(trait_ty, source)
                    .maybe_trait_ty(source)
                    .map(|full_trait_ty| {
                        let self_ty = Resolve::<Type>::resolve(self_ty, source);
                        let type_path: TypePath = parse_quote!(<#self_ty as #full_trait_ty>);
                        Type::Path(type_path.joined(&path.segments.last().expect("Expect ident").ident))
                    }).unwrap_or_else(|| path.to_type()),
            Aspect::RawTarget(TypeContext::EnumVariant { ident, variant_ident, .. }) =>
                Resolve::<Type>::resolve(ident, source)
                    .joined_ident(variant_ident),
        }
    }
}
