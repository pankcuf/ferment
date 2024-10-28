use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Field};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use crate::ast::{DelimiterTrait, Depunctuated, Void};
use crate::composable::{AttrsModel, FieldComposer};
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, CtorSpec, FFIBindingsSpec, FFIConversionsSpec, FFIObjectSpec, FieldsComposerRef, PresentableExprComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpec, PresentableArgumentComposerRef, ItemComposerExprSpec, ConversionSequenceComposer, FieldPathConversionResolveSpec, FieldPathResolver, AttrComposable, GenericsComposable, TypeAspect, FieldsContext, SourceAccessible, FieldsConversionComposable, FFIFieldsSpec, AspectPresentable, CtorSharedComposerLink, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposerLink};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::ext::{ConversionType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, ArgKind, SeqKind, ScopeContextPresentable, InterfaceKind};
use crate::presentation::Name;

pub type UnitVariantComposer<LANG, SPEC> = EnumVariantComposer<Void, LANG, SPEC>;
pub type NamedVariantComposer<LANG, SPEC> = EnumVariantComposer<Brace, LANG, SPEC>;
pub type UnnamedVariantComposer<LANG, SPEC> = EnumVariantComposer<Paren, LANG, SPEC>;

pub struct EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {

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
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: MaybeSequenceOutputComposerLink<T, LANG, SPEC> = Some(
        LinkedContextComposer::new(
            SeqKind::empty_root,
            SeqKind::empty)
    );
}

impl<I, LANG, SPEC> FieldPathConversionResolveSpec<LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
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
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpec<LANG, SPEC>
          + ItemComposerExprSpec<LANG, SPEC>
          + FieldPathConversionResolveSpec<LANG, SPEC> {
    const COMPOSER: MaybeFFIComposerLink<T, LANG, SPEC> = Some(
        constants::ffi_conversions_composer::<T, Self, LANG, SPEC>(
            SeqKind::variant_from,
            SeqKind::fields_from,
            // Self::TO_ROOT_CONVERSION_PRESENTER,
            |c| ((T::raw_target_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),

            SeqKind::variant_to,
            SeqKind::fields_to,
            // Self::ROOT_CONVERSION_PRESENTER,
            // constants::ffi_aspect_seq_context(),

            // SeqKind::unboxed_root,
            // SeqKind::fields_from,

            SeqKind::variant_drop,
            SeqKind::fields_from
        )
    );
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for NamedVariantComposer<LANG, SPEC>
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
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::variant_ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c|
            (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), false), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        ArgKind::named_struct_ctor_pair;
}
impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for UnnamedVariantComposer<LANG, SPEC>
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
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::variant_ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c|
            (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), true), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        ArgKind::unnamed_struct_ctor_pair;
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
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: CtorSpec<ComposerLink<T>, LANG, SPEC> {
    const COMPOSER: MaybeFFIBindingsComposerLink<T, LANG, SPEC> =
        Some(constants::ffi_bindings_composer::<T, Self, LANG, SPEC>());
}


impl<T, LANG, SPEC> CtorSpec<ComposerLink<T>, LANG, SPEC> for UnitVariantComposer<LANG, SPEC>
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
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC> =
        BindingPresentableContext::variant_ctor;
    const CONTEXT: CtorSharedComposerLink<T, LANG, SPEC> =
        |c|
            (((T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)), false), T::field_composers(c));
    const ARG: ConstructorArgComposerRef<LANG, SPEC> =
        ArgKind::named_struct_ctor_pair;
}


// Variant::Named
impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for NamedVariantComposer<LANG, SPEC>
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
        SeqKind::NamedFields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        ArgKind::attr_name;
    // const TO_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> = Self::TO::ROOT_PRESENTER;
    // // <Self as AspectComposerSpec<LANG, SPEC, Comma>>::ROOT_PRESENTER;
    //     // Self::ROOT_PRESENTER;
    // const FROM_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> = Self::FROM::ROOT_PRESENTER;
    //     // Self::ROOT_PRESENTER;
    // const DROP_ROOT_CONVERSION_PRESENTER: DropSequenceComposer<LANG, SPEC> =
    //     SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        constants::struct_named_fields_composer();
    const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> =
        Depunctuated::from_iter([

        ]);
    // type FROM = Self;
    // type TO = Self;
    // type DROP = Self;
}

// Variant::Unnamed
impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for UnnamedVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
        SeqKind::UnnamedFields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        ArgKind::attr_name;
    // const TO_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
    //     Self::ROOT_PRESENTER;
    // const FROM_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
    //     Self::ROOT_PRESENTER;
    // const DROP_ROOT_CONVERSION_PRESENTER: DropSequenceComposer<LANG, SPEC> =
    //     SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        |fields| constants::field_composers_iterator(fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::typed(Name::UnnamedArg(index), ty, false, attrs));
    const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> = Depunctuated::from_iter([

    ]);
    // type FROM = Self;
    // type TO = Self;
    // type DROP = Self;
}

// impl<LANG, SPEC, SEP> ItemInterfaceComposerSpec<LANG, SPEC, SEP> for UnnamedVariantComposer<LANG, SPEC>
//     where LANG: LangFermentable,
//           SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
//           SPEC::Expr: ScopeContextPresentable,
//           Name<LANG, SPEC>: ToTokens,
//           Aspect<SPEC::TYC>: ScopeContextPresentable,
//           ArgKind<LANG, SPEC>: ScopeContextPresentable,
//           SeqKind<LANG, SPEC>: ScopeContextPresentable,
//           SEP: ToTokens {
//     const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC, SEP> =
//         SeqKind::no_fields;
//     const PRESENTER: PresentableExprComposerRef<LANG, SPEC> =
//         Expression::bypass;
//     const RESOLVER: FieldPathResolver<LANG, SPEC> =
//         |c | (c.name.clone(), ConversionType::expr_from(c, Some(Expression::deref_tokens(&c.name))));
// }

// Variant::Unit
impl<LANG, SPEC> ItemComposerSpec<LANG, SPEC> for UnitVariantComposer<LANG, SPEC>
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
        SeqKind::no_fields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> =
        ArgKind::attr_name;
    // const TO_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
    //     Self::ROOT_PRESENTER;
    // const FROM_ROOT_CONVERSION_PRESENTER: ConversionSequenceComposer<LANG, SPEC> =
    //     Self::ROOT_PRESENTER;
    // const DROP_ROOT_CONVERSION_PRESENTER: DropSequenceComposer<LANG, SPEC> =
    //     SeqKind::DropCode;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> =
        |_| Punctuated::new();
    const INTERFACE_COMPOSERS: Depunctuated<InterfaceKind<Self, LANG, SPEC>> = Depunctuated::from_iter([

    ]);
    // type FROM = Self;
    // type TO = Self;
    // type DROP = Self;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for NamedVariantComposer<LANG, SPEC>
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
        Expression::terminated;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for UnnamedVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
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
        Expression::terminated;
}
impl<LANG, SPEC> ItemComposerExprSpec<LANG, SPEC> for UnitVariantComposer<LANG, SPEC>
    where LANG: LangFermentable,
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
        Expression::empty_conversion;
}
