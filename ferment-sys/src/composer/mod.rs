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
mod opaque_struct;
mod ffi_bindings;
mod generics_composer;
mod r#abstract;
mod variable;
mod from_conversion;
mod ffi_full_path;
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
mod r#struct;
mod enum_variant;
mod type_alias;
mod vtable;

use std::rc::Rc;
use syn::__private::TokenStream2;
use syn::Field;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use crate::ast::CommaPunctuated;
use crate::composable::{FieldComposer, NestedArgument};
use crate::composer::r#abstract::{LinkedContextComposer, SequenceComposer, SequenceMixer};
use crate::composer::vtable::VTableComposer;
use crate::ext::ConversionType;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::ArgPresentation;
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
pub use self::enum_variant::*;
pub use self::ffi_bindings::*;
pub use self::ffi_conversions::*;
pub use self::from_conversion::*;
// pub use self::ffi_full_path::*;
pub use self::generic::*;
pub use self::generics_composer::*;
pub use self::group::*;
pub use self::r#impl::*;
pub use self::item::*;
pub use self::item_wrapper::*;
pub use self::map::*;
pub use self::method::*;
pub use self::opaque_struct::*;
pub use self::result::*;
pub use self::signature::*;
pub use self::slice::*;
pub use self::r#struct::*;
pub use self::to_conversion::*;
pub use self::trait_composer::*;
pub use self::tuple::*;
pub use self::r#type::TypeComposer;
pub use self::type_alias::*;
pub use self::variable::*;

/// Composer Context Presenters
pub type ComposerLink<T> = Rc<std::cell::RefCell<T>>;
pub type ComposerLinkRef<'a, T> = std::cell::Ref<'a, T>;
pub type Composer<T, U> = fn(context: T) -> U;
pub type ComposerByRef<T, U> = fn(context: &T) -> U;
pub type SharedComposer<Link, U> = ComposerByRef<<Link as SharedAccess>::ImmutableAccess, U>;
pub type SharedComposerLink<C, U> = SharedComposer<ComposerLink<C>, U>;
// pub type ComposerLinkDelegateByRef<'a, C, T> = ComposerByRef<ComposerLinkRef<'a, C>, T>;
pub type ItemComposerLink<I, LANG, SPEC> = ComposerLink<ItemComposer<I, LANG, SPEC>>;
pub type EnumVariantComposerLink<I, LANG, SPEC> = ComposerLink<EnumVariantComposer<I, LANG, SPEC>>;
pub type StructComposerLink<I, LANG, SPEC> = ComposerLink<StructComposer<I, LANG, SPEC>>;
pub type OpaqueStructComposerLink<I, LANG, SPEC> = ComposerLink<OpaqueStructComposer<I, LANG, SPEC>>;
pub type TypeAliasComposerLink<I, LANG, SPEC> = ComposerLink<TypeAliasComposer<I, LANG, SPEC>>;
pub type EnumComposerLink<LANG, SPEC> = ComposerLink<EnumComposer<LANG, SPEC>>;
pub type SigComposerLink<LANG, SPEC> = ComposerLink<SigComposer<LANG, SPEC>>;
pub type VTableComposerLink<LANG, SPEC> = ComposerLink<VTableComposer<LANG, SPEC>>;
pub type ImplComposerLink<LANG, SPEC> = ComposerLink<ImplComposer<LANG, SPEC>>;
pub type TraitComposerLink<LANG, SPEC> = ComposerLink<TraitComposer<LANG, SPEC>>;
pub type TypeContextComposer<Link, TYC, U> = LinkedContextComposer<Link, <Aspect<TYC> as ScopeContextPresentable>::Presentation, U>;
pub type TypeContextComposerLink<T, TYC, U> = TypeContextComposer<ComposerLink<T>, TYC, U>;
pub type SequenceOutputComposer<Link, LANG, SPEC> = LinkedContextComposer<
    Link,
    PresentableSequence<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;
