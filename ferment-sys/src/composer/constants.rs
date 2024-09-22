use std::clone::Clone;
use syn::__private::TokenStream2;
use syn::Field;
use crate::ast::CommaPunctuated;
use crate::composable::{CfgAttributes, FieldComposer, FieldTypeKind};
use crate::composer::{ArgComposers, AspectPresentable, AttrComposable, BindingCtorComposer, CommaPunctuatedFields, ComposerLink, ComposerLinkDelegateByRef, ComposerPresenter, ComposerRef, ConstructorArgComposerRef, ConstructorFieldsContext, CtorSequenceComposer, DestructorContext, FieldComposers, FieldPathResolver, FieldsComposerRef, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, PresentableExpressionComposerRef, GenericsComposable, LinkedContextComposer, LocalConversionContext, LocallyOwnedFieldComposers, PresentableArgumentComposerRef, SharedComposerLink, SourceAccessible, TypeAspect, TypeContextComposer, AspectSequenceComposer, FieldComposerProducer, DropSequenceExprMixer};
use crate::ext::ConversionType;
use crate::lang::{LangAttrSpecification, Specification};
use crate::presentable::{Aspect, NameTreeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{default_doc, DictionaryName, Name};
pub const fn composer_doc<C, TYC>() -> TypeContextComposer<ComposerLink<C>, TYC, TokenStream2>
    where TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable,
          C: AspectPresentable<TYC>
          + SourceAccessible
          + 'static {
    LinkedContextComposer::new(default_doc, |c: &ComposerRef<C>| C::present_target_aspect(c))
}

pub const fn field_types_composer<'a, C, LANG, SPEC>()
    -> ComposerLinkDelegateByRef<'a, C, FieldComposers<LANG, SPEC>>
    where C: FieldsContext<LANG, SPEC>,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| C::field_composers(c)
}

pub const fn ffi_aspect_seq_context<C, LANG, SPEC>()
    -> SharedComposerLink<C, LocalConversionContext<LANG, SPEC>>
    where C: TypeAspect<SPEC::TYC>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| ((C::ffi_type_aspect(c), C::field_composers(c)), C::compose_generics(c))
}

pub const fn target_aspect_seq_context<C, LANG, SPEC>()
    -> SharedComposerLink<C, LocalConversionContext<LANG, SPEC>>
    where C: TypeAspect<SPEC::TYC>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| ((C::target_type_aspect(c), C::field_composers(c)), C::compose_generics(c))
}
pub const fn raw_target_aspect_seq_context<C, LANG, SPEC>()
    -> SharedComposerLink<C, LocalConversionContext<LANG, SPEC>>
    where C: TypeAspect<SPEC::TYC>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| ((C::raw_target_type_aspect(c), C::field_composers(c)), C::compose_generics(c))
}

pub const fn fields_from_composer<C, LANG, SPEC>(
    root_presenter: AspectSequenceComposer<LANG, SPEC>,
    field_presenter: PresentableArgumentComposerRef<LANG, SPEC>
) -> FieldsOwnedSequenceComposer<ComposerLink<C>, LANG, SPEC>
    where C: SourceAccessible
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsConversionComposable<LANG, SPEC>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    FieldsOwnedSequenceComposer::with_iterator_setup(
        root_presenter,
        ffi_aspect_seq_context(),
        fields_composer_iterator_root(),
        field_presenter)
}
pub const fn fields_to_composer<C, LANG, SPEC>(
    root_presenter: AspectSequenceComposer<LANG, SPEC>,
    field_presenter: PresentableArgumentComposerRef<LANG, SPEC>
) -> FieldsOwnedSequenceComposer<ComposerLink<C>, LANG, SPEC>
    where C: SourceAccessible
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsConversionComposable<LANG, SPEC>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    FieldsOwnedSequenceComposer::with_iterator_setup(
        root_presenter,
        target_aspect_seq_context(),
        fields_composer_iterator_root(),
        field_presenter)
}

pub const fn named_opaque_ctor_context_composer<C, LANG, SPEC>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC>>
    where C: TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + AspectPresentable<SPEC::TYC>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    move |c| (((composer_target_binding::<C, LANG, SPEC>()(c), false), c.field_composers()), c.compose_generics())

}
pub const fn unnamed_opaque_ctor_context_composer<C, LANG, SPEC>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC>>
    where C: TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + AspectPresentable<SPEC::TYC>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    move |c| (((composer_target_binding::<C, LANG, SPEC>()(c), true), c.field_composers()), c.compose_generics())
}

pub const fn unnamed_struct_ctor_context_composer<C, LANG, SPEC>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC>>
    where C: SourceAccessible
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + AspectPresentable<SPEC::TYC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (((composer_ffi_binding::<C, LANG, SPEC>()(c), true), c.field_composers()), c.compose_generics())
}
pub const fn named_struct_ctor_context_composer<C, LANG, SPEC>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC>>
    where C: SourceAccessible
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + AspectPresentable<SPEC::TYC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (((composer_ffi_binding::<C, LANG, SPEC>()(c), false), c.field_composers()), c.compose_generics())
}

