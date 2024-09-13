use crate::composer::{Composer, ComposerPresenter, ComposerPresenterByRef, Linkable};
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
    type Output = Out;
    fn compose(&self, source: &Self::Source) -> Self::Output {
        // TODO: avoid cloning
        (self.set_output)((source.clone(), self.mapper))
    }
}

impl<Parent, In, Ctx, Map, Out> Linkable<Parent> for IterativeComposer<In, Ctx, Map, Out>
    where Parent: SharedAccess, In: Clone {
    fn link(&mut self, _parent: &Parent) {}
}

