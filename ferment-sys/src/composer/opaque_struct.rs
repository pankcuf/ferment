use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics};
use syn::token::{Brace, Paren};
use crate::ast::DelimiterTrait;
use crate::composable::AttrsModel;
use crate::composer::{CommaPunctuatedFields, ComposerLink, constants, FieldsComposerRef, PresentableArgumentComposerRef, ConversionSequenceComposer, ItemComposerLink, FFIFieldsSpec, ItemComposerSpec, CtorSpec, FFIConversionsSpec, ItemComposer, FFIBindingsSpec, BindingCtorComposer, ConstructorArgComposerRef, FFIObjectSpec, ItemComposerExprSpec, PresentableExprComposerRef, AttrComposable, GenericsComposable, TypeAspect, FieldsContext, SourceAccessible, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink, FieldsConversionComposable, DropSequenceComposer, AspectSharedComposerLink, NameKindComposable, SourceComposerByRef, AspectArgComposers, FieldComposerProducer, PresentableArgumentPair, OwnerAspectSequence};
use crate::context::ScopeContextLink;
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::Name;

pub struct OpaqueStructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> OpaqueStructComposer<I, LANG, SPEC>
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
impl<T, I, LANG, SPEC> FFIConversionsSpec<ComposerLink<T>, LANG, SPEC> for OpaqueStructComposer<I, LANG, SPEC>
    where T: 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpec<LANG, SPEC>
          + ItemComposerExprSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<T, LANG, SPEC> = None;
}

impl<T, I, LANG, SPEC> FFIObjectSpec<ComposerLink<T>, LANG, SPEC> for OpaqueStructComposer<I, LANG, SPEC>
    where T: FieldsConversionComposable<LANG, SPEC> + 'static,
          I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: MaybeSequenceOutputComposerLink<T, LANG, SPEC> = None;
}

impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for OpaqueStructComposer<Paren, LANG, SPEC>
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
        Aspect::target;
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
    const ITER: SourceComposerByRef<AspectArgComposers<LANG, SPEC>, FieldComposerProducer<LANG, SPEC, PresentableArgumentPair<LANG, SPEC>>, OwnerAspectSequence<LANG, SPEC, Vec<PresentableArgumentPair<LANG, SPEC>>>> =
        constants::args_composer_iterator_root();
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for OpaqueStructComposer<Brace, LANG, SPEC>
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
        Aspect::target;
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        ArgKind::opaque_named_struct_ctor_pair;
    const ITER: SourceComposerByRef<AspectArgComposers<LANG, SPEC>, FieldComposerProducer<LANG, SPEC, PresentableArgumentPair<LANG, SPEC>>, OwnerAspectSequence<LANG, SPEC, Vec<PresentableArgumentPair<LANG, SPEC>>>> =
        constants::args_composer_iterator_root();
}

impl<T, I, LANG, SPEC> FFIBindingsSpec<ComposerLink<T>, LANG, SPEC> for OpaqueStructComposer<I, LANG, SPEC>
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
    const COMPOSER: MaybeFFIBindingsComposerLink<T, LANG, SPEC> = None;
}

impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for OpaqueStructComposer<Brace, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
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
        SeqKind::DropCode;

    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_named_fields_composer();

    // type From = ();
    // const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> =
    //     Depunctuated::new();
}
// pub struct OpaqueStructFrom {
//
// }
//
impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for OpaqueStructComposer<Paren, LANG, SPEC>
    where LANG: LangFermentable,
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
        Self::TO_ROOT_CONVERSION_PRESENTER;
    const DROP_ROOT_CONVERSION_PRESENTER: DropSequenceComposer<LANG, SPEC> =
        SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_unnamed_fields_composer();
    // const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> =
    //     Depunctuated::new();

}

impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for OpaqueStructComposer<Brace, LANG, SPEC>
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
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for OpaqueStructComposer<Paren, LANG, SPEC>
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

