use std::fmt::Debug;
use syn::token::{Comma, Semi};
use ferment_macro::Display;
use crate::composer::{AspectSharedComposerLink, AttrComposable, ComposerLink, DropSequenceMixer, FFIConversionsMixer, FFIInterfaceMethodSpec, FieldsContext, GenericsComposable, InterfaceSequenceMixer, Linkable, NameKindComposable, RootSequenceComposer, SequenceSharedComposerLink, SourceComposable, TypeAspect};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, SeqKind};
use crate::shared::SharedAccess;

#[derive(Display)]
pub enum InterfaceKind<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    Ctor(FFIConversionsMixer<LANG, SPEC, Link>),
    From(FFIConversionsMixer<LANG, SPEC, Link>),
    To(FFIConversionsMixer<LANG, SPEC, Link>),
    Drop(DropSequenceMixer<LANG, SPEC, Link>),
}

impl<LANG, SPEC, C> InterfaceKind<LANG, SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Comma> + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn ctor(
        root: RootSequenceComposer<LANG, SPEC>,
        context: SequenceSharedComposerLink<LANG, SPEC, C>,
        aspect: AspectSharedComposerLink<LANG, SPEC, C>
    ) -> Self {
        Self::Ctor(InterfaceSequenceMixer::with_aspect(root, context, aspect))
    }
}
impl<LANG, SPEC, C> InterfaceKind<LANG, SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Comma> + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn from(
        root: RootSequenceComposer<LANG, SPEC>,
        context: SequenceSharedComposerLink<LANG, SPEC, C>,
        aspect: AspectSharedComposerLink<LANG, SPEC, C>
    ) -> Self {
        Self::From(InterfaceSequenceMixer::with_aspect(root, context, aspect))
    }
}
impl<LANG, SPEC, C> InterfaceKind<LANG, SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Comma>
            + FieldsContext<LANG, SPEC>
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + NameKindComposable
            + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn to(
        root: RootSequenceComposer<LANG, SPEC>,
        context: SequenceSharedComposerLink<LANG, SPEC, C>
    ) -> Self {
        Self::To(InterfaceSequenceMixer::with_aspect(root, context, Aspect::ffi))
    }
}
impl<LANG, SPEC, C> InterfaceKind<LANG, SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<LANG, SPEC, Semi>
            + FieldsContext<LANG, SPEC>
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + NameKindComposable
            + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn drop(
        root: RootSequenceComposer<LANG, SPEC>,
        context: SequenceSharedComposerLink<LANG, SPEC, C>
    ) -> Self {
        Self::Drop(InterfaceSequenceMixer::with_aspect(root, context, Aspect::ffi))
    }
}

impl<LANG, SPEC, Link> Linkable<Link> for InterfaceKind<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn link(&mut self, parent: &Link) {
        match self {
            InterfaceKind::Ctor(c) => c.link(parent),
            InterfaceKind::From(c) |
            InterfaceKind::To(c) => c.link(parent),
            InterfaceKind::Drop(c) => c.link(parent),
        }
    }
}

impl<LANG, SPEC, Link> SourceComposable for InterfaceKind<LANG, SPEC, Link>
    where Link: SharedAccess,
      LANG: LangFermentable,
      SPEC: Specification<LANG> {
    type Source = ();
    type Output = SeqKind<LANG, SPEC>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            InterfaceKind::Ctor(c) => c.compose(source),
            InterfaceKind::From(c) |
            InterfaceKind::To(c) => c.compose(source),
            InterfaceKind::Drop(c) => c.compose(source),
        }
    }
}


