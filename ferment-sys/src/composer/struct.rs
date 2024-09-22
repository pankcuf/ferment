use std::cell::RefCell;
use std::rc::Rc;
use syn::{Attribute, Generics};
use syn::token::{Brace, Paren};
use crate::ast::DelimiterTrait;
use crate::composable::AttrsModel;
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, ConstructorFieldsContext, CtorSpecification, FFIBindingsComposer, FFIBindingsSpecification, FFIComposer, FFIConversionsSpecification, FFIFieldsSpecification, FFIObjectSpecification, FieldsComposerRef, PresentableExpressionComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpecification, MethodComposer, PresentableArgumentComposerRef, OwnerCommaIteratorConversionComposer, SequenceOutputComposer, SharedComposerLink, ItemComposerExpressionSpecification, AspectSequenceComposer, SharedComposer, ComposerRef};
use crate::composer::r#abstract::{LinkedContextComposer, SequenceComposer, SequenceMixer};
use crate::context::ScopeContext;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, PresentableSequence, ScopeContextPresentable};
use crate::shared::SharedAccess;

pub struct StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self:
            ItemComposerSpecification<LANG, SPEC>
            + CtorSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
            + FFIFieldsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
            + FFIConversionsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> {
    pub fn new(
        ty_context: SPEC::TYC,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
    ) -> ComposerLink<Self> {
        Rc::new(RefCell::new(Self { composer: ItemComposer::new::<Self>(ty_context, Some(generics.clone()), AttrsModel::from(attrs), fields, context) }))
    }
}
impl<'a, Link, I, LANG, SPEC> FFIObjectSpecification<Link, LANG, SPEC> for StructComposer<I, LANG, SPEC>
    where Link: SharedAccess<ImmutableAccess = ComposerRef<'a, ItemComposer<I, LANG, SPEC>>>,
          I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: Option<SequenceOutputComposer<Link, LANG, SPEC>> = Some(LinkedContextComposer::new(PresentableSequence::bypass, PresentableSequence::fields_from));
}

impl<I, LANG, SPEC> FFIConversionsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for StructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpecification<LANG, SPEC> + ItemComposerExpressionSpecification<LANG, SPEC> {
    const COMPOSER: Option<FFIComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>> = Some(FFIComposer::new(
        SequenceMixer::with_sequence(
            PresentableSequence::from_root,
            PresentableSequence::deref_ffi,
            SequenceComposer::with_iterator_setup(
                Self::ROOT_CONVERSION_PRESENTER,
                constants::target_aspect_seq_context(),
                |(((aspect, field_types), _), presenter)|
                    (aspect, constants::field_conversion_expressions_iterator((field_types, presenter), constants::resolver_from_struct_field_statement())),
                Self::CONVERSION
            )
        ),
        SequenceMixer::with_sequence(
            PresentableSequence::boxed,
            PresentableSequence::empty,
            SequenceComposer::with_iterator_setup(
                Self::ROOT_CONVERSION_PRESENTER,
                constants::ffi_aspect_seq_context(),
                |(((aspect, field_types), _), presenter)|
                    (aspect, constants::field_conversion_expressions_iterator((field_types, presenter), constants::resolver_to_struct_field_statement())),
                Self::CONVERSION
            )
        ),
        LinkedContextComposer::new(PresentableSequence::unboxed_root, PresentableSequence::empty),
        constants::struct_drop_sequence_mixer(),
    ));
}
impl<LANG, SPEC> CtorSpecification<ItemComposerLink<Brace, LANG, SPEC>, LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::ctor;
    const CTOR_CONTEXT: SharedComposerLink<ItemComposer<Brace, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::named_struct_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::named_struct_ctor_pair;
}
impl<LANG, SPEC> CtorSpecification<ItemComposerLink<Paren, LANG, SPEC>, LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::ctor;
    const CTOR_CONTEXT: SharedComposer<ItemComposerLink<Paren, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::unnamed_struct_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::unnamed_struct_ctor_pair;
}
impl<I, LANG, SPEC> FFIBindingsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for StructComposer<I, LANG, SPEC>
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

// Struct::Named
impl<LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> = PresentableSequence::NamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> = PresentableArgument::public_named;
    const ROOT_CONVERSION_PRESENTER: OwnerCommaIteratorConversionComposer<LANG, SPEC> = PresentableSequence::CurlyBracesFields;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> = constants::struct_named_fields_composer();
}

// Struct::Unnamed
impl<LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> = PresentableSequence::UnnamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> = PresentableArgument::default_field_type;
    const ROOT_CONVERSION_PRESENTER: OwnerCommaIteratorConversionComposer<LANG, SPEC> = PresentableSequence::RoundBracesFields;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> = constants::struct_unnamed_fields_composer();
}

impl<LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for StructComposer<Brace, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = Expression::named_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = Expression::bypass_conversion_type;
}

// Struct::Unnamed
impl<LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for StructComposer<Paren, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = SPEC::Expr::bypass_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = SPEC::Expr::bypass_conversion_type;
}
