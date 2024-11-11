use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics};
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::DelimiterTrait;
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{constants, AspectSharedComposerLink, AttrComposable, BindingComposer, CommaPunctuatedFields, ComposerLink, PresentableArgumentPairComposerRef, ConversionSeqKindComposer, CtorSpec, DropSeqKindComposer, FFIBindingsSpec, FFIConversionsSpec, FFIFieldsSpec, FFIObjectSpec, FieldsComposerRef, FieldsContext, FieldsConversionComposable, GenericsComposable, ItemComposer, ItemComposerExprSpec, ItemComposerLink, ItemComposerSpec, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, NameKindComposable, OwnerAspectSequence, ArgKindPair, PresentableExprComposerRef, SourceAccessible, TypeAspect, ArgKindPairs, FieldSpec, ArgKindProducerByRef, FieldNameSpec, FieldComposerProducer, AspectArgSourceComposer, ItemAspectsSpec, FFIInterfaceMethodSpec, FromStub, FieldPathConversionResolveSpec, FieldPathResolver, AspectSeqKindComposer, ToStub, DropStub};
use crate::context::ScopeContextLink;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::Name;

pub struct OpaqueStructComposer<LANG, SPEC, I>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub composer: ItemComposerLink<LANG, SPEC, I>
}

impl<LANG, SPEC, I> OpaqueStructComposer<LANG, SPEC, I>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          ItemComposer<LANG, SPEC, I>: NameKindComposable,
          Self: ItemComposerSpec<LANG, SPEC>
          + CtorSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>, ArgKindPairs<LANG, SPEC>>
          + FFIFieldsSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>>
          + FieldSpec<LANG, SPEC>
          + FFIConversionsSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>> {
    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink,
    ) -> ComposerLink<Self> {
        Rc::new(RefCell::new(Self {
            composer: ItemComposer::new::<Self>(
                ty_context,
                Some(generics.clone()),
                AttrsModel::from(attrs),
                fields,
                context)
        }))
    }
}
impl<LANG, SPEC, I, T> FFIObjectSpec<LANG, SPEC, ComposerLink<T>> for OpaqueStructComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: FieldsConversionComposable<LANG, SPEC> + 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<LANG, SPEC, T> = None;
}
impl<LANG, SPEC, T, I> FFIConversionsSpec<LANG, SPEC, ComposerLink<T>> for OpaqueStructComposer<LANG, SPEC, I>
where T: 'static,
      I: DelimiterTrait + ?Sized,
      LANG: LangFermentable,
      SPEC: Specification<LANG>,
      Self: ItemComposerSpec<LANG, SPEC>
      + ItemComposerExprSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<LANG, SPEC, T> = None;
}


impl<LANG, SPEC, T, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for OpaqueStructComposer<LANG, SPEC, Paren>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          Iter: IntoIterator<Item=ArgKindPair<LANG, SPEC>> + FromIterator<Iter::Item> {
    const ROOT: BindingComposer<LANG, SPEC, OwnerAspectSequence<LANG, SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::target;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        constants::args_composer_iterator_root();
}
impl<LANG, SPEC, T, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for OpaqueStructComposer<LANG, SPEC, Brace>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          T: AttrComposable<SPEC::Attr>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<LANG, SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    const ROOT: BindingComposer<LANG, SPEC, OwnerAspectSequence<LANG, SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::target;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::opaque_named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        constants::args_composer_iterator_root();
}

impl<LANG, SPEC, T, I, Iter> FFIBindingsSpec<LANG, SPEC, ComposerLink<T>, Iter> for OpaqueStructComposer<LANG, SPEC, I>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Iter: IntoIterator<Item=ArgKindPair<LANG, SPEC>> + FromIterator<Iter::Item>,
        Self: CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> {
    const COMPOSER: MaybeFFIBindingsComposerLink<LANG, SPEC, T, Iter> = None;
}
impl<LANG, SPEC> FieldNameSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, Brace>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        FieldComposer::named_producer;
}
impl<LANG, SPEC> FieldNameSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, Paren>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        FieldComposer::unnamed_struct_producer;
}
impl<LANG, SPEC> FieldSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, Brace>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const FIELD_PRODUCER: ArgKindProducerByRef<LANG, SPEC> =
        ArgKind::public_named;
}

impl<LANG, SPEC> FieldSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, Paren>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    const FIELD_PRODUCER: ArgKindProducerByRef<LANG, SPEC> =
        ArgKind::default_field_type;
}

impl<LANG, SPEC, I> ItemComposerSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
          I: DelimiterTrait,
          Name<LANG, SPEC>: ToTokens,
          Self: FieldSpec<LANG, SPEC> + FieldNameSpec<LANG, SPEC> {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::StubStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::StubStruct;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::FromStub;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToStub;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC> =
        SeqKind::DropStub;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        Self::FIELDS;
}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, Brace>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, Paren>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
}

impl<LANG, SPEC, I> ItemAspectsSpec<LANG, SPEC> for OpaqueStructComposer<LANG, SPEC, I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      I: DelimiterTrait,
      SPEC::Expr: ScopeContextPresentable,
      FromStub<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      ToStub<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      DropStub<I>: FFIInterfaceMethodSpec<LANG, SPEC, Semi>
{

    type DTOR = DropStub<I>;
    type FROM = FromStub<I>;
    type INTO = ToStub<I>;
}

impl<LANG, SPEC, I> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for FromStub<I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::FromStub;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}
impl<LANG, SPEC, I> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for ToStub<I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::ToStub;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}
impl<LANG, SPEC, I> FFIInterfaceMethodSpec<LANG, SPEC, Semi> for DropStub<I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::DROP;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Semi> =
        SeqKind::DropStub;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}