pub type MaybeSequenceOutputComposer<Link, LANG, SPEC> = Option<SequenceOutputComposer<Link, LANG, SPEC>>;
pub type MaybeSequenceOutputComposerLink<T, LANG, SPEC> = Option<SequenceOutputComposer<ComposerLink<T>, LANG, SPEC>>;
pub type SequenceOutputComposerLink<T, LANG, SPEC> = LinkedContextComposer<
    ComposerLink<T>,
    PresentableSequence<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;

pub type VariantComposerRef<LANG, SPEC> = ComposerByRef<
    AspectCommaPunctuatedArguments<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;
pub type ConstructorArgComposerRef<LANG, SPEC> = FieldComposerProducer<LANG, SPEC, PresentableArgumentPair<LANG, SPEC>>;
pub type FieldsComposerRef<LANG, SPEC> = ComposerByRef<CommaPunctuatedFields, FieldComposers<LANG, SPEC>>;
pub type PresentableExprComposerRef<LANG, SPEC> = ComposerByRef<FieldTypeLocalContext<LANG, SPEC>, Expression<LANG, SPEC>>;
pub type PresentableArgumentComposerRef<LANG, SPEC> = FieldComposerProducer<LANG, SPEC, PresentableArgument<LANG, SPEC>>;
/// Bindings
pub type BindingComposer<T, LANG, SPEC> = Composer<T, BindingPresentableContext<LANG, SPEC>>;
pub type BindingCtorComposer<LANG, SPEC> = BindingComposer<FunctionContext<LANG, SPEC>, LANG, SPEC>;
pub type PresentableArguments<SEP, LANG, SPEC> = Punctuated<PresentableArgument<LANG, SPEC>, SEP>;
pub type AspectPresentableArguments<SEP, LANG, SPEC> = (GenericAspect<LANG, SPEC>, PresentableArguments<SEP, LANG, SPEC>);
pub type AspectCommaPunctuatedArguments<LANG, SPEC> = AspectPresentableArguments<Comma, LANG, SPEC>;
pub type AspectTerminatedArguments<LANG, SPEC> = AspectPresentableArguments<Semi, LANG, SPEC>;
pub type ArgComposers<SEP, LANG, SPEC> = Punctuated<FieldComposer<LANG, SPEC>, SEP>;
pub type FieldComposers<LANG, SPEC> = CommaPunctuated<FieldComposer<LANG, SPEC>>;
// pub type TerminatedArguments<LANG, SPEC> = PresentableArguments<Semi, LANG, SPEC>;
pub type LocallyOwnedFieldComposers<T, LANG, SPEC> = (T, FieldComposers<LANG, SPEC>);
pub type GenericAspect<LANG, SPEC> = (Aspect<<SPEC as Specification<LANG>>::TYC>, <SPEC as Specification<LANG>>::Gen);
pub type LocalConversionContext<LANG, SPEC> = LocallyOwnedFieldComposers<GenericAspect<LANG, SPEC>, LANG, SPEC>;
pub type ConstructorFieldsContext<LANG, SPEC> = LocallyOwnedFieldComposers<(DestructorContext<LANG, SPEC>, bool), LANG, SPEC>;
pub type BindingAccessorContext<LANG, SPEC> = (
    <Aspect<<SPEC as Specification<LANG>>::TYC> as ScopeContextPresentable>::Presentation,
    TokenStream2,
    <SPEC as Specification<LANG>>::Var,
    <SPEC as Specification<LANG>>::Attr,
    <SPEC as Specification<LANG>>::Gen
);
pub type DestructorContext<LANG, SPEC> = (
    <Aspect<<SPEC as Specification<LANG>>::TYC> as ScopeContextPresentable>::Presentation,
    <SPEC as Specification<LANG>>::Attr,
    <SPEC as Specification<LANG>>::Gen
);
pub type FieldTypeLocalContext<LANG, SPEC> = (<SPEC as Specification<LANG>>::Name, ConversionType<LANG, SPEC>);
pub type FunctionContext<LANG, SPEC> = ((DestructorContext<LANG, SPEC>, bool), Vec<PresentableArgumentPair<LANG, SPEC>>);
pub type PresentableArgumentPair<LANG, SPEC> = (PresentableArgument<LANG, SPEC>, PresentableArgument<LANG, SPEC>);
pub type CommaPunctuatedArgs = CommaPunctuated<ArgPresentation>;
pub type CommaPunctuatedPresentableArguments<LANG, SPEC> = PresentableArguments<Comma, LANG, SPEC>;
pub type CommaPunctuatedFields = CommaPunctuated<Field>;
pub type CommaPunctuatedNestedArguments = CommaPunctuated<NestedArgument>;

