use crate::composer::{Composer, ComposerPresenter, ComposerPresenterByRef, Linkable, SequenceComposer, SharedComposer};
use crate::presentable::ScopeContextPresentable;
use crate::shared::SharedAccess;

pub struct SequenceMixer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone,
        SeqMixOut: ScopeContextPresentable,
        Out: ScopeContextPresentable {
    parent: Option<Parent>,
    post_processor: ComposerPresenterByRef<(MixCtx, SeqMixOut), Out>,
    context: SharedComposer<Parent, MixCtx>,
    sequence: SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut>,
}
impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> Linkable<Parent>
for SequenceMixer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone,
        SeqMixOut: ScopeContextPresentable,
        Out: ScopeContextPresentable {
    fn link(&mut self, parent: &Parent) {
        self.sequence.link(parent);
        self.parent = Some(parent.clone_container());
    }
}
impl<'a, Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> Composer<'a>
for SequenceMixer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone,
        SeqMixOut: ScopeContextPresentable,
        Out: ScopeContextPresentable {
    type Source = ();
    type Result = Out;
    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.post_processor)(&(
            self.parent.as_ref().expect("no parent").access(self.context),
            self.sequence.compose(&())))
    }
}
impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> SequenceMixer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone,
        SeqMixOut: ScopeContextPresentable,
        Out: ScopeContextPresentable {
    pub const fn new(
        post_processor: ComposerPresenterByRef<(MixCtx, SeqMixOut), Out>,
        context: SharedComposer<Parent, MixCtx>,
        seq_root: ComposerPresenter<SeqOut, SeqMixOut>,
        seq_context: SharedComposer<Parent, ParentCtx>,
        seq_iterator_item: ComposerPresenterByRef<SeqCtx, SeqMap>,
        seq_iterator_root: ComposerPresenter<(ParentCtx, ComposerPresenterByRef<SeqCtx, SeqMap>), SeqOut>,
    ) -> Self {
        Self {
            parent: None,
            post_processor,
            context,
            sequence: SequenceComposer::with_iterator_setup(
                seq_root,
                seq_context,
                seq_iterator_root,
                seq_iterator_item
            )
        }
    }
    pub const fn with_sequence(
        post_processor: ComposerPresenterByRef<(MixCtx, SeqMixOut), Out>,
        context: SharedComposer<Parent, MixCtx>,
        sequence: SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut>,
    ) -> Self {
        Self {
            parent: None,
            post_processor,
            context,
            sequence
        }
    }
}
