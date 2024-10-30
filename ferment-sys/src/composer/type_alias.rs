use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics};
use crate::ast::DelimiterTrait;
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, CtorSpec, FFIBindingsSpec, FFIConversionsSpec, FFIObjectSpec, FieldsComposerRef, PresentableExprComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpec, PresentableArgumentComposerRef, ItemComposerExprSpec, ConversionSequenceComposer, FieldPathConversionResolveSpec, FieldPathResolver, AttrComposable, GenericsComposable, TypeAspect, FieldsContext, SourceAccessible, AspectSharedComposerLink, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, FieldsConversionComposable, DropSequenceComposer, NameKindComposable, AspectArgComposers, PresentableArgumentPair, FieldComposerProducer, SourceComposerByRef, OwnerAspectSequence};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, ArgKind, SeqKind, ScopeContextPresentable};
use crate::presentation::Name;


pub struct TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          ItemComposer<I, LANG, SPEC>: NameKindComposable {
    pub(crate) fn new(
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
impl<T, I, LANG, SPEC> FFIBindingsSpec<ComposerLink<T>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: CtorSpec<ComposerLink<T>, LANG, SPEC> {
    const COMPOSER: MaybeFFIBindingsComposerLink<T, LANG, SPEC> =
        Some(constants::ffi_bindings_composer::<T, Self, LANG, SPEC>());
}


impl<T, I, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
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
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>
            + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<T, LANG, SPEC> =
        Aspect::ffi;
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: SourceComposerByRef<AspectArgComposers<LANG, SPEC>, FieldComposerProducer<LANG, SPEC, PresentableArgumentPair<LANG, SPEC>>, OwnerAspectSequence<LANG, SPEC, Vec<PresentableArgumentPair<LANG, SPEC>>>> =
        constants::args_composer_iterator_root();
}

impl<I, LANG, SPEC> FieldPathConversionResolveSpec<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const FROM: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_FROM;
    const TO: FieldPathResolver<LANG, SPEC> =
        FieldComposer::TYPE_TO;
    const DROP: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_DROP;
}

impl<T, I, LANG, SPEC> FFIConversionsSpec<ComposerLink<T>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where T: FieldsContext<LANG, SPEC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + NameKindComposable
            + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpec<LANG, SPEC>
              + ItemComposerExprSpec<LANG, SPEC>
              + FieldPathConversionResolveSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<T, LANG, SPEC> = Some(
        constants::ffi_conversions_composer::<T, Self, LANG, SPEC>(
            SeqKind::struct_from,
            SeqKind::deref_ffi,
            // SeqKind::TypeAliasFromConversion,
            Aspect::target,

            SeqKind::struct_to,
            SeqKind::obj,
            // SeqKind::empty,
            SeqKind::struct_drop_post_processor,
            SeqKind::empty
        )
    );
}


impl<I, LANG, SPEC> ItemComposerSpec<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          // Self: ItemInterfaceComposerSpec<LANG, SPEC, Comma> + ItemInterfaceComposerSpec<LANG, SPEC, Semi>
{
    const ROOT_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::UnnamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        ArgKind::default_field_type;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::UnnamedFields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::TypeAliasFromConversion;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSequenceComposer<LANG, SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_unnamed_fields_composer::<LANG, SPEC>();
    // const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> = Depunctuated::from_iter([

    // ]);
}
impl<I, LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
}

impl<T, I, LANG, SPEC> FFIObjectSpec<ComposerLink<T>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where T: FieldsConversionComposable<LANG, SPEC> + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpec<LANG, SPEC> {
    const COMPOSER: MaybeSequenceOutputComposerLink<T, LANG, SPEC> = Some(
        LinkedContextComposer::new(SeqKind::bypass, SeqKind::fields_from));
}
