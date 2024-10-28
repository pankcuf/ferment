use crate::composer::{SourceComposable, ComposerByRef, Linkable, SourceContextComposerByRef};
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

