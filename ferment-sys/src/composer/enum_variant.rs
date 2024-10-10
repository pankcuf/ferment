use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Field};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, CtorSpec, FFIBindingsSpec, FFIConversionsSpec, FFIObjectSpec, FieldsComposerRef, PresentableExprComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpec, PresentableArgumentComposerRef, ItemComposerExprSpec, AspectSequenceComposer, FieldPathConversionResolveSpec, FieldPathResolver, AttrComposable, GenericsComposable, TypeAspect, FieldsContext, SourceAccessible, FieldsConversionComposable, FFIFieldsSpec, AspectPresentable, CtorSharedComposerLink, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::ext::{ConversionType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, PresentableSequence, ScopeContextPresentable};
use crate::presentation::Name;

pub struct EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &Vec<Attribute>,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink,
    ) -> ComposerLink<Self>
        where Self: ItemComposerSpec<LANG, SPEC>
        + CtorSpec<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
        + FFIFieldsSpec<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
        + FFIConversionsSpec<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> {
        Rc::new(RefCell::new(Self {
            composer: ItemComposer::new::<Self>(
                ty_context,
                None,
                AttrsModel::from(attrs),
                fields,
                context) }))
    }
}

impl<T, I, LANG, SPEC> FFIObjectSpec<ComposerLink<T>, LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where T: 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: MaybeSequenceOutputComposerLink<T, LANG, SPEC> = Some(
        LinkedContextComposer::new(
            PresentableSequence::empty_root,
            PresentableSequence::empty)
    );
}

impl<I, LANG, SPEC> FieldPathConversionResolveSpec<LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const FROM: FieldPathResolver<LANG, SPEC> =
        |c | (c.name.clone(), ConversionType::expr_from(c, Some(Expression::deref_tokens(&c.name))));
    const TO: FieldPathResolver<LANG, SPEC> =
        |c | (c.name.clone(), ConversionType::expr_to(c, Some(Expression::name(&c.name))));
    const DROP: FieldPathResolver<LANG, SPEC> =
        |c| (c.name.clone(), ConversionType::expr_destroy(c, Some(Expression::deref_tokens(&c.name))));
}
impl<T, I, LANG, SPEC> FFIConversionsSpec<ComposerLink<T>, LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where T: FieldsContext<LANG, SPEC>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsConversionComposable<LANG, SPEC>
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
            PresentableSequence::lambda,
            PresentableSequence::fields_from,
            // Self::TO_ROOT_CONVERSION_PRESENTER,
            |c| ((T::raw_target_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),

            PresentableSequence::lambda,
            PresentableSequence::fields_to,
            // Self::ROOT_CONVERSION_PRESENTER,
            // constants::ffi_aspect_seq_context(),

            // PresentableSequence::unboxed_root,
            // PresentableSequence::fields_from,

            PresentableSequence::lambda,
            PresentableSequence::fields_from,
            PresentableSequence::DropCode
        )
    );
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for EnumVariantComposer<Brace, LANG, SPEC>
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
        BindingPresentableContext::variant_ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c| (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), false), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        PresentableArgument::named_struct_ctor_pair;
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for EnumVariantComposer<Paren, LANG, SPEC>
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
        BindingPresentableContext::variant_ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c|
            (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), true), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        PresentableArgument::unnamed_struct_ctor_pair;
}

impl<T, I, LANG, SPEC> FFIBindingsSpec<ComposerLink<T>, LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
    + GenericsComposable<SPEC::Gen>
    + TypeAspect<SPEC::TYC>
    + FieldsContext<LANG, SPEC>
    + SourceAccessible
    + 'static,
          I: DelimiterTrait + ?Sized,
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


impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for EnumVariantComposer<Void, LANG, SPEC>
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
        BindingPresentableContext::variant_ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c| (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), false), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        PresentableArgument::named_struct_ctor_pair;
}


// Variant::Named
impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for EnumVariantComposer<Brace, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::CurlyBracesFields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        PresentableArgument::attr_name;
    const TO_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::CurlyBracesFields;
    const FROM_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> = Self::TO_ROOT_CONVERSION_PRESENTER;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_named_fields_composer();
}

// Variant::Unnamed
impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for EnumVariantComposer<Paren, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::RoundBracesFields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        PresentableArgument::attr_name;
    const TO_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::RoundBracesFields;
    const FROM_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> = Self::TO_ROOT_CONVERSION_PRESENTER;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        |fields| constants::field_composers_iterator(fields,
        |index, Field { ty, attrs, .. }| FieldComposer::typed(Name::UnnamedArg(index), ty, false, attrs));
}
// Variant::Unit
impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for EnumVariantComposer<Void, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::no_fields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        PresentableArgument::attr_name;
    const TO_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::no_fields;
    const FROM_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> = Self::TO_ROOT_CONVERSION_PRESENTER;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        |_| Punctuated::new();
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for EnumVariantComposer<Brace, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const DESTROY: PresentableExprComposerRef<LANG, SPEC> =
        Expression::terminated_conversion;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for EnumVariantComposer<Paren, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass_conversion;
    const DESTROY: PresentableExprComposerRef<LANG, SPEC> =
        Expression::terminated_conversion;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for EnumVariantComposer<Void, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass_conversion;
    const DESTROY: PresentableExprComposerRef<LANG, SPEC> =
        Expression::empty_conversion;
}
