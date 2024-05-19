mod attrs;
mod ffi_conversions;
mod item;
mod method;
pub mod constants;
pub mod chain;
pub mod enum_composer;
pub mod composable;
mod r#type;
pub mod generic;
pub mod signature;
pub mod basic;
pub mod trait_composer;


use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Field, Type};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
pub use constants::BYPASS_FIELD_CONTEXT;
pub use enum_composer::EnumComposer;
use crate::composer::generic::GenericComposer;
use crate::composer::signature::SigComposer;
use crate::composer::trait_composer::TraitComposer;
use crate::composition::FnSignatureContext;
use crate::conversion::FieldTypeConversion;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{BindingPresentableContext, FieldTypePresentableContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::context::name::Aspect;
use crate::shared::{ParentLinker, SharedAccess};
use crate::wrapped::Void;
pub use self::attrs::{AttrsComposer};
pub use self::ffi_conversions::{FFIAspect, FFIComposer};
pub use self::item::ItemComposer;
pub use self::method::MethodComposer;

#[derive(Clone)]
pub enum ConstructorPresentableContext {
    EnumVariant(Type, TokenStream2),
    Default(Type, TokenStream2)
}
impl Debug for ConstructorPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnumVariant(ty, attrs) =>
                f.write_str(format!("EnumVariant({}, {})", ty.to_token_stream(), attrs).as_str()),
            Self::Default(ty, attrs) =>
                f.write_str(format!("Default({}, {})", ty.to_token_stream(), attrs).as_str()),
        }
    }
}
impl Display for ConstructorPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}



/// Composer Context Presenters
#[allow(unused)]
pub type ComposerPresenter<Context, Presentation> = fn(context: Context) -> Presentation;
#[allow(unused)]
pub type ComposerPresenterByRef<Context, Presentation> = fn(context: &Context) -> Presentation;
#[allow(unused)]
pub type SharedComposer<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, Context>;
#[allow(unused)]
pub type SharedComposerRef<'a, Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, &'a Context>;
#[allow(unused)]
pub type SharedComposerMut<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::MutableAccess, Context>;
pub type ParentComposer<T> = Rc<std::cell::RefCell<T>>;
pub type ParentComposerRef<'a, T> = std::cell::Ref<'a, T>;
pub type ItemParentComposer = ParentComposer<ItemComposer>;
pub type EnumParentComposer = ParentComposer<EnumComposer>;
pub type SigParentComposer = ParentComposer<SigComposer>;
pub type TraitParentComposer = ParentComposer<TraitComposer>;
pub type GenericParentComposer = ParentComposer<GenericComposer>;
pub type ItemParentComposerRef<'a> = ParentComposerRef<'a, ItemComposer>;
pub type EnumParentComposerRef<'a> = ParentComposerRef<'a, EnumComposer>;
pub type ItemComposerPresenterRef<'a, T> = ComposerPresenterByRef<ItemParentComposerRef<'a>, T>;
pub type EnumComposerPresenterRef<'a, T> = ComposerPresenterByRef<EnumParentComposerRef<'a>, T>;
pub type ItemComposerFieldTypesContextPresenter<'a> = ItemComposerPresenterRef<'a, FieldTypesContext>;
pub type NameContextComposer<Parent> = ContextComposer<Name, TokenStream2, Parent>;
pub type TypeContextComposer<Parent> = ContextComposer<Type, TokenStream2, Parent>;
pub type OwnerIteratorConversionComposer<T> = ComposerPresenter<OwnerAspectIteratorLocalContext<T>, OwnerIteratorPresentationContext>;
pub type OwnerIteratorPostProcessingComposer<T> = ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, T>;
pub type VariantComposer = ComposerPresenterByRef<VariantIteratorLocalContext, OwnerIteratorPresentationContext>;
pub type FieldsComposer = ComposerPresenterByRef<Punctuated<Field, Comma>, FieldTypesContext>;
pub type FieldTypePresentationContextPassRef = ComposerPresenterByRef<FieldTypeLocalContext, FieldTypePresentableContext>;
/// Bindings
pub type BindingComposer<T> = ComposerPresenter<T, BindingPresentation>;
pub type BindingDtorComposer = BindingComposer<DestructorContext>;
// pub type BindingSigComposer = BindingComposer<FnSignatureComposition>;
pub type FieldTypeComposer = ComposerPresenterByRef<FieldTypeConversion, FieldTypePresentableContext>;
pub type OwnedFieldTypeComposerRef = ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentableContext>;
pub type OwnerIteratorLocalContext<A, T> = (A, Punctuated<OwnedItemPresentableContext, T>);
pub type OwnerAspectIteratorLocalContext<T> = OwnerIteratorLocalContext<Aspect, T>;
pub type VariantIteratorLocalContext = OwnerAspectIteratorLocalContext<Comma>;
pub type FieldTypesContext = Punctuated<FieldTypeConversion, Comma>;
pub type OwnedStatement = Punctuated<OwnedItemPresentableContext, Semi>;
pub type FieldsOwnerContext<T> = (T, FieldTypesContext);
pub type LocalConversionContext = FieldsOwnerContext<Aspect>;
pub type ConstructorFieldsContext = FieldsOwnerContext<ConstructorPresentableContext>;
pub type BindingAccessorContext = (Type, TokenStream2, TokenStream2, TokenStream2);
pub type DestructorContext = (Type, TokenStream2);
pub type FieldTypeLocalContext = (TokenStream2, FieldTypePresentableContext);

pub type OwnedItemPresentablePair = (OwnedItemPresentableContext, OwnedItemPresentableContext);
pub type OwnedItemPresentationPair = (OwnerIteratorPresentationContext, OwnerIteratorPresentationContext);
pub type FieldsSequenceMixer<Parent, Context, Statement> = SequenceMixer<
    Parent,
    Context,
    FieldTypeLocalContext,
    FieldTypePresentableContext,
    Statement,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext>;
pub type FFIConversionMixer<Parent> = FieldsSequenceMixer<Parent, LocalConversionContext, VariantIteratorLocalContext>;
pub type DropSequenceMixer<Parent> = FieldsSequenceMixer<Parent, FieldTypesContext, OwnedStatement>;
pub type FieldsOwnedComposer<Parent> = SequenceComposer<
    Parent,
    LocalConversionContext,
    FieldTypeConversion,
    OwnedItemPresentableContext,
    VariantIteratorLocalContext,
    OwnerIteratorPresentationContext
>;
pub type ConstructorComposer<Parent> = SequenceComposer<
    Parent,
    ConstructorFieldsContext,
    FieldTypeConversion,
    OwnedItemPresentablePair,
    (ConstructorPresentableContext, Vec<OwnedItemPresentablePair>),
    BindingPresentableContext
>;
#[allow(unused)]
pub type FnComposer<Parent> = SequenceComposer<
    Parent,
    FnSignatureContext,
    FieldTypeConversion,
    OwnedItemPresentablePair,
    (ConstructorPresentableContext, Vec<OwnedItemPresentablePair>),
    BindingPresentableContext
>;
pub type Depunctuated<T> = Punctuated<T, Void>;

pub trait Composer<'a> {
    type Source;
    type Result;
    fn compose(&self, source: &'a Self::Source) -> Self::Result;
}
#[allow(unused)]
pub trait LinkedComposer<'a, Parent>: Composer<'a> + ParentLinker<Parent> + Sized {}
// pub trait Decomposer: Composer where Self::Result: Composition {}


pub struct ContextComposer<Context, Result, Parent: SharedAccess> {
    parent: Option<Parent>,
    post_processor: ComposerPresenter<Context, Result>,
    context: SharedComposer<Parent, Context>,
}

