use std::clone::Clone;
use syn::Field;
use crate::ast::CommaPunctuated;
use crate::composable::FieldComposer;
use crate::composer::{ArgComposers, AspectArgComposers, AspectSharedComposerLink, AttrComposable, CommaArgComposers, CommaPunctuatedFields, ComposerLink, CtorSpec, FFIBindingsComposer, FFIBindingsComposerLink, FFIComposer, FFIComposerLink, ArgProducerByRef, FieldPathConversionResolveSpec, FieldPathResolver, FieldsContext, FieldsConversionComposable, ArgsSequenceComposer, FieldsSequenceMixer, GenericsComposable, ItemComposerExprSpec, ItemComposerSpec, OwnedArgComposers, MethodComposer, NameKindComposable, OwnerAspectSequence, OwnerAspectSequenceSpec, ArgKindPair, PresentableExprComposerRef, RootSequenceComposer, SequenceComposer, SequenceSharedComposerLink, SourceAccessible, SourceComposerByRef, TypeAspect, IterativeComposer, ItemAspectsSpec, FFIInterfaceMethodSpec};
use crate::composer::r#abstract::SequenceMixer;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, InterfaceKind, SeqKind};

pub(crate) const fn args_composer_iterator_root<LANG, SPEC, CTX, Item, OUT>()
    -> SourceComposerByRef<OwnedArgComposers<LANG, SPEC, CTX>, ArgProducerByRef<LANG, SPEC, Item>, (CTX, OUT)>
    where CTX: Clone,
          OUT: FromIterator<Item>,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    |(aspect, arg_composers), composer|
        (aspect.clone(), arg_conversions_iterator(arg_composers, composer))
}

pub(crate) fn arg_conversion_expressions_iterator<LANG, SPEC, Iter>(
    (arg_composers, expr_composer): (&CommaArgComposers<LANG, SPEC>, PresentableExprComposerRef<LANG, SPEC>),
    resolver: FieldPathResolver<LANG, SPEC>
) -> Iter
where Iter: FromIterator<ArgKind<LANG, SPEC>>,
      LANG: LangFermentable,
      SPEC: Specification<LANG> {
    arg_conversions_iterator(arg_composers, |c| ArgKind::attr_expr_composer(c, resolver, expr_composer))
}


pub fn field_composers_iterator<LANG, SPEC, MAP>(
    fields: &CommaPunctuatedFields,
    mapper: MAP
) -> CommaArgComposers<LANG, SPEC>
    where MAP: Fn(&Field, usize) -> FieldComposer<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    CommaPunctuated::from_iter(fields.iter().enumerate().map(|(index, field)| mapper(field, index)))
}
pub fn arg_conversions_iterator<LANG, SPEC, MAP, Out, Iter, SEP>(
    composers: &ArgComposers<LANG, SPEC, SEP>,
    mapper: MAP
) -> Iter
    where MAP: Fn(&FieldComposer<LANG, SPEC>) -> Out,
          Iter: FromIterator<Out>,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    Iter::from_iter(composers.iter().map(mapper))
}


