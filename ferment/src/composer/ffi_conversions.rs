use crate::composer::{DropSequenceMixer, FFIConversionMixer, OwnerIteratorPostProcessingComposer};
use crate::composer::r#abstract::{Composer, Linkable};
use crate::presentable::SequenceOutput;
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIAspect {
    From,
    To,
    Destroy,
    Drop,
}

pub struct FFIComposer<Parent> where Parent: SharedAccess {
    pub parent: Option<Parent>,
    pub from_conversion_composer: FFIConversionMixer<Parent>,
    pub to_conversion_composer: FFIConversionMixer<Parent>,
    pub drop_composer: DropSequenceMixer<Parent>,
    pub destroy_composer: OwnerIteratorPostProcessingComposer<Parent>,
}

impl<Parent> Linkable<Parent> for FFIComposer<Parent> where Parent: SharedAccess {
    fn link(&mut self, parent: &Parent) {
        self.from_conversion_composer.link(parent);
        self.to_conversion_composer.link(parent);
        self.destroy_composer.link(parent);
        self.drop_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent> FFIComposer<Parent> where Parent: SharedAccess {
    pub const fn new(
        from_conversion_composer: FFIConversionMixer<Parent>,
        to_conversion_composer: FFIConversionMixer<Parent>,
        destroy_composer: OwnerIteratorPostProcessingComposer<Parent>,
        drop_composer: DropSequenceMixer<Parent>,
    ) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, parent: None }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect) -> SequenceOutput {
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
