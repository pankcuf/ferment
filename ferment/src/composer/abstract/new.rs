use crate::composable::{FieldTypeComposition, FnSignatureContext};
use crate::composer::{Composer, ConstructorFieldsContext, FieldTypeLocalContext, FieldTypesContext, FunctionContext, Linkable, LocalConversionContext, OwnedItemPresentablePair, OwnedStatement, OwnerAspectWithCommaPunctuatedItems};
use crate::presentable::{BindingPresentableContext, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::shared::SharedAccess;


// pub type ComposerPresenter<Context, Presentation> = dyn Fn(Context) -> Presentation;
// pub type ComposerPresenterByRef<Context, Presentation> = dyn Fn(&Context) -> Presentation;
// pub type SharedComposer<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, Context>;

// pub type ComposerPresenter<Context, Presentation> = dyn Fn(Context) -> Presentation;
// pub type ComposerPresenterByRef<Context, Presentation> = dyn Fn(&Context) -> Presentation;
// pub type SharedComposer<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, Context>;

pub type FieldsSequenceMixer<Parent, Context, Statement> = SequenceMixer<
    Parent,
    Context,
    FieldTypeLocalContext,
    Expression,
    Statement,
    SequenceOutput,
    SequenceOutput,
    SequenceOutput>;

pub type FFIConversionMixer<Parent> = FieldsSequenceMixer<
    Parent,
    LocalConversionContext,
    OwnerAspectWithCommaPunctuatedItems
>;
pub type DropSequenceMixer<Parent> = FieldsSequenceMixer<
    Parent,
    FieldTypesContext,
    OwnedStatement
>;
pub type FieldsOwnedComposer<Parent> = SequenceComposer<
    Parent,
    LocalConversionContext,
    FieldTypeComposition,
    OwnedItemPresentableContext,
    OwnerAspectWithCommaPunctuatedItems,
    SequenceOutput
>;
pub type CtorSequenceComposer<Parent, S, SP, I> = SequenceComposer<
    Parent,
    ConstructorFieldsContext,
    FieldTypeComposition,
    OwnedItemPresentablePair,
    FunctionContext,
    BindingPresentableContext<S, SP, I>,
>;
#[allow(unused)]
pub type FnSignatureSequenceComposer<Parent, S, SP, I> = SequenceComposer<
    Parent,
    FnSignatureContext,
    FieldTypeComposition,
    OwnedItemPresentablePair,
    FunctionContext,
    BindingPresentableContext<S, SP, I>,
>;
pub type FieldsSequenceComposer<Parent, OwnerAspect, B, C, Presentable> = SequenceComposer<
    Parent,
    OwnerAspect,
    FieldTypeComposition,
    B,
    C,
    Presentable>;

pub struct IterativeComposer<In, Ctx, Map, Out>  {
    set_output: Box<dyn Fn((&In, &dyn Fn(&Ctx) -> Map)) -> Out>,
    mapper: Box<dyn Fn(&Ctx) -> Map>,
}

impl<In, Ctx, Map, Out> IterativeComposer<In, Ctx, Map, Out>
    where {
    pub const fn new(
        set_output: Box<dyn Fn((&In, &dyn Fn(&Ctx) -> Map)) -> Out + 'static>,
        mapper: Box<dyn Fn(&Ctx) -> Map + 'static>
    ) -> Self {
        Self { set_output, mapper }
    }
}

impl<'a, In, Ctx, Map, Out> Composer<'a> for IterativeComposer<In, Ctx, Map, Out> {
    type Source = In;
    type Result = Out;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        (self.set_output)((source, &self.mapper))

    }
}
impl<Parent, In, Ctx, Map, Out> Linkable<Parent> for IterativeComposer<In, Ctx, Map, Out>
    where
        Parent: SharedAccess {
    fn link(&mut self, _parent: &Parent) {}
}


pub struct ContextComposer<Context, Result, Parent, MapFn, OutFn>
    where
        Parent: SharedAccess,
        MapFn: Fn(&Parent::ImmutableAccess) -> Context,
        OutFn: Fn(Context) -> Result {
    parent: Option<Parent>,
    get_context: MapFn,
    set_output: OutFn,
}

impl<Context, Result, Parent, MapFn, OutFn> ContextComposer<Context, Result, Parent, MapFn, OutFn>
    where
        Parent: SharedAccess,
        MapFn: Fn(&Parent::ImmutableAccess) -> Context,
        OutFn: Fn(Context) -> Result {
    pub const fn new(
        set_output: OutFn,
        get_context: MapFn
    ) -> Self {
        Self { parent: None, set_output, get_context }
    }
}

