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
mod variable;
mod from_conversion;
mod to_conversion;
mod destroy_conversion;
mod callback;
mod result;
mod attr_type;
mod tuple;
mod map;
mod group;
mod slice;
mod any_other;
mod bounds;
mod r#impl;
mod item_wrapper;


use std::marker::PhantomData;
use std::rc::Rc;
use syn::__private::TokenStream2;
use syn::{Field, Type};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use crate::ast::CommaPunctuated;
use crate::composable::{FieldComposer, FnSignatureContext, NestedArgument};
use crate::composer::r#abstract::{ContextComposer, SequenceComposer, SequenceMixer};
use crate::ext::ConversionType;
use crate::presentable::{Aspect, BindingPresentableContext, Context, Expression, OwnedItemPresentableContext, SequenceOutput};
use crate::presentation::{ArgPresentation, BindingPresentation, InterfacePresentation, Name};
use crate::shared::SharedAccess;

pub use self::r#abstract::*;
pub use self::any_other::*;
pub use self::attrs::*;
pub use self::basic::*;
pub use self::bounds::*;
pub use self::callback::*;
pub use self::constants::*;
pub use self::destroy_conversion::*;
pub use self::enum_composer::*;
pub use self::ffi_bindings::*;
pub use self::ffi_conversions::*;
pub use self::from_conversion::*;
pub use self::generic::*;
pub use self::generics_composer::*;
pub use self::group::*;
pub use self::r#impl::*;
pub use self::item::*;
pub use self::item_wrapper::*;
pub use self::map::*;
pub use self::method::*;
pub use self::opaque_item::*;
pub use self::result::*;
pub use self::signature::*;
pub use self::slice::*;
pub use self::to_conversion::*;
pub use self::trait_composer::*;
pub use self::tuple::*;
pub use self::r#type::TypeComposer;
pub use self::variable::*;

/// Composer Context Presenters
pub type ComposerLink<T> = Rc<std::cell::RefCell<T>>;
pub type ComposerRef<'a, T> = std::cell::Ref<'a, T>;
pub type ComposerPresenter<Context, Presentation> = fn(context: Context) -> Presentation;
pub type ComposerPresenterByRef<Context, Presentation> = fn(context: &Context) -> Presentation;
pub type SharedComposer<Link, Context> = ComposerPresenterByRef<<Link as SharedAccess>::ImmutableAccess, Context>;
pub type SharedComposerLink<C, Context> = SharedComposer<ComposerLink<C>, Context>;
pub type ComposerLinkDelegateByRef<'a, C, T> = ComposerPresenterByRef<ComposerRef<'a, C>, T>;
pub type ItemComposerLink<I, LANG, SPEC, Gen> = ComposerLink<ItemComposer<I, LANG, SPEC, Gen>>;
pub type OpaqueItemComposerLink<I, LANG, SPEC, Gen> = ComposerLink<OpaqueItemComposer<I, LANG, SPEC, Gen>>;
pub type EnumComposerLink<LANG, SPEC, Gen> = ComposerLink<EnumComposer<LANG, SPEC, Gen>>;
pub type SigComposerLink<LANG, SPEC, Gen> = ComposerLink<SigComposer<LANG, SPEC, Gen>>;
pub type ImplComposerLink<LANG, SPEC, Gen> = ComposerLink<ImplComposer<LANG, SPEC, Gen>>;
pub type TraitComposerLink<LANG, SPEC, Gen> = ComposerLink<TraitComposer<LANG, SPEC, Gen>>;

// pub type GenericComposerLink<LANG, SPEC> = ComposerLink<GenericComposer<LANG, SPEC>>;
pub type TypeContextComposer<Link> = ContextComposer<
    Type,
    TokenStream2,
    Link
>;

pub type SequenceOutputComposer<LANG, SPEC, IN, OUT> = ContextComposer<
    IN,
    SequenceOutput<LANG, SPEC>,
    OUT
