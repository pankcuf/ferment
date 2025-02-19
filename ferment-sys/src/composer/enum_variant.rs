use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::Attribute;
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{constants, AspectSharedComposerLink, AttrComposable, BindingComposer, CommaPunctuatedFields, ComposerLink, PresentableArgumentPairComposerRef, ConversionSeqKindComposer, CtorSpec, DropSeqKindComposer, FFIBindingsSpec, FFIConversionsSpec, FFIFieldsSpec, FFIObjectSpec, FieldPathConversionResolveSpec, FieldPathResolver, FieldsComposerRef, FieldsContext, FieldsConversionComposable, GenericsComposable, ItemComposer, ItemComposerExprSpec, ItemComposerLink, ItemComposerSpec, LinkedContextComposer, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, NameKindComposable, OwnerAspectSequence, ArgKindPair, PresentableExprComposerRef, SourceAccessible, TypeAspect, FFIInterfaceMethodSpec, AspectSeqKindComposer, ArgKindPairs, FieldNameSpec, FieldComposerProducer, FieldSpec, ArgKindProducerByRef, AspectArgSourceComposer, ItemAspectsSpec, ToStruct, FromStruct};
use crate::context::ScopeContextLink;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::Name;

pub type UnitVariantComposer<LANG, SPEC> = EnumVariantComposer<LANG, SPEC, Void>;
pub type NamedVariantComposer<LANG, SPEC> = EnumVariantComposer<LANG, SPEC, Brace>;
pub type UnnamedVariantComposer<LANG, SPEC> = EnumVariantComposer<LANG, SPEC, Paren>;

pub struct EnumVariantComposer<LANG, SPEC, I>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub composer: ItemComposerLink<LANG, SPEC, I>
}

impl<LANG, SPEC, I> EnumVariantComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          I: DelimiterTrait + ?Sized,
          ItemComposer<LANG, SPEC, I>: NameKindComposable {

    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &Vec<Attribute>,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink,
    ) -> ComposerLink<Self>
        where Self: ItemComposerSpec<LANG, SPEC>
        + CtorSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>, ArgKindPairs<LANG, SPEC>>
        + FFIFieldsSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>>
        + FFIConversionsSpec<LANG, SPEC, ItemComposerLink<LANG, SPEC, I>> {
        Rc::new(RefCell::new(Self {
            composer: ItemComposer::new::<Self>(
                ty_context,
                None,
                vec![],
                AttrsModel::from(attrs),
                fields,
                context) }))
    }
}

impl<LANG, SPEC, T, I> FFIObjectSpec<LANG, SPEC, ComposerLink<T>> for EnumVariantComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<LANG, SPEC, T> = Some(
        LinkedContextComposer::new(
            SeqKind::empty_root,
            SeqKind::empty)
    );
}



impl<LANG, SPEC, I> FieldPathConversionResolveSpec<LANG, SPEC> for EnumVariantComposer<LANG, SPEC, I>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens {
    const FROM: FieldPathResolver<LANG, SPEC> =
        FieldComposer::VARIANT_FROM;
    const TO: FieldPathResolver<LANG, SPEC> =
        FieldComposer::VARIANT_TO;
    const DROP: FieldPathResolver<LANG, SPEC> =
        FieldComposer::VARIANT_DROP;
}
impl<LANG, SPEC, T, I> FFIConversionsSpec<LANG, SPEC, ComposerLink<T>> for EnumVariantComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: FieldsContext<LANG, SPEC>
              + AttrComposable<SPEC::Attr>
              + GenericsComposable<SPEC::Gen>
              + TypeAspect<SPEC::TYC>
              + NameKindComposable
              + FieldsConversionComposable<LANG, SPEC>
              + 'static,
          I: DelimiterTrait + ?Sized,
          Self: ItemComposerSpec<LANG, SPEC>
              + ItemComposerExprSpec<LANG, SPEC>
              + FieldPathConversionResolveSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<LANG, SPEC, T> = Some(
        constants::ffi_conversions_composer::<LANG, SPEC, T, Self>(
            SeqKind::variant_from,
            SeqKind::fields_from,
            Aspect::target,
            SeqKind::variant_to,
            SeqKind::fields_to,
            SeqKind::variant_drop,
            SeqKind::fields_from
        )
    );
}
impl<LANG, SPEC, T, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for NamedVariantComposer<LANG, SPEC>
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
    const ROOT: BindingComposer<LANG, SPEC, OwnerAspectSequence<LANG, SPEC, Iter>, > =
        BindingPresentableContext::variant_ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        constants::args_composer_iterator_root();
}
impl<LANG, SPEC, T, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for UnnamedVariantComposer<LANG, SPEC>
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
        BindingPresentableContext::variant_ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        constants::args_composer_iterator_root();
}

