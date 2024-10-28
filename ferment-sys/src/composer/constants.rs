use std::clone::Clone;
use std::fmt::Debug;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Field;
use syn::token::Comma;
use crate::ast::CommaPunctuated;
use crate::composable::FieldComposer;
use crate::composer::{ArgComposers, AspectPresentable, AttrComposable, CommaPunctuatedFields, ComposerLink, ComposerLinkRef, CtorSequenceComposer, FieldComposers, FieldPathResolver, FieldsComposerRef, FieldsContext, PresentableExprComposerRef, GenericsComposable, LinkedContextComposer, LocallyOwnedFieldComposers, SourceAccessible, TypeAspect, FieldComposerProducer, FieldPathConversionResolveSpec, ItemComposerExprSpec, FFIComposer, ItemComposerSpec, FFIBindingsComposer, MethodComposer, CtorSpec, FFIComposerLink, TypeContextComposerLink, FFIBindingsComposerLink, FieldsOwnedSequenceComposerLink, FieldsConversionComposable, FieldsOwnedSequenceComposer, AspectSharedComposerLink, SequenceSharedComposerLink, FieldsSequenceMixer, RootSequenceComposer, SourceComposerByRef, ItemInterfaceComposerSpec, FFIConversionsMixer, FFIInterfaceMethodSpec};
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, NameTreeContext, ArgKind, ScopeContextPresentable, SeqKind, Expression, BindingPresentableContext};
use crate::presentation::{default_doc, Name};
use crate::shared::SharedAccess;

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
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    FieldsOwnedSequenceComposer::with_iterator_setup(
        C::ROOT_PRESENTER,
        aspect,
        fields_composer_iterator_root(),
        C::FIELD_PRESENTER)
}

pub const fn struct_unnamed_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields| field_composers_iterator(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::typed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty, false, attrs))
}
pub const fn struct_named_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields| field_composers_iterator(
        fields,
        |_index, Field { ident, ty, attrs, .. }|
            FieldComposer::typed(Name::Optional(ident.clone()), ty, true, attrs))
}

pub(crate) const fn fields_composer_iterator_root<CTX, Item, OUT, LANG, SPEC>()
    -> SourceComposerByRef<LocallyOwnedFieldComposers<CTX, LANG, SPEC>, FieldComposerProducer<LANG, SPEC, Item>, (CTX, OUT)>
    where CTX: Clone,
          OUT: FromIterator<Item>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          CommaPunctuated<ArgKind<LANG, SPEC>>: FromIterator<ArgKind<LANG, SPEC>> {
    |(aspect, field_composers), composer|
        (aspect.clone(), field_conversions_iterator(field_composers, composer))
}


pub fn field_composers_iterator<MAP, LANG, SPEC>(
    fields: &CommaPunctuatedFields,
    mapper: MAP
) -> FieldComposers<LANG, SPEC>
    where MAP: Fn(usize, &Field) -> FieldComposer<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    CommaPunctuated::from_iter(fields.iter().enumerate().map(|(index, field)| mapper(index, field)))
}
pub fn field_conversions_iterator<MAP, Out, It, LANG, SPEC, SEP>(
    composers: &ArgComposers<SEP, LANG, SPEC>,
    mapper: MAP
) -> It
    where MAP: Fn(&FieldComposer<LANG, SPEC>) -> Out,
          It: FromIterator<Out>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    It::from_iter(composers.iter().map(mapper))
}

pub(crate) fn field_conversion_expressions_iterator<It, LANG, SPEC>(
    (field_composers, expr_composer): (&FieldComposers<LANG, SPEC>, PresentableExprComposerRef<LANG, SPEC>),
    resolver: FieldPathResolver<LANG, SPEC>
) -> It
    where It: FromIterator<ArgKind<LANG, SPEC>>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    field_conversions_iterator(
        field_composers,
        |c| ArgKind::attr_expr_composer(c, resolver, expr_composer))
}

pub(crate) const fn ffi_conversions_composer<T, C, LANG, SPEC>(
    from_ffi_root: RootSequenceComposer<LANG, SPEC>,
    from_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    // from_seq3: AspectSequenceComposer<LANG, SPEC>,

    from_aspect: AspectSharedComposerLink<T, LANG, SPEC>,

    to_ffi_root: RootSequenceComposer<LANG, SPEC>,
    to_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    // to_seq3: OwnerCommaIteratorConversionComposer<LANG, SPEC>,
    // to_aspect: SharedAspectComposerLink<T, LANG, SPEC>,
    // destroy_seq1: Composer<SeqKind<LANG, SPEC>, SeqKind<LANG, SPEC>>,
    // destroy_seq2: SharedSequenceComposerLink<T, LANG, SPEC>,
    drop_root: RootSequenceComposer<LANG, SPEC>,
    drop_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    // drop_seq3: DropSequenceComposer<LANG, SPEC>,
) -> FFIComposerLink<T, LANG, SPEC>
    where T: FieldsContext<LANG, SPEC> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen>,
          C: ItemComposerSpec<LANG, SPEC> + ItemComposerExprSpec<LANG, SPEC> + FieldPathConversionResolveSpec<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    FFIComposer::new(
        FieldsSequenceMixer::new(
            from_ffi_root,
            from_context,
            C::FROM_ROOT_CONVERSION_PRESENTER,
            from_aspect,
            C::CONVERSION,
            |((aspect, field_composers), expr_composer)|
                (aspect, field_conversion_expressions_iterator((field_composers, expr_composer), C::FROM))
        ),
        FieldsSequenceMixer::new(
            to_ffi_root,
            to_context,
            C::TO_ROOT_CONVERSION_PRESENTER,
            |c: &ComposerLinkRef<T>| ((T::ffi_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
            C::CONVERSION,
            |((aspect, field_composers), expr_composer)|
                (aspect, field_conversion_expressions_iterator((field_composers, expr_composer), C::TO))
        ),
        LinkedContextComposer::new(
            SeqKind::unboxed_root,
            drop_context),
        FieldsSequenceMixer::new(
            drop_root,
            drop_context,
            drop_seq3,
            |c: &ComposerLinkRef<T>| ((T::ffi_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
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
//           It: FromIterator<ArgKind<LANG, SPEC>>,
//           Aspect<SPEC::TYC>: ScopeContextPresentable,
//           SeqKind<LANG, SPEC>: ScopeContextPresentable,
//           ArgKind<LANG, SPEC>: ScopeContextPresentable {
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
//           SeqKind<LANG, SPEC>: ScopeContextPresentable,
//           ArgKind<LANG, SPEC>: ScopeContextPresentable {
//     ffi_conversions_composer::<T, C, LANG, SPEC>(
//         SeqKind::ffi_from_root,
//         SeqKind::deref_ffi,
//         from_seq3,
//         |c| ((T::target_type_aspect(c), T::compose_generics(c)), T::field_composers(c)),
//         SeqKind::ffi_to_root,
//         to_seq2,
//         // SeqKind::empty,
//         SeqKind::struct_drop_post_processor,
//         SeqKind::empty,
//         SeqKind::StructDropBody
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
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
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