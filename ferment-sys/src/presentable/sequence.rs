use std::fmt::Debug;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::Type;
use ferment_macro::Display;
use crate::ast::CommaPunctuated;
use crate::composer::{AspectCommaPunctuatedArguments, AttrComposable, TypeAspect, VariantComposable, FieldsConversionComposable, SourceComposable, ComposerLinkRef, AspectTerminatedArguments, AspectPresentableArguments, CommaPunctuatedArgKinds};
use crate::lang::Specification;
use crate::presentable::Aspect;


#[derive(Clone, Debug, Display)]
pub enum SeqKind<SPEC>
    where SPEC: Specification {
    FromStub(AspectCommaPunctuatedArguments<SPEC>),
    FromNamedFields(AspectCommaPunctuatedArguments<SPEC>),
    ToNamedFields(AspectCommaPunctuatedArguments<SPEC>),
    FromUnnamedFields(AspectCommaPunctuatedArguments<SPEC>),
    TraitImplFnCall(Type, Type, Ident, CommaPunctuatedArgKinds<SPEC>),
    ToUnnamedFields(AspectCommaPunctuatedArguments<SPEC>),
    ToStub(AspectCommaPunctuatedArguments<SPEC>),
    NamedVariantFields(AspectCommaPunctuatedArguments<SPEC>),
    UnnamedVariantFields(AspectCommaPunctuatedArguments<SPEC>),
    EnumUnitFields(AspectCommaPunctuatedArguments<SPEC>),

    Variants(Aspect<SPEC::TYC>, SPEC::Attr, CommaPunctuated<SeqKind<SPEC>>),
    Unit(Aspect<SPEC::TYC>),
    NoFieldsConversion(Aspect<SPEC::TYC>),
    TypeAliasFromConversion(AspectCommaPunctuatedArguments<SPEC>),
    NamedStruct(AspectCommaPunctuatedArguments<SPEC>),
    UnnamedStruct(AspectCommaPunctuatedArguments<SPEC>),
    StubStruct(AspectCommaPunctuatedArguments<SPEC>),
    Enum(Box<SeqKind<SPEC>>),

    StructFrom(Box<SeqKind<SPEC>>, Box<SeqKind<SPEC>>),
    StructTo(Box<SeqKind<SPEC>>, Box<SeqKind<SPEC>>),

    EnumVariantFrom(Box<SeqKind<SPEC>>, Box<SeqKind<SPEC>>),
    EnumVariantTo(Box<SeqKind<SPEC>>, Box<SeqKind<SPEC>>),
    EnumVariantDrop(Box<SeqKind<SPEC>>, Box<SeqKind<SPEC>>),

    DerefFFI,
    Obj,
    Empty,

    DropStub(AspectTerminatedArguments<SPEC>),
    StructDropBody(AspectTerminatedArguments<SPEC>),
    DropCode(AspectTerminatedArguments<SPEC>),
}

impl<SPEC> SeqKind<SPEC>
    where SPEC: Specification {
    pub fn struct_to(field_path: &SeqKind<SPEC>, conversions: SeqKind<SPEC>) -> Self {
        Self::StructTo(field_path.clone().into(), conversions.into())
    }
    pub fn struct_from(field_path: &SeqKind<SPEC>, conversions: SeqKind<SPEC>) -> Self {
        Self::StructFrom(field_path.clone().into(), conversions.into())
    }
    pub fn variant_from(left: &SeqKind<SPEC>, right: SeqKind<SPEC>) -> Self {
        Self::EnumVariantFrom(left.clone().into(), right.clone().into())
    }
    pub fn variant_to(left: &SeqKind<SPEC>, right: SeqKind<SPEC>) -> Self {
        Self::EnumVariantTo(left.clone().into(), right.clone().into())
    }
    pub fn variant_drop(left: &SeqKind<SPEC>, right: SeqKind<SPEC>) -> Self {
        Self::EnumVariantDrop(left.clone().into(), right.clone().into())
    }
    pub fn struct_drop_post_processor(_: &SeqKind<SPEC>, right: SeqKind<SPEC>) -> Self {
        right
    }

    pub fn no_fields<SEP: ToTokens>(((aspect, ..), _): AspectPresentableArguments<SPEC, SEP>) -> Self {
        Self::NoFieldsConversion(match &aspect {
            Aspect::Target(context) => Aspect::RawTarget(context.clone()),
            _ => aspect.clone(),
        })
    }
    pub fn unit(((aspect, ..), _): &AspectCommaPunctuatedArguments<SPEC>) -> Self {
        Self::Unit(aspect.clone())
    }
    pub fn variants<C>(composer_ref: &ComposerLinkRef<C>) -> Self
        where C: AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + VariantComposable<SPEC> {
        Self::Variants(C::target_type_aspect(composer_ref), C::compose_attributes(composer_ref), C::compose_variants(composer_ref))
    }
    pub fn deref_ffi<C>(_ctx: &ComposerLinkRef<C>) -> Self {
        Self::DerefFFI
    }
    pub fn empty<C>(_ctx: &ComposerLinkRef<C>) -> Self {
        Self::Empty
    }
    pub fn obj<C>(_ctx: &ComposerLinkRef<C>) -> Self {
        Self::Obj
    }
    pub fn unit_fields(context: &AspectCommaPunctuatedArguments<SPEC>) -> Self {
        Self::EnumUnitFields(context.clone())
    }
    pub fn brace_variants(context: &AspectCommaPunctuatedArguments<SPEC>) -> Self {
        Self::NamedVariantFields(context.clone())
    }
    pub fn paren_variants(context: &AspectCommaPunctuatedArguments<SPEC>) -> Self {
        Self::UnnamedVariantFields(context.clone())
    }
    pub fn empty_root(_: SeqKind<SPEC>) -> Self {
        Self::Empty
    }
    pub fn bypass(sequence: SeqKind<SPEC>) -> Self {
        sequence
    }
    pub fn r#enum(context: SeqKind<SPEC>) -> Self {
        Self::Enum(Box::new(context))
    }
    pub fn fields_from<C>(ctx: &ComposerLinkRef<C>) -> Self
        where C: FieldsConversionComposable<SPEC> + 'static {
        ctx.fields_from().compose(&())
    }
    pub fn fields_to<C>(ctx: &ComposerLinkRef<C>) -> Self
        where C: FieldsConversionComposable<SPEC> + 'static {
        ctx.fields_to().compose(&())
    }
}