>;
pub type OwnerIteratorConversionComposer<T, LANG, SPEC> = ComposerPresenter<
    OwnerAspectWithItems<T, LANG, SPEC>,
    SequenceOutput<LANG, SPEC>
>;
pub type OwnerIteratorPostProcessingComposer<T, LANG, SPEC> = SequenceOutputComposer<
    LANG,
    SPEC,
    SequenceOutput<LANG, SPEC>,
    T>;
pub type VariantComposerRef<LANG, SPEC> = ComposerPresenterByRef<
    OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>,
    SequenceOutput<LANG, SPEC>
>;
pub type ConstructorArgComposerRef<LANG, SPEC> = ComposerPresenterByRef<
    FieldComposer<LANG, SPEC>,
    OwnedItemPresentablePair<LANG, SPEC>
>;
pub type FieldsComposerRef<LANG, SPEC> = ComposerPresenterByRef<
    CommaPunctuatedFields,
    FieldComposers<LANG, SPEC>
>;
pub type FieldTypePresentationContextPassRef<LANG, SPEC> = ComposerPresenterByRef<
    FieldTypeLocalContext<LANG, SPEC>,
    Expression<LANG, SPEC>
>;
pub type OwnedFieldTypeComposerRef<LANG, SPEC> = ComposerPresenterByRef<
    FieldComposer<LANG, SPEC>,
    OwnedItemPresentableContext<LANG, SPEC>
>;

#[allow(unused)]
pub type InterfaceComposer<T> = ComposerPresenter<T, InterfacePresentation>;

/// Bindings
pub type BindingComposer<T, LANG, SPEC, Gen> = ComposerPresenter<T, BindingPresentableContext<LANG, SPEC, Gen>>;
pub type RustBindingComposer<T> = ComposerPresenter<T, BindingPresentation>;
pub type BindingCtorComposer<LANG, SPEC, Gen> = BindingComposer<FunctionContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>;
pub type BindingDtorComposer<LANG, SPEC, Gen> = BindingComposer<DestructorContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>;
pub type BindingAccessorComposer<LANG, SPEC, Gen> = BindingComposer<BindingAccessorContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>;
// pub type RustBindingDtorComposer<LANG, SPEC> = RustBindingComposer<DestructorContext<LANG, SPEC>>;
pub type OwnedItemsPunctuated<SEP, LANG, SPEC> = Punctuated<OwnedItemPresentableContext<LANG, SPEC>, SEP>;
pub type OwnerWithItems<ASPECT, SEP, LANG, SPEC> = (ASPECT, OwnedItemsPunctuated<SEP, LANG, SPEC>);
pub type OwnerAspectWithItems<SEP, LANG, SPEC> = OwnerWithItems<Aspect<Context>, SEP, LANG, SPEC>;
pub type OwnerAspectWithCommaPunctuatedItems<LANG, SPEC> = OwnerAspectWithItems<Comma, LANG, SPEC>;
pub type ArgComposers<SEP, LANG, SPEC> = Punctuated<FieldComposer<LANG, SPEC>, SEP>;
pub type FieldComposers<LANG, SPEC> = CommaPunctuated<FieldComposer<LANG, SPEC>>;
pub type OwnedStatement<LANG, SPEC> = OwnedItemsPunctuated<Semi, LANG, SPEC>;
pub type OwnedFieldComposers<T, LANG, SPEC> = (T, FieldComposers<LANG, SPEC>);
pub type LocallyOwnedFieldComposers<T, LANG, SPEC, Gen> = (OwnedFieldComposers<T, LANG, SPEC>, Gen);
pub type LocalConversionContext<LANG, SPEC, Gen> = LocallyOwnedFieldComposers<Aspect<Context>, LANG, SPEC, Gen>;
pub type ConstructorFieldsContext<LANG, SPEC, Gen> = LocallyOwnedFieldComposers<(DestructorContext<LANG, SPEC, Gen>, bool), LANG, SPEC, Gen>;
pub type BindingAccessorContext<LANG, SPEC, Gen> = (Type, TokenStream2, Type, SPEC, Gen, PhantomData<LANG>);
// pub type ConstructorContext<LANG, SPEC> = (DestructorContext<LANG, SPEC>, Vec<(CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>)>);
pub type DestructorContext<LANG, SPEC, Gen> = (Type, SPEC, Gen, PhantomData<LANG>);
pub type FieldTypeLocalContext<LANG, SPEC> = (Name, ConversionType<LANG, SPEC>);
pub type FunctionContext<LANG, SPEC, Gen> = ((DestructorContext<LANG, SPEC, Gen>, bool), Vec<OwnedItemPresentablePair<LANG, SPEC>>);
pub type OwnedItemPresentablePair<LANG, SPEC> = (OwnedItemPresentableContext<LANG, SPEC>, OwnedItemPresentableContext<LANG, SPEC>);
pub type SequenceOutputPair<LANG, SPEC> = (SequenceOutput<LANG, SPEC>, SequenceOutput<LANG, SPEC>);
pub type FieldsSequenceMixer<Link, Context, Statement, LANG, SPEC> = SequenceMixer<
    Link,
    Context,
    FieldTypeLocalContext<LANG, SPEC>,
    Expression<LANG, SPEC>,
    Statement,
    SequenceOutput<LANG, SPEC>,
    SequenceOutput<LANG, SPEC>,
    SequenceOutput<LANG, SPEC>>;
