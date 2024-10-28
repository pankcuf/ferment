use crate::composer::{SourceComposable, Composer, ComposerByRef, IterativeComposer, Linkable, SharedComposer, SourceContextComposerByRef};
use crate::shared::SharedAccess;
//pub const fn mix<A, B, C, F1: Fn(A, B) -> C, F2: Fn(A, B) -> C>() -> F1 { |context, presenter: F1<A, C>| presenter(context) }

pub struct SequenceComposer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, Out>
    where Link: SharedAccess {
    parent: Option<Link>,
    set_output: Composer<SeqOut, Out>,
    get_context: SharedComposer<Link, LinkCtx>,
    iterator: IterativeComposer<LinkCtx, SeqCtx, SeqMap, SeqOut>,
}

impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, Out> SequenceComposer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Link: SharedAccess {
    #[allow(unused)]
    pub const fn with_iterator_setup(
        set_output: Composer<SeqOut, Out>,
        get_context: SharedComposer<Link, LinkCtx>,
        iterator_post_processor: SourceContextComposerByRef<LinkCtx, SeqCtx, SeqMap, SeqOut>,
        iterator_item: ComposerByRef<SeqCtx, SeqMap>,
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
        set_output: Composer<SeqOut, Out>,
        get_context: SharedComposer<Link, LinkCtx>,
        iterator: IterativeComposer<LinkCtx, SeqCtx, SeqMap, SeqOut>,
    ) -> Self {
        Self { set_output, get_context, iterator, parent: None }
    }
}

impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, Out> Linkable<Link> for SequenceComposer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Link: SharedAccess {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, Out> SourceComposable for SequenceComposer<Link, LinkCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Link: SharedAccess {
    type Source = ();
    type Output = Out;
    fn compose(&self, _: &Self::Source) -> Self::Output {
        let source = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.get_context);
        let sequence_composition = self.iterator.compose(&source);
        (self.set_output)(sequence_composition)
    }
}
