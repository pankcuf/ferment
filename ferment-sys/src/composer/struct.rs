use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics, Lifetime};
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{constants, AttrComposable, CommaPunctuatedFields, ComposerLink, ConversionSeqKindComposer, DropSeqKindComposer, FFIConversionsSpec, FFIFieldsSpec, FFIObjectSpec, FieldPathConversionResolveSpec, FieldPathResolver, FieldsComposerRef, FieldsContext, FieldsConversionComposable, GenericsComposable, ItemComposer, ItemComposerExprSpec, ItemComposerLink, ItemComposerSpec, LinkedContextComposer, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, NameKindComposable, PresentableExprComposerRef, TypeAspect, FFIInterfaceMethodSpec, AspectSeqKindComposer, FieldNameSpec, FieldComposerProducer, FieldSpec, ArgKindProducerByRef, ItemAspectsSpec, LifetimesComposable};
#[cfg(feature = "accessors")]
use crate::composer::{ArgKindPair, ArgKindPairs, AspectArgSourceComposer, AspectSharedComposerLink, BindingComposer, CtorSpec, FFIBindingsSpec, MaybeFFIBindingsComposerLink, OwnerAspectSequence, PresentableArgKindPairComposerRef, SourceAccessible};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentable::{ArgKind, Aspect, Expression, ScopeContextPresentable, SeqKind};
#[cfg(feature = "accessors")]
use crate::presentable::BindingPresentableContext;
use crate::presentation::Name;

pub struct StructComposer<SPEC, I>
    where SPEC: Specification + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub composer: ItemComposerLink<SPEC, I>
}

#[cfg(not(feature = "accessors"))]
impl<SPEC, I> StructComposer<SPEC, I>
    where I: DelimiterTrait + ?Sized,
          SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          ItemComposer<SPEC, I>: NameKindComposable,
          Self: ItemComposerSpec<SPEC>
            + FFIFieldsSpec<SPEC, ItemComposerLink<SPEC, I>>
            + FFIConversionsSpec<SPEC, ItemComposerLink<SPEC, I>> {
    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &[Attribute],
        lifetimes: &[Lifetime],
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink,
    ) -> ComposerLink<Self> {
        Rc::new(RefCell::new(Self {
            composer: ItemComposer::new::<Self>(
                ty_context,
                AttrsModel::from(attrs),
                lifetimes.to_owned(),
                Some(generics.clone()),
                fields,
                context)
        }))
    }
}
#[cfg(feature = "accessors")]
impl<SPEC, I> StructComposer<SPEC, I>
    where I: DelimiterTrait + ?Sized,
          SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          ItemComposer<SPEC, I>: NameKindComposable,
          Self: ItemComposerSpec<SPEC>
            + CtorSpec<SPEC, ItemComposerLink<SPEC, I>, ArgKindPairs<SPEC>>
            + FFIFieldsSpec<SPEC, ItemComposerLink<SPEC, I>>
            + FFIConversionsSpec<SPEC, ItemComposerLink<SPEC, I>> {
    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &[Attribute],
        lifetimes: &[Lifetime],
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink,
    ) -> ComposerLink<Self> {
        Rc::new(RefCell::new(Self {
            composer: ItemComposer::new::<Self>(
                ty_context,
                AttrsModel::from(attrs),
                lifetimes.to_owned(),
                Some(generics.clone()),
                fields,
                context)
        }))
    }
}
impl<SPEC, T, I> FFIObjectSpec<SPEC, ComposerLink<T>> for StructComposer<SPEC, I>
    where SPEC: Specification,
          T: FieldsConversionComposable<SPEC> + 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<SPEC, T> = Some(
        LinkedContextComposer::new(
            SeqKind::bypass,
            SeqKind::fields_from));
}


impl<SPEC, I> FieldPathConversionResolveSpec<SPEC> for StructComposer<SPEC, I>
    where SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
          I: DelimiterTrait + ?Sized,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens {
    const FROM: FieldPathResolver<SPEC> =
        FieldComposer::STRUCT_FROM;
    const TO: FieldPathResolver<SPEC> =
        FieldComposer::STRUCT_TO;
    const DROP: FieldPathResolver<SPEC> =
        FieldComposer::STRUCT_DROP;
}