pub type PresentableSequencePair<LANG, SPEC> = (PresentableSequence<LANG, SPEC>, PresentableSequence<LANG, SPEC>);
pub type FieldPathResolver<LANG, SPEC> = ComposerByRef<FieldComposer<LANG, SPEC>, FieldTypeLocalContext<LANG, SPEC>>;
pub type AspectSequenceComposer<LANG, SPEC> = Composer<
    AspectCommaPunctuatedArguments<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;
pub type DropSequenceComposer<LANG, SPEC> = Composer<
    AspectTerminatedArguments<LANG, SPEC>,
    // TerminatedArguments<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;
pub type FieldComposerProducer<LANG, SPEC, OUT> = ComposerByRef<FieldComposer<LANG, SPEC>, OUT>;

pub type FieldsSequenceMixer<Link, Context, Statement, LANG, SPEC> = SequenceMixer<
    Link,
    Context,
    FieldTypeLocalContext<LANG, SPEC>,
    <SPEC as Specification<LANG>>::Expr,
    Statement,
    PresentableSequence<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;
pub type FFIConversionsMixer<Link, LANG, SPEC> = FieldsSequenceMixer<
    Link,
    LocalConversionContext<LANG, SPEC>,
    AspectCommaPunctuatedArguments<LANG, SPEC>,
    LANG,
    SPEC,
>;
pub type DropSequenceMixer<Link, LANG, SPEC> = FieldsSequenceMixer<
    Link,
    LocalConversionContext<LANG, SPEC>,
    AspectTerminatedArguments<LANG, SPEC>,
    // TerminatedArguments<LANG, SPEC>,
    LANG,
    SPEC,
>;
pub type FieldsSequenceComposer<Link, A, B, C, Presentable, LANG, SPEC> = SequenceComposer<
    Link,
    A,
    FieldComposer<LANG, SPEC>,
    B,
    C,
    Presentable
>;
pub type FieldsOwnedSequenceComposer<Link, LANG, SPEC> = FieldsSequenceComposer<
    Link,
    LocalConversionContext<LANG, SPEC>,
    PresentableArgument<LANG, SPEC>,
    AspectCommaPunctuatedArguments<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>,
    LANG,
    SPEC
>;
pub type FieldsOwnedSequenceComposerLink<T, LANG, SPEC> = FieldsOwnedSequenceComposer<
    ComposerLink<T>,
    LANG,
    SPEC
>;
pub type FnSequenceComposer<Link, OwnerAspect, LANG, SPEC> = FieldsSequenceComposer<
    Link,
    OwnerAspect,
    PresentableArgumentPair<LANG, SPEC>,
    FunctionContext<LANG, SPEC>,
    BindingPresentableContext<LANG, SPEC>,
    LANG,
    SPEC
>;

pub type CtorSequenceComposer<Link, LANG, SPEC> = FnSequenceComposer<
    Link,
    ConstructorFieldsContext<LANG, SPEC>,
    LANG,
    SPEC,
>;
// pub type CtorSequenceComposerLink<T, LANG, SPEC> = CtorSequenceComposer<ComposerLink<T>, LANG, SPEC>;

pub type CtorSharedComposerLink<T, LANG, SPEC> = SharedComposerLink<T, ConstructorFieldsContext<LANG, SPEC>>;
pub type AspectSharedComposerLink<T, LANG, SPEC> = SharedComposerLink<T, LocalConversionContext<LANG, SPEC>>;
pub type SequenceSharedComposerLink<T, LANG, SPEC> = SharedComposerLink<T, PresentableSequence<LANG, SPEC>>;

pub type RootSequenceComposer<LANG, SPEC> = ComposerByRef<PresentableSequencePair<LANG, SPEC>, PresentableSequence<LANG, SPEC>>;