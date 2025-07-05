use std::fmt::Debug;
use crate::composer::ComposerLink;
use crate::composer::r#abstract::{SourceComposable, Linkable};
use crate::lang::Specification;
use crate::presentable::{SeqKind, InterfaceKind};
use crate::shared::SharedAccess;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FFIAspect {
    From,
    To,
    Drop,
}

pub type FFIComposerLink<SPEC, T> = FFIComposer<SPEC, ComposerLink<T>>;
pub type MaybeFFIComposerLink<SPEC, T> = Option<FFIComposerLink<SPEC, T>>;
pub struct FFIComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    pub parent: Option<Link>,
    pub from_conversion_composer: InterfaceKind<SPEC, Link>,
    pub to_conversion_composer: InterfaceKind<SPEC, Link>,
    pub drop_composer: InterfaceKind<SPEC, Link>,
}
impl<SPEC, Link> FFIComposer<SPEC, Link>
    where SPEC: Specification,
          Link: SharedAccess {
    pub const fn new(
        from_conversion_composer: InterfaceKind<SPEC, Link>,
        to_conversion_composer: InterfaceKind<SPEC, Link>,
        drop_composer: InterfaceKind<SPEC, Link>,
    ) -> Self {
        Self { from_conversion_composer, to_conversion_composer, drop_composer, parent: None }
    }
}


impl<SPEC, Link> Linkable<Link> for FFIComposer<SPEC, Link>
    where SPEC: Specification,
          Link: SharedAccess {
    fn link(&mut self, parent: &Link) {
        self.from_conversion_composer.link(parent);
        self.to_conversion_composer.link(parent);
        self.drop_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<SPEC, Link> SourceComposable for FFIComposer<SPEC, Link>
    where SPEC: Specification,
          Link: SharedAccess {
    type Source = FFIAspect;
    type Output = SeqKind<SPEC>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match source {
            FFIAspect::From =>
                self.from_conversion_composer.compose(&()),
            FFIAspect::To =>
                self.to_conversion_composer.compose(&()),
            FFIAspect::Drop =>
                self.drop_composer.compose(&())
        }
    }
}
