use std::cell::RefCell;
use std::rc::Rc;
use syn::{Attribute, Generics};
use crate::ast::DelimiterTrait;
use crate::composable::AttrsModel;
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, CtorSpec, FFIBindingsSpec, FFIConversionsSpec, FFIObjectSpec, FieldsComposerRef, PresentableExprComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpec, PresentableArgumentComposerRef, ItemComposerExprSpec, AspectSequenceComposer, FieldPathConversionResolveSpec, FieldPathResolver, AttrComposable, GenericsComposable, TypeAspect, FieldsContext, SourceAccessible, AspectPresentable, CtorSharedComposerLink, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, FieldsConversionComposable};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::ext::{ConversionType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, PresentableSequence, ScopeContextPresentable};
use crate::presentation::{DictionaryName, Name};

pub struct TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
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
            + SourceAccessible
            + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: CtorSpec<ComposerLink<T>, LANG, SPEC> {
    const COMPOSER: MaybeFFIBindingsComposerLink<T, LANG, SPEC> =
        Some(constants::ffi_bindings_composer::<T, Self, LANG, SPEC>());
}


impl<T, I, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + 'static,
          I: DelimiterTrait
            + ?Sized
            + 'static,
          LANG: LangFermentable
            + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>
            + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c| (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), true), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        PresentableArgument::unnamed_struct_ctor_pair;
}

impl<I, LANG, SPEC> FieldPathConversionResolveSpec<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const FROM: FieldPathResolver<LANG, SPEC> =
        |c | (c.name.clone(), ConversionType::expr_from(c, Some(Expression::ffi_ref_with_name(&c.name))));
    const TO: FieldPathResolver<LANG, SPEC> =
        |c| (Name::Empty, ConversionType::expr_to(c, Some(Expression::name(&Name::Dictionary(DictionaryName::Obj)))));
    const DROP: FieldPathResolver<LANG, SPEC> =
        |c| (Name::Empty, ConversionType::expr_destroy(c, Some(Expression::ffi_ref_with_name(&c.name))));
}



impl<T, I, LANG, SPEC> FFIConversionsSpec<ComposerLink<T>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where T: FieldsContext<LANG, SPEC>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
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
            PresentableSequence::TypeAliasFromConversion,
            |c| ((T::target_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),

            PresentableSequence::ffi_to_root,
            PresentableSequence::obj,
            // PresentableSequence::empty,
            PresentableSequence::struct_drop_post_processor,
            PresentableSequence::empty,
            PresentableSequence::StructDropBody
        )
    );
}


impl<I, LANG, SPEC> ItemComposerSpec<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::UnnamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        PresentableArgument::default_field_type;
    const ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC> =
        PresentableSequence::RoundBracesFields;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_unnamed_fields_composer::<LANG, SPEC>();
}
impl<I, LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass_conversion;
    const DESTROY: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass_conversion;
}

impl<T, I, LANG, SPEC> FFIObjectSpec<ComposerLink<T>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where T: FieldsConversionComposable<LANG, SPEC> + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpec<LANG, SPEC> {
    const COMPOSER: MaybeSequenceOutputComposerLink<T, LANG, SPEC> = Some(
        LinkedContextComposer::new(
            PresentableSequence::bypass,
            PresentableSequence::fields_from));
}