impl<LANG, SPEC, T, I, Iter> FFIBindingsSpec<LANG, SPEC, ComposerLink<T>, Iter> for EnumVariantComposer<LANG, SPEC, I>
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
    const COMPOSER: MaybeFFIBindingsComposerLink<LANG, SPEC, T, Iter> =
        Some(constants::ffi_bindings_composer::<LANG, SPEC, T, Self, Iter>());
}


impl<LANG, SPEC, T, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for UnitVariantComposer<LANG, SPEC>
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
        BindingPresentableContext::variant_ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        constants::args_composer_iterator_root();
}

impl<LANG, SPEC> FieldNameSpec<LANG, SPEC> for NamedVariantComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        FieldComposer::named_producer;
}
impl<LANG, SPEC> FieldNameSpec<LANG, SPEC> for UnnamedVariantComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        FieldComposer::unnamed_variant_producer;
}
impl<LANG, SPEC> FieldNameSpec<LANG, SPEC> for UnitVariantComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        FieldComposer::unit_variant_producer;
}

impl<LANG, SPEC, I> FieldSpec<LANG, SPEC> for EnumVariantComposer<LANG, SPEC, I>
where LANG: LangFermentable,
      SPEC: Specification<LANG>,
      I: DelimiterTrait {
    const FIELD_PRODUCER: ArgKindProducerByRef<LANG, SPEC> =
        ArgKind::attr_name;
}

impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for NamedVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
          Self: FieldNameSpec<LANG, SPEC> + FieldSpec<LANG, SPEC>,
          Name<LANG, SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::FromNamedFields;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToNamedFields;
    // const FIELD_ITER: OwnedFieldsIterator<LANG, SPEC> =
    //     IterativeComposer::aspect_fields(Self::PRODUCER);
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::FromNamedFields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToNamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC> =
        SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        Self::FIELDS;
}

impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for UnnamedVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
          Self: FieldNameSpec<LANG, SPEC> + FieldSpec<LANG, SPEC>,
          Name<LANG, SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::FromUnnamedFields;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToUnnamedFields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::FromUnnamedFields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToUnnamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC> =
        SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        Self::FIELDS;
}

impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for UnitVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
          Self: FieldNameSpec<LANG, SPEC> + FieldSpec<LANG, SPEC>,
          Name<LANG, SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::no_fields;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::no_fields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::no_fields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::no_fields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC> =
        SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        Self::FIELDS;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for NamedVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::terminated;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for UnnamedVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::terminated;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for UnitVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::empty_conversion;
}
#[allow(unused)]
pub struct DropEnumVariant<I: DelimiterTrait + 'static>(PhantomData<I>);

impl<LANG, SPEC, I> FFIInterfaceMethodSpec<LANG, SPEC, Semi> for DropEnumVariant<I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::DROP;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Semi> =
        SeqKind::DropCode;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::terminated;
}


impl<LANG, SPEC, I> ItemAspectsSpec<LANG, SPEC> for EnumVariantComposer<LANG, SPEC, I>
where LANG: LangFermentable,
      SPEC: Specification<LANG>,
      I: DelimiterTrait,
      FromStruct<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      ToStruct<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      DropEnumVariant<I>: FFIInterfaceMethodSpec<LANG, SPEC, Semi>
{

    type DTOR = DropEnumVariant<I>;
    type FROM = FromStruct<I>;
    type INTO = ToStruct<I>;
}