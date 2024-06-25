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
pub mod opaque_item;
mod ffi_bindings;
mod generics_composer;
pub mod r#abstract;


use std::rc::Rc;
use syn::__private::TokenStream2;
use syn::{Field, Generics, Type};
use syn::punctuated::Punctuated;
use syn::token::{Add, Brace, Colon2, Comma, Dot, FatArrow, Paren, Semi};
pub use enum_composer::EnumComposer;
use crate::composer::generic::GenericComposer;
use crate::composer::opaque_item::OpaqueItemComposer;
use crate::composer::r#abstract::{Composer, ContextComposer, LinkedComposer, ParentLinker, SequenceComposer, SequenceMixer};
use crate::composer::signature::SigComposer;
use crate::composer::trait_composer::TraitComposer;
use crate::composition::{FnSignatureContext, NestedArgument};
use crate::conversion::FieldTypeConversion;
use crate::naming::Name;
use crate::opposed::Opposed;
use crate::presentation::{BindingPresentation, Expansion, ScopeContextPresentable};
use crate::presentation::context::{BindingPresentableContext, ConstructorPresentableContext, FieldContext, OwnedItemPresentableContext, SequenceOutput};
use crate::presentation::context::name::Aspect;
use crate::shared::SharedAccess;
use crate::wrapped::{Void, Wrapped};
pub use self::attrs::{AttrsComposer};
pub use self::ffi_conversions::{FFIAspect, FFIComposer};
pub use self::item::{ItemComposer, ItemComposerWrapper};
pub use self::method::MethodComposer;




/// Composer Context Presenters
pub type ParentComposer<T> = Rc<std::cell::RefCell<T>>;
pub type ParentComposerRef<'a, T> = std::cell::Ref<'a, T>;

pub type ComposerPresenter<Context, Presentation> = fn(context: Context) -> Presentation;
pub type ComposerPresenterByRef<Context, Presentation> = fn(context: &Context) -> Presentation;
pub type SharedComposer<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, Context>;
pub type ParentSharedComposer<C, Context> = SharedComposer<ParentComposer<C>, Context>;
pub type ParentComposerPresenterByRef<'a, C, T> = ComposerPresenterByRef<ParentComposerRef<'a, C>, T>;

pub type ItemParentComposer<I> = ParentComposer<ItemComposer<I>>;
pub type OpaqueItemParentComposer<I> = ParentComposer<OpaqueItemComposer<I>>;
pub type EnumParentComposer<I> = ParentComposer<EnumComposer<I>>;
pub type SigParentComposer = ParentComposer<SigComposer>;
pub type TraitParentComposer = ParentComposer<TraitComposer>;
pub type GenericParentComposer = ParentComposer<GenericComposer>;
pub type ItemParentComposerRef<'a, I> = ParentComposerRef<'a, ItemComposer<I>>;
pub type EnumParentComposerRef<'a, I> = ParentComposerRef<'a, EnumComposer<I>>;
pub type ItemComposerPresenterRef<'a, T, I> = ComposerPresenterByRef<ItemParentComposerRef<'a, I>, T>;
pub type EnumComposerPresenterRef<'a, T, I> = ComposerPresenterByRef<EnumParentComposerRef<'a, I>, T>;
pub type ItemComposerFieldTypesContextPresenter<'a, I> = ItemComposerPresenterRef<'a, FieldTypesContext, I>;
pub type NameContextComposer<Parent> = ContextComposer<Name, TokenStream2, Parent>;
pub type TypeContextComposer<Parent> = ContextComposer<Type, TokenStream2, Parent>;
pub type OwnerIteratorConversionComposer<T> = ComposerPresenter<OwnerAspectIteratorLocalContext<T>, SequenceOutput>;
pub type OwnerIteratorPostProcessingComposer<T> = ContextComposer<SequenceOutput, SequenceOutput, T>;
pub type VariantComposer = ComposerPresenterByRef<VariantIteratorLocalContext, SequenceOutput>;
pub type ConstructorArgComposer = ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair>;

pub type FieldsComposer = ComposerPresenterByRef<CommaPunctuatedFields, FieldTypesContext>;
pub type FieldTypePresentationContextPassRef = ComposerPresenterByRef<FieldTypeLocalContext, FieldContext>;
/// Bindings
pub type BindingComposer<T> = ComposerPresenter<T, BindingPresentation>;
pub type BindingDtorComposer = BindingComposer<DestructorContext>;
pub type OwnedFieldTypeComposerRef = ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentableContext>;
pub type OwnerIteratorLocalContext<A, T> = (A, Punctuated<OwnedItemPresentableContext, T>);
pub type OwnerAspectIteratorLocalContext<T> = OwnerIteratorLocalContext<Aspect, T>;
pub type VariantIteratorLocalContext = OwnerAspectIteratorLocalContext<Comma>;
pub type FieldTypesContext = CommaPunctuated<FieldTypeConversion>;
pub type OwnedStatement = SemiPunctuated<OwnedItemPresentableContext>;
pub type FieldsOwnerContext<T> = (T, FieldTypesContext);
pub type LocalConversionContext = (FieldsOwnerContext<Aspect>, Option<Generics>);
pub type ConstructorFieldsContext = FieldsOwnerContext<ConstructorPresentableContext>;
pub type BindingAccessorContext = (Type, TokenStream2, Type, TokenStream2, Option<Generics>);
pub type DestructorContext = (Type, Depunctuated<Expansion>, Option<Generics>);
pub type FieldTypeLocalContext = (TokenStream2, FieldContext);
pub type FunctionContext = (ConstructorPresentableContext, Vec<OwnedItemPresentablePair>);
pub type OwnedItemPresentablePair = (OwnedItemPresentableContext, OwnedItemPresentableContext);
pub type OwnedItemPresentationPair = (SequenceOutput, SequenceOutput);
pub type FieldsSequenceMixer<Parent, Context, Statement> = SequenceMixer<
    Parent,
    Context,
    FieldTypeLocalContext,
    FieldContext,
    Statement,
    SequenceOutput,
    SequenceOutput,
    SequenceOutput>;
pub type FFIConversionMixer<Parent> = FieldsSequenceMixer<Parent, LocalConversionContext, VariantIteratorLocalContext>;
pub type DropSequenceMixer<Parent> = FieldsSequenceMixer<Parent, FieldTypesContext, OwnedStatement>;
pub type FieldsOwnedComposer<Parent> = SequenceComposer<
    Parent,
    LocalConversionContext,
    FieldTypeConversion,
    OwnedItemPresentableContext,
    VariantIteratorLocalContext,
    SequenceOutput
>;
pub type ConstructorComposer<Parent, S, SP, I> = SequenceComposer<
    Parent,
    ConstructorFieldsContext,
    FieldTypeConversion,
    OwnedItemPresentablePair,
    FunctionContext,
    BindingPresentableContext<S, SP, I>
>;
#[allow(unused)]
pub type FnComposer<Parent, S, SP, I> = SequenceComposer<
    Parent,
    FnSignatureContext,
    FieldTypeConversion,
    OwnedItemPresentablePair,
    FunctionContext,
    BindingPresentableContext<S, SP, I>
>;

#[allow(unused)]
pub type Depunctuated<T> = Punctuated<T, Void>;
#[allow(unused)]
pub type CommaPunctuated<T> = Punctuated<T, Comma>;
#[allow(unused)]
pub type SemiPunctuated<T> = Punctuated<T, Semi>;
#[allow(unused)]
pub type Colon2Punctuated<T> = Punctuated<T, Colon2>;
#[allow(unused)]
pub type AddPunctuated<T> = Punctuated<T, Add>;
#[allow(unused)]
pub type DotPunctuated<T> = Punctuated<T, Dot>;
#[allow(unused)]
pub type BraceWrapped<S, SP> = Wrapped<S, SP, Brace>;
#[allow(unused)]
pub type ParenWrapped<S, SP> = Wrapped<S, SP, Paren>;

#[allow(unused)]
pub type Assignment<T1, T2> = Opposed<T1, T2, syn::token::Eq>;
#[allow(unused)]
pub type Lambda<T1, T2> = Opposed<T1, T2, FatArrow>;

#[allow(unused)]
pub type CommaPunctuatedTokens = CommaPunctuated<TokenStream2>;
#[allow(unused)]
pub type CommaPunctuatedOwnedItems = CommaPunctuated<OwnedItemPresentableContext>;
#[allow(unused)]
pub type CommaPunctuatedFields = CommaPunctuated<Field>;
#[allow(unused)]
pub type CommaPunctuatedNestedArguments = CommaPunctuated<NestedArgument>;