impl<Context, Result, Parent: SharedAccess> ContextComposer<Context, Result, Parent> {
    pub const fn new(
        post_processor: ComposerPresenter<Context, Result>,
        context: SharedComposer<Parent, Context>
    ) -> Self {
        Self { parent: None, post_processor, context }
    }
}

impl<Context, Result, Parent: SharedAccess> ParentLinker<Parent>
for ContextComposer<Context, Result, Parent> {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Context, Result, Parent> Composer<'a>
for ContextComposer<Context, Result, Parent>
    where Parent: SharedAccess {
    type Source = ();
    type Result = Result;
    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.post_processor)(
            self.parent.as_ref()
                .expect("no parent")
                .access(self.context))
    }
}

impl<'a, Context, Result, Parent: SharedAccess> LinkedComposer<'a, Parent> for ContextComposer<Context, Result, Parent> {}

pub struct IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    post_processor: ComposerPresenter<(In, ComposerPresenterByRef<Ctx, Map>), Out>,
    item: ComposerPresenterByRef<Ctx, Map>
}

impl<In, Ctx, Map, Out> IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    pub const fn new(
        post_processor: ComposerPresenter<(In, ComposerPresenterByRef<Ctx, Map>), Out>,
        item: ComposerPresenterByRef<Ctx, Map>,
    ) -> Self {
        Self { post_processor, item }
    }
}
impl<'a, In, Ctx, Map, Out> Composer<'a>
for IterativeComposer<In, Ctx, Map, Out>
    where In: Clone {
    type Source = In;
    type Result = Out;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        // TODO: avoid cloning
        (self.post_processor)((source.clone(), self.item))
    }
}
impl<'a, Parent, In, Ctx, Map, Out> LinkedComposer<'a, Parent>
for IterativeComposer<In, Ctx, Map, Out>
    where 
        Parent: SharedAccess,
        In: Clone {}


impl<Parent, In, Ctx, Map, Out> ParentLinker<Parent>
for IterativeComposer<In, Ctx, Map, Out>
    where 
        Parent: SharedAccess, 
        In: Clone {
    fn link(&mut self, _parent: &Parent) {}
}

pub struct SequenceComposer<
    Parent,
    ParentCtx,
    SeqCtx,
    SeqMap,
    SeqOut,
    Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone {
    parent: Option<Parent>,
    post_processor: ComposerPresenter<SeqOut, Out>,
    context: SharedComposer<Parent, ParentCtx>,
    iterator: IterativeComposer<ParentCtx, SeqCtx, SeqMap, SeqOut>,
}

impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone {
    pub const fn new(
        post_processor: ComposerPresenter<SeqOut, Out>,
        context: SharedComposer<Parent, ParentCtx>,
        iterator_post_processor: ComposerPresenter<(ParentCtx, ComposerPresenterByRef<SeqCtx, SeqMap>), SeqOut>,
        iterator_item: ComposerPresenterByRef<SeqCtx, SeqMap>,
    ) -> Self {
        Self {
            post_processor,
            context,
            parent: None,
            iterator: IterativeComposer::new(
                iterator_post_processor,
                iterator_item
            )
        }
    }
}

impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> ParentLinker<Parent>
for SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out> Composer<'a>
for SequenceComposer<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, Out>
    where
        Parent: SharedAccess,
        ParentCtx: Clone {
    type Source = ();
    type Result = Out;
    fn compose(&self, _: &Self::Source) -> Self::Result {
        (self.post_processor)(
            self.iterator.compose(&self.parent
                .as_ref()
                .expect("no parent")
                .access(self.context)))
    }
}

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
impl<Parent, ParentCtx, SeqCtx, SeqMap, SeqOut, SeqMixOut, MixCtx, Out> ParentLinker<Parent>
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
            sequence: SequenceComposer::new(
                seq_root,
                seq_context,
                seq_iterator_root,
                seq_iterator_item
            )
        }
    }
    pub const fn new_new(
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
