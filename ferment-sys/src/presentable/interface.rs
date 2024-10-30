use std::fmt::Debug;
use quote::ToTokens;
use syn::token::{Comma, Semi};
use ferment_macro::Display;
use crate::composer::{arg_conversion_expressions_iterator, AspectPresentableArguments, AspectSharedComposerLink, AttrComposable, ComposerLink, AspectArgComposers, DropSequenceMixer, FFIConversionsMixer, FFIInterfaceMethodSpec, FieldTypeLocalContext, FieldsContext, FieldsSequenceMixer, GenericsComposable, Linkable, LinkedContextComposer, NameKindComposable, RootSequenceComposer, SequenceSharedComposerLink, SourceComposable, SourceContextComposerByRef, TypeAspect};
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, Expression, ScopeContextPresentable, SeqKind};
use crate::shared::SharedAccess;

pub const fn seq_iterator_root<C, LANG, SPEC, SEP>()
    -> SourceContextComposerByRef<AspectArgComposers<LANG, SPEC>, FieldTypeLocalContext<LANG, SPEC>, Expression<LANG, SPEC>, AspectPresentableArguments<SEP, LANG, SPEC>>
where C: FFIInterfaceMethodSpec<LANG, SPEC, SEP>,
      LANG: LangFermentable,
      SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
      SPEC::Expr: ScopeContextPresentable,
      Aspect<SPEC::TYC>: ScopeContextPresentable,
      SeqKind<LANG, SPEC>: ScopeContextPresentable,
      ArgKind<LANG, SPEC>: ScopeContextPresentable,
      SEP: ToTokens + Default {
    |(aspect, field_composers), expr_composer|
        (aspect.clone(), arg_conversion_expressions_iterator((field_composers, expr_composer), C::RESOLVER))
}
#[derive(Display)]
pub enum InterfaceKind<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    From(FFIConversionsMixer<Link, LANG, SPEC>),
    To(FFIConversionsMixer<Link, LANG, SPEC>),
    Destroy(LinkedContextComposer<Link, SeqKind<LANG, SPEC>, SeqKind<LANG, SPEC>>),
    Drop(DropSequenceMixer<Link, LANG, SPEC>),
}

impl<C, LANG, SPEC> InterfaceKind<ComposerLink<C>, LANG, SPEC>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Comma> + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    pub fn from(
        root: RootSequenceComposer<LANG, SPEC>,
        context: SequenceSharedComposerLink<C, LANG, SPEC>,
        aspect: AspectSharedComposerLink<C, LANG, SPEC>
    ) -> Self {
        Self::From(FieldsSequenceMixer::new(
            root,
            context,
            C::SEQ,
            aspect,
            C::EXPR,
            seq_iterator_root::<C, LANG, SPEC, Comma>()
        ))
    }
}
impl<C, LANG, SPEC> InterfaceKind<ComposerLink<C>, LANG, SPEC>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Comma>
            + FieldsContext<LANG, SPEC>
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + NameKindComposable
            + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    pub fn to(
        root: RootSequenceComposer<LANG, SPEC>,
        context: SequenceSharedComposerLink<C, LANG, SPEC>
    ) -> Self {
        Self::To(FieldsSequenceMixer::new(
            root,
            context,
            C::SEQ,
            Aspect::ffi,
            C::EXPR,
            seq_iterator_root::<C, LANG, SPEC, Comma>()
        ))
    }
}
impl<C, LANG, SPEC> InterfaceKind<ComposerLink<C>, LANG, SPEC>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Comma>
            + FieldsContext<LANG, SPEC>
            + TypeAspect<SPEC::TYC>
            + GenericsComposable<SPEC::Gen>
            + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    pub fn destroy(context: SequenceSharedComposerLink<C, LANG, SPEC>) -> Self {
        Self::Destroy(LinkedContextComposer::new(
            SeqKind::unboxed_root,
            context,
        ))
    }
}
impl<C, LANG, SPEC> InterfaceKind<ComposerLink<C>, LANG, SPEC>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Semi>
            + FieldsContext<LANG, SPEC>
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + NameKindComposable
            + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    pub fn drop(
        root: RootSequenceComposer<LANG, SPEC>,
        context: SequenceSharedComposerLink<C, LANG, SPEC>
    ) -> Self {
        Self::Drop(FieldsSequenceMixer::new(
            root,
            context,
            C::SEQ,
            Aspect::ffi,
            C::EXPR,
            seq_iterator_root::<C, LANG, SPEC, Semi>()
        ))
    }
}

impl<Link, LANG, SPEC> Linkable<Link> for InterfaceKind<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        match self {
            InterfaceKind::From(c) |
            InterfaceKind::To(c) => c.link(parent),
            InterfaceKind::Destroy(c) => c.link(parent),
            InterfaceKind::Drop(c) => c.link(parent),
        }
    }
}

impl<Link, LANG, SPEC> SourceComposable for InterfaceKind<Link, LANG, SPEC>
    where Link: SharedAccess,
      LANG: LangFermentable,
      SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
      SPEC::Expr: ScopeContextPresentable,
      Aspect<SPEC::TYC>: ScopeContextPresentable,
      ArgKind<LANG, SPEC>: ScopeContextPresentable,
      SeqKind<LANG, SPEC>: ScopeContextPresentable {
    type Source = ();
    type Output = SeqKind<LANG, SPEC>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            InterfaceKind::From(c) |
            InterfaceKind::To(c) => c.compose(source),
            InterfaceKind::Destroy(c) => c.compose(source),
            InterfaceKind::Drop(c) => c.compose(source),
        }
    }
}