pub type FFIConversionMixer<Link, LANG, SPEC, Gen> = FieldsSequenceMixer<
    Link,
    LocalConversionContext<LANG, SPEC, Gen>,
    OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>,
    LANG,
    SPEC
>;
pub type DropSequenceMixer<Link, LANG, SPEC> = FieldsSequenceMixer<
    Link,
    FieldComposers<LANG, SPEC>,
    OwnedStatement<LANG, SPEC>,
    LANG,
    SPEC
>;
pub type FieldsSequenceComposer<Link, OwnerAspect, B, C, Presentable, LANG, SPEC> = SequenceComposer<
    Link,
    OwnerAspect,
    FieldComposer<LANG, SPEC>,
    B,
    C,
    Presentable
>;
pub type FnSequenceComposer<Link, OwnerAspect, LANG, SPEC, Gen> = FieldsSequenceComposer<
    Link,
    OwnerAspect,
    OwnedItemPresentablePair<LANG, SPEC>,
    FunctionContext<LANG, SPEC, Gen>,
    BindingPresentableContext<LANG, SPEC, Gen>,
    LANG, SPEC
>;
pub type FieldsOwnedSequenceComposer<Link, LANG, SPEC, Gen> = FieldsSequenceComposer<
    Link,
    LocalConversionContext<LANG, SPEC, Gen>,
    OwnedItemPresentableContext<LANG, SPEC>,
    OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>,
    SequenceOutput<LANG, SPEC>,
    LANG,
    SPEC
>;
pub type CtorSequenceComposer<Link, LANG, SPEC, Gen> = FnSequenceComposer<
    Link,
    ConstructorFieldsContext<LANG, SPEC, Gen>,
    LANG,
    SPEC,
    Gen
>;
#[allow(unused)]
pub type FnSignatureSequenceComposer<Link, LANG, SPEC, Gen> = FnSequenceComposer<
    Link,
    FnSignatureContext,
    LANG,
    SPEC,
    Gen
>;
#[allow(unused)]
pub type CommaPunctuatedArgs = CommaPunctuated<ArgPresentation>;
#[allow(unused)]
pub type CommaPunctuatedOwnedItems<LANG, SPEC> = OwnedItemsPunctuated<Comma, LANG, SPEC>;
#[allow(unused)]
pub type CommaPunctuatedFields = CommaPunctuated<Field>;
#[allow(unused)]
pub type CommaPunctuatedNestedArguments = CommaPunctuated<NestedArgument>;


