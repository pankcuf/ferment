use quote::ToTokens;
use syn::{Attribute, Fields, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use crate::ast::Void;
use crate::composer::{AttrComposable, EnumComposer, EnumComposerLink, EnumVariantComposer, EnumVariantComposerLink, FFIAspect, FFIBindingsComposer, ImplComposer, ImplComposerLink, OpaqueStructComposer, OpaqueStructComposerLink, SigComposer, SigComposerLink, SourceFermentable, StructComposer, StructComposerLink, TraitComposer, TraitComposerLink, TypeAliasComposerLink};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{BindingPresentableContext, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{Name, RustFermentate};

#[allow(unused)]
pub enum ItemComposerWrapper<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    Enum(EnumComposerLink<LANG, SPEC>),
    EnumVariantNamed(EnumVariantComposerLink<LANG, SPEC, Brace>),
    EnumVariantUnnamed(EnumVariantComposerLink<LANG, SPEC, Paren>),
    EnumVariantUnit(EnumVariantComposerLink<LANG, SPEC, Void>),
    StructNamed(StructComposerLink<LANG, SPEC, Brace>),
    StructUnnamed(StructComposerLink<LANG, SPEC, Paren>),
    OpaqueStructNamed(OpaqueStructComposerLink<LANG, SPEC, Brace>),
    OpaqueStructUnnamed(OpaqueStructComposerLink<LANG, SPEC, Paren>),
    Sig(SigComposerLink<LANG, SPEC>),
    TypeAlias(TypeAliasComposerLink<LANG, SPEC, Paren>),
    Trait(TraitComposerLink<LANG, SPEC>),
    Impl(ImplComposerLink<LANG, SPEC>),
}

impl<LANG, SPEC> ItemComposerWrapper<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens {

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
                ItemComposerWrapper::EnumVariantUnit(EnumVariantComposer::<LANG, SPEC, Void>::new(ty_context, attrs, &Punctuated::new(), context)),
            Fields::Unnamed(fields) =>
                ItemComposerWrapper::EnumVariantUnnamed(EnumVariantComposer::<LANG, SPEC, Paren>::new(ty_context, attrs, &fields.unnamed, context)),
            Fields::Named(fields) =>
                ItemComposerWrapper::EnumVariantNamed(EnumVariantComposer::<LANG, SPEC, Brace>::new(ty_context, attrs, &fields.named, context)),
        }
    }
    pub fn r#struct(item_struct: &ItemStruct, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        let ItemStruct { attrs, fields: ref f, generics, .. } = item_struct;
        match f {
            Fields::Unnamed(ref fields) =>
                ItemComposerWrapper::StructUnnamed(StructComposer::<LANG, SPEC, Paren>::new(ty_context, attrs, generics, &vec![], &fields.unnamed, context)),
            Fields::Named(ref fields) =>
                ItemComposerWrapper::StructNamed(StructComposer::<LANG, SPEC, Brace>::new(ty_context, attrs, generics, &vec![], &fields.named, context)),
            Fields::Unit =>
                ItemComposerWrapper::StructNamed(StructComposer::<LANG, SPEC, Brace>::new(ty_context, attrs, generics, &vec![], &Punctuated::new(), context)),
        }
    }
    pub fn opaque_struct(item_struct: &ItemStruct, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        let ItemStruct { attrs, fields: ref f, generics, .. } = item_struct;
        match f {
            Fields::Unnamed(ref fields) =>
                ItemComposerWrapper::OpaqueStructUnnamed(
                    OpaqueStructComposer::<LANG, SPEC, Paren>::new(ty_context, attrs, generics, &vec![], &fields.unnamed, context)),
            Fields::Named(ref fields) =>
                ItemComposerWrapper::OpaqueStructNamed(
                    OpaqueStructComposer::<LANG, SPEC, Brace>::new(ty_context, attrs, generics, &vec![], &fields.named, context)),
            Fields::Unit =>
                ItemComposerWrapper::OpaqueStructNamed(
                    OpaqueStructComposer::<LANG, SPEC, Brace>::new(ty_context, attrs, generics, &vec![], &Punctuated::new(), context))
        }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect) -> SeqKind<LANG, SPEC> {
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
            _ => SeqKind::Empty
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
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
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

