use std::clone::Clone;
use std::fmt::Debug;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Field;
use crate::ast::CommaPunctuated;
use crate::composable::FieldComposer;
use crate::composer::{ArgComposers, AspectPresentable, AttrComposable, CommaPunctuatedFields, ComposerLink, CommaArgComposers, FieldPathResolver, FieldsComposerRef, FieldsContext, PresentableExprComposerRef, GenericsComposable, LinkedContextComposer, LocallyOwnedFieldComposers, SourceAccessible, TypeAspect, FieldComposerProducer, FieldPathConversionResolveSpec, ItemComposerExprSpec, FFIComposer, ItemComposerSpec, FFIBindingsComposer, MethodComposer, CtorSpec, FFIComposerLink, TypeContextComposerLink, FFIBindingsComposerLink, AspectSharedComposerLink, SequenceSharedComposerLink, FieldsSequenceMixer, RootSequenceComposer, SourceComposerByRef, NameKindComposable, SequenceSpec, FieldsSequenceComposer, PresentableArgumentPair};
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, NameTreeContext, ArgKind, ScopeContextPresentable, SeqKind, Expression, BindingPresentableContext, InterfaceKind};
use crate::presentation::{default_doc, Name};

pub const fn composer_doc<T, TYC>() -> TypeContextComposerLink<T, TYC, TokenStream2>
    where TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable,
          T: AspectPresentable<TYC>
          + SourceAccessible
          + 'static {
    LinkedContextComposer::new(default_doc, T::present_target_aspect_ref)
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

pub(crate) const fn args_composer_iterator_root<CTX, Item, OUT, LANG, SPEC>()
    -> SourceComposerByRef<LocallyOwnedFieldComposers<CTX, LANG, SPEC>, FieldComposerProducer<LANG, SPEC, Item>, (CTX, OUT)>
    where CTX: Clone,
          OUT: FromIterator<Item>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          CommaPunctuated<ArgKind<LANG, SPEC>>: FromIterator<ArgKind<LANG, SPEC>> {
    |(aspect, field_composers), composer|
        (aspect.clone(), arg_conversions_iterator(field_composers, composer))
}


pub fn field_composers_iterator<MAP, LANG, SPEC>(
    fields: &CommaPunctuatedFields,
    mapper: MAP
) -> CommaArgComposers<LANG, SPEC>
    where MAP: Fn(usize, &Field) -> FieldComposer<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    CommaPunctuated::from_iter(fields.iter().enumerate().map(|(index, field)| mapper(index, field)))
}
pub fn arg_conversions_iterator<MAP, Out, It, LANG, SPEC, SEP>(
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

pub(crate) fn arg_conversion_expressions_iterator<It, LANG, SPEC>(
    (arg_composers, expr_composer): (&CommaArgComposers<LANG, SPEC>, PresentableExprComposerRef<LANG, SPEC>),
    resolver: FieldPathResolver<LANG, SPEC>
) -> It
    where It: FromIterator<ArgKind<LANG, SPEC>>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    arg_conversions_iterator(
        arg_composers,
        |c| ArgKind::attr_expr_composer(c, resolver, expr_composer))
}

pub(crate) const fn ffi_conversions_composer<T, C, LANG, SPEC>(
    from_ffi_root: RootSequenceComposer<LANG, SPEC>,
    from_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    from_aspect: AspectSharedComposerLink<T, LANG, SPEC>,
    to_ffi_root: RootSequenceComposer<LANG, SPEC>,
    to_context: SequenceSharedComposerLink<T, LANG, SPEC>,
    drop_root: RootSequenceComposer<LANG, SPEC>,
    drop_context: SequenceSharedComposerLink<T, LANG, SPEC>,
) -> FFIComposerLink<T, LANG, SPEC>
    where T: FieldsContext<LANG, SPEC> + AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen> + NameKindComposable,
          C: ItemComposerSpec<LANG, SPEC> + ItemComposerExprSpec<LANG, SPEC> + FieldPathConversionResolveSpec<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable
{
    FFIComposer::new(
        InterfaceKind::From(FieldsSequenceMixer::new(
            from_ffi_root,
            from_context,
            C::FROM_ROOT_CONVERSION_PRESENTER,
            from_aspect,
            C::FROM_CONVERSION,
            |(aspect, field_composers), expr_composer|
                (aspect.clone(), arg_conversion_expressions_iterator((field_composers, expr_composer), C::FROM))
            )),
        InterfaceKind::To(FieldsSequenceMixer::new(
            to_ffi_root,
            to_context,
            C::TO_ROOT_CONVERSION_PRESENTER,
            Aspect::ffi,
            C::TO_CONVERSION,
            |(aspect, field_composers), expr_composer|
                (aspect.clone(), arg_conversion_expressions_iterator((field_composers, expr_composer), C::TO))
        )),
        InterfaceKind::Destroy(LinkedContextComposer::new(SeqKind::unboxed_root, drop_context)),
        InterfaceKind::Drop(FieldsSequenceMixer::new(
            drop_root,
            drop_context,
            C::DROP_ROOT_CONVERSION_PRESENTER,
            Aspect::ffi,
            C::DROP_CONVERSION,
            |(aspect, field_composers), expr_composer|
                (aspect.clone(), arg_conversion_expressions_iterator((field_composers, expr_composer), C::DROP))
        ))

    )
}

pub(crate) const fn ffi_bindings_composer<T, C, LANG, SPEC>()
    -> FFIBindingsComposerLink<T, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
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
        FieldsSequenceComposer::with_iterator_setup(
            C::ROOT,
            C::ASPECT,
            C::ITER,
            C::ARG),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::dtor),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::get),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::set),
        true
    )
}
pub(crate) const fn ffi_bindings_composer2<T, C, LANG, SPEC, II>()
    -> FFIBindingsComposerLink<T, LANG, SPEC>
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + NameKindComposable
            + SourceAccessible
            + 'static,
        II: Iterator<Item=PresentableArgumentPair<LANG, SPEC>>,
          C: SequenceSpec<
              ComposerLink<T>,
              LANG,
              SPEC,
              BindingPresentableContext<LANG, SPEC>
          >,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    FFIBindingsComposer::new(
        FieldsSequenceComposer::with_iterator_setup(
            C::ROOT,
            C::ASPECT,
            C::ITER,
            C::ARG),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::dtor),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::get),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::set),
        true
    )
}
