mod attrs;
mod ffi_conversions;
mod item;
mod method;
mod constants;
mod chain;
mod enum_composer;
mod r#type;
mod generic;
mod signature;
mod basic;
mod trait_composer;
mod opaque_item;
mod ffi_bindings;
mod generics_composer;
mod r#abstract;


use std::rc::Rc;
use syn::__private::TokenStream2;
use syn::{Field, Generics, Type};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{FieldTypeComposition, FnSignatureContext, NestedArgument};
use crate::composer::r#abstract::{ContextComposer, SequenceComposer, SequenceMixer};
use crate::presentable::{Aspect, ConstructorBindingPresentableContext, ConstructorPresentableContext, Expression, OwnedItemPresentableContext, SequenceOutput};
use crate::presentation::{ArgPresentation, BindingPresentation, Expansion, Name};
use crate::shared::SharedAccess;

pub use self::r#abstract::*;
pub use self::attrs::*;
pub use self::basic::*;
pub use self::constants::*;
// pub use self::chain::*;
pub use self::enum_composer::*;
pub use self::ffi_bindings::*;
pub use self::ffi_conversions::*;
pub use self::generic::*;
pub use self::generics_composer::*;
pub use self::item::*;
pub use self::method::*;
pub use self::opaque_item::*;
pub use self::signature::*;
pub use self::trait_composer::*;
pub use self::r#type::TypeComposer;




/// Composer Context Presenters
pub type ParentComposer<T> = Rc<std::cell::RefCell<T>>;
pub type ParentComposerRef<'a, T> = std::cell::Ref<'a, T>;
pub type ComposerPresenter<Context, Presentation> = fn(context: Context) -> Presentation;
pub type ComposerPresenterByRef<Context, Presentation> = fn(context: &Context) -> Presentation;
pub type SharedComposer<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, Context>;
pub type ParentSharedComposer<C, Context> = SharedComposer<ParentComposer<C>, Context>;
pub type ParentComposerPresenterByRef<'a, C, T> = ComposerPresenterByRef<ParentComposerRef<'a, C>, T>;

pub type ItemParentComposer<I> = ParentComposer<ItemComposer<I>>;
pub type EnumParentComposer<I> = ParentComposer<EnumComposer<I>>;
pub type SigParentComposer = ParentComposer<SigComposer>;
pub type TraitParentComposer = ParentComposer<TraitComposer>;
pub type GenericParentComposer = ParentComposer<GenericComposer>;

pub type NameContextComposer<Parent> = ContextComposer<Name, TokenStream2, Parent>;
pub type TypeContextComposer<Parent> = ContextComposer<Type, TokenStream2, Parent>;
pub type OwnerIteratorConversionComposer<T> = ComposerPresenter<OwnerAspectWithItems<T>, SequenceOutput>;
pub type OwnerIteratorPostProcessingComposer<T> = ContextComposer<SequenceOutput, SequenceOutput, T>;

pub type VariantComposerRef = ComposerPresenterByRef<OwnerAspectWithCommaPunctuatedItems, SequenceOutput>;
pub type ConstructorArgComposerRef = ComposerPresenterByRef<FieldTypeComposition, OwnedItemPresentablePair>;

pub type FieldsComposerRef = ComposerPresenterByRef<CommaPunctuatedFields, FieldTypesContext>;
pub type FieldTypePresentationContextPassRef = ComposerPresenterByRef<FieldTypeLocalContext, Expression>;
pub type OwnedFieldTypeComposerRef = ComposerPresenterByRef<FieldTypeComposition, OwnedItemPresentableContext>;

/// Bindings
pub type BindingComposer<T> = ComposerPresenter<T, BindingPresentation>;
pub type BindingDtorComposer = BindingComposer<DestructorContext>;
pub type OwnedItemsPunctuated<SEP> = Punctuated<OwnedItemPresentableContext, SEP>;
pub type OwnerWithItems<ASPECT, SEP> = (ASPECT, OwnedItemsPunctuated<SEP>);
pub type OwnerAspectWithItems<SEP> = OwnerWithItems<Aspect, SEP>;
pub type OwnerAspectWithCommaPunctuatedItems = OwnerAspectWithItems<Comma>;
pub type FieldTypesContext = CommaPunctuated<FieldTypeComposition>;
pub type OwnedStatement = OwnedItemsPunctuated<Semi>;
pub type FieldsOwnerContext<T> = (T, FieldTypesContext);
pub type LocalFieldsOwnerContext<T> = (FieldsOwnerContext<T>, Option<Generics>);
pub type LocalConversionContext = LocalFieldsOwnerContext<Aspect>;
pub type ConstructorFieldsContext = LocalFieldsOwnerContext<ConstructorPresentableContext>;


pub type BindingAccessorContext = (Type, TokenStream2, Type, Depunctuated<Expansion>, Option<Generics>);
pub type DestructorContext = (Type, Depunctuated<Expansion>, Option<Generics>);




pub type FieldTypeLocalContext = (TokenStream2, Expression);
pub type FunctionContext = (ConstructorPresentableContext, Vec<OwnedItemPresentablePair>);
pub type OwnedItemPresentablePair = (OwnedItemPresentableContext, OwnedItemPresentableContext);
pub type SequenceOutputPair = (SequenceOutput, SequenceOutput);
pub type FieldsSequenceMixer<Parent, Context, Statement> = SequenceMixer<
    Parent,
    Context,
    FieldTypeLocalContext,
    Expression,
    Statement,
    SequenceOutput,
    SequenceOutput,
    SequenceOutput>;
pub type FFIConversionMixer<Parent> =
    FieldsSequenceMixer<Parent, LocalConversionContext, OwnerAspectWithCommaPunctuatedItems>;
pub type DropSequenceMixer<Parent> =
    FieldsSequenceMixer<Parent, FieldTypesContext, OwnedStatement>;
pub type FieldsSequenceComposer<Parent, OwnerAspect, B, C, Presentable> =
    SequenceComposer<Parent, OwnerAspect, FieldTypeComposition, B, C, Presentable>;
pub type FnSequenceComposer<Parent, OwnerAspect, I> = FieldsSequenceComposer<
    Parent,
    OwnerAspect,
    OwnedItemPresentablePair,
    FunctionContext,
    ConstructorBindingPresentableContext<I>
>;
pub type FieldsOwnedSequenceComposer<Parent> = FieldsSequenceComposer<
    Parent,
    LocalConversionContext,
    OwnedItemPresentableContext,
    OwnerAspectWithCommaPunctuatedItems,
    SequenceOutput
>;
pub type CtorSequenceComposer<Parent, I> = FnSequenceComposer<Parent, ConstructorFieldsContext, I>;
#[allow(unused)]
pub type FnSignatureSequenceComposer<Parent, I> = FnSequenceComposer<Parent, FnSignatureContext, I>;


#[allow(unused)]
pub type CommaPunctuatedArgs = CommaPunctuated<ArgPresentation>;
#[allow(unused)]
pub type CommaPunctuatedOwnedItems = OwnedItemsPunctuated<Comma>;
#[allow(unused)]
pub type CommaPunctuatedFields = CommaPunctuated<Field>;
#[allow(unused)]
pub type CommaPunctuatedNestedArguments = CommaPunctuated<NestedArgument>;


