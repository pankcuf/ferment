use std::clone::Clone;
use syn::__private::TokenStream2;
use syn::Field;
use crate::ast::CommaPunctuated;
use crate::composable::FieldComposer;
use crate::composer::{ArgComposers, AspectPresentable, AttrComposable, CommaPunctuatedFields, ComposerLink, Composer, ComposerLinkRef, CtorSequenceComposer, FieldComposers, FieldPathResolver, FieldsComposerRef, FieldsContext, PresentableExprComposerRef, GenericsComposable, LinkedContextComposer, LocallyOwnedFieldComposers, SourceAccessible, TypeAspect, FieldComposerProducer, FieldPathConversionResolveSpec, ItemComposerExprSpec, AspectSequenceComposer, FFIComposer, ItemComposerSpec, FFIBindingsComposer, MethodComposer, CtorSpec, FFIComposerLink, TypeContextComposerLink, FFIBindingsComposerLink, FieldsOwnedSequenceComposerLink, FieldsConversionComposable, FieldsOwnedSequenceComposer, AspectSharedComposerLink, SequenceSharedComposerLink, FieldsSequenceMixer, DropSequenceComposer, RootSequenceComposer};
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, NameTreeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression, BindingPresentableContext};
use crate::presentation::{default_doc, Name};

pub const fn composer_doc<T, TYC>() -> TypeContextComposerLink<T, TYC, TokenStream2>
    where TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable,
          T: AspectPresentable<TYC>
          + SourceAccessible
          + 'static {
    LinkedContextComposer::new(default_doc, T::present_target_aspect_ref)
}

pub const fn field_owned_sequence_composer<T, C, LANG, SPEC>(
    aspect: AspectSharedComposerLink<T, LANG, SPEC>
) -> FieldsOwnedSequenceComposerLink<T, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC>
            + SourceAccessible
            + 'static,
          C: ItemComposerSpec<LANG, SPEC> + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    FieldsOwnedSequenceComposer::with_iterator_setup(
        C::ROOT_PRESENTER,
        aspect,
        fields_composer_iterator_root(),
        C::FIELD_PRESENTER)
}

pub const fn struct_unnamed_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields| field_composers_iterator(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::typed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty, false, attrs))
}
pub const fn struct_named_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields| field_composers_iterator(
        fields,
        |_index, Field { ident, ty, attrs, .. }|
            FieldComposer::typed(Name::Optional(ident.clone()), ty, true, attrs))
}

pub(crate) const fn fields_composer_iterator_root<CTX, Item, OUT, LANG, SPEC>()
    -> Composer<(LocallyOwnedFieldComposers<CTX, LANG, SPEC>, FieldComposerProducer<LANG, SPEC, Item>), (CTX, OUT)>
    where OUT: FromIterator<Item>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          CommaPunctuated<PresentableArgument<LANG, SPEC>>: FromIterator<PresentableArgument<LANG, SPEC>> {
    |((aspect, field_composers), composer)|
        (aspect, field_conversions_iterator(field_composers, composer))
}


pub fn field_composers_iterator<MAP, LANG, SPEC>(
    fields: &CommaPunctuatedFields,
    mapper: MAP
) -> FieldComposers<LANG, SPEC>
    where MAP: Fn(usize, &Field) -> FieldComposer<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
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
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    It::from_iter(composers.iter().map(mapper))
}

pub(crate) fn field_conversion_expressions_iterator<It, LANG, SPEC>(
    (composers, presenter): (FieldComposers<LANG, SPEC>, PresentableExprComposerRef<LANG, SPEC>),
    resolver: FieldPathResolver<LANG, SPEC>
) -> It
    where It: FromIterator<PresentableArgument<LANG, SPEC>>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
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

