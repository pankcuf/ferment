use std::fmt::Debug;
use syn::token::{Comma, Semi};
use ferment_macro::Display;
use crate::composer::{AspectSharedComposerLink, AttrComposable, ComposerLink, DropSequenceMixer, FFIConversionsMixer, FFIInterfaceMethodSpec, FieldsContext, GenericsComposable, InterfaceSequenceMixer, LifetimesComposable, Linkable, NameKindComposable, RootSequenceComposer, SequenceSharedComposerLink, SourceComposable, TypeAspect};
use crate::lang::Specification;
use crate::presentable::{Aspect, SeqKind};
use crate::shared::SharedAccess;

#[derive(Display)]
pub enum InterfaceKind<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    Ctor(FFIConversionsMixer<SPEC, Link>),
    From(FFIConversionsMixer<SPEC, Link>),
    To(FFIConversionsMixer<SPEC, Link>),
    Drop(DropSequenceMixer<SPEC, Link>),
}

impl<SPEC, C> InterfaceKind<SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<SPEC, Comma> + 'static,
          SPEC: Specification {
    pub fn ctor(
        root: RootSequenceComposer<SPEC>,
        context: SequenceSharedComposerLink<SPEC, C>,
        aspect: AspectSharedComposerLink<SPEC, C>
    ) -> Self {
        Self::Ctor(InterfaceSequenceMixer::with_aspect(root, context, aspect))
    }
}
impl<SPEC, C> InterfaceKind<SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<SPEC, Comma> + 'static,
          SPEC: Specification {
    pub fn from(
        root: RootSequenceComposer<SPEC>,
        context: SequenceSharedComposerLink<SPEC, C>,
        aspect: AspectSharedComposerLink<SPEC, C>
    ) -> Self {
        Self::From(InterfaceSequenceMixer::with_aspect(root, context, aspect))
    }
}
impl<SPEC, C> InterfaceKind<SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<SPEC, Comma>
            + FieldsContext<SPEC>
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + LifetimesComposable<SPEC::Lt>
            + GenericsComposable<SPEC::Gen>
            + NameKindComposable
            + 'static,
          SPEC: Specification {
    pub fn to(
        root: RootSequenceComposer<SPEC>,
        context: SequenceSharedComposerLink<SPEC, C>
    ) -> Self {
        Self::To(InterfaceSequenceMixer::with_aspect(root, context, Aspect::ffi))
    }
}
impl<SPEC, C> InterfaceKind<SPEC, ComposerLink<C>>
    where C: FFIInterfaceMethodSpec<SPEC, Semi>
            + FieldsContext<SPEC>
            + TypeAspect<SPEC::TYC>
            + AttrComposable<SPEC::Attr>
            + LifetimesComposable<SPEC::Lt>
            + GenericsComposable<SPEC::Gen>
            + NameKindComposable
            + 'static,
          SPEC: Specification {
    pub fn drop(
        root: RootSequenceComposer<SPEC>,
        context: SequenceSharedComposerLink<SPEC, C>
    ) -> Self {
        Self::Drop(InterfaceSequenceMixer::with_aspect(root, context, Aspect::ffi))
    }
}

impl<SPEC, Link> Linkable<Link> for InterfaceKind<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        match self {
            InterfaceKind::Ctor(c) => c.link(parent),
            InterfaceKind::From(c) |
            InterfaceKind::To(c) => c.link(parent),
            InterfaceKind::Drop(c) => c.link(parent),
        }
    }
}

impl<SPEC, Link> SourceComposable for InterfaceKind<SPEC, Link>
    where Link: SharedAccess,
      SPEC: Specification {
    type Source = ();
    type Output = SeqKind<SPEC>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            InterfaceKind::Ctor(c) => c.compose(source),
            InterfaceKind::From(c) |
            InterfaceKind::To(c) => c.compose(source),
            InterfaceKind::Drop(c) => c.compose(source),
        }
    }
}


