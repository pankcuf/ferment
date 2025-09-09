use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::Attribute;
use syn::token::{Brace, Comma, Paren, Semi};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{constants, AspectSharedComposerLink, AttrComposable, BindingComposer, CommaPunctuatedFields, ComposerLink, PresentableArgKindPairComposerRef, ConversionSeqKindComposer, CtorSpec, DropSeqKindComposer, FFIBindingsSpec, FFIConversionsSpec, FFIFieldsSpec, FFIObjectSpec, FieldPathConversionResolveSpec, FieldPathResolver, FieldsComposerRef, FieldsContext, FieldsConversionComposable, GenericsComposable, ItemComposer, ItemComposerExprSpec, ItemComposerLink, ItemComposerSpec, LinkedContextComposer, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, NameKindComposable, OwnerAspectSequence, ArgKindPair, PresentableExprComposerRef, SourceAccessible, TypeAspect, FFIInterfaceMethodSpec, AspectSeqKindComposer, ArgKindPairs, FieldNameSpec, FieldComposerProducer, FieldSpec, ArgKindProducerByRef, AspectArgSourceComposer, ItemAspectsSpec, ToStruct, FromStruct, LifetimesComposable};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::Name;

pub type UnitVariantComposer<SPEC> = EnumVariantComposer<SPEC, Void>;
pub type NamedVariantComposer<SPEC> = EnumVariantComposer<SPEC, Brace>;
pub type UnnamedVariantComposer<SPEC> = EnumVariantComposer<SPEC, Paren>;

pub struct EnumVariantComposer<SPEC, I>
    where SPEC: Specification + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub composer: ItemComposerLink<SPEC, I>
}

impl<SPEC, I> EnumVariantComposer<SPEC, I>
    where SPEC: Specification,
          I: DelimiterTrait + ?Sized,
          ItemComposer<SPEC, I>: NameKindComposable {

    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &[Attribute],
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink,
    ) -> ComposerLink<Self>
        where Self: ItemComposerSpec<SPEC>
        + CtorSpec<SPEC, ItemComposerLink<SPEC, I>, ArgKindPairs<SPEC>>
        + FFIFieldsSpec<SPEC, ItemComposerLink<SPEC, I>>
        + FFIConversionsSpec<SPEC, ItemComposerLink<SPEC, I>> {
        Rc::new(RefCell::new(Self {
            composer: ItemComposer::new::<Self>(
                ty_context,
                AttrsModel::from(attrs),
                vec![],
                None,
                fields,
                context) }))
    }
}

impl<SPEC, T, I> FFIObjectSpec<SPEC, ComposerLink<T>> for EnumVariantComposer<SPEC, I>
    where SPEC: Specification,
          T: 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<SPEC, T> = Some(
        LinkedContextComposer::new(
            SeqKind::empty_root,
            SeqKind::empty)
    );
}



impl<SPEC, I> FieldPathConversionResolveSpec<SPEC> for EnumVariantComposer<SPEC, I>
    where I: DelimiterTrait + ?Sized,
          SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens {
    const FROM: FieldPathResolver<SPEC> =
        FieldComposer::VARIANT_FROM;
    const TO: FieldPathResolver<SPEC> =
        FieldComposer::VARIANT_TO;
    const DROP: FieldPathResolver<SPEC> =
        FieldComposer::VARIANT_DROP;
}
impl<SPEC, T, I> FFIConversionsSpec<SPEC, ComposerLink<T>> for EnumVariantComposer<SPEC, I>
    where SPEC: Specification,
          T: FieldsContext<SPEC>
              + AttrComposable<SPEC::Attr>
              + LifetimesComposable<SPEC::Lt>
              + GenericsComposable<SPEC::Gen>
              + TypeAspect<SPEC::TYC>
              + NameKindComposable
              + FieldsConversionComposable<SPEC>
              + 'static,
          I: DelimiterTrait + ?Sized,
          Self: ItemComposerSpec<SPEC>
              + ItemComposerExprSpec<SPEC>
              + FieldPathConversionResolveSpec<SPEC> {
    const COMPOSER: MaybeFFIComposerLink<SPEC, T> = Some(
        constants::ffi_conversions_composer::<SPEC, T, Self>(
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
impl<SPEC, T, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for NamedVariantComposer<SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + LifetimesComposable<SPEC::Lt>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          SPEC: Specification + 'static,
          Iter: IntoIterator<Item=ArgKindPair<SPEC>> + FromIterator<Iter::Item> {
    const ROOT: BindingComposer<SPEC, OwnerAspectSequence<SPEC, Iter>, > =
        BindingPresentableContext::variant_ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        constants::args_composer_iterator_root();
}
impl<SPEC, T, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for UnnamedVariantComposer<SPEC>
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
        BindingPresentableContext::variant_ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        constants::args_composer_iterator_root();
}

impl<SPEC, T, I, Iter> FFIBindingsSpec<SPEC, ComposerLink<T>, Iter> for EnumVariantComposer<SPEC, I>
    where T: AttrComposable<SPEC::Attr>
    + LifetimesComposable<SPEC::Lt>
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
    const COMPOSER: MaybeFFIBindingsComposerLink<SPEC, T, Iter> =
        Some(constants::ffi_bindings_composer::<SPEC, T, Self, Iter>());
}


impl<SPEC, T, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for UnitVariantComposer<SPEC>
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
        BindingPresentableContext::variant_ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::named_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        constants::args_composer_iterator_root();
}

