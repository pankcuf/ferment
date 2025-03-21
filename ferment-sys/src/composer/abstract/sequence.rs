use crate::composer::{SourceComposable, Composer, ComposerByRef, IterativeComposer, Linkable, SharedComposer, SourceContextComposerByRef};
use crate::shared::SharedAccess;
//pub const fn mix<A, B, C, F1: Fn(A, B) -> C, F2: Fn(A, B) -> C>() -> F1 { |context, presenter: F1<A, C>| presenter(context) }

pub struct SequenceComposer<L, LC, SeqCtx, SeqMap, SeqOut, Out>
    where L: SharedAccess {
    parent: Option<L>,
    set_output: Composer<SeqOut, Out>,
    get_context: SharedComposer<L, LC>,
    iterator: IterativeComposer<LC, SeqCtx, SeqMap, SeqOut>,
}

impl<L, LC, SeqCtx, SeqMap, SeqOut, Out> SequenceComposer<L, LC, SeqCtx, SeqMap, SeqOut, Out>
    where L: SharedAccess {
    #[allow(unused)]
    pub const fn with_iterator_setup(
        set_output: Composer<SeqOut, Out>,
        get_context: SharedComposer<L, LC>,
        iterator_post_processor: SourceContextComposerByRef<LC, SeqCtx, SeqMap, SeqOut>,
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
        get_context: SharedComposer<L, LC>,
        iterator: IterativeComposer<LC, SeqCtx, SeqMap, SeqOut>,
    ) -> Self {
        Self { set_output, get_context, iterator, parent: None }
    }
}

impl<L, LC, SeqCtx, SeqMap, SeqOut, Out> Linkable<L> for SequenceComposer<L, LC, SeqCtx, SeqMap, SeqOut, Out>
    where L: SharedAccess {
    fn link(&mut self, parent: &L) {
        self.parent = Some(parent.clone_container());
    }
}

impl<L, LC, SeqCtx, SeqMap, SeqOut, Out> SourceComposable for SequenceComposer<L, LC, SeqCtx, SeqMap, SeqOut, Out>
    where L: SharedAccess {
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

// impl<LANG, SPEC, C, SEP> SequenceComposer<
//     ComposerLink<C>,
//     AspectArgComposers<LANG, SPEC>,
//     FieldTypeLocalContext<LANG, SPEC>,
//     SPEC::Expr,
//     AspectPresentableArguments<LANG, SPEC, SEP>,
//     SeqKind<LANG, SPEC>>
//     where LANG: LangFermentable,
//           SPEC: Specification<LANG>,
//           C: FFIInterfaceMethodSpec<LANG, SPEC, SEP> + 'static,
//           SEP: ToTokens + Default {
//     pub const fn aspect_seq(aspect: ComposerByRef<ComposerLinkRef<C>, AspectArgComposers<LANG, SPEC>>) -> Self {
//         SequenceComposer::new(C::SEQ, aspect, IterativeComposer::aspect_sequence_expr::<C, SEP>())
//     }
// }