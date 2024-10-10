use std::fmt::Debug;
use quote::ToTokens;
use syn::{Attribute, Fields, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use crate::ast::Void;
use crate::composer::{AttrComposable, EnumComposer, EnumComposerLink, EnumVariantComposer, EnumVariantComposerLink, FFIAspect, FFIBindingsComposer, ImplComposer, ImplComposerLink, OpaqueStructComposer, OpaqueStructComposerLink, SigComposer, SigComposerLink, SourceFermentable, StructComposer, StructComposerLink, TraitComposer, TraitComposerLink, TypeAliasComposerLink};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::ToType;
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{Name, RustFermentate};

#[allow(unused)]
pub enum ItemComposerWrapper<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    Enum(EnumComposerLink<LANG, SPEC>),
    EnumVariantNamed(EnumVariantComposerLink<Brace, LANG, SPEC>),
    EnumVariantUnnamed(EnumVariantComposerLink<Paren, LANG, SPEC>),
    EnumVariantUnit(EnumVariantComposerLink<Void, LANG, SPEC>),
    StructNamed(StructComposerLink<Brace, LANG, SPEC>),
    StructUnnamed(StructComposerLink<Paren, LANG, SPEC>),
    OpaqueStructNamed(OpaqueStructComposerLink<Brace, LANG, SPEC>),
    OpaqueStructUnnamed(OpaqueStructComposerLink<Paren, LANG, SPEC>),
    Sig(SigComposerLink<LANG, SPEC>),
    TypeAlias(TypeAliasComposerLink<Paren, LANG, SPEC>),
    Trait(TraitComposerLink<LANG, SPEC>),
    Impl(ImplComposerLink<LANG, SPEC>),
}

impl<LANG, SPEC> ItemComposerWrapper<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {

