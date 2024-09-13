use crate::composer::{DropSequenceMixer, FFIConversionMixer, OwnerIteratorPostProcessingComposer};
use crate::composer::r#abstract::{Composer, Linkable};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{ScopeContextPresentable, SequenceOutput};
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIAspect {
    From,
    To,
    Destroy,
    Drop,
}

pub struct FFIComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    pub parent: Option<Parent>,
    pub from_conversion_composer: FFIConversionMixer<Parent, LANG, SPEC, Gen>,
    pub to_conversion_composer: FFIConversionMixer<Parent, LANG, SPEC, Gen>,
    pub drop_composer: DropSequenceMixer<Parent, LANG, SPEC>,
    pub destroy_composer: OwnerIteratorPostProcessingComposer<Parent, LANG, SPEC>,
}

impl<Parent, LANG, SPEC, Gen> Linkable<Parent> for FFIComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Parent) {
        self.from_conversion_composer.link(parent);
        self.to_conversion_composer.link(parent);
        self.destroy_composer.link(parent);
        self.drop_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent, LANG, SPEC, Gen> FFIComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        from_conversion_composer: FFIConversionMixer<Parent, LANG, SPEC, Gen>,
        to_conversion_composer: FFIConversionMixer<Parent, LANG, SPEC, Gen>,
        destroy_composer: OwnerIteratorPostProcessingComposer<Parent, LANG, SPEC>,
        drop_composer: DropSequenceMixer<Parent, LANG, SPEC>,
    ) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, parent: None }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect) -> SequenceOutput<LANG, SPEC> {
        match aspect {
            FFIAspect::From =>
                self.from_conversion_composer.compose(&()),
            FFIAspect::To =>
                self.to_conversion_composer.compose(&()),
            FFIAspect::Destroy =>
                self.destroy_composer.compose(&()),
            FFIAspect::Drop =>
                self.drop_composer.compose(&())
        }
    }
}
