use std::cell::RefCell;
use std::rc::Rc;
use syn::{Attribute, Generics};
use syn::token::{Brace, Paren};
use crate::ast::DelimiterTrait;
use crate::composable::AttrsModel;
use crate::composer::{CommaPunctuatedFields, ComposerLink, constants, FFIBindingsComposer, FieldsComposerRef, PresentableArgumentComposerRef, OwnerCommaIteratorConversionComposer, AspectSequenceComposer, ItemComposerLink, FFIFieldsSpecification, ItemComposerSpecification, CtorSpecification, FFIConversionsSpecification, ItemComposer, FFIBindingsSpecification, BindingCtorComposer, ConstructorFieldsContext, SharedComposer, ConstructorArgComposerRef, SequenceOutputComposer, FFIObjectSpecification, ComposerRef, ItemComposerExpressionSpecification, FFIComposer, PresentableExpressionComposerRef};
use crate::context::ScopeContext;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::shared::SharedAccess;

pub struct OpaqueStructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub composer: ItemComposerLink<I, LANG, SPEC>
}

impl<I, LANG, SPEC> OpaqueStructComposer<I, LANG, SPEC>
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
impl<I, LANG, SPEC> FFIConversionsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for OpaqueStructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: ItemComposerSpecification<LANG, SPEC>
          + ItemComposerExpressionSpecification<LANG, SPEC> {
    const COMPOSER: Option<FFIComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>> = None;
}

impl<'a, Link, I, LANG, SPEC> FFIObjectSpecification<Link, LANG, SPEC> for OpaqueStructComposer<I, LANG, SPEC>
    where Link: SharedAccess<ImmutableAccess = ComposerRef<'a, ItemComposer<I, LANG, SPEC>>>,
          I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: Option<SequenceOutputComposer<Link, LANG, SPEC>> = None;
}

impl<LANG, SPEC> CtorSpecification<ItemComposerLink<Paren, LANG, SPEC>, LANG, SPEC> for OpaqueStructComposer<Paren, LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::ctor;
    const CTOR_CONTEXT: SharedComposer<ItemComposerLink<Paren, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::unnamed_opaque_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::unnamed_struct_ctor_pair;
}
impl<LANG, SPEC> CtorSpecification<ItemComposerLink<Brace, LANG, SPEC>, LANG, SPEC> for OpaqueStructComposer<Brace, LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC> = BindingPresentableContext::ctor;
    const CTOR_CONTEXT: SharedComposer<ItemComposerLink<Brace, LANG, SPEC>, ConstructorFieldsContext<LANG, SPEC>> = constants::named_opaque_ctor_context_composer();
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC> = PresentableArgument::opaque_named_struct_ctor_pair;
}

impl<I, LANG, SPEC> FFIBindingsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for OpaqueStructComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: CtorSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>
{
    const COMPOSER: Option<FFIBindingsComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC>> = None;
}

impl<LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for OpaqueStructComposer<Brace, LANG, SPEC>
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
impl<LANG, SPEC> ItemComposerSpecification<LANG, SPEC> for OpaqueStructComposer<Paren, LANG, SPEC>
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

impl<LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for OpaqueStructComposer<Brace, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = Expression::named_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = Expression::bypass_conversion_type;
}
impl<LANG, SPEC> ItemComposerExpressionSpecification<LANG, SPEC> for OpaqueStructComposer<Paren, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC> = SPEC::Expr::bypass_conversion_type;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC> = SPEC::Expr::bypass_conversion_type;
}

