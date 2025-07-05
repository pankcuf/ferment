use std::marker::PhantomData;
use quote::ToTokens;
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::FieldComposer;
use crate::composer::{AspectSeqKindComposer, AttrComposable, ComposerLink, PresentableArgumentPairComposerRef, ConversionSeqKindComposer, DropSeqKindComposer, FFIBindingsComposer, FFIComposer, ArgProducerByRef, FieldPathResolver, FieldsComposerRef, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, FieldsOwnedSequenceComposerLink, GenericsComposable, MaybeSequenceOutputComposer, NameKindComposable, OwnerAspectSequenceComposer, OwnerAspectSequenceSpecComposer, ArgKindPair, PresentableExprComposerRef, SequenceComposer, SharedAspectArgComposer, SourceAccessible, TypeAspect, PresentableArgsSequenceComposer, IterativeComposer, FFIInterfaceMethodIterator, ArgKindProducerByRef, FieldComposerProducer, OwnerAspect, OwnedArgComposers, CommaPunctuatedArgKinds, OwnerAspectSequence, constants, AspectArgSourceComposer, LifetimesComposable};
use crate::lang::Specification;
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::shared::SharedAccess;

pub trait FFIObjectSpec<SPEC, L>
    where L: SharedAccess,
          SPEC: Specification {
    const COMPOSER: MaybeSequenceOutputComposer<SPEC, L>;
}

pub trait FFIConversionsSpec<SPEC, L>
    where SPEC: Specification,
          L: SharedAccess {
    const COMPOSER: Option<FFIComposer<SPEC, L>>;
}
pub trait FFIBindingsSpec<SPEC, L, Iter>
    where SPEC: Specification,
          L: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> +  {
    const COMPOSER: Option<FFIBindingsComposer<SPEC, L, Iter>>;
}

#[allow(unused)]
pub trait OwnerAspectSequenceSpec<SPEC, L, Iter, IterMap, Out>
    where SPEC: Specification,
          L: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=IterMap> {
    const ROOT: OwnerAspectSequenceComposer<SPEC, Iter, Out>;
    const ASPECT: SharedAspectArgComposer<SPEC, L>;
    const ITER: AspectArgSourceComposer<SPEC, Iter>;
    const ARG: ArgProducerByRef<SPEC, Iter::Item>;
    const COMPOSER: OwnerAspectSequenceSpecComposer<SPEC, L, Iter, Out> =
        SequenceComposer::with_iterator_setup(Self::ROOT, Self::ASPECT, Self::ITER, Self::ARG);
}
pub trait CtorSpec<SPEC, L, Iter>
    where SPEC: Specification,
          L: SharedAccess,
          Iter: IntoIterator<Item=ArgKindPair<SPEC>> {
    const ROOT: OwnerAspectSequenceComposer<SPEC, Iter, BindingPresentableContext<SPEC>>;
    const ASPECT: SharedAspectArgComposer<SPEC, L>;
    const ARG: PresentableArgumentPairComposerRef<SPEC>;
    const ITER: AspectArgSourceComposer<SPEC, Iter>;
}

pub trait FieldSpec<SPEC>
    where SPEC: Specification {
    const FIELD_PRODUCER: ArgKindProducerByRef<SPEC>;
    const PRODUCIBLE_FIELDS: IterativeComposer<OwnedArgComposers<SPEC, OwnerAspect<SPEC>>, FieldComposer<SPEC>, ArgKind<SPEC>, OwnerAspectSequence<SPEC, CommaPunctuatedArgKinds<SPEC>>> =
        IterativeComposer::aspect_fields(Self::FIELD_PRODUCER);
}

pub trait FieldNameSpec<SPEC>
    where SPEC: Specification {
    const COMPOSER: FieldComposerProducer<SPEC>;
    const FIELDS: FieldsComposerRef<SPEC> =
        |fields| constants::field_composers_iterator(fields, Self::COMPOSER);
}

pub trait FFIFieldsSpec<SPEC, L>
    where SPEC: Specification,
          L: SharedAccess {
    const FROM: FieldsOwnedSequenceComposer<SPEC, L>;
    const TO: FieldsOwnedSequenceComposer<SPEC, L>;
}

impl<SPEC, T, C> FFIFieldsSpec<SPEC, ComposerLink<T>> for C
    where SPEC: Specification,
          T: AttrComposable<SPEC::Attr>
              + GenericsComposable<SPEC::Gen>
              + LifetimesComposable<SPEC::Lt>
              + TypeAspect<SPEC::TYC>
              + NameKindComposable
              + FieldsContext<SPEC>
              + FieldsConversionComposable<SPEC>
              + SourceAccessible
              + 'static,
          C: ItemComposerSpec<SPEC> + FieldSpec<SPEC> {
    const FROM: FieldsOwnedSequenceComposerLink<SPEC, T> =
        SequenceComposer::new(
            C::FROM_ROOT_PRESENTER,
            Aspect::ffi,
            C::PRODUCIBLE_FIELDS);
    const TO: FieldsOwnedSequenceComposerLink<SPEC, T> =
        SequenceComposer::new(
            C::TO_ROOT_PRESENTER,
            Aspect::target,
            C::PRODUCIBLE_FIELDS);
}


pub trait ItemComposerSpec<SPEC>
    where SPEC: Specification {
    const FROM_ROOT_PRESENTER: PresentableArgsSequenceComposer<SPEC>;
    const TO_ROOT_PRESENTER: PresentableArgsSequenceComposer<SPEC>;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC>;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC>;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC>;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC>;
}

pub trait FieldPathConversionResolveSpec<SPEC>
    where SPEC: Specification {
    const FROM: FieldPathResolver<SPEC>;
    const TO: FieldPathResolver<SPEC>;
    const DROP: FieldPathResolver<SPEC>;
}

#[allow(unused)]
pub trait FFIInterfaceMethodSpec<SPEC, SEP>
    where SPEC: Specification,
          SEP: ToTokens + Default {
    const RESOLVER: FieldPathResolver<SPEC>;
    const SEQ: AspectSeqKindComposer<SPEC, SEP>;
    const EXPR: PresentableExprComposerRef<SPEC>;
    const ITER: FFIInterfaceMethodIterator<SPEC, SEP> =
        IterativeComposer::aspect_sequence_expr::<Self, SEP>();
}

pub trait ItemComposerExprSpec<SPEC>
    where SPEC: Specification {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC>;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC>;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC>;
}

#[allow(unused)]
pub trait ItemAspectsSpec<SPEC>
    where SPEC: Specification {
    // type CTOR: FFIInterfaceMethodSpec<SPEC, Comma>
    //     + AttrComposable<SPEC::Attr>
    //     + GenericsComposable<SPEC::Gen>
    //     + TypeAspect<SPEC::TYC>
    //     + FieldsContext<SPEC>
    //     + NameKindComposable;
    type DTOR: FFIInterfaceMethodSpec<SPEC, Semi>;
    type FROM: FFIInterfaceMethodSpec<SPEC, Comma>;
    type INTO: FFIInterfaceMethodSpec<SPEC, Comma>;
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

impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for FromStrategy<Brace>
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

impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for FromStrategy<Paren>
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
impl<SPEC> FFIInterfaceMethodSpec<SPEC, Comma> for FromStrategy<Void>
where SPEC: Specification<Expr=Expression<SPEC>>,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::no_fields;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}
