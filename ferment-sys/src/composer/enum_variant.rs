use std::cell::RefCell;
use std::rc::Rc;
use syn::{Attribute, Field};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use crate::ast::{DelimiterTrait, Void};
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, FieldTypeKind};
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, ConstructorFieldsContext, CtorSpecification, FFIBindingsComposer, FFIBindingsSpecification, FFIComposer, FFIConversionsSpecification, FFIFieldsSpecification, FFIObjectSpecification, FieldsComposerRef, PresentableExpressionComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpecification, MethodComposer, PresentableArgumentComposerRef, OwnerCommaIteratorConversionComposer, SequenceOutputComposer, ItemComposerExpressionSpecification, AspectSequenceComposer, DropSequenceExprMixer, SharedComposer, ComposerRef};
use crate::composer::r#abstract::{LinkedContextComposer, SequenceMixer};
use crate::context::ScopeContext;
use crate::lang::{LangAttrSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, PresentableSequence, ScopeContextPresentable};
use crate::presentation::Name;
use crate::shared::SharedAccess;

pub struct EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &Vec<Attribute>,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
    ) -> ComposerLink<Self>
        where Self: ItemComposerSpecification<LANG, SPEC>
        + CtorSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
        + FFIFieldsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
        + FFIConversionsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> {
        Rc::new(RefCell::new(Self { composer: ItemComposer::new::<Self>(ty_context, None, AttrsModel::from(attrs), fields, context) }))
    }
}

impl<'a, Link, I, LANG, SPEC> FFIObjectSpecification<Link, LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where Link: SharedAccess<ImmutableAccess = ComposerRef<'a, ItemComposer<I, LANG, SPEC>>>,
          I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpecification<LANG, SPEC> {
    const COMPOSER: Option<SequenceOutputComposer<Link, LANG, SPEC>> = Some(LinkedContextComposer::new(PresentableSequence::empty_root, PresentableSequence::empty));
}
impl<I, LANG, SPEC> FFIConversionsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpecification<LANG, SPEC>
          + ItemComposerExpressionSpecification<LANG, SPEC> {
    const COMPOSER: Option<FFIComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>> = Some(FFIComposer::new(
        SequenceMixer::new(
            PresentableSequence::lambda,
            PresentableSequence::fields_from,
            Self::ROOT_CONVERSION_PRESENTER,
            constants::raw_target_aspect_seq_context(),
            Self::CONVERSION,
            |(((aspect, field_types), _generics), presenter)|
                (aspect, constants::field_conversion_expressions_iterator((field_types, presenter), constants::resolver_from_enum_variant_statement()))),
        SequenceMixer::new(
            PresentableSequence::lambda,
            PresentableSequence::fields_to,
            Self::ROOT_CONVERSION_PRESENTER,
            constants::ffi_aspect_seq_context(),
            Self::CONVERSION,
            |(((aspect, field_types), _generics), presenter)|
                (aspect, constants::field_conversion_expressions_iterator((field_types, presenter), constants::resolver_to_enum_variant_statement()))),
        LinkedContextComposer::new(PresentableSequence::unboxed_root, PresentableSequence::fields_from),
        DropSequenceExprMixer::new(
            PresentableSequence::lambda,
            PresentableSequence::fields_from,
            PresentableSequence::DropCode,
            constants::field_types_composer(),
            Self::DESTROY,
            |(field_types, presenter)|
                constants::field_conversion_expressions_iterator((field_types, presenter), constants::resolver_drop_enum_variant_statement()))
    ));
}
impl<LANG, SPEC> CtorSpecification<ItemComposerLink<Brace, LANG, SPEC>, LANG, SPEC> for EnumVariantComposer<Brace, LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::variant_ctor;
    const CTOR_CONTEXT: SharedComposer<ItemComposerLink<Brace, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::named_struct_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::named_struct_ctor_pair;
}
impl<LANG, SPEC> CtorSpecification<ItemComposerLink<Paren, LANG, SPEC>, LANG, SPEC> for EnumVariantComposer<Paren, LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::variant_ctor;
    const CTOR_CONTEXT: SharedComposer<ItemComposerLink<Paren, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::unnamed_struct_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::unnamed_struct_ctor_pair;
}

impl<I, LANG, SPEC> FFIBindingsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for EnumVariantComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: CtorSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
{
    const COMPOSER: Option<FFIBindingsComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>> = Some(FFIBindingsComposer::new(
        constants::struct_ctor_sequence_composer(Self::CTOR_ROOT, Self::CTOR_CONTEXT, Self::CTOR_ARG),
        MethodComposer::new(BindingPresentableContext::dtor, constants::composer_ffi_binding::<ItemComposer<I, LANG, SPEC>, LANG, SPEC>()),
        MethodComposer::new(BindingPresentableContext::get, constants::ffi_aspect_seq_context()),
        MethodComposer::new(BindingPresentableContext::set, constants::ffi_aspect_seq_context()),
        true
    ));
}

impl<LANG, SPEC> CtorSpecification<ItemComposerLink<Void, LANG, SPEC>, LANG, SPEC> for EnumVariantComposer<Void, LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::variant_ctor;
    const CTOR_CONTEXT: SharedComposer<ItemComposerLink<Void, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::named_struct_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::named_struct_ctor_pair;
}


// Variant::Named
impl<LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for EnumVariantComposer<Brace, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> = PresentableSequence::CurlyBracesFields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> = PresentableArgument::attr_expr;
    const ROOT_CONVERSION_PRESENTER: OwnerCommaIteratorConversionComposer<LANG, SPEC> = PresentableSequence::CurlyBracesFields;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> = constants::struct_named_fields_composer();
}

// Variant::Unnamed
impl<LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for EnumVariantComposer<Paren, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> = PresentableSequence::RoundBracesFields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> = PresentableArgument::attr_expr;
    const ROOT_CONVERSION_PRESENTER: OwnerCommaIteratorConversionComposer<LANG, SPEC> = PresentableSequence::RoundBracesFields;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> = |fields| constants::field_composers_iterator(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::new(Name::UnnamedArg(index), FieldTypeKind::r#type(ty), false, SPEC::Attr::from_attrs(attrs.cfg_attributes())));// fn root_presenter() -> Self::RootPresenter { PresentableSequence::RoundBracesFields }
}
// Variant::Unit
impl<LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for EnumVariantComposer<Void, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> = PresentableSequence::no_fields;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> = PresentableArgument::attr_expr;
    const ROOT_CONVERSION_PRESENTER: OwnerCommaIteratorConversionComposer<LANG, SPEC> = PresentableSequence::no_fields;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> = |_| Punctuated::new();
}
impl<LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for EnumVariantComposer<Brace, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = Expression::named_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = Expression::terminated;
}
impl<LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for EnumVariantComposer<Paren, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = Expression::bypass_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = Expression::terminated;
}
impl<LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for EnumVariantComposer<Void, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = Expression::bypass_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = Expression::empty_conversion_type;
}
