use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics};
use syn::token::{Brace, Paren};
use crate::ast::DelimiterTrait;
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, CtorSpec, FFIBindingsSpec, FFIConversionsSpec, FFIObjectSpec, FieldsComposerRef, PresentableExprComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpec, PresentableArgumentComposerRef, ItemComposerExprSpec, ConversionSequenceComposer, FieldPathConversionResolveSpec, FieldPathResolver, AttrComposable, GenericsComposable, TypeAspect, FieldsContext, SourceAccessible, FFIFieldsSpec, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, FieldsConversionComposable, MaybeSequenceOutputComposerLink, DropSequenceComposer, AspectSharedComposerLink, NameKindComposable, SourceComposerByRef, FieldComposerProducer, PresentableArgumentPair, AspectArgComposers, OwnerAspectSequence};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, ArgKind, SeqKind, ScopeContextPresentable};
use crate::presentation::Name;

pub struct StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          ItemComposer<I, LANG, SPEC>: NameKindComposable,
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
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: MaybeSequenceOutputComposerLink<T, LANG, SPEC> = Some(
        LinkedContextComposer::new(
            SeqKind::bypass,
            SeqKind::fields_from));
}


impl<I, LANG, SPEC> FieldPathConversionResolveSpec<LANG, SPEC> for StructComposer<I, LANG, SPEC>
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
        FieldComposer::STRUCT_TO;
    const DROP: FieldPathResolver<LANG, SPEC> =
        FieldComposer::STRUCT_DROP;
}

impl<T, I, LANG, SPEC> FFIConversionsSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<I, LANG, SPEC>
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
            // Self::TO_ROOT_CONVERSION_PRESENTER,
            Aspect::target,
            SeqKind::struct_to,
            SeqKind::empty,
            // SeqKind::empty,
            SeqKind::struct_drop_post_processor,
            SeqKind::empty
        )

    );
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::ctor;
    const ASPECT: AspectSharedComposerLink<T, LANG, SPEC> = Aspect::ffi;
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        ArgKind::named_struct_ctor_pair;
    const ITER: SourceComposerByRef<AspectArgComposers<LANG, SPEC>, FieldComposerProducer<LANG, SPEC, PresentableArgumentPair<LANG, SPEC>>, OwnerAspectSequence<LANG, SPEC, Vec<PresentableArgumentPair<LANG, SPEC>>>> =
        constants::args_composer_iterator_root();

}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
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
impl<T, I, LANG, SPEC> FFIBindingsSpec<ComposerLink<T>, LANG, SPEC> for StructComposer<I, LANG, SPEC>
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
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
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
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
    // Self: FieldsContext<LANG, SPEC> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen>
          // Self: ItemInterfaceComposerSpec<LANG, SPEC, Comma> + ItemInterfaceComposerSpec<LANG, SPEC, Semi>
{
    const ROOT_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::NamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        ArgKind::public_named;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::NamedFields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        Self::TO_ROOT_CONVERSION_PRESENTER;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSequenceComposer<LANG, SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_named_fields_composer();

    // const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> =
    //     Depunctuated::from_iter([
    //         InterfaceKind::From(
    //             FieldsSequenceMixer::new(
    //                 SeqKind::struct_from,
    //                 SeqKind::deref_ffi,
    //                 SeqKind::UnnamedFields,
    //                 |c| ((Self::target_type_aspect(c), Self::compose_generics(c)), Self::field_composers(c)),
    //                 SPEC::Expr::bypass,
    //                 |(aspect, field_composers), expr_composer|
    //                     (aspect.clone(), field_conversion_expressions_iterator((field_composers, expr_composer), |c |
    //                         (c.name.clone(), ConversionType::expr_from(c, Some(Expression::ffi_ref_with_name(&c.name))))))
    //             )
    //         ),
    //         InterfaceKind::To(FieldsSequenceMixer::new(
    //             SeqKind::struct_to,
    //             SeqKind::empty,
    //             SeqKind::UnnamedFields,
    //             |c: &ComposerLinkRef<Self>| ((Self::ffi_type_aspect(c), Self::compose_generics(c)), Self::field_composers(c)),
    //             SPEC::Expr::bypass,
    //             |(aspect, field_composers), expr_composer|
    //                 (aspect.clone(), field_conversion_expressions_iterator((field_composers, expr_composer), |c|
    //                     (c.name.clone(), ConversionType::expr_to(c, Some(Expression::obj_name(&c.name))))))
    //         )),
    //         InterfaceKind::Destroy(
    //             LinkedContextComposer::new(
    //                 SeqKind::unboxed_root,
    //                 SeqKind::empty)
    //         ),
    //         InterfaceKind::Drop(FieldsSequenceMixer::new(
    //             SeqKind::struct_drop_post_processor,
    //             SeqKind::empty,
    //             SeqKind::StructDropBody,
    //             |c: &ComposerLinkRef<Self>| ((Self::ffi_type_aspect(c), Self::compose_generics(c)), Self::field_composers(c)),
    //             SPEC::Expr::bypass,
    //             |(aspect, field_composers), expr_composer|
    //                 (aspect.clone(), field_conversion_expressions_iterator((field_composers, expr_composer), |c|
    //                     (Name::Empty, ConversionType::expr_destroy(c, Some(Expression::ffi_ref_with_name(&c.name))))))
    //         ))
    //     ]);
}

impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::UnnamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        ArgKind::default_field_type;
    const TO_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::UnnamedFields;
    const FROM_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        Self::TO_ROOT_CONVERSION_PRESENTER;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSequenceComposer<LANG, SPEC> =
        SeqKind::StructDropBody;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_unnamed_fields_composer();

    // const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> = Depunctuated::from_iter([

    // ]);
}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::named_conversion;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        Expression::bypass;
}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC> =
        SPEC::Expr::bypass;
}
