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


use std::rc::Rc;
use syn::__private::TokenStream2;
use syn::{Field, Type};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use crate::ast::CommaPunctuated;
use crate::composable::{FieldComposer, NestedArgument};
use crate::composer::r#abstract::{LinkedContextComposer, SequenceComposer, SequenceMixer};
use crate::ext::ConversionType;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{ArgPresentation, Name};
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
pub type ComposerRef<'a, T> = std::cell::Ref<'a, T>;
pub type ComposerPresenter<T, U> = fn(context: T) -> U;
pub type ComposerPresenterByRef<T, U> = fn(context: &T) -> U;
pub type SharedComposer<Link, U> = ComposerPresenterByRef<<Link as SharedAccess>::ImmutableAccess, U>;
pub type SharedComposerLink<C, U> = SharedComposer<ComposerLink<C>, U>;
pub type ComposerLinkDelegateByRef<'a, C, T> = ComposerPresenterByRef<ComposerRef<'a, C>, T>;
pub type ItemComposerLink<I, LANG, SPEC> = ComposerLink<ItemComposer<I, LANG, SPEC>>;
pub type EnumVariantComposerLink<I, LANG, SPEC> = ComposerLink<EnumVariantComposer<I, LANG, SPEC>>;
pub type StructComposerLink<I, LANG, SPEC> = ComposerLink<StructComposer<I, LANG, SPEC>>;
pub type OpaqueStructComposerLink<I, LANG, SPEC> = ComposerLink<OpaqueStructComposer<I, LANG, SPEC>>;
pub type TypeAliasComposerLink<I, LANG, SPEC> = ComposerLink<TypeAliasComposer<I, LANG, SPEC>>;
pub type EnumComposerLink<LANG, SPEC> = ComposerLink<EnumComposer<LANG, SPEC>>;
pub type SigComposerLink<LANG, SPEC> = ComposerLink<SigComposer<LANG, SPEC>>;
pub type ImplComposerLink<LANG, SPEC> = ComposerLink<ImplComposer<LANG, SPEC>>;
pub type TraitComposerLink<LANG, SPEC> = ComposerLink<TraitComposer<LANG, SPEC>>;
pub type TypeContextComposer<Link, TYC, U> = LinkedContextComposer<Link, <Aspect<TYC> as ScopeContextPresentable>::Presentation, U>;
pub type OwnerCommaIteratorConversionComposer<LANG, SPEC> = ComposerPresenter<
    AspectPresentableArguments<Comma, LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;
pub type SequenceOutputComposer<Link, LANG, SPEC> = LinkedContextComposer<
    Link,
    PresentableSequence<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;

pub type VariantComposerRef<LANG, SPEC> = ComposerPresenterByRef<
    AspectCommaPunctuatedArguments<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;
