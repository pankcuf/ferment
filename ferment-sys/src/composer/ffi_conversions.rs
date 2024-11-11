use std::fmt::Debug;
use crate::composer::ComposerLink;
use crate::composer::r#abstract::{SourceComposable, Linkable};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{SeqKind, InterfaceKind};
use crate::shared::SharedAccess;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FFIAspect {
    From,
    To,
    // Destroy,
    Drop,
}

pub type FFIComposerLink<LANG, SPEC, T> = FFIComposer<LANG, SPEC, ComposerLink<T>>;
pub type MaybeFFIComposerLink<LANG, SPEC, T> = Option<FFIComposerLink<LANG, SPEC, T>>;
pub struct FFIComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub parent: Option<Link>,
    // pub ctor_composer: InterfaceKind<LANG, SPEC, Link>,
    pub from_conversion_composer: InterfaceKind<LANG, SPEC, Link>,
    pub to_conversion_composer: InterfaceKind<LANG, SPEC, Link>,
    pub drop_composer: InterfaceKind<LANG, SPEC, Link>,
}
impl<LANG, SPEC, Link> FFIComposer<LANG, SPEC, Link>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Link: SharedAccess {
    pub const fn new(
        // ctor_composer: InterfaceKind<LANG, SPEC, L>,
        from_conversion_composer: InterfaceKind<LANG, SPEC, Link>,
        to_conversion_composer: InterfaceKind<LANG, SPEC, Link>,
        drop_composer: InterfaceKind<LANG, SPEC, Link>,
    ) -> Self {
        Self { from_conversion_composer, to_conversion_composer, drop_composer, parent: None }
    }
}


impl<LANG, SPEC, Link> Linkable<Link> for FFIComposer<LANG, SPEC, Link>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Link: SharedAccess {
    fn link(&mut self, parent: &Link) {
        // self.ctor_composer.link(parent);
        self.from_conversion_composer.link(parent);
        self.to_conversion_composer.link(parent);
        self.drop_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<LANG, SPEC, Link> SourceComposable for FFIComposer<LANG, SPEC, Link>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Link: SharedAccess {
    type Source = FFIAspect;
    type Output = SeqKind<LANG, SPEC>;

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