    pub fn r#trait(item_trait: &ItemTrait, ty_context: SPEC::TYC, scope: &ScopeChain, context: &ScopeContextLink) -> Self {
        ItemComposerWrapper::Trait(TraitComposer::from_item_trait(item_trait, ty_context, scope, context))
    }
    pub fn r#impl(item_impl: &ItemImpl, ty_context: SPEC::TYC, scope: &ScopeChain, context: &ScopeContextLink) -> Self {
        ItemComposerWrapper::Impl(ImplComposer::from_item_impl(item_impl, ty_context, scope, context))
    }
    pub fn r#fn(item_fn: &ItemFn, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        ItemComposerWrapper::Sig(SigComposer::from_item_fn(item_fn, ty_context, context))
    }
    pub fn r#enum(item_enum: &ItemEnum, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        ItemComposerWrapper::<LANG, SPEC>::Enum(EnumComposer::<LANG, SPEC>::new(item_enum, ty_context, context))
    }
    pub fn variant(fields: &Fields, ty_context: SPEC::TYC, attrs: &Vec<Attribute>, context: &ScopeContextLink) -> Self {
        match fields {
            Fields::Unit =>
                ItemComposerWrapper::EnumVariantUnit(EnumVariantComposer::<Void, LANG, SPEC>::new(ty_context, attrs, &Punctuated::new(), context)),
            Fields::Unnamed(fields) =>
                ItemComposerWrapper::EnumVariantUnnamed(EnumVariantComposer::<Paren, LANG, SPEC>::new(ty_context, attrs, &fields.unnamed, context)),
            Fields::Named(fields) =>
                ItemComposerWrapper::EnumVariantNamed(EnumVariantComposer::<Brace, LANG, SPEC>::new(ty_context, attrs, &fields.named, context)),
        }
        // match fields {
        //     Fields::Unit =>
        //         ItemComposerWrapper::EnumVariantUnit(EnumVariantComposer::<Void, LANG, SPEC>::unit(ty_context, attrs, &Punctuated::new(), context)),
        //     Fields::Unnamed(fields) =>
        //         ItemComposerWrapper::EnumVariantUnnamed(EnumVariantComposer::<Paren, LANG, SPEC>::unnamed(ty_context, attrs, &fields.unnamed, context)),
        //     Fields::Named(fields) =>
        //         ItemComposerWrapper::EnumVariantNamed(EnumVariantComposer::<Brace, LANG, SPEC>::named(ty_context, attrs, &fields.named, context)),
        // }
    }
    pub fn r#struct(item_struct: &ItemStruct, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        let ItemStruct { attrs, fields: ref f, generics, .. } = item_struct;
        match f {
            Fields::Unnamed(ref fields) =>
                ItemComposerWrapper::StructUnnamed(StructComposer::<Paren, LANG, SPEC>::new(ty_context, attrs, generics, &fields.unnamed, context)),
            Fields::Named(ref fields) =>
                ItemComposerWrapper::StructNamed(StructComposer::<Brace, LANG, SPEC>::new(ty_context, attrs, generics, &fields.named, context)),
            Fields::Unit =>
                ItemComposerWrapper::StructNamed(StructComposer::<Brace, LANG, SPEC>::new(ty_context, attrs, generics, &Punctuated::new(), context)),
        }
    }
    pub fn opaque_struct(item_struct: &ItemStruct, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        let ItemStruct { attrs, fields: ref f, generics, .. } = item_struct;
        match f {
            Fields::Unnamed(ref fields) =>
                ItemComposerWrapper::OpaqueStructUnnamed(
                    OpaqueStructComposer::<Paren, LANG, SPEC>::new(ty_context, attrs, generics, &fields.unnamed, context)),
            Fields::Named(ref fields) =>
                ItemComposerWrapper::OpaqueStructNamed(
                    OpaqueStructComposer::<Brace, LANG, SPEC>::new(ty_context, attrs, generics, &fields.named, context)),
            Fields::Unit =>
                ItemComposerWrapper::OpaqueStructNamed(
                    OpaqueStructComposer::<Brace, LANG, SPEC>::new(ty_context, attrs, generics, &Punctuated::new(), context))
        }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect) -> PresentableSequence<LANG, SPEC> {
        match self {
            ItemComposerWrapper::EnumVariantNamed(composer) =>
                composer.borrow().composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::EnumVariantUnnamed(composer) =>
                composer.borrow().composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::EnumVariantUnit(composer) =>
                composer.borrow().composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::StructNamed(composer) =>
                composer.borrow().composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::StructUnnamed(composer) =>
                composer.borrow().composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::TypeAlias(composer) =>
                composer.borrow().composer.borrow().compose_aspect(aspect),
            _ => PresentableSequence::Empty
        }
    }
    pub fn compose_ctor(&self) -> Option<BindingPresentableContext<LANG, SPEC>> {
        match self {
            ItemComposerWrapper::EnumVariantNamed(composer) =>
                composer.borrow().composer.borrow().bindings_composer.as_ref().map(FFIBindingsComposer::compose_ctor),
            ItemComposerWrapper::EnumVariantUnnamed(composer) =>
                composer.borrow().composer.borrow().bindings_composer.as_ref().map(FFIBindingsComposer::compose_ctor),
            ItemComposerWrapper::EnumVariantUnit(composer) =>
                composer.borrow().composer.borrow().bindings_composer.as_ref().map(FFIBindingsComposer::compose_ctor),
            ItemComposerWrapper::StructNamed(composer) =>
                composer.borrow().composer.borrow().bindings_composer.as_ref().map(FFIBindingsComposer::compose_ctor),
            ItemComposerWrapper::StructUnnamed(composer) =>
                composer.borrow().composer.borrow().bindings_composer.as_ref().map(FFIBindingsComposer::compose_ctor),
            ItemComposerWrapper::OpaqueStructUnnamed(composer) =>
                composer.borrow().composer.borrow().bindings_composer.as_ref().map(FFIBindingsComposer::compose_ctor),
            ItemComposerWrapper::OpaqueStructNamed(composer) =>
                composer.borrow().composer.borrow().bindings_composer.as_ref().map(FFIBindingsComposer::compose_ctor),
            _ => None,
        }
    }
}

impl<LANG, SPEC> AttrComposable<SPEC::Attr> for ItemComposerWrapper<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC::Attr {
        match self {
            ItemComposerWrapper::Enum(composer) =>
                composer.borrow().compose_attributes(),
            ItemComposerWrapper::EnumVariantUnit(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::EnumVariantNamed(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::StructNamed(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::EnumVariantUnnamed(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::StructUnnamed(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::OpaqueStructUnnamed(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::OpaqueStructNamed(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::Sig(composer) =>
                composer.borrow().compose_attributes(),
            ItemComposerWrapper::TypeAlias(composer) =>
                composer.borrow().composer.borrow().compose_attributes(),
            ItemComposerWrapper::Trait(composer) =>
                composer.borrow().compose_attributes(),
            ItemComposerWrapper::Impl(composer) =>
                composer.borrow().compose_attributes(),
        }    }
}

impl<SPEC> ItemComposerWrapper<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    pub fn ferment(&self) -> RustFermentate {
        match self {
            ItemComposerWrapper::Enum(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::StructNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::OpaqueStructUnnamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::OpaqueStructNamed(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::Sig(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::TypeAlias(composer) => composer.borrow().composer.borrow().ferment(),
            ItemComposerWrapper::Trait(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::Impl(composer) => composer.borrow().ferment(),
        }
    }
}

