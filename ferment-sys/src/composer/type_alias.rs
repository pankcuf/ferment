use std::cell::RefCell;
use std::rc::Rc;
use syn::{Attribute, Generics};
use crate::ast::DelimiterTrait;
use crate::composable::AttrsModel;
use crate::composer::{BindingCtorComposer, CommaPunctuatedFields, ComposerLink, constants, ConstructorArgComposerRef, ConstructorFieldsContext, CtorSpecification, FFIBindingsComposer, FFIBindingsSpecification, FFIComposer, FFIConversionsSpecification, FFIObjectSpecification, FieldsComposerRef, PresentableExpressionComposerRef, ItemComposer, ItemComposerLink, ItemComposerSpecification, MethodComposer, PresentableArgumentComposerRef, OwnerCommaIteratorConversionComposer, SequenceOutputComposer, ItemComposerExpressionSpecification, AspectSequenceComposer, SharedComposer, ComposerRef};
use crate::composer::r#abstract::{LinkedContextComposer, SequenceComposer, SequenceMixer};
use crate::context::ScopeContext;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, PresentableSequence, ScopeContextPresentable};
use crate::shared::SharedAccess;

pub struct TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub(crate) fn new(
        ty_context: SPEC::TYC,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
    ) -> ComposerLink<Self> {
        Rc::new(RefCell::new(Self { composer: ItemComposer::new::<Self>(ty_context, Some(generics.clone()), AttrsModel::from(attrs), fields, context) }))
    }
}
impl<I, LANG, SPEC> FFIBindingsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: CtorSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> {
    const COMPOSER: Option<FFIBindingsComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>> = Some(FFIBindingsComposer::new(
        constants::struct_ctor_sequence_composer(Self::CTOR_ROOT, Self::CTOR_CONTEXT, Self::CTOR_ARG),
        MethodComposer::new(BindingPresentableContext::dtor, constants::composer_ffi_binding::<ItemComposer<I, LANG, SPEC>, LANG, SPEC>()),
        MethodComposer::new(BindingPresentableContext::get, constants::ffi_aspect_seq_context()),
        MethodComposer::new(BindingPresentableContext::set, constants::ffi_aspect_seq_context()),
        true
    ));
}


impl<I, LANG, SPEC> CtorSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized + 'static,
          LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::ctor;
    const CTOR_CONTEXT: SharedComposer<ItemComposerLink<I, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::unnamed_struct_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::unnamed_struct_ctor_pair;
}

impl<I, LANG, SPEC> FFIConversionsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
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
                PresentableSequence::TypeAliasFromConversion,
                constants::target_aspect_seq_context(),
                |(((aspect, field_types), _generics), presenter)|
                    (aspect, constants::field_conversion_expressions_iterator((field_types, presenter), constants::resolver_from_struct_field_statement())),
                Expression::bypass_conversion_type
            )
        ),
        SequenceMixer::with_sequence(
            PresentableSequence::boxed,
            PresentableSequence::obj,
            SequenceComposer::with_iterator_setup(
                PresentableSequence::RoundBracesFields,
                constants::ffi_aspect_seq_context(),
                |(((aspect, field_types), _generics), presenter)|
                    (aspect, constants::field_conversion_expressions_iterator((field_types, presenter), constants::resolver_to_type_alias_statement())),
                Expression::bypass_conversion_type
            )
        ),
        LinkedContextComposer::new(PresentableSequence::unboxed_root, PresentableSequence::empty),
        constants::struct_drop_sequence_mixer()
    ));
}


impl<I, LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC> = PresentableSequence::UnnamedStruct;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC> = PresentableArgument::default_field_type;
    const ROOT_CONVERSION_PRESENTER: OwnerCommaIteratorConversionComposer<LANG, SPEC> = PresentableSequence::RoundBracesFields;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC> = constants::struct_unnamed_fields_composer::<LANG, SPEC>();
}
impl<I, LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = Expression::bypass_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = Expression::bypass_conversion_type;
}

impl<'a, Link, I, LANG, SPEC> FFIObjectSpecification<Link, LANG, SPEC> for TypeAliasComposer<I, LANG, SPEC>
    where Link: SharedAccess<ImmutableAccess = ComposerRef<'a, ItemComposer<I, LANG, SPEC>>>,
          I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpecification<LANG, SPEC> {
    const COMPOSER: Option<SequenceOutputComposer<Link, LANG, SPEC>> = Some(LinkedContextComposer::new(PresentableSequence::bypass, PresentableSequence::fields_from));
}
