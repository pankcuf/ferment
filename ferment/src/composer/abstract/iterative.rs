use crate::composer::{ComposerPresenter, ComposerPresenterByRef};
use crate::composer::r#abstract::{Composer, LinkedComposer, ParentLinker};
use crate::shared::SharedAccess;

pub struct IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    set_output: ComposerPresenter<(In, ComposerPresenterByRef<Ctx, Map>), Out>,
    mapper: ComposerPresenterByRef<Ctx, Map>
}

impl<In, Ctx, Map, Out> IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    pub const fn new(
        set_output: ComposerPresenter<(In, ComposerPresenterByRef<Ctx, Map>), Out>,
        mapper: ComposerPresenterByRef<Ctx, Map>,
    ) -> Self {
        Self { set_output, mapper }
    }
}
impl<'a, In, Ctx, Map, Out> Composer<'a> for IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    type Source = In;
    type Result = Out;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        // TODO: avoid cloning
        (self.set_output)((source.clone(), self.mapper))
    }
}
impl<'a, Parent, In, Ctx, Map, Out> LinkedComposer<'a, Parent> for IterativeComposer<In, Ctx, Map, Out>
    where Parent: SharedAccess, In: Clone {}

impl<Parent, In, Ctx, Map, Out> ParentLinker<Parent>
for IterativeComposer<In, Ctx, Map, Out>
    where
        Parent: SharedAccess,
        In: Clone {
    fn link(&mut self, _parent: &Parent) {}
}
