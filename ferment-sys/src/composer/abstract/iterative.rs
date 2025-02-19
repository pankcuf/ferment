use quote::ToTokens;
use crate::composable::FieldComposer;
use crate::composer::{SourceComposable, ComposerByRef, Linkable, SourceContextComposerByRef, constants, OwnedArgComposers, OwnerAspect, OwnerAspectSequence, FieldTypeLocalContext, FFIInterfaceMethodSpec, CommaPunctuatedArgKinds, AspectArgComposers, PunctuatedArgKinds};
use crate::lang::{LangFermentable, Specification};
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

impl<LANG, SPEC> IterativeComposer<OwnedArgComposers<LANG, SPEC, OwnerAspect<LANG, SPEC>>, FieldComposer<LANG, SPEC>, ArgKind<LANG, SPEC>, OwnerAspectSequence<LANG, SPEC, CommaPunctuatedArgKinds<LANG, SPEC>>>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub const fn aspect_fields(mapper: ComposerByRef<FieldComposer<LANG, SPEC>, ArgKind<LANG, SPEC>>) -> Self {
        Self::new(constants::args_composer_iterator_root(), mapper)
    }
}


// impl<LANG, SPEC, Iter> IterativeComposer<
//     AspectArgComposers<LANG, SPEC>,
//     FieldTypeLocalContext<LANG, SPEC>,
//     SPEC::Expr,
//     OwnerAspectSequence<LANG, SPEC, Iter>>
// where LANG: LangFermentable,
//       SPEC: Specification<LANG>,
//       Iter: FromIterator<ArgKind<LANG, SPEC>>,
// {
//     pub const fn aspect_sequence<C, SEP>(
//         mapper: ComposerByRef<FieldTypeLocalContext<LANG, SPEC>, SPEC::Expr>
//     ) -> Self
//         where C: FFIInterfaceMethodSpec<LANG, SPEC, SEP>,
//               SEP: ToTokens + Default {
//         Self::new(
//             |(aspect, arg_composers), expr_composer|
//                 (aspect.clone(), constants::arg_conversion_expressions_iterator((arg_composers, expr_composer), C::RESOLVER)),
//             mapper)
//     }
// }

pub type FFIInterfaceMethodIterator<LANG, SPEC, SEP> = IterativeComposer<
    AspectArgComposers<LANG, SPEC>,
    FieldTypeLocalContext<LANG, SPEC>,
    <SPEC as Specification<LANG>>::Expr,
    OwnerAspectSequence<LANG, SPEC, PunctuatedArgKinds<LANG, SPEC, SEP>>>;
impl<LANG, SPEC, Iter> IterativeComposer<
    AspectArgComposers<LANG, SPEC>,
    FieldTypeLocalContext<LANG, SPEC>,
    SPEC::Expr,
    OwnerAspectSequence<LANG, SPEC, Iter>>
where LANG: LangFermentable,
      SPEC: Specification<LANG>,
      Iter: FromIterator<ArgKind<LANG, SPEC>>,
{
    pub const fn aspect_sequence_expr<C, SEP>() -> Self
        where C: FFIInterfaceMethodSpec<LANG, SPEC, SEP> + ?Sized,
              SEP: ToTokens + Default {
        Self::new(
            |(aspect, arg_composers), expr_composer|
                (aspect.clone(), constants::arg_conversion_expressions_iterator((arg_composers, expr_composer), C::RESOLVER)),
            C::EXPR)
    }
}