impl<SPEC> FieldNameSpec<SPEC> for NamedVariantComposer<SPEC>
where SPEC: Specification<Name=Name<SPEC>> {
    const COMPOSER: FieldComposerProducer<SPEC> =
        FieldComposer::named_producer;
}
impl<SPEC> FieldNameSpec<SPEC> for UnnamedVariantComposer<SPEC>
where SPEC: Specification<Name=Name<SPEC>> {
    const COMPOSER: FieldComposerProducer<SPEC> =
        FieldComposer::unnamed_variant_producer;
}
impl<SPEC> FieldNameSpec<SPEC> for UnitVariantComposer<SPEC>
where SPEC: Specification<Name=Name<SPEC>> {
    const COMPOSER: FieldComposerProducer<SPEC> =
        FieldComposer::unit_variant_producer;
}

impl<SPEC, I> FieldSpec<SPEC> for EnumVariantComposer<SPEC, I>
where SPEC: Specification,
      I: DelimiterTrait {
    const FIELD_PRODUCER: ArgKindProducerByRef<SPEC> =
        ArgKind::attr_name;
}

impl<SPEC> ItemComposerSpec<SPEC> for NamedVariantComposer<SPEC>
    where SPEC: Specification<Name=Name<SPEC>>,
          Self: FieldNameSpec<SPEC> + FieldSpec<SPEC>,
          Name<SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::FromNamedFields;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToNamedFields;
    // const FIELD_ITER: OwnedFieldsIterator<SPEC> =
    //     IterativeComposer::aspect_fields(Self::PRODUCER);
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::FromNamedFields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToNamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC> =
        SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC> =
        Self::FIELDS;
}

impl<SPEC> ItemComposerSpec<SPEC> for UnnamedVariantComposer<SPEC>
    where SPEC: Specification<Name=Name<SPEC>>,
          Self: FieldNameSpec<SPEC> + FieldSpec<SPEC>,
          Name<SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::FromUnnamedFields;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToUnnamedFields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::FromUnnamedFields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToUnnamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC> =
        SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC> =
        Self::FIELDS;
}

impl<SPEC> ItemComposerSpec<SPEC> for UnitVariantComposer<SPEC>
    where SPEC: Specification<Name=Name<SPEC>>,
          Self: FieldNameSpec<SPEC> + FieldSpec<SPEC>,
          Name<SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::no_fields;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::no_fields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::no_fields;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::no_fields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC> =
        SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC> =
        Self::FIELDS;
}
impl<SPEC> ItemComposerExprSpec<SPEC> for NamedVariantComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::named_conversion;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::named_conversion;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::terminated;
}
impl<SPEC> ItemComposerExprSpec<SPEC> for UnnamedVariantComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::terminated;
}
impl<SPEC> ItemComposerExprSpec<SPEC> for UnitVariantComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::empty_conversion;
}
#[allow(unused)]
pub struct DropEnumVariant<I: DelimiterTrait + 'static>(PhantomData<I>);

impl<SPEC, I> FFIInterfaceMethodSpec<SPEC, Semi> for DropEnumVariant<I>
where SPEC: Specification<Expr=Expression<SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::DROP;
    const SEQ: AspectSeqKindComposer<SPEC, Semi> =
        SeqKind::DropCode;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::terminated;
}


impl<SPEC, I> ItemAspectsSpec<SPEC> for EnumVariantComposer<SPEC, I>
where SPEC: Specification,
      I: DelimiterTrait,
      FromStruct<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      ToStruct<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      DropEnumVariant<I>: FFIInterfaceMethodSpec<SPEC, Semi>
{

    type DTOR = DropEnumVariant<I>;
    type FROM = FromStruct<I>;
    type INTO = ToStruct<I>;
}