use quote::ToTokens;
use syn::{Attribute, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use crate::ast::Void;
use crate::composer::{AttrComposable, EnumComposer, EnumComposerLink, EnumVariantComposer, EnumVariantComposerLink, FFIAspect, FFIBindingsComposer, ImplComposer, ImplComposerLink, OpaqueStructComposer, OpaqueStructComposerLink, SigComposer, SigComposerLink, StructComposer, StructComposerLink, TraitComposer, TraitComposerLink, TypeAliasComposerLink};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::lang::Specification;
use crate::presentable::{BindingPresentableContext, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::Name;

#[allow(unused)]
pub enum ItemComposerWrapper<SPEC>
    where SPEC: Specification + 'static {
    Enum(EnumComposerLink<SPEC>),
    EnumVariantNamed(EnumVariantComposerLink<SPEC, Brace>),
    EnumVariantUnnamed(EnumVariantComposerLink<SPEC, Paren>),
    EnumVariantUnit(EnumVariantComposerLink<SPEC, Void>),
    StructNamed(StructComposerLink<SPEC, Brace>),
    StructUnnamed(StructComposerLink<SPEC, Paren>),
    OpaqueStructNamed(OpaqueStructComposerLink<SPEC, Brace>),
    OpaqueStructUnnamed(OpaqueStructComposerLink<SPEC, Paren>),
    Sig(SigComposerLink<SPEC>),
    TypeAlias(TypeAliasComposerLink<SPEC, Paren>),
    Trait(TraitComposerLink<SPEC>),
    Impl(ImplComposerLink<SPEC>),
}



impl<SPEC> ItemComposerWrapper<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens {

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
        ItemComposerWrapper::<SPEC>::Enum(EnumComposer::<SPEC>::new(item_enum, ty_context, context))
    }
    pub fn variant(fields: &Fields, ty_context: SPEC::TYC, attrs: &Vec<Attribute>, context: &ScopeContextLink) -> Self {
        match fields {
            Fields::Unit =>
                ItemComposerWrapper::EnumVariantUnit(EnumVariantComposer::<SPEC, Void>::new(ty_context, attrs, &Punctuated::new(), context)),
            Fields::Unnamed(fields) =>
                ItemComposerWrapper::EnumVariantUnnamed(EnumVariantComposer::<SPEC, Paren>::new(ty_context, attrs, &fields.unnamed, context)),
            Fields::Named(fields) =>
                ItemComposerWrapper::EnumVariantNamed(EnumVariantComposer::<SPEC, Brace>::new(ty_context, attrs, &fields.named, context)),
        }
    }
    pub fn r#struct(item_struct: &ItemStruct, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        let ItemStruct { attrs, fields: ref f, generics, .. } = item_struct;
        match f {
            Fields::Unnamed(ref fields) =>
                ItemComposerWrapper::StructUnnamed(StructComposer::<SPEC, Paren>::new(ty_context, attrs, generics, &vec![], &fields.unnamed, context)),
            Fields::Named(ref fields) =>
                ItemComposerWrapper::StructNamed(StructComposer::<SPEC, Brace>::new(ty_context, attrs, generics, &vec![], &fields.named, context)),
            Fields::Unit =>
                ItemComposerWrapper::StructNamed(StructComposer::<SPEC, Brace>::new(ty_context, attrs, generics, &vec![], &Punctuated::new(), context)),
        }
    }
    pub fn opaque_struct(item_struct: &ItemStruct, ty_context: SPEC::TYC, context: &ScopeContextLink) -> Self {
        let ItemStruct { attrs, fields: ref f, generics, .. } = item_struct;
        let lifetimes = vec![];
        match f {
            Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) =>
                ItemComposerWrapper::OpaqueStructUnnamed(
                    OpaqueStructComposer::<SPEC, Paren>::new(ty_context, attrs, generics, &lifetimes, unnamed, context)),
            Fields::Named(FieldsNamed { ref named, .. }) =>
                ItemComposerWrapper::OpaqueStructNamed(
                    OpaqueStructComposer::<SPEC, Brace>::new(ty_context, attrs, generics, &lifetimes, named, context)),
            Fields::Unit =>
                ItemComposerWrapper::OpaqueStructNamed(
                    OpaqueStructComposer::<SPEC, Brace>::new(ty_context, attrs, generics, &lifetimes, &Punctuated::new(), context))
        }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect) -> SeqKind<SPEC> {
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
    pub fn compose_ctor(&self) -> Option<BindingPresentableContext<SPEC>> {
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

impl<SPEC> AttrComposable<SPEC::Attr> for ItemComposerWrapper<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
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
        }
    }
}
