use crate::composer::{SourceComposable, Composer, ComposerByRef, Linkable};
use crate::shared::SharedAccess;

pub struct IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    set_output: Composer<(In, ComposerByRef<Ctx, Map>), Out>,
    mapper: ComposerByRef<Ctx, Map>
}

impl<In, Ctx, Map, Out> IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    pub const fn new(
        set_output: Composer<(In, ComposerByRef<Ctx, Map>), Out>,
        mapper: ComposerByRef<Ctx, Map>,
    ) -> Self {
        Self { set_output, mapper }
    }
}
impl<In, Ctx, Map, Out> SourceComposable for IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    type Source = In;
    type Output = Out;
    fn compose(&self, source: &Self::Source) -> Self::Output {
        // TODO: avoid cloning
        (self.set_output)((source.clone(), self.mapper))
    }
}

impl<Link, In, Ctx, Map, Out> Linkable<Link> for IterativeComposer<In, Ctx, Map, Out>
    where Link: SharedAccess, In: Clone {
    fn link(&mut self, _parent: &Link) {}
}

