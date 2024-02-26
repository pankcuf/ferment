use crate::composer::{Composer, ComposerPresenter, ComposerPresenterByRef, SharedComposer};
use crate::shared::{HasParent, SharedAccess};

pub struct IteratorComposer<IN, CTX, MAP, OUT> where IN: Clone {
    root_composer: ComposerPresenter<(IN, ComposerPresenterByRef<CTX, MAP>), OUT>,
    item_composer: ComposerPresenterByRef<CTX, MAP>
}

impl<IN, CTX, MAP, OUT> IteratorComposer<IN, CTX, MAP, OUT>
    where IN: Clone {
    pub const fn new(
        root_composer: ComposerPresenter<(IN, ComposerPresenterByRef<CTX, MAP>), OUT>,
        item_composer: ComposerPresenterByRef<CTX, MAP>,
    ) -> Self {
        Self { root_composer, item_composer }
    }
}
impl<Parent, IN, CTX, MAP, OUT> Composer<Parent> for IteratorComposer<IN, CTX, MAP, OUT>
    where Parent: SharedAccess,
          IN: Clone {
    type Source = IN;
    type Result = OUT;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        // TODO: avoid cloning
        (self.root_composer)((source.clone(), self.item_composer))
    }
}


impl<Parent, IN, CTX, MAP, OUT> HasParent<Parent> for IteratorComposer<IN, CTX, MAP, OUT>
    where Parent: SharedAccess,
          IN: Clone {
    fn set_parent(&mut self, _parent: &Parent) {}
}

pub struct OwnedComposer<Parent, CTX, L1CTX, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    parent: Option<Parent>,
    root_composer: ComposerPresenter<L1OUT, OUT>,
    context_composer: SharedComposer<Parent, CTX>,
    local_context_composer: IteratorComposer<CTX, L1CTX, L1MAP, L1OUT>,
}

impl<Parent, CTX, L1CTX, L1MAP, L1OUT, OUT> OwnedComposer<Parent, CTX, L1CTX, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    pub const fn new(
        root_composer: ComposerPresenter<L1OUT, OUT>,
        context_composer: SharedComposer<Parent, CTX>,
        iterator_root_composer: ComposerPresenter<(CTX, ComposerPresenterByRef<L1CTX, L1MAP>), L1OUT>,
        iterator_item_composer: ComposerPresenterByRef<L1CTX, L1MAP>,
    ) -> Self {
        Self {
            parent: None,
            root_composer,
            context_composer,
            local_context_composer: IteratorComposer::new(
                iterator_root_composer,
                iterator_item_composer
            ),
        }
    }
}

impl<Parent, CTX, L1, L1MAP, L1OUT, OUT> HasParent<Parent> for OwnedComposer<Parent, CTX, L1, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    fn set_parent(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent, CTX, L1, L1MAP, L1OUT, OUT> Composer<Parent> for OwnedComposer<Parent, CTX, L1, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    type Source = ();
    type Result = OUT;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = <IteratorComposer<CTX, L1, L1MAP, L1OUT> as Composer<Parent>>::compose(&self.local_context_composer, &context);
        let out = (self.root_composer)(local_context);
        out
    }
}
