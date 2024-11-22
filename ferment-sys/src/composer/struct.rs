use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics};
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{constants, AspectSharedComposerLink, AttrComposable, BindingComposer, CommaPunctuatedFields, ComposerLink, PresentableArgumentPairComposerRef, ConversionSeqKindComposer, CtorSpec, DropSeqKindComposer, FFIBindingsSpec, FFIConversionsSpec, FFIFieldsSpec, FFIObjectSpec, FieldPathConversionResolveSpec, FieldPathResolver, FieldsComposerRef, FieldsContext, FieldsConversionComposable, GenericsComposable, ItemComposer, ItemComposerExprSpec, ItemComposerLink, ItemComposerSpec, LinkedContextComposer, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, NameKindComposable, OwnerAspectSequence, ArgKindPair, PresentableExprComposerRef, SourceAccessible, TypeAspect, FFIInterfaceMethodSpec, AspectSeqKindComposer, ArgKindPairs, FieldNameSpec, FieldComposerProducer, FieldSpec, ArgKindProducerByRef, AspectArgSourceComposer, ItemAspectsSpec};
use crate::context::ScopeContextLink;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::Name;

pub struct StructComposer<LANG, SPEC, I>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub composer: ItemComposerLink<LANG, SPEC, I>
}

impl<LANG, SPEC, I> StructComposer<LANG, SPEC, I>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          ItemComposer<LANG, SPEC, I>: NameKindComposable,
          Self: ItemComposerSpec<LANG, SPEC>
            + CtorSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>, ArgKindPairs<LANG, SPEC>>
            + FFIFieldsSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>>
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
impl<LANG, SPEC, T, I> FFIObjectSpec<LANG, SPEC, ComposerLink<T>> for StructComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: FieldsConversionComposable<LANG, SPEC> + 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<LANG, SPEC, T> = Some(
        LinkedContextComposer::new(
            SeqKind::bypass,
            SeqKind::fields_from));
}


impl<LANG, SPEC, I> FieldPathConversionResolveSpec<LANG, SPEC> for StructComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          I: DelimiterTrait + ?Sized,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens {
    const FROM: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_FROM;
    const TO: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_TO;
    const DROP: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_DROP;
}

impl<LANG, SPEC, T, I> FFIConversionsSpec<LANG, SPEC, ComposerLink<T>> for StructComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: FieldsContext<LANG, SPEC>
              + AttrComposable<SPEC::Attr>
              + GenericsComposable<SPEC::Gen>
              + TypeAspect<SPEC::TYC>
              + NameKindComposable
              + 'static,
          I: DelimiterTrait + ?Sized,
          Self: ItemComposerSpec<LANG, SPEC>
              + ItemComposerExprSpec<LANG, SPEC>
              + FieldPathConversionResolveSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<LANG, SPEC, T> = Some(
        constants::ffi_conversions_composer::<LANG, SPEC, T, Self>(
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
impl<LANG, SPEC, T, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for StructComposer<LANG, SPEC, Brace>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          T: AttrComposable<SPEC::Attr>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<LANG, SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          Iter: IntoIterator<Item=ArgKindPair<LANG, SPEC>> + FromIterator<Iter::Item> {
    const ROOT: BindingComposer<LANG, SPEC, OwnerAspectSequence<LANG, SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        constants::args_composer_iterator_root();

}
impl<LANG, SPEC, T, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for StructComposer<LANG, SPEC, Paren>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    const ROOT: BindingComposer<LANG, SPEC, OwnerAspectSequence<LANG, SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        constants::args_composer_iterator_root();

}
impl<LANG, SPEC, T, I, Iter> FFIBindingsSpec<LANG, SPEC, ComposerLink<T>, Iter> for StructComposer<LANG, SPEC, I>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          I: DelimiterTrait
            + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Iter: IntoIterator<Item=ArgKindPair<LANG, SPEC>> + FromIterator<Iter::Item>,
          Self: CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> {
    const COMPOSER: MaybeFFIBindingsComposerLink<LANG, SPEC, T, Iter> =
        Some(constants::ffi_bindings_composer::<LANG, SPEC, T, Self, Iter>());
}
impl<LANG, SPEC> FieldNameSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Brace>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        FieldComposer::named_producer;
}

impl<LANG, SPEC> FieldNameSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Paren>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        FieldComposer::unnamed_struct_producer;
}

impl<LANG, SPEC> FieldSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Brace>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    const FIELD_PRODUCER: ArgKindProducerByRef<LANG, SPEC> =
        ArgKind::public_named;
}

impl<LANG, SPEC> FieldSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Paren>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    const FIELD_PRODUCER: ArgKindProducerByRef<LANG, SPEC> =
        ArgKind::default_field_type;
}


impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Brace>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
          Name<LANG, SPEC>: ToTokens,
          Self: FieldSpec<LANG, SPEC> + FieldNameSpec<LANG, SPEC> {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::NamedStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::NamedStruct;
    // const FIELD_ITER: OwnedFieldsIterator<LANG, SPEC> =
    //     IterativeComposer::aspect_fields(Self::PRODUCER);
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::FromNamedFields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToNamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        Self::FIELDS;
}

impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Paren>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
          Name<LANG, SPEC>: ToTokens,
          Self: FieldSpec<LANG, SPEC> + FieldNameSpec<LANG, SPEC> {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::UnnamedStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::UnnamedStruct;
    // const FIELD_ITER: OwnedFieldsIterator<LANG, SPEC> =
    //     IterativeComposer::aspect_fields(Self::PRODUCER);
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToUnnamedFields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::FromUnnamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        Self::FIELDS;
}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Brace>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::named_conversion;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::named_conversion;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for StructComposer<LANG, SPEC, Paren>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
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

impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for FromStruct<Brace>
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
impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for FromStruct<Paren>
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

impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for ToStruct<Brace>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::ToNamedFields;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::named_conversion;
}
impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for ToStruct<Paren>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::ToUnnamedFields;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}
impl<LANG, SPEC> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for ToStruct<Void>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::TO;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::no_fields;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}

impl<LANG, SPEC, I> FFIInterfaceMethodSpec<LANG, SPEC, Semi> for DropStruct<I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::DROP;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Semi> =
        SeqKind::StructDropBody;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}

impl<LANG, SPEC, I> ItemAspectsSpec<LANG, SPEC> for StructComposer<LANG, SPEC, I>
where LANG: LangFermentable,
      SPEC: Specification<LANG>,
      I: DelimiterTrait,
      FromStruct<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      ToStruct<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      DropStruct<I>: FFIInterfaceMethodSpec<LANG, SPEC, Semi>
{

    type DTOR = DropStruct<I>;
    type FROM = FromStruct<I>;
    type INTO = ToStruct<I>;
}