use std::marker::PhantomData;
use quote::ToTokens;
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::FieldComposer;
use crate::composer::{AspectSeqKindComposer, AttrComposable, ComposerLink, PresentableArgumentPairComposerRef, ConversionSeqKindComposer, DropSeqKindComposer, FFIBindingsComposer, FFIComposer, ArgProducerByRef, FieldPathResolver, FieldsComposerRef, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, FieldsOwnedSequenceComposerLink, GenericsComposable, MaybeSequenceOutputComposer, NameKindComposable, OwnerAspectSequenceComposer, OwnerAspectSequenceSpecComposer, ArgKindPair, PresentableExprComposerRef, SequenceComposer, SharedAspectArgComposer, SourceAccessible, TypeAspect, PresentableArgsSequenceComposer, IterativeComposer, FFIInterfaceMethodIterator, ArgKindProducerByRef, FieldComposerProducer, OwnerAspect, OwnedArgComposers, CommaPunctuatedArgKinds, OwnerAspectSequence, constants, AspectArgSourceComposer};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::shared::SharedAccess;

pub trait FFIObjectSpec<LANG, SPEC, L>
    where L: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const COMPOSER: MaybeSequenceOutputComposer<LANG, SPEC, L>;
}

pub trait FFIConversionsSpec<LANG, SPEC, L>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          L: SharedAccess {
    const COMPOSER: Option<FFIComposer<LANG, SPEC, L>>;
}
pub trait FFIBindingsSpec<LANG, SPEC, L, Iter>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          L: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> +  {
    const COMPOSER: Option<FFIBindingsComposer<LANG, SPEC, L, Iter>>;
}

#[allow(unused)]
pub trait OwnerAspectSequenceSpec<LANG, SPEC, L, Iter, IterMap, Out>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          L: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=IterMap> {
    const ROOT: OwnerAspectSequenceComposer<LANG, SPEC, Iter, Out>;
    const ASPECT: SharedAspectArgComposer<LANG, SPEC, L>;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter>;
    const ARG: ArgProducerByRef<LANG, SPEC, Iter::Item>;
    const COMPOSER: OwnerAspectSequenceSpecComposer<LANG, SPEC, L, Iter, Out> =
        SequenceComposer::with_iterator_setup(Self::ROOT, Self::ASPECT, Self::ITER, Self::ARG);
}
pub trait CtorSpec<LANG, SPEC, L, Iter>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          L: SharedAccess,
          Iter: IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    const ROOT: OwnerAspectSequenceComposer<LANG, SPEC, Iter, BindingPresentableContext<LANG, SPEC>>;
    const ASPECT: SharedAspectArgComposer<LANG, SPEC, L>;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC>;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter>;
}

pub trait FieldSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const FIELD_PRODUCER: ArgKindProducerByRef<LANG, SPEC>;
    const PRODUCIBLE_FIELDS: IterativeComposer<OwnedArgComposers<LANG, SPEC, OwnerAspect<LANG, SPEC>>, FieldComposer<LANG, SPEC>, ArgKind<LANG, SPEC>, OwnerAspectSequence<LANG, SPEC, CommaPunctuatedArgKinds<LANG, SPEC>>> =
        IterativeComposer::aspect_fields(Self::FIELD_PRODUCER);
}

pub trait FieldNameSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC>;
    const FIELDS: FieldsComposerRef<LANG, SPEC> =
        |fields| constants::field_composers_iterator(fields, Self::COMPOSER);
}

pub trait FFIFieldsSpec<LANG, SPEC, L>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          L: SharedAccess {
    const FROM: FieldsOwnedSequenceComposer<LANG, SPEC, L>;
    const TO: FieldsOwnedSequenceComposer<LANG, SPEC, L>;
}

impl<LANG, SPEC, T, C> FFIFieldsSpec<LANG, SPEC, ComposerLink<T>> for C
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: AttrComposable<SPEC::Attr>
              + GenericsComposable<SPEC::Gen>
              + TypeAspect<SPEC::TYC>
              + NameKindComposable
              + FieldsContext<LANG, SPEC>
              + FieldsConversionComposable<LANG, SPEC>
              + SourceAccessible
              + 'static,
          C: ItemComposerSpec<LANG, SPEC> + FieldSpec<LANG, SPEC> {
    const FROM: FieldsOwnedSequenceComposerLink<LANG, SPEC, T> =
        SequenceComposer::new(
            C::FROM_ROOT_PRESENTER,
            Aspect::ffi,
            C::PRODUCIBLE_FIELDS);
    const TO: FieldsOwnedSequenceComposerLink<LANG, SPEC, T> =
        SequenceComposer::new(
            C::TO_ROOT_PRESENTER,
            Aspect::target,
            C::PRODUCIBLE_FIELDS);
}


pub trait ItemComposerSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const FROM_ROOT_PRESENTER: PresentableArgsSequenceComposer<LANG, SPEC>;
    const TO_ROOT_PRESENTER: PresentableArgsSequenceComposer<LANG, SPEC>;
    // const FIELD_ITER: OwnedFieldsIterator<LANG, SPEC>;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC>;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC>;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC>;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC>;
}

pub trait FieldPathConversionResolveSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const FROM: FieldPathResolver<LANG, SPEC>;
    const TO: FieldPathResolver<LANG, SPEC>;
    const DROP: FieldPathResolver<LANG, SPEC>;
}

#[allow(unused)]
pub trait FFIInterfaceMethodSpec<LANG, SPEC, SEP>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          SEP: ToTokens + Default {
    const RESOLVER: FieldPathResolver<LANG, SPEC>;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, SEP>;
    const EXPR: PresentableExprComposerRef<LANG, SPEC>;
    const ITER: FFIInterfaceMethodIterator<LANG, SPEC, SEP> =
        IterativeComposer::aspect_sequence_expr::<Self, SEP>();
}

pub trait ItemComposerExprSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC>;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC>;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC>;
}

#[allow(unused)]
pub trait ItemAspectsSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    // type CTOR: FFIInterfaceMethodSpec<LANG, SPEC, Comma>
    //     + AttrComposable<SPEC::Attr>
    //     + GenericsComposable<SPEC::Gen>
    //     + TypeAspect<SPEC::TYC>
    //     + FieldsContext<LANG, SPEC>
    //     + NameKindComposable;
    type DTOR: FFIInterfaceMethodSpec<LANG, SPEC, Semi>;
    type FROM: FFIInterfaceMethodSpec<LANG, SPEC, Comma>;
    type INTO: FFIInterfaceMethodSpec<LANG, SPEC, Comma>;
}

#[allow(unused)]
pub struct FromStub<I: DelimiterTrait + 'static>(PhantomData<I>);
pub struct ToStub<I: DelimiterTrait + 'static>(PhantomData<I>);
pub struct DropStub<I: DelimiterTrait + 'static>(PhantomData<I>);

#[allow(unused)]
pub struct FromStrategy<I: DelimiterTrait + 'static>(PhantomData<I>);
#[allow(unused)]
pub struct ToStrategy<I: DelimiterTrait + 'static>(PhantomData<I>);
#[allow(unused)]
pub struct DropStrategy<I: DelimiterTrait + 'static>(PhantomData<I>);

impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for FromStrategy<Brace>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::FromNamedFields;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::named_conversion;
}

impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for FromStrategy<Paren>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::FromUnnamedFields;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}
impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for FromStrategy<Void>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::no_fields;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}