pub(crate) const fn struct_ctor_sequence_composer<Link, LANG, SPEC>(
    root: BindingCtorComposer<LANG, SPEC>,
    context: SharedComposerLink<Link, ConstructorFieldsContext<LANG, SPEC>>,
    iterator_item: ConstructorArgComposerRef<LANG, SPEC>,
) -> CtorSequenceComposer<ComposerLink<Link>, LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    CtorSequenceComposer::with_iterator_setup(
        root,
        context,
        fields_composer_iterator_root(),
        iterator_item
    )
}

pub const fn struct_drop_sequence_mixer<C, LANG, SPEC>()
    -> DropSequenceExprMixer<ComposerLink<C>, LANG, SPEC>
    where C: SourceAccessible
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    DropSequenceExprMixer::new(
        PresentableSequence::struct_drop_post_processor,
        PresentableSequence::empty,
        PresentableSequence::StructDropBody,
        field_types_composer::<C, LANG, SPEC>(),
        Expression::bypass_conversion_type,
        |(field_types, presenter)|
            field_conversion_expressions_iterator((field_types, presenter), resolver_drop_struct_field_statement())
    )
}

pub const fn struct_unnamed_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields| field_composers_iterator(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::new(Name::UnnamedStructFieldsComp(ty.clone(), index), FieldTypeKind::r#type(ty), false, SPEC::Attr::from_attrs(attrs.cfg_attributes())))
}
pub const fn struct_named_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields| field_composers_iterator(
        fields,
        |_index, Field { ident, ty, attrs, .. }|
            FieldComposer::new(Name::Optional(ident.clone()), FieldTypeKind::r#type(ty), true, SPEC::Attr::from_attrs(attrs.cfg_attributes())))
}


/// Enum composers
pub const fn composer_ffi_binding<C, LANG, SPEC>()
    -> SharedComposerLink<C, DestructorContext<LANG, SPEC>>
    where C: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + AspectPresentable<SPEC::TYC>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (C::present_ffi_aspect(c), C::compose_attributes(c), C::compose_generics(c))
}
pub const fn composer_target_binding<C, LANG, SPEC>()
    -> SharedComposerLink<C, DestructorContext<LANG, SPEC>>
    where C: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + AspectPresentable<SPEC::TYC>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (C::present_target_aspect(c), C::compose_attributes(c), C::compose_generics(c))
}

pub const fn resolver_from_struct_field_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c | (c.name.clone(), ConversionType::expr_from(c, Some(Expression::ffi_ref_with_name(&c.name))))
}
pub const fn resolver_from_enum_variant_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c | (c.name.clone(), ConversionType::expr_from(c, Some(Expression::deref_tokens(&c.name))))
}
pub const fn resolver_to_enum_variant_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c | (c.name.clone(), ConversionType::expr_to(c, Some(Expression::name(&c.name))))
}

pub const fn resolver_to_struct_field_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (c.name.clone(), ConversionType::expr_to(c, Some(Expression::obj_name(&c.name))))
}
pub const fn resolver_to_type_alias_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (Name::Empty, ConversionType::expr_to(c, Some(Expression::name(&Name::Dictionary(DictionaryName::Obj)))))
}
pub const fn resolver_drop_enum_variant_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (c.name.clone(), ConversionType::expr_destroy(c, Some(Expression::deref_tokens(&c.name))))
}
pub const fn resolver_drop_struct_field_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |c| (Name::Empty, ConversionType::expr_destroy(c, Some(Expression::ffi_ref_with_name(&c.name))))
}

const fn fields_composer_iterator_root<CTX, Item, OUT, LANG, SPEC>()
    -> ComposerPresenter<
        (LocallyOwnedFieldComposers<CTX, LANG, SPEC>, FieldComposerProducer<LANG, SPEC, Item>),
        (CTX, OUT)
    >
    where OUT: FromIterator<Item>,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          CommaPunctuated<PresentableArgument<LANG, SPEC>>: FromIterator<PresentableArgument<LANG, SPEC>> {
    |(((aspect, field_composers), _generics), composer)|
        (aspect, field_conversions_iterator(field_composers, composer))
}


pub fn field_composers_iterator<MAP, LANG, SPEC>(
    fields: &CommaPunctuatedFields,
    mapper: MAP
) -> FieldComposers<LANG, SPEC>
    where MAP: Fn(usize, &Field) -> FieldComposer<LANG, SPEC>,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    CommaPunctuated::from_iter(fields.iter().enumerate().map(|(index, field)| mapper(index, field)))
}
pub fn field_conversions_iterator<MAP, Out, It, LANG, SPEC, SEP>(
    composers: ArgComposers<SEP, LANG, SPEC>,
    mapper: MAP
) -> It
    where MAP: Fn(&FieldComposer<LANG, SPEC>) -> Out,
          It: FromIterator<Out>,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    It::from_iter(composers.iter().map(mapper))
}

pub(crate) fn field_conversion_expressions_iterator<It, LANG, SPEC>(
    (composers, presenter): (FieldComposers<LANG, SPEC>, PresentableExpressionComposerRef<LANG, SPEC>),
    resolver: FieldPathResolver<LANG, SPEC>
) -> It
    where It: FromIterator<PresentableArgument<LANG, SPEC>>,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    field_conversions_iterator(
        composers,
        |composer| {
            let template = resolver(composer);
            let expr = presenter(&template);
            PresentableArgument::AttrExpression(expr, composer.attrs.clone())
        })
}
