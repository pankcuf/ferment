use std::clone::Clone;
use syn::Field;
use crate::ast::CommaPunctuated;
use crate::composable::FieldComposer;
use crate::composer::{ArgComposers, AspectArgComposers, AspectSharedComposerLink, AttrComposable, CommaArgComposers, CommaPunctuatedFields, ComposerLink, CtorSpec, FFIBindingsComposer, FFIBindingsComposerLink, FFIComposer, FFIComposerLink, ArgProducerByRef, FieldPathConversionResolveSpec, FieldPathResolver, FieldsContext, FieldsConversionComposable, ArgsSequenceComposer, FieldsSequenceMixer, GenericsComposable, ItemComposerExprSpec, ItemComposerSpec, OwnedArgComposers, MethodComposer, NameKindComposable, OwnerAspectSequence, OwnerAspectSequenceSpec, ArgKindPair, PresentableExprComposerRef, RootSequenceComposer, SequenceComposer, SequenceSharedComposerLink, SourceAccessible, SourceComposerByRef, TypeAspect, IterativeComposer, ItemAspectsSpec, FFIInterfaceMethodSpec};
use crate::composer::r#abstract::SequenceMixer;
use crate::lang::Specification;
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, InterfaceKind, SeqKind};

pub(crate) const fn args_composer_iterator_root<SPEC, CTX, Item, OUT>()
    -> SourceComposerByRef<OwnedArgComposers<SPEC, CTX>, ArgProducerByRef<SPEC, Item>, (CTX, OUT)>
    where CTX: Clone,
          OUT: FromIterator<Item>,
          SPEC: Specification {
    |(aspect, arg_composers), composer|
        (aspect.clone(), arg_conversions_iterator(arg_composers, composer))
}

pub(crate) fn arg_conversion_expressions_iterator<SPEC, Iter>(
    (arg_composers, expr_composer): (&CommaArgComposers<SPEC>, PresentableExprComposerRef<SPEC>),
    resolver: FieldPathResolver<SPEC>
) -> Iter
where Iter: FromIterator<ArgKind<SPEC>>,
      SPEC: Specification {
    arg_conversions_iterator(arg_composers, |c| ArgKind::attr_expr_composer(c, resolver, expr_composer))
}


pub fn field_composers_iterator<SPEC, MAP>(
    fields: &CommaPunctuatedFields,
    mapper: MAP
) -> CommaArgComposers<SPEC>
    where MAP: Fn(&Field, usize) -> FieldComposer<SPEC>,
          SPEC: Specification {
    CommaPunctuated::from_iter(fields.iter().enumerate().map(|(index, field)| mapper(field, index)))
}
pub fn arg_conversions_iterator<SPEC, MAP, Out, Iter, SEP>(
    composers: &ArgComposers<SPEC, SEP>,
    mapper: MAP
) -> Iter
    where MAP: Fn(&FieldComposer<SPEC>) -> Out,
          Iter: FromIterator<Out>,
          SPEC: Specification {
    Iter::from_iter(composers.iter().map(mapper))
}


pub(crate) const fn ffi_conversions_composer<SPEC, T, C>(
    from_ffi_root: RootSequenceComposer<SPEC>,
    from_context: SequenceSharedComposerLink<SPEC, T>,
    from_aspect: AspectSharedComposerLink<SPEC, T>,
    to_ffi_root: RootSequenceComposer<SPEC>,
    to_context: SequenceSharedComposerLink<SPEC, T>,
    drop_root: RootSequenceComposer<SPEC>,
    drop_context: SequenceSharedComposerLink<SPEC, T>,
) -> FFIComposerLink<SPEC, T>
    where T: FieldsContext<SPEC> + AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen> + NameKindComposable,
          C: ItemComposerSpec<SPEC> + ItemComposerExprSpec<SPEC> + FieldPathConversionResolveSpec<SPEC>,
          SPEC: Specification {
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
pub(crate) const fn ffi_conversions_composer2<SPEC, T, C>(
    from_ffi_root: RootSequenceComposer<SPEC>,
    from_context: SequenceSharedComposerLink<SPEC, T>,
    from_aspect: AspectSharedComposerLink<SPEC, T>,
    to_ffi_root: RootSequenceComposer<SPEC>,
    to_context: SequenceSharedComposerLink<SPEC, T>,
    drop_root: RootSequenceComposer<SPEC>,
    drop_context: SequenceSharedComposerLink<SPEC, T>,
) -> FFIComposerLink<SPEC, T>
    where T: FieldsContext<SPEC> + AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + GenericsComposable<SPEC::Gen> + NameKindComposable,
          C: ItemAspectsSpec<SPEC>,
          SPEC: Specification {
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

pub(crate) const fn ffi_bindings_composer<SPEC, T, C, Iter>()
    -> FFIBindingsComposerLink<SPEC, T, Iter>
    where SPEC: Specification,
          T: AttrComposable<SPEC::Attr>
          + GenericsComposable<SPEC::Gen>
          + TypeAspect<SPEC::TYC>
          + FieldsContext<SPEC>
          + NameKindComposable
          + SourceAccessible
          + 'static,
          C: CtorSpec<SPEC, ComposerLink<T>, Iter>,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    FFIBindingsComposer::new(
        ArgsSequenceComposer::with_iterator_setup(C::ROOT, C::ASPECT, C::ITER, C::ARG),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::dtor),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::get),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::set),
        true
    )
}
#[allow(unused)]
pub(crate) const fn ffi_bindings_composer2<SPEC, T, C, Iter>()
    -> FFIBindingsComposerLink<SPEC, T, Iter>
    where SPEC: Specification,
          T: AttrComposable<SPEC::Attr>
              + GenericsComposable<SPEC::Gen>
              + TypeAspect<SPEC::TYC>
              + FieldsContext<SPEC>
              + NameKindComposable
              + SourceAccessible
              + 'static,
          C: OwnerAspectSequenceSpec<SPEC, ComposerLink<T>, Iter, ArgKindPair<SPEC>, BindingPresentableContext<SPEC>>,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    FFIBindingsComposer::new(
        C::COMPOSER,
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::dtor),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::get),
        MethodComposer::new(Aspect::ffi, BindingPresentableContext::set),
        true
    )
}


#[allow(unused)]
pub const fn fields_sequence<SPEC, T, C, Iter>()
    -> ArgsSequenceComposer<SPEC, ComposerLink<T>, AspectArgComposers<SPEC>, Iter::Item, OwnerAspectSequence<SPEC, Iter>, SeqKind<SPEC>>
    where C: OwnerAspectSequenceSpec<SPEC, ComposerLink<T>, Iter, ArgKind<SPEC>, SeqKind<SPEC>>,
    T: AttrComposable<SPEC::Attr>
        + GenericsComposable<SPEC::Gen>
        + TypeAspect<SPEC::TYC>
        + NameKindComposable
        + FieldsContext<SPEC>
        + FieldsConversionComposable<SPEC>
        + SourceAccessible
        + 'static,
    SPEC: Specification,
    Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKind<SPEC>> {
    ArgsSequenceComposer::with_iterator_setup(C::ROOT, C::ASPECT, C::ITER, C::ARG)
}