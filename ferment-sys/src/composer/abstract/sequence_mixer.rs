use crate::composer::{SourceComposable, Composer, ComposerByRef, Linkable, SequenceComposer, SharedComposer};
use crate::shared::SharedAccess;

pub struct SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess,
          LinkCtx: Clone {
    parent: Option<Link>,
    post_processor: ComposerByRef<(MixCtx, SeqMixOut), Out>,
    context: SharedComposer<Link, MixCtx>,
    sequence: SequenceComposer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut>,
}
impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> Linkable<Link>
for SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess,
          LinkCtx: Clone {
    fn link(&mut self, parent: &Link) {
        self.sequence.link(parent);
        self.parent = Some(parent.clone_container());
    }
}
impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> SourceComposable
for SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess,
          LinkCtx: Clone {
    type Source = ();
    type Output = Out;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        (self.post_processor)(&(
            self.parent.as_ref().expect("no parent").access(self.context),
            self.sequence.compose(&())))
    }
}
impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> SequenceMixer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where Link: SharedAccess,
          LinkCtx: Clone {
    pub const fn new(
        post_processor: ComposerByRef<(MixCtx, SeqMixOut), Out>,
        context: SharedComposer<Link, MixCtx>,
        seq_root: Composer<SeqOut, SeqMixOut>,
        seq_context: SharedComposer<Link, LinkCtx>,
        seq_iterator_item: ComposerByRef<SeqCtx, SeqMap>,
        seq_iterator_root: Composer<(LinkCtx, ComposerByRef<SeqCtx, SeqMap>), SeqOut>,
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
    // pub const fn with_sequence(
    //     post_processor: ComposerByRef<(MixCtx, SeqMixOut), Out>,
    //     context: SharedComposer<Parent, MixCtx>,
    //     sequence: SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut>,
    // ) -> Self {
    //     Self {
    //         parent: None,
    //         post_processor,
    //         context,
    //         sequence
    //     }
    // }
}