impl<SPEC, T, I> FFIConversionsSpec<SPEC, ComposerLink<T>> for StructComposer<SPEC, I>
    where SPEC: Specification,
          T: FieldsContext<SPEC>
              + AttrComposable<SPEC::Attr>
              + LifetimesComposable<SPEC::Lt>
              + GenericsComposable<SPEC::Gen>
              + TypeAspect<SPEC::TYC>
              + NameKindComposable
              + 'static,
          I: DelimiterTrait + ?Sized,
          Self: ItemComposerSpec<SPEC>
              + ItemComposerExprSpec<SPEC>
              + FieldPathConversionResolveSpec<SPEC> {
    const COMPOSER: MaybeFFIComposerLink<SPEC, T> = Some(
        constants::ffi_conversions_composer::<SPEC, T, Self>(
            SeqKind::struct_from,
            SeqKind::deref_ffi,
            Aspect::target,
            SeqKind::struct_to,
            SeqKind::empty,
            SeqKind::struct_drop_post_processor,
            SeqKind::empty
        )

    );
}
#[cfg(feature = "accessors")]
impl<SPEC, T, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for StructComposer<SPEC, Brace>
    where SPEC: Specification + 'static,
          T: AttrComposable<SPEC::Attr>
          + LifetimesComposable<SPEC::Lt>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          Iter: IntoIterator<Item=ArgKindPair<SPEC>> + FromIterator<Iter::Item> {
    const ROOT: BindingComposer<SPEC, OwnerAspectSequence<SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        constants::args_composer_iterator_root();

}
#[cfg(feature = "accessors")]
impl<SPEC, T, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for StructComposer<SPEC, Paren>
    where T: AttrComposable<SPEC::Attr>
            + LifetimesComposable<SPEC::Lt>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          SPEC: Specification + 'static,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    const ROOT: BindingComposer<SPEC, OwnerAspectSequence<SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        constants::args_composer_iterator_root();

}
#[cfg(feature = "accessors")]
impl<SPEC, T, I, Iter> FFIBindingsSpec<SPEC, ComposerLink<T>, Iter> for StructComposer<SPEC, I>
    where T: AttrComposable<SPEC::Attr>
            + LifetimesComposable<SPEC::Lt>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          I: DelimiterTrait
            + ?Sized,
          SPEC: Specification,
          Iter: IntoIterator<Item=ArgKindPair<SPEC>> + FromIterator<Iter::Item>,
          Self: CtorSpec<SPEC, ComposerLink<T>, Iter> {
    const COMPOSER: MaybeFFIBindingsComposerLink<SPEC, T, Iter> =
        Some(constants::ffi_bindings_composer::<SPEC, T, Self, Iter>());
}
impl<SPEC> FieldNameSpec<SPEC> for StructComposer<SPEC, Brace>
where SPEC: Specification<Name=Name<SPEC>> {
    const COMPOSER: FieldComposerProducer<SPEC> =
        FieldComposer::named_producer;
}

impl<SPEC> FieldNameSpec<SPEC> for StructComposer<SPEC, Paren>
where SPEC: Specification<Name=Name<SPEC>> {
    const COMPOSER: FieldComposerProducer<SPEC> =
        FieldComposer::unnamed_struct_producer;
}

impl<SPEC> FieldSpec<SPEC> for StructComposer<SPEC, Brace>
where SPEC: Specification {
    const FIELD_PRODUCER: ArgKindProducerByRef<SPEC> =
        ArgKind::public_named;
}

impl<SPEC> FieldSpec<SPEC> for StructComposer<SPEC, Paren>
where SPEC: Specification {
    const FIELD_PRODUCER: ArgKindProducerByRef<SPEC> =
        ArgKind::default_field_type;
}


impl<SPEC> ItemComposerSpec<SPEC> for StructComposer<SPEC, Brace>
    where SPEC: Specification<Name=Name<SPEC>>,
          Name<SPEC>: ToTokens,
          Self: FieldSpec<SPEC> + FieldNameSpec<SPEC> {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::NamedStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::NamedStruct;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::FromNamedFields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToNamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC> =
        Self::FIELDS;
}

impl<SPEC> ItemComposerSpec<SPEC> for StructComposer<SPEC, Paren>
    where SPEC: Specification<Name=Name<SPEC>>,
          Name<SPEC>: ToTokens,
          Self: FieldSpec<SPEC> + FieldNameSpec<SPEC> {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::UnnamedStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::UnnamedStruct;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToUnnamedFields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::FromUnnamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC> =
        Self::FIELDS;
}

impl<SPEC> ItemComposerExprSpec<SPEC> for StructComposer<SPEC, Brace>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::named_conversion;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::named_conversion;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}

impl<SPEC> ItemComposerExprSpec<SPEC> for StructComposer<SPEC, Paren>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}

#[allow(unused)]
pub struct FromStruct<I: DelimiterTrait + 'static>(PhantomData<I>);
#[allow(unused)]
pub struct ToStruct<I: DelimiterTrait + 'static>(PhantomData<I>);
#[allow(unused)]
pub struct DropStruct<I: DelimiterTrait + 'static>(PhantomData<I>);
#[allow(unused)]
pub struct CtorStruct<I: DelimiterTrait + 'static>(PhantomData<I>);

impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for FromStruct<Brace>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Self: FieldPathConversionResolveSpec<SPEC>,
          SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::FromNamedFields;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::named_conversion;
}
impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for FromStruct<Paren>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Self: FieldPathConversionResolveSpec<SPEC>,
          SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::FromUnnamedFields;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}

impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for ToStruct<Brace>
where SPEC: Specification<Expr=Expression<SPEC>>,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::ToNamedFields;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::named_conversion;
}
impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for ToStruct<Paren>
where SPEC: Specification<Expr=Expression<SPEC>>,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::ToUnnamedFields;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}
impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for ToStruct<Void>
where SPEC: Specification<Expr=Expression<SPEC>>,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::no_fields;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}

impl<SPEC, I> FFIInterfaceMethodSpec<SPEC, Semi> for DropStruct<I>
where SPEC: Specification<Expr=Expression<SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::DROP;
    const SEQ: AspectSeqKindComposer<SPEC, Semi> =
        SeqKind::StructDropBody;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}

impl<SPEC, I> ItemAspectsSpec<SPEC> for StructComposer<SPEC, I>
where SPEC: Specification,
      I: DelimiterTrait,
      FromStruct<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      ToStruct<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      DropStruct<I>: FFIInterfaceMethodSpec<SPEC, Semi>
{

    type DTOR = DropStruct<I>;
    type FROM = FromStruct<I>;
    type INTO = ToStruct<I>;
}