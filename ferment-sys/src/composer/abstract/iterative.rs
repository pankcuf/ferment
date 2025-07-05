use quote::ToTokens;
use crate::composable::FieldComposer;
use crate::composer::{SourceComposable, ComposerByRef, Linkable, SourceContextComposerByRef, constants, OwnedArgComposers, OwnerAspect, OwnerAspectSequence, FieldTypeLocalContext, FFIInterfaceMethodSpec, CommaPunctuatedArgKinds, AspectArgComposers, PunctuatedArgKinds};
use crate::lang::Specification;
use crate::presentable::ArgKind;
use crate::shared::SharedAccess;

pub struct IterativeComposer<In, Ctx, Map, Out> {
    set_output: SourceContextComposerByRef<In, Ctx, Map, Out>,
    mapper: ComposerByRef<Ctx, Map>
}

impl<In, Ctx, Map, Out> IterativeComposer<In, Ctx, Map, Out> {
    pub const fn new(
        set_output: SourceContextComposerByRef<In, Ctx, Map, Out>,
        mapper: ComposerByRef<Ctx, Map>,
    ) -> Self {
        Self { set_output, mapper }
    }
}
impl<In, Ctx, Map, Out> SourceComposable for IterativeComposer<In, Ctx, Map, Out> {
    type Source = In;
    type Output = Out;
    fn compose(&self, source: &Self::Source) -> Self::Output {
        (self.set_output)(source, self.mapper)
    }
}

impl<Link, In, Ctx, Map, Out> Linkable<Link> for IterativeComposer<In, Ctx, Map, Out>
    where Link: SharedAccess {
    fn link(&mut self, _parent: &Link) {}
}

impl<SPEC> IterativeComposer<OwnedArgComposers<SPEC, OwnerAspect<SPEC>>, FieldComposer<SPEC>, ArgKind<SPEC>, OwnerAspectSequence<SPEC, CommaPunctuatedArgKinds<SPEC>>>
where SPEC: Specification {
    pub const fn aspect_fields(mapper: ComposerByRef<FieldComposer<SPEC>, ArgKind<SPEC>>) -> Self {
        Self::new(constants::args_composer_iterator_root(), mapper)
    }
}

pub type FFIInterfaceMethodIterator<SPEC, SEP> = IterativeComposer<
    AspectArgComposers<SPEC>,
    FieldTypeLocalContext<SPEC>,
    <SPEC as Specification>::Expr,
    OwnerAspectSequence<SPEC, PunctuatedArgKinds<SPEC, SEP>>>;
impl<SPEC, Iter> IterativeComposer<
    AspectArgComposers<SPEC>,
    FieldTypeLocalContext<SPEC>,
    SPEC::Expr,
    OwnerAspectSequence<SPEC, Iter>>
where SPEC: Specification,
      Iter: FromIterator<ArgKind<SPEC>> {
    pub const fn aspect_sequence_expr<C, SEP>() -> Self
    where C: FFIInterfaceMethodSpec<SPEC, SEP> + ?Sized,
          SEP: ToTokens + Default {
        Self::new(
            |(aspect, arg_composers), expr_composer|
                (aspect.clone(), constants::arg_conversion_expressions_iterator((arg_composers, expr_composer), C::RESOLVER)),
            C::EXPR)
    }
}