pub(crate) const fn ffi_conversions_composer<LANG, SPEC, T, C>(
    from_ffi_root: RootSequenceComposer<LANG, SPEC>,
    from_context: SequenceSharedComposerLink<LANG, SPEC, T>,
    from_aspect: AspectSharedComposerLink<LANG, SPEC, T>,
    to_ffi_root: RootSequenceComposer<LANG, SPEC>,
    to_context: SequenceSharedComposerLink<LANG, SPEC, T>,
    drop_root: RootSequenceComposer<LANG, SPEC>,
    drop_context: SequenceSharedComposerLink<LANG, SPEC, T>,
) -> FFIComposerLink<LANG, SPEC, T>
    where T: FieldsContext<LANG, SPEC> + AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen> + NameKindComposable,
          C: ItemComposerSpec<LANG, SPEC> + ItemComposerExprSpec<LANG, SPEC> + FieldPathConversionResolveSpec<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    FFIComposer::new(
        InterfaceKind::From(FieldsSequenceMixer::with_sequence(
            from_ffi_root,
            from_context,
            SequenceComposer::new(
                C::FROM_ROOT_CONVERSION_PRESENTER,
                from_aspect,
                IterativeComposer::new(
                |(aspect, field_composers), expr_composer|
                    (aspect.clone(), arg_conversion_expressions_iterator((field_composers, expr_composer), C::FROM)),
                C::FROM_CONVERSION)))),

        InterfaceKind::To(FieldsSequenceMixer::with_sequence(
            to_ffi_root,
            to_context,
            SequenceComposer::new(
                C::TO_ROOT_CONVERSION_PRESENTER,
                Aspect::ffi,
                IterativeComposer::new(
                |(aspect, field_composers), expr_composer|
                    (aspect.clone(), arg_conversion_expressions_iterator((field_composers, expr_composer), C::TO)),
                C::TO_CONVERSION)))),
        InterfaceKind::Drop(FieldsSequenceMixer::with_sequence(
            drop_root,
            drop_context,
            SequenceComposer::new(
                C::DROP_ROOT_CONVERSION_PRESENTER,
                Aspect::ffi,
                IterativeComposer::new(|(aspect, field_composers), expr_composer|
                    (aspect.clone(), arg_conversion_expressions_iterator((field_composers, expr_composer), C::DROP)),
                C::DROP_CONVERSION))))

    )
}
#[allow(unused)]
pub(crate) const fn ffi_conversions_composer2<LANG, SPEC, T, C>(
    from_ffi_root: RootSequenceComposer<LANG, SPEC>,
    from_context: SequenceSharedComposerLink<LANG, SPEC, T>,
    from_aspect: AspectSharedComposerLink<LANG, SPEC, T>,
    to_ffi_root: RootSequenceComposer<LANG, SPEC>,
    to_context: SequenceSharedComposerLink<LANG, SPEC, T>,
    drop_root: RootSequenceComposer<LANG, SPEC>,
    drop_context: SequenceSharedComposerLink<LANG, SPEC, T>,
) -> FFIComposerLink<LANG, SPEC, T>
    where T: FieldsContext<LANG, SPEC> + AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen> + NameKindComposable,
          C: ItemAspectsSpec<LANG, SPEC>,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    FFIComposer::new(
        InterfaceKind::From(
            SequenceMixer::with_sequence(
                from_ffi_root,
                from_context,
                SequenceComposer::new(C::FROM::SEQ, from_aspect, C::FROM::ITER))),
        InterfaceKind::To(
            SequenceMixer::with_sequence(
                to_ffi_root,
                to_context,
                SequenceComposer::new(C::INTO::SEQ, Aspect::ffi, C::INTO::ITER))),
        InterfaceKind::Drop(
            SequenceMixer::with_sequence(
                drop_root,
                drop_context,
                SequenceComposer::new(C::DTOR::SEQ, Aspect::ffi, C::DTOR::ITER)))
    )
}

pub(crate) const fn ffi_bindings_composer<LANG, SPEC, T, C, Iter>()
    -> FFIBindingsComposerLink<LANG, SPEC, T, Iter>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: AttrComposable<SPEC::Attr>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<LANG, SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          C: CtorSpec<LANG, SPEC, ComposerLink<T>, Iter>,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    FFIBindingsComposer::new(
        ArgsSequenceComposer::with_iterator_setup(C::ROOT, C::ASPECT, C::ITER, C::ARG),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::dtor),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::get),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::set),
        true
    )
}
#[allow(unused)]
pub(crate) const fn ffi_bindings_composer2<LANG, SPEC, T, C, Iter>()
    -> FFIBindingsComposerLink<LANG, SPEC, T, Iter>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          T: AttrComposable<SPEC::Attr>
              + GenericsComposable<SPEC::Gen>
              + TypeAspect<SPEC::TYC>
              + FieldsContext<LANG, SPEC>
              + NameKindComposable
              + SourceAccessible
              + 'static,
          C: OwnerAspectSequenceSpec<LANG, SPEC, ComposerLink<T>, Iter, ArgKindPair<LANG, SPEC>, BindingPresentableContext<LANG, SPEC>>,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    FFIBindingsComposer::new(
        C::COMPOSER,
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::dtor),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::get),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::set),
        true
    )
}


#[allow(unused)]
pub const fn fields_sequence<LANG, SPEC, T, C, Iter>()
    -> ArgsSequenceComposer<LANG, SPEC, ComposerLink<T>, AspectArgComposers<LANG, SPEC>, Iter::Item, OwnerAspectSequence<LANG, SPEC, Iter>, SeqKind<LANG, SPEC>>
    where C: OwnerAspectSequenceSpec<LANG, SPEC, ComposerLink<T>, Iter, ArgKind<LANG, SPEC>, SeqKind<LANG, SPEC>>,
    T: AttrComposable<SPEC::Attr>
        + GenericsComposable<SPEC::Gen>
        + TypeAspect<SPEC::TYC>
        + NameKindComposable
        + FieldsContext<LANG, SPEC>
        + FieldsConversionComposable<LANG, SPEC>
        + SourceAccessible
        + 'static,
    LANG: LangFermentable,
    SPEC: Specification<LANG>,
    Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKind<LANG, SPEC>> {
    ArgsSequenceComposer::with_iterator_setup(C::ROOT, C::ASPECT, C::ITER, C::ARG)
}