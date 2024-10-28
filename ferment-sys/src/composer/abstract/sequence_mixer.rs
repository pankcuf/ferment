use crate::composer::{SourceComposable, Composer, ComposerByRef, Linkable, SequenceComposer, SharedComposer, SourceContextComposerByRef, SourceComposerByRef};
use crate::shared::SharedAccess;

pub struct SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess {
    parent: Option<Link>,
    post_processor: SourceComposerByRef<MixCtx, SeqMixOut, Out>,
    context: SharedComposer<Link, MixCtx>,
    sequence: SequenceComposer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut>,
}
impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> Linkable<Link>
for SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess {
    fn link(&mut self, parent: &Link) {
        self.sequence.link(parent);
        self.parent = Some(parent.clone_container());
    }
}
impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> SourceComposable
for SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess {
    type Source = ();
    type Output = Out;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        let source = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        let sequence_composition = self.sequence.compose(&());
        (self.post_processor)(&source, sequence_composition)
    }
}
impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess {
    pub const fn new(
        post_processor: SourceComposerByRef<MixCtx, SeqMixOut, Out>,
        context: SharedComposer<Link, MixCtx>,
        seq_root: Composer<SeqOut, SeqMixOut>,
        seq_context: SharedComposer<Link, LinkCtx>,
        seq_iterator_item: ComposerByRef<SeqCtx, SeqMap>,
        seq_iterator_root: SourceContextComposerByRef<LinkCtx, SeqCtx, SeqMap, SeqOut>,
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

}