pub type ConstructorArgComposerRef<LANG, SPEC> = FieldComposerProducer<LANG, SPEC, PresentableArgumentPair<LANG, SPEC>>;
pub type FieldsComposerRef<LANG, SPEC> = ComposerPresenterByRef<CommaPunctuatedFields, FieldComposers<LANG, SPEC>>;
pub type PresentableExpressionComposerRef<LANG, SPEC> = ComposerPresenterByRef<FieldTypeLocalContext<LANG, SPEC>, Expression<LANG, SPEC>>;
pub type PresentableArgumentComposerRef<LANG, SPEC> = FieldComposerProducer<LANG, SPEC, PresentableArgument<LANG, SPEC>>;
/// Bindings
pub type BindingComposer<T, LANG, SPEC> = ComposerPresenter<T, BindingPresentableContext<LANG, SPEC>>;
pub type BindingCtorComposer<LANG, SPEC> = BindingComposer<FunctionContext<LANG, SPEC>, LANG, SPEC>;
pub type PresentableArguments<SEP, LANG, SPEC> = Punctuated<PresentableArgument<LANG, SPEC>, SEP>;
pub type AspectPresentableArguments<SEP, LANG, SPEC> = (Aspect<<SPEC as Specification<LANG>>::TYC>, PresentableArguments<SEP, LANG, SPEC>);
pub type AspectCommaPunctuatedArguments<LANG, SPEC> = AspectPresentableArguments<Comma, LANG, SPEC>;
pub type ArgComposers<SEP, LANG, SPEC> = Punctuated<FieldComposer<LANG, SPEC>, SEP>;
pub type FieldComposers<LANG, SPEC> = CommaPunctuated<FieldComposer<LANG, SPEC>>;
pub type TerminatedArguments<LANG, SPEC> = PresentableArguments<Semi, LANG, SPEC>;
pub type LocallyOwnedFieldComposers<T, LANG, SPEC> = ((T, FieldComposers<LANG, SPEC>), <SPEC as Specification<LANG>>::Gen);
pub type LocalConversionContext<LANG, SPEC> = LocallyOwnedFieldComposers<Aspect<<SPEC as Specification<LANG>>::TYC>, LANG, SPEC>;
pub type ConstructorFieldsContext<LANG, SPEC> = LocallyOwnedFieldComposers<(DestructorContext<LANG, SPEC>, bool), LANG, SPEC>;
pub type BindingAccessorContext<LANG, SPEC> = (
    <Aspect<<SPEC as Specification<LANG>>::TYC> as ScopeContextPresentable>::Presentation,
    TokenStream2,
    Type,
    <SPEC as Specification<LANG>>::Attr,
    <SPEC as Specification<LANG>>::Gen
);
pub type DestructorContext<LANG, SPEC> = (
    <Aspect<<SPEC as Specification<LANG>>::TYC> as ScopeContextPresentable>::Presentation,
    <SPEC as Specification<LANG>>::Attr,
    <SPEC as Specification<LANG>>::Gen
);
pub type FieldTypeLocalContext<LANG, SPEC> = (Name, ConversionType<LANG, SPEC>);
pub type FunctionContext<LANG, SPEC> = ((DestructorContext<LANG, SPEC>, bool), Vec<PresentableArgumentPair<LANG, SPEC>>);
pub type PresentableArgumentPair<LANG, SPEC> = (PresentableArgument<LANG, SPEC>, PresentableArgument<LANG, SPEC>);

pub type FieldsSequenceExprMixer<Link, Context, Statement, LANG, SPEC> = SequenceMixer<
    Link,
    Context,
    FieldTypeLocalContext<LANG, SPEC>,
    <SPEC as Specification<LANG>>::Expr,
    Statement,
    PresentableSequence<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>,
    PresentableSequence<LANG, SPEC>
>;

pub type FFIConversionExprMixer<Link, LANG, SPEC> = FieldsSequenceExprMixer<
    Link,
    LocalConversionContext<LANG, SPEC>,
    AspectCommaPunctuatedArguments<LANG, SPEC>,
    LANG,
    SPEC,
>;
pub type DropSequenceExprMixer<Link, LANG, SPEC> = FieldsSequenceExprMixer<
    Link,
    FieldComposers<LANG, SPEC>,
    TerminatedArguments<LANG, SPEC>,
    LANG,
    SPEC,
>;
pub type FieldsSequenceComposer<Link, OwnerAspect, B, C, Presentable, LANG, SPEC> = SequenceComposer<
    Link,
    OwnerAspect,
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
#[allow(unused)]
pub type CommaPunctuatedArgs = CommaPunctuated<ArgPresentation>;
#[allow(unused)]
pub type CommaPunctuatedOwnedItems<LANG, SPEC> = PresentableArguments<Comma, LANG, SPEC>;
#[allow(unused)]
pub type CommaPunctuatedFields = CommaPunctuated<Field>;
#[allow(unused)]
pub type CommaPunctuatedNestedArguments = CommaPunctuated<NestedArgument>;
pub type FieldPathResolver<LANG, SPEC> = ComposerPresenterByRef<FieldComposer<LANG, SPEC>, FieldTypeLocalContext<LANG, SPEC>>;
pub type AspectSequenceComposer<LANG, SPEC> = ComposerPresenter<AspectCommaPunctuatedArguments<LANG, SPEC>, PresentableSequence<LANG, SPEC>>;
pub type FieldComposerProducer<LANG, SPEC, OUT> = ComposerPresenterByRef<FieldComposer<LANG, SPEC>, OUT>;