impl<Context, Result, Parent, MapFn, OutFn> Linkable<Parent> for ContextComposer<Context, Result, Parent, MapFn, OutFn>
    where
        Parent: SharedAccess,
        MapFn: Fn(&Parent::ImmutableAccess) -> Context,
        OutFn: Fn(Context) -> Result {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Context, Result, Parent, MapFn, OutFn> Composer<'a> for ContextComposer<Context, Result, Parent, MapFn, OutFn>
    where
        Parent: SharedAccess,
        MapFn: Fn(&Parent::ImmutableAccess) -> Context,
        OutFn: Fn(Context) -> Result {
    type Source = ();
    type Result = Result;
    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.set_output)(
            self.parent.as_ref()
                .expect("no parent")
                .access(&self.get_context))
    }
}

pub struct SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where Parent: SharedAccess {
    parent: Option<Parent>,
    set_output: Box<dyn Fn(SeqOut) -> Out>,
    get_context: Box<dyn Fn(&Parent::ImmutableAccess) -> ParentCtx>,
    iterator: IterativeComposer<ParentCtx, SeqCtx, SeqMap, SeqOut>,
}

impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess,
{
    pub const fn with_iterator_setup(
        set_output: Box<dyn Fn(SeqOut) -> Out + 'static>,
        get_context: Box<dyn Fn(&Parent::ImmutableAccess) -> ParentCtx + 'static>,
        iterator_post_processor: Box<dyn Fn((&ParentCtx, &dyn Fn(&SeqCtx) -> SeqMap)) -> SeqOut + 'static>,
        iterator_item: Box<dyn Fn(&SeqCtx) -> SeqMap + 'static>,
    ) -> Self {
        Self {
            set_output,
            get_context,
            parent: None,
            iterator: IterativeComposer::new(
                iterator_post_processor,
                iterator_item,
            ),
        }
    }
    pub const fn new(
        set_output: Box<dyn Fn(SeqOut) -> Out + 'static>,
        get_context: Box<dyn Fn(&Parent::ImmutableAccess) -> ParentCtx + 'static>,
        iterator: IterativeComposer<ParentCtx, SeqCtx, SeqMap, SeqOut>,
    ) -> Self {
        Self {
            set_output,
            get_context,
            iterator,
            parent: None,
        }
    }
}

impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> Linkable<Parent> for SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> Composer<'a> for SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess {
    type Source = ();
    type Result = Out;
    fn compose(&self, _: &Self::Source) -> Self::Result {
        (self.set_output)(
            self.iterator.compose(&self.parent
                .as_ref()
                .expect("no parent")
                .access(&self.get_context)))
    }
}

pub struct SequenceMixer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where
        Parent: SharedAccess,
        SeqMixOut: ScopeContextPresentable,
        Out: ScopeContextPresentable {
    parent: Option<Parent>,
    post_processor: Box<dyn Fn(&(MixCtx, SeqMixOut)) -> Out + 'static>,
    context: Box<dyn Fn(&Parent::ImmutableAccess) -> MixCtx + 'static>,
    sequence: SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut>,
}



impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> Linkable<Parent>
for SequenceMixer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where
        Parent: SharedAccess,
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
        SeqMixOut: ScopeContextPresentable,
        Out: ScopeContextPresentable {
    type Source = ();
    type Result = Out;
    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.post_processor)(&(
            self.parent.as_ref().expect("no parent").access(&self.context),
            self.sequence.compose(&())))
    }
}
impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> SequenceMixer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out>
    where
        Parent: SharedAccess,
        SeqMixOut: ScopeContextPresentable,
        Out: ScopeContextPresentable,
{
    pub const fn new(
        post_processor: Box<dyn Fn(&(MixCtx, SeqMixOut)) -> Out + 'static>,
        context: Box<dyn Fn(&Parent::ImmutableAccess) -> MixCtx + 'static>,
        seq_root: Box<dyn Fn(SeqOut) -> SeqMixOut + 'static>,
        seq_context: Box<dyn Fn(&Parent::ImmutableAccess) -> ParentCtx + 'static>,
        seq_iterator_item: Box<dyn Fn(&SeqCtx) -> SeqMap + 'static>,
        seq_iterator_root: Box<dyn Fn((&ParentCtx, &dyn Fn(&SeqCtx) -> SeqMap)) -> SeqOut + 'static>,
    ) -> Self {
        Self {
            parent: None,
            post_processor,
            context,
            sequence: SequenceComposer::with_iterator_setup(
                seq_root,
                seq_context,
                seq_iterator_root,
                seq_iterator_item,
            ),
        }
    }
    pub const fn with_sequence(
        post_processor: Box<dyn Fn(&(MixCtx, SeqMixOut)) -> Out + 'static>,
        context: Box<dyn Fn(&Parent::ImmutableAccess) -> MixCtx + 'static>,
        sequence: SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut>,
    ) -> Self {
        Self {
            parent: None,
            post_processor,
            context,
            sequence,
        }
    }
}