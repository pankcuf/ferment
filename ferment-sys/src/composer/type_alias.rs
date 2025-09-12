use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics, Lifetime};
use crate::ast::DelimiterTrait;
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::*;
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentable::{ArgKind, Aspect, Expression, ScopeContextPresentable, SeqKind};
#[cfg(feature = "accessors")]
use crate::composer::BindingPresentableContext;
use crate::presentation::Name;


pub struct TypeAliasComposer<SPEC, I>
    where SPEC: Specification + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub composer: ItemComposerLink<SPEC, I>
}

impl<SPEC, I> TypeAliasComposer<SPEC, I>
    where I: DelimiterTrait,
          SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens,
          ItemComposer<SPEC, I>: NameKindComposable {
    pub(crate) fn new(
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
impl<SPEC, T, I> FFIObjectSpec<SPEC, ComposerLink<T>> for TypeAliasComposer<SPEC, I>
    where SPEC: Specification,
          T: FieldsConversionComposable<SPEC> + 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<SPEC, T> = Some(
        LinkedContextComposer::new(
            SeqKind::bypass,
            SeqKind::fields_from));
}

#[cfg(feature = "accessors")]
impl<SPEC, T, I, Iter> FFIBindingsSpec<SPEC, ComposerLink<T>, Iter> for TypeAliasComposer<SPEC, I>
    where SPEC: Specification,
          T: AttrComposable<SPEC::Attr>
          + LifetimesComposable<SPEC::Lt>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          I: DelimiterTrait + ?Sized,
          Iter: IntoIterator<Item=ArgKindPair<SPEC>> + FromIterator<Iter::Item>,
          Self: CtorSpec<SPEC, ComposerLink<T>, Iter> {
    const COMPOSER: MaybeFFIBindingsComposerLink<SPEC, T, Iter> =
        Some(ffi_bindings_composer::<SPEC, T, Self, Iter>());
}

#[cfg(feature = "accessors")]
impl<SPEC, T, I, Iter> CtorSpec<SPEC, ComposerLink<T>, Iter> for TypeAliasComposer<SPEC, I>
    where T: AttrComposable<SPEC::Attr>
            + LifetimesComposable<SPEC::Lt>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          I: DelimiterTrait
            + ?Sized
            + 'static,
          SPEC: Specification
            + 'static,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    const ROOT: BindingComposer<SPEC, OwnerAspectSequence<SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgKindPairComposerRef<SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<SPEC, Iter> =
        args_composer_iterator_root();
}

impl<SPEC, I> FieldPathConversionResolveSpec<SPEC> for TypeAliasComposer<SPEC, I>
    where I: DelimiterTrait + ?Sized,
          SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens {
    const FROM: FieldPathResolver<SPEC> =
        FieldComposer::STRUCT_FROM;
    const TO: FieldPathResolver<SPEC> =
        FieldComposer::TYPE_TO;
    const DROP: FieldPathResolver<SPEC> =
        FieldComposer::STRUCT_DROP;
}

impl<SPEC, T, I> FFIConversionsSpec<SPEC, ComposerLink<T>> for TypeAliasComposer<SPEC, I>
    where T: FieldsContext<SPEC>
            + AttrComposable<SPEC::Attr>
            + LifetimesComposable<SPEC::Lt>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + NameKindComposable
            + 'static,
          I: DelimiterTrait + ?Sized,
          SPEC: Specification,
          Self: ItemComposerSpec<SPEC>
              + ItemComposerExprSpec<SPEC>
              + FieldPathConversionResolveSpec<SPEC> {
    const COMPOSER: MaybeFFIComposerLink<SPEC, T> = Some(
        ffi_conversions_composer::<SPEC, T, Self>(
            SeqKind::struct_from,
            SeqKind::deref_ffi,
            Aspect::target,
            SeqKind::struct_to,
            SeqKind::obj,
            SeqKind::struct_drop_post_processor,
            SeqKind::empty
        )
    );
}

impl<SPEC, I> FieldNameSpec<SPEC> for TypeAliasComposer<SPEC, I>
where SPEC: Specification<Name=Name<SPEC>>,
      I: DelimiterTrait {
    const COMPOSER: FieldComposerProducer<SPEC> =
        |Field { ty, attrs, .. }, index|
            FieldComposer::unnamed_typed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty, attrs);
}

impl<SPEC, I> FieldSpec<SPEC> for TypeAliasComposer<SPEC, I>
where SPEC: Specification,
      I: DelimiterTrait {
    const FIELD_PRODUCER: ArgKindProducerByRef<SPEC> = ArgKind::default_field_type;
}


impl<SPEC, I> ItemComposerSpec<SPEC> for TypeAliasComposer<SPEC, I>
    where SPEC: Specification<Name=Name<SPEC>>,
          I: DelimiterTrait,
          Self: FieldSpec<SPEC>,
          Name<SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::UnnamedStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::UnnamedStruct;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::TypeAliasFromConversion;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<SPEC> =
        SeqKind::ToUnnamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<SPEC> =
        Self::FIELDS;
}
impl<SPEC, I> ItemComposerExprSpec<SPEC> for TypeAliasComposer<SPEC, I>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          I: DelimiterTrait + ?Sized,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<SPEC> =
        Expression::bypass;
}

#[allow(unused)]
pub struct FromTypeAlias<I: DelimiterTrait + 'static>(PhantomData<I>);

impl<SPEC, I> FFIInterfaceMethodSpec<SPEC, Comma> for FromTypeAlias<I>
where SPEC: Specification<Expr=Expression<SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<SPEC, Comma> =
        SeqKind::TypeAliasFromConversion;
    const EXPR: PresentableExprComposerRef<SPEC> =
        SPEC::Expr::bypass;
}

impl<SPEC, I> ItemAspectsSpec<SPEC> for TypeAliasComposer<SPEC, I>
where SPEC: Specification,
      I: DelimiterTrait,
      FromTypeAlias<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      ToStruct<I>: FFIInterfaceMethodSpec<SPEC, Comma>,
      DropStruct<I>: FFIInterfaceMethodSpec<SPEC, Semi>
{

    type DTOR = DropStruct<I>;
    type FROM = FromTypeAlias<I>;
    type INTO = ToStruct<I>;
}