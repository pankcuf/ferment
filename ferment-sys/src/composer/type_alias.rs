use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics, Lifetime};
use crate::ast::DelimiterTrait;
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::*;
use crate::context::ScopeContextLink;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::Name;


pub struct TypeAliasComposer<LANG, SPEC, I>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub composer: ItemComposerLink<LANG, SPEC, I>
}

impl<LANG, SPEC, I> TypeAliasComposer<LANG, SPEC, I>
    where I: DelimiterTrait,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          ItemComposer<LANG, SPEC, I>: NameKindComposable {
    pub(crate) fn new(
        ty_context: SPEC::TYC,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        lifetimes: &Vec<Lifetime>,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink,
    ) -> ComposerLink<Self> {
        Rc::new(RefCell::new(Self {
            composer: ItemComposer::new::<Self>(
                ty_context,
                Some(generics.clone()),
                lifetimes.clone(),
                AttrsModel::from(attrs),
                fields,
                context)
        }))
    }
}
impl<LANG, SPEC, T, I> FFIObjectSpec<LANG, SPEC, ComposerLink<T>> for TypeAliasComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: FieldsConversionComposable<LANG, SPEC> + 'static,
          I: DelimiterTrait + ?Sized {
    const COMPOSER: MaybeSequenceOutputComposerLink<LANG, SPEC, T> = Some(
        LinkedContextComposer::new(
            SeqKind::bypass,
            SeqKind::fields_from));
}

impl<LANG, SPEC, T, I, Iter> FFIBindingsSpec<LANG, SPEC, ComposerLink<T>, Iter> for TypeAliasComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: AttrComposable<SPEC::Attr>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<LANG, SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          I: DelimiterTrait + ?Sized,
          Iter: IntoIterator<Item=ArgKindPair<LANG, SPEC>> + FromIterator<Iter::Item>,
          Self: CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> {
    const COMPOSER: MaybeFFIBindingsComposerLink<LANG, SPEC, T, Iter> =
        Some(ffi_bindings_composer::<LANG, SPEC, T, Self, Iter>());
}


impl<LANG, SPEC, T, I, Iter> CtorSpec<LANG, SPEC, ComposerLink<T>, Iter> for TypeAliasComposer<LANG, SPEC, I>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          I: DelimiterTrait
            + ?Sized
            + 'static,
          LANG: LangFermentable
            + 'static,
          SPEC: Specification<LANG>
            + 'static,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    const ROOT: BindingComposer<LANG, SPEC, OwnerAspectSequence<LANG, SPEC, Iter>> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<LANG, SPEC, T> =
        Aspect::ffi;
    const ARG: PresentableArgumentPairComposerRef<LANG, SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: AspectArgSourceComposer<LANG, SPEC, Iter> =
        args_composer_iterator_root();
}

impl<LANG, SPEC, I> FieldPathConversionResolveSpec<LANG, SPEC> for TypeAliasComposer<LANG, SPEC, I>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens {
    const FROM: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_FROM;
    const TO: FieldPathResolver<LANG, SPEC> =
        FieldComposer::TYPE_TO;
    const DROP: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_DROP;
}

impl<LANG, SPEC, T, I> FFIConversionsSpec<LANG, SPEC, ComposerLink<T>> for TypeAliasComposer<LANG, SPEC, I>
    where T: FieldsContext<LANG, SPEC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + NameKindComposable
            + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Self: ItemComposerSpec<LANG, SPEC>
              + ItemComposerExprSpec<LANG, SPEC>
              + FieldPathConversionResolveSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<LANG, SPEC, T> = Some(
        ffi_conversions_composer::<LANG, SPEC, T, Self>(
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

impl<LANG, SPEC, I> FieldNameSpec<LANG, SPEC> for TypeAliasComposer<LANG, SPEC, I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
      I: DelimiterTrait {
    const COMPOSER: FieldComposerProducer<LANG, SPEC> =
        |Field { ty, attrs, .. }, index|
            FieldComposer::typed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty, false, attrs);
}

impl<LANG, SPEC, I> FieldSpec<LANG, SPEC> for TypeAliasComposer<LANG, SPEC, I>
where LANG: LangFermentable,
      SPEC: Specification<LANG>,
      I: DelimiterTrait {
    const FIELD_PRODUCER: ArgKindProducerByRef<LANG, SPEC> = ArgKind::default_field_type;
}


impl<LANG, SPEC, I> ItemComposerSpec<LANG, SPEC> for TypeAliasComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>>,
          I: DelimiterTrait,
          Self: FieldSpec<LANG, SPEC>,
          Name<LANG, SPEC>: ToTokens {
    const FROM_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::UnnamedStruct;
    const TO_ROOT_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::UnnamedStruct;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::TypeAliasFromConversion;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSeqKindComposer<LANG, SPEC> =
        SeqKind::ToUnnamedFields;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSeqKindComposer<LANG, SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        Self::FIELDS;
}
impl<LANG, SPEC, I> ItemComposerExprSpec<LANG, SPEC> for TypeAliasComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          I: DelimiterTrait + ?Sized,
          SPEC::Expr: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
}

#[allow(unused)]
pub struct FromTypeAlias<I: DelimiterTrait + 'static>(PhantomData<I>);

impl<LANG, SPEC, I> FFIInterfaceMethodSpec<LANG, SPEC, Comma> for FromTypeAlias<I>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      I: DelimiterTrait,
      Self: FieldPathConversionResolveSpec<LANG, SPEC>,
      SPEC::Expr: ScopeContextPresentable {
    const RESOLVER: FieldPathResolver<LANG, SPEC> =
        Self::FROM;
    const SEQ: AspectSeqKindComposer<LANG, SPEC, Comma> =
        SeqKind::TypeAliasFromConversion;
    const EXPR: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}

impl<LANG, SPEC, I> ItemAspectsSpec<LANG, SPEC> for TypeAliasComposer<LANG, SPEC, I>
where LANG: LangFermentable,
      SPEC: Specification<LANG>,
      I: DelimiterTrait,
      FromTypeAlias<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      ToStruct<I>: FFIInterfaceMethodSpec<LANG, SPEC, Comma>,
      DropStruct<I>: FFIInterfaceMethodSpec<LANG, SPEC, Semi>
{

    type DTOR = DropStruct<I>;
    type FROM = FromTypeAlias<I>;
    type INTO = ToStruct<I>;
}