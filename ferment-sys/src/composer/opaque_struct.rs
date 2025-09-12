use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics, Lifetime};
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::DelimiterTrait;
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{CommaPunctuatedFields, ComposerLink, ConversionSeqKindComposer, DropSeqKindComposer, FFIConversionsSpec, FFIFieldsSpec, FFIObjectSpec, FieldsComposerRef, FieldsConversionComposable, ItemComposer, ItemComposerExprSpec, ItemComposerLink, ItemComposerSpec, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, NameKindComposable, PresentableExprComposerRef, FieldSpec, ArgKindProducerByRef, FieldNameSpec, FieldComposerProducer, ItemAspectsSpec, FFIInterfaceMethodSpec, FromStub, FieldPathConversionResolveSpec, FieldPathResolver, AspectSeqKindComposer, ToStub, DropStub};
#[cfg(feature = "accessors")]
use crate::composer::{ArgKindPair, ArgKindPairs, AspectArgSourceComposer, AspectSharedComposerLink, AttrComposable, BindingComposer, constants, CtorSpec, FFIBindingsSpec, FieldsContext, GenericsComposable, LifetimesComposable, MaybeFFIBindingsComposerLink, OwnerAspectSequence, PresentableArgKindPairComposerRef, SourceAccessible, TypeAspect};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentable::{ArgKind, Expression, ScopeContextPresentable, SeqKind};
#[cfg(feature = "accessors")]
use crate::presentable::{Aspect, BindingPresentableContext};
use crate::presentation::Name;

pub struct OpaqueStructComposer<SPEC, I>
    where I: DelimiterTrait + 'static + ?Sized,
          SPEC: Specification + 'static {
    pub composer: ItemComposerLink<SPEC, I>
}

#[cfg(not(feature = "accessors"))]
impl<SPEC, I> OpaqueStructComposer<SPEC, I>
    where I: DelimiterTrait + ?Sized,
          SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          ItemComposer<SPEC, I>: NameKindComposable,
          Self: ItemComposerSpec<SPEC>
          + FFIFieldsSpec<SPEC, ItemComposerLink<SPEC, I>>
          + FieldSpec<SPEC>
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
impl<SPEC, I> OpaqueStructComposer<SPEC, I>
    where I: DelimiterTrait + ?Sized,
          SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          ItemComposer<SPEC, I>: NameKindComposable,
          Self: ItemComposerSpec<SPEC>
          + CtorSpec<SPEC, ItemComposerLink<SPEC, I>, ArgKindPairs<SPEC>>
          + FFIFieldsSpec<SPEC, ItemComposerLink<SPEC, I>>
          + FieldSpec<SPEC>
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
impl<SPEC, I, T> FFIObjectSpec<SPEC, ComposerLink<T>> for OpaqueStructComposer<SPEC, I>
    where SPEC: Specification,
          T: FieldsConversionComposable<SPEC> + 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<SPEC, T> = None;
}
impl<SPEC, T, I> FFIConversionsSpec<SPEC, ComposerLink<T>> for OpaqueStructComposer<SPEC, I>
where T: 'static,
      I: DelimiterTrait + ?Sized,
      SPEC: Specification,
      Self: ItemComposerSpec<SPEC>
      + ItemComposerExprSpec<SPEC> {
    const COMPOSER: MaybeFFIComposerLink<SPEC, T> = None;
}


#[cfg(feature = "accessors")]
impl<SPEC, T, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for OpaqueStructComposer<SPEC, Paren>
    where T: AttrComposable<SPEC::Attr>
            + LifetimesComposable<SPEC::Lt>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          SPEC: Specification + 'static,
          Iter: IntoIterator<Item=ArgKindPair<SPEC>> + FromIterator<Iter::Item> {
    const ROOT: BindingComposer<SPEC, OwnerAspectSequence<SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::target;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        constants::args_composer_iterator_root();
}
#[cfg(feature = "accessors")]
impl<SPEC, T, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for OpaqueStructComposer<SPEC, Brace>
    where SPEC: Specification + 'static,
          T: AttrComposable<SPEC::Attr>
          + LifetimesComposable<SPEC::Lt>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    const ROOT: BindingComposer<SPEC, OwnerAspectSequence<SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::target;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::opaque_named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        constants::args_composer_iterator_root();
}

#[cfg(feature = "accessors")]
impl<SPEC, T, I, Iter> FFIBindingsSpec<SPEC, ComposerLink<T>, Iter> for OpaqueStructComposer<SPEC, I>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          I: DelimiterTrait + ?Sized,
          SPEC: Specification,
          Iter: IntoIterator<Item=ArgKindPair<SPEC>> + FromIterator<Iter::Item>,
        Self: CtorSpec<SPEC, ComposerLink<T>, Iter> {
    const COMPOSER: MaybeFFIBindingsComposerLink<SPEC, T, Iter> = None;
}
impl<SPEC> FieldNameSpec<SPEC> for OpaqueStructComposer<SPEC, Brace>
where SPEC: Specification<Name=Name<SPEC>> {
    const COMPOSER: FieldComposerProducer<SPEC> =
        FieldComposer::named_producer;
}
impl<SPEC> FieldNameSpec<SPEC> for OpaqueStructComposer<SPEC, Paren>
where SPEC: Specification<Name=Name<SPEC>> {
    const COMPOSER: FieldComposerProducer<SPEC> =
        FieldComposer::unnamed_struct_producer;
}
impl<SPEC> FieldSpec<SPEC> for OpaqueStructComposer<SPEC, Brace>
    where SPEC: Specification {
    const FIELD_PRODUCER: ArgKindProducerByRef<SPEC> =
        ArgKind::public_named;
}

impl<SPEC> FieldSpec<SPEC> for OpaqueStructComposer<SPEC, Paren>
    where SPEC: Specification {
    const FIELD_PRODUCER: ArgKindProducerByRef<SPEC> =
        ArgKind::default_field_type;
}

impl<SPEC, I> ItemComposerSpec<SPEC> for OpaqueStructComposer<SPEC, I>
    where SPEC: Specification<Name=Name<SPEC>>,
          I: DelimiterTrait,
          Name<SPEC>: ToTokens,
          Self: FieldSpec<SPEC> + FieldNameSpec<SPEC> {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::StubStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::StubStruct;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::FromStub;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToStub;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC> =
        SeqKind::DropStub;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC> =
        Self::FIELDS;
}

impl<SPEC> ItemComposerExprSpec<SPEC> for OpaqueStructComposer<SPEC, Brace>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::named_conversion;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::named_conversion;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
}
impl<SPEC> ItemComposerExprSpec<SPEC> for OpaqueStructComposer<SPEC, Paren>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
}

impl<SPEC, I> ItemAspectsSpec<SPEC> for OpaqueStructComposer<SPEC, I>
where SPEC: Specification<Expr=Expression<SPEC>>,
      I: DelimiterTrait,
      SPEC::Expr: ScopeContextPresentable,
      FromStub<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      ToStub<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      DropStub<I>: FFIInterfaceMethodSpec<SPEC, Semi>
{

    type DTOR = DropStub<I>;
    type FROM = FromStub<I>;
    type INTO = ToStub<I>;
}

impl<SPEC, I> FFIInterfaceMethodSpec<SPEC, Comma> for FromStub<I>
where SPEC: Specification<Expr=Expression<SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::FromStub;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}
impl<SPEC, I> FFIInterfaceMethodSpec<SPEC, Comma> for ToStub<I>
where SPEC: Specification<Expr=Expression<SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::ToStub;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}
impl<SPEC, I> FFIInterfaceMethodSpec<SPEC, Semi> for DropStub<I>
where SPEC: Specification<Expr=Expression<SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::DROP;
    const SEQ: AspectSeqKindComposer<SPEC, Semi> =
        SeqKind::DropStub;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}