pub(crate) const fn ffi_conversions_composer<T, C, LANG, SPEC>(
    from_ffi_root: RootSequenceComposer<LANG, SPEC>,
    from_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    from_seq3: AspectSequenceComposer<LANG, SPEC>,

    from_aspect: AspectSharedComposerLink<T, LANG, SPEC>,

    to_ffi_root: RootSequenceComposer<LANG, SPEC>,
    to_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    // to_seq3: OwnerCommaIteratorConversionComposer<LANG, SPEC>,
    // to_aspect: SharedAspectComposerLink<T, LANG, SPEC>,
    // destroy_seq1: Composer<PresentableSequence<LANG, SPEC>, PresentableSequence<LANG, SPEC>>,
    // destroy_seq2: SharedSequenceComposerLink<T, LANG, SPEC>,
    drop_root: RootSequenceComposer<LANG, SPEC>,
    drop_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    drop_seq3: DropSequenceComposer<LANG, SPEC>,
) -> FFIComposerLink<T, LANG, SPEC>
    where T: FieldsContext<LANG, SPEC> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen>,
          C: ItemComposerSpec<LANG, SPEC> + ItemComposerExprSpec<LANG, SPEC> + FieldPathConversionResolveSpec<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    FFIComposer::new(
        FieldsSequenceMixer::new(
            from_ffi_root,
            from_context,
            from_seq3,
            from_aspect,
            C::CONVERSION,
            |((aspect, field_composers), expr_composer)|
                (aspect, field_conversion_expressions_iterator((field_composers, expr_composer), C::FROM))
        ),
        FieldsSequenceMixer::new(
            to_ffi_root,
            to_context,
            C::ROOT_CONVERSION_PRESENTER,
            |c: &ComposerLinkRef<T>| ((T::ffi_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
            C::CONVERSION,
            |((aspect, field_composers), expr_composer)|
                (aspect, field_conversion_expressions_iterator((field_composers, expr_composer), C::TO))
        ),
        LinkedContextComposer::new(
            PresentableSequence::unboxed_root,
            drop_context),
        FieldsSequenceMixer::new(
            drop_root,
            drop_context,
            drop_seq3,
            |c: &ComposerLinkRef<T>| ((T::ffi_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
            // T::field_composers_by_ref,
            C::DESTROY,
            |((aspect, field_composers), expr_composer)|
                (aspect, field_conversion_expressions_iterator((field_composers, expr_composer), C::DROP))
        ),
    )
}

// pub const fn seq_iterator_root<T, C, LANG, SPEC, It>()
//     -> Composer<(LocalConversionContext<LANG, SPEC>, PresentableExprComposerRef<LANG, SPEC>), (GenericAspect<LANG, SPEC>, It)>
//     where T: FieldsContext<LANG, SPEC> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen>,
//           C: FieldPathResolveSpec<LANG, SPEC>,
//           LANG: LangFermentable,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           It: FromIterator<PresentableArgument<LANG, SPEC>>,
//           Aspect<SPEC::TYC>: ScopeContextPresentable,
//           PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
//           PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
//     |((aspect, field_composers), expr_composer)|
//         (aspect, field_conversion_expressions_iterator((field_composers, expr_composer), C::RESOLVER))
// }

// pub(crate) const fn struct_ffi_conversions_composer<T, C, LANG, SPEC>(
//     from_seq3: AspectSequenceComposer<LANG, SPEC>,
//     to_seq2: SequenceSharedComposerLink<T, LANG, SPEC>,
// ) -> FFIComposerLink<T, LANG, SPEC>
//     where T: FieldsContext<LANG, SPEC>
//             + TypeAspect<SPEC::TYC>
//             + GenericsComposable<SPEC::Gen>,
//           C: ItemComposerSpec<LANG, SPEC>
//             + ItemComposerExprSpec<LANG, SPEC>
//             + FieldPathConversionResolveSpec<LANG, SPEC>,
//           LANG: LangFermentable,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable,
//           PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
//           PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
//     ffi_conversions_composer::<T, C, LANG, SPEC>(
//         PresentableSequence::ffi_from_root,
//         PresentableSequence::deref_ffi,
//         from_seq3,
//         |c| ((T::target_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
//         PresentableSequence::ffi_to_root,
//         to_seq2,
//         // PresentableSequence::empty,
//         PresentableSequence::struct_drop_post_processor,
//         PresentableSequence::empty,
//         PresentableSequence::StructDropBody
//     )
// }

pub(crate) const fn ffi_bindings_composer<T, C, LANG, SPEC>()
    -> FFIBindingsComposerLink<T, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + 'static,
          C: CtorSpec<ComposerLink<T>, LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    FFIBindingsComposer::new(
        CtorSequenceComposer::with_iterator_setup(
            C::ROOT,
            C::CONTEXT,
            fields_composer_iterator_root(),
            C::ARG),
        MethodComposer::new(
            |c: &ComposerLinkRef<T>| (T::present_ffi_aspect(c), T::compose_attributes(c), T::compose_generics(c)),
            BindingPresentableContext::dtor),
        MethodComposer::new(
            |c: &ComposerLinkRef<T>| ((T::ffi_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
            BindingPresentableContext::get),
        MethodComposer::new(
            |c: &ComposerLinkRef<T>| ((T::ffi_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
            BindingPresentableContext::set),
        true
    )
}