use crate::composer::{Composer, ComposerPresenter, ComposerPresenterByRef, IterativeComposer, Linkable, SharedComposer};
use crate::shared::SharedAccess;
//pub const fn mix<A, B, C, F1: Fn(A, B) -> C, F2: Fn(A, B) -> C>() -> F1 { |context, presenter: F1<A, C>| presenter(context) }

pub struct SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where Parent: SharedAccess, ParentCtx: Clone {
    parent: Option<Parent>,
    set_output: ComposerPresenter<SeqOut, Out>,
    get_context: SharedComposer<Parent, ParentCtx>,
    iterator: IterativeComposer<ParentCtx, SeqCtx, SeqMap, SeqOut>,
}

impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone {
    #[allow(unused)]
    pub const fn with_iterator_setup(
        set_output: ComposerPresenter<SeqOut, Out>,
        get_context: SharedComposer<Parent, ParentCtx>,
        iterator_post_processor: ComposerPresenter<(ParentCtx, ComposerPresenterByRef<SeqCtx, SeqMap>), SeqOut>,
        iterator_item: ComposerPresenterByRef<SeqCtx, SeqMap>,
    ) -> Self {
        Self {
            set_output,
            get_context,
            parent: None,
            iterator: IterativeComposer::new(
                iterator_post_processor,
                iterator_item
            )
        }
    }
    #[allow(unused)]
    pub const fn new(
        set_output: ComposerPresenter<SeqOut, Out>,
        get_context: SharedComposer<Parent, ParentCtx>,
        iterator: IterativeComposer<ParentCtx, SeqCtx, SeqMap, SeqOut>,
    ) -> Self {
        Self { set_output, get_context, iterator, parent: None }
    }
}

impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> Linkable<Parent> for SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> Composer<'a> for SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone {
    type Source = ();
    type Output = Out;
    fn compose(&self, _: &Self::Source) -> Self::Output {
        (self.set_output)(
            self.iterator.compose(&self.parent
                .as_ref()
                .expect("no parent")
                .access(self.get_context)))
    }
}
