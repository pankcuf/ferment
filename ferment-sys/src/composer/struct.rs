use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics};
use syn::token::{Brace, Paren};
use crate::ast::DelimiterTrait;
use crate::composable::AttrsModel;
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, CtorSpec, FFIBindingsSpec, FFIConversionsSpec, FFIObjectSpec, FieldsComposerRef, PresentableExprComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpec, PresentableArgumentComposerRef, ItemComposerExprSpec, AspectSequenceComposer, FieldPathConversionResolveSpec, FieldPathResolver, AttrComposable, GenericsComposable, TypeAspect, FieldsContext, SourceAccessible, FFIFieldsSpec, AspectPresentable, CtorSharedComposerLink, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, FieldsConversionComposable, MaybeSequenceOutputComposerLink};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::ext::{ConversionType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, PresentableSequence, ScopeContextPresentable};
use crate::presentation::Name;

pub struct StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpec<LANG, SPEC>
            + CtorSpec<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
            + FFIFieldsSpec<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
            + FFIConversionsSpec<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> {
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
impl<T, I, LANG, SPEC> FFIObjectSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<I, LANG, SPEC>
    where T: FieldsConversionComposable<LANG, SPEC> + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: MaybeSequenceOutputComposerLink<T, LANG, SPEC> = Some(
        LinkedContextComposer::new(
            PresentableSequence::bypass,
            PresentableSequence::fields_from));
}

impl<I, LANG, SPEC> FieldPathConversionResolveSpec<LANG, SPEC> for StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const FROM: FieldPathResolver<LANG, SPEC> =
        |c |
            (c.name.clone(), ConversionType::expr_from(c, Some(Expression::ffi_ref_with_name(&c.name))));
    const TO: FieldPathResolver<LANG, SPEC> =
        |c|
            (c.name.clone(), ConversionType::expr_to(c, Some(Expression::obj_name(&c.name))));
    const DROP: FieldPathResolver<LANG, SPEC> =
        |c|
            (Name::Empty, ConversionType::expr_destroy(c, Some(Expression::ffi_ref_with_name(&c.name))));
}

impl<T, I, LANG, SPEC> FFIConversionsSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<I, LANG, SPEC>
    where T: FieldsContext<LANG, SPEC>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpec<LANG, SPEC>
          + ItemComposerExprSpec<LANG, SPEC>
          + FieldPathConversionResolveSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<T, LANG, SPEC> = Some(
        constants::ffi_conversions_composer::<T, Self, LANG, SPEC>(
            PresentableSequence::ffi_from_root,
            PresentableSequence::deref_ffi,
            // Self::TO_ROOT_CONVERSION_PRESENTER,
            |c| ((T::target_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
            PresentableSequence::ffi_to_root,
            PresentableSequence::empty,
            // PresentableSequence::empty,
            PresentableSequence::struct_drop_post_processor,
            PresentableSequence::empty,
            PresentableSequence::StructDropBody
        )

    );
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + 'static,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c|
            (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), false), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        PresentableArgument::named_struct_ctor_pair;
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + 'static,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c|
            (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), true), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        PresentableArgument::unnamed_struct_ctor_pair;
}
impl<T, I, LANG, SPEC> FFIBindingsSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<I, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + 'static,
          I: DelimiterTrait
            + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: CtorSpec<ComposerLink<T>, LANG, SPEC> {
    const COMPOSER: MaybeFFIBindingsComposerLink<T, LANG, SPEC> =
        Some(constants::ffi_bindings_composer::<T, Self, LANG, SPEC>());
}


impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::NamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        PresentableArgument::public_named;
    const TO_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::CurlyBracesFields;
    const FROM_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> = Self::TO_ROOT_CONVERSION_PRESENTER;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_named_fields_composer();
}

impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::UnnamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        PresentableArgument::default_field_type;
    const TO_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::RoundBracesFields;
    const FROM_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> = Self::TO_ROOT_CONVERSION_PRESENTER;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_unnamed_fields_composer();
}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const DESTROY: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass_conversion;
}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass_conversion;
    const DESTROY: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass_conversion;
}
