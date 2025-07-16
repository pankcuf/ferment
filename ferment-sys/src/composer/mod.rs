mod attrs;
mod ffi_conversions;
mod item;
mod method;
mod constants;
mod enum_composer;
mod r#type;
mod generic;
mod signature;
mod basic;
mod trait_composer;
mod opaque_struct;
mod ffi_bindings;
mod generics_composer;
pub(crate) mod r#abstract;
mod variable;
mod conversion_from;
mod conversion_to;
mod conversion_drop;
mod callback;
mod result;
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
#[allow(unused)]
mod scope_search;
mod lifetimes;
mod array;
mod target_var;
mod var;

use std::rc::Rc;
use syn::__private::TokenStream2;
use syn::{Field, Type};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use crate::ast::{CommaPunctuated, SemiPunctuated};
use crate::composable::{FieldComposer, NestedArgument};
use crate::composer::r#abstract::{SequenceComposer, SequenceMixer};
use crate::composer::vtable::VTableComposer;
use crate::ext::Conversion;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, ArgKind, SeqKind};
use crate::presentation::ArgPresentation;
use crate::shared::SharedAccess;

pub use self::r#abstract::*;
pub use self::any_other::*;
#[allow(unused)]
pub use self::array::*;
pub use self::attrs::*;
pub use self::basic::*;
pub use self::bounds::*;
pub use self::callback::*;
pub use self::constants::*;
pub use self::conversion_drop::*;
pub use self::enum_composer::*;
pub use self::enum_variant::*;
pub use self::ffi_bindings::*;
pub use self::ffi_conversions::*;
pub use self::conversion_from::*;
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
#[allow(unused)]
pub use self::target_var::*;
pub use self::conversion_to::*;
pub use self::trait_composer::*;
pub use self::tuple::*;
pub use self::r#type::TypeComposer;
pub use self::type_alias::*;
#[allow(unused)]
pub use self::var::*;
pub use self::variable::*;

/// Composer Context Presenters
pub type ComposerLink<T> = Rc<std::cell::RefCell<T>>;
pub type ComposerLinkRef<'a, T> = std::cell::Ref<'a, T>;
pub type Composer<T, U> = fn(context: T) -> U;
pub type ComposerByRef<T, U> = fn(context: &T) -> U;
pub type SourceComposerByRef<S, T, U> = fn(source: &S, context: T) -> U;
pub type SourceContextComposerByRef<S, C, M, U> = fn(source: &S, composer: ComposerByRef<C, M>) -> U;
pub type SharedComposer<Link, U> = ComposerByRef<<Link as SharedAccess>::ImmutableAccess, U>;
pub type FieldComposerProducer<SPEC> = SourceComposerByRef<Field, usize, FieldComposer<SPEC>>;
pub type AspectArgSourceComposer<SPEC, Iter> = SourceComposerByRef<AspectArgComposers<SPEC>, ArgProducerByRef<SPEC, <Iter as IntoIterator>::Item>, OwnerAspectSequence<SPEC, Iter>>;
pub type SharedAspectArgComposer<SPEC, Link> = SharedComposer<Link, AspectArgComposers<SPEC>>;
pub type SharedComposerLink<C, U> = SharedComposer<ComposerLink<C>, U>;
pub type ItemComposerLink<SPEC, I> = ComposerLink<ItemComposer<SPEC, I>>;
pub type EnumVariantComposerLink<SPEC, I> = ComposerLink<EnumVariantComposer<SPEC, I>>;
pub type StructComposerLink<SPEC, I> = ComposerLink<StructComposer<SPEC, I>>;
pub type OpaqueStructComposerLink<SPEC, I> = ComposerLink<OpaqueStructComposer<SPEC, I>>;
pub type TypeAliasComposerLink<SPEC, I> = ComposerLink<TypeAliasComposer<SPEC, I>>;
pub type EnumComposerLink<SPEC> = ComposerLink<EnumComposer<SPEC>>;
pub type SigComposerLink<SPEC> = ComposerLink<SigComposer<SPEC>>;
pub type VTableComposerLink<SPEC> = ComposerLink<VTableComposer<SPEC>>;
pub type ImplComposerLink<SPEC> = ComposerLink<ImplComposer<SPEC>>;
pub type TraitComposerLink<SPEC> = ComposerLink<TraitComposer<SPEC>>;
pub type BiLinkedContextComposer<L, T> = LinkedContextComposer<L, T, T>;
pub type SeqKindComposer<SPEC, L> = BiLinkedContextComposer<L, SeqKind<SPEC>>;
pub type MaybeSequenceOutputComposer<SPEC, L> = Option<SeqKindComposer<SPEC, L>>;
pub type MaybeSequenceOutputComposerLink<SPEC, T> = Option<SeqKindComposer<SPEC, ComposerLink<T>>>;
pub type SeqKindComposerLink<SPEC, T> = SeqKindComposer<SPEC, ComposerLink<T>>;
pub type VariantComposerRef<SPEC> = ComposerByRef<AspectCommaPunctuatedArguments<SPEC>, SeqKind<SPEC>>;
pub type PresentableArgumentPairComposerRef<SPEC> = ArgProducerByRef<SPEC, ArgKindPair<SPEC>>;
pub type FieldsComposerRef<SPEC> = ComposerByRef<CommaPunctuatedFields, CommaArgComposers<SPEC>>;
pub type PresentableExprComposerRef<SPEC> = ComposerByRef<FieldTypeLocalContext<SPEC>, <SPEC as Specification>::Expr>;
pub type PresentableArgumentComposerRef<SPEC> = ArgProducerByRef<SPEC, ArgKind<SPEC>>;
#[allow(unused)]
pub type OwnedFieldsIterator<SPEC> =
    IterativeComposer<
        OwnedArgComposers<SPEC, OwnerAspect<SPEC>>,
        FieldComposer<SPEC>,
        ArgKind<SPEC>,
        OwnerAspectSequence<SPEC, CommaPunctuatedArgKinds<SPEC>>>;
pub type BindingComposer<SPEC, T> = Composer<T, BindingPresentableContext<SPEC>>;
pub type PunctuatedArgKinds<SPEC, SEP> = Punctuated<ArgKind<SPEC>, SEP>;
pub type OwnerAspect<SPEC> = (Aspect<<SPEC as Specification>::TYC>, <SPEC as Specification>::Attr, <SPEC as Specification>::Gen, NameKind);
pub type OwnerAspectSequence<SPEC, T> = (OwnerAspect<SPEC>, T);
pub type OwnerAspectSequenceComposer<SPEC, T, U> = Composer<OwnerAspectSequence<SPEC, T>, U>;
pub type PresentableArgsSequenceComposer<SPEC> = OwnerAspectSequenceComposer<SPEC, CommaPunctuatedArgKinds<SPEC>, SeqKind<SPEC>>;
pub type OwnedArgComposers<SPEC, T> = (T, CommaArgComposers<SPEC>);
pub type AspectPresentableArguments<SPEC, SEP> = OwnerAspectSequence<SPEC, PunctuatedArgKinds<SPEC, SEP>>;
pub type AspectArgComposers<SPEC> = OwnerAspectSequence<SPEC, CommaArgComposers<SPEC>>;
pub type AspectCommaPunctuatedArguments<SPEC> = AspectPresentableArguments<SPEC, Comma>;
pub type AspectTerminatedArguments<SPEC> = AspectPresentableArguments<SPEC, Semi>;
pub type ArgComposers<SPEC, SEP> = Punctuated<FieldComposer<SPEC>, SEP>;
pub type CommaArgComposers<SPEC> = ArgComposers<SPEC, Comma>;

pub type BindingAccessorContext<SPEC> = (
    Aspect<<SPEC as Specification>::TYC>,
    <SPEC as Specification>::Attr,
    <SPEC as Specification>::Gen,
    VariableComposer<SPEC>,
    TokenStream2,
);
pub type FieldTypeLocalContext<SPEC> = (<SPEC as Specification>::Name, Conversion<SPEC>);
pub type ArgKindPair<SPEC> = (ArgKind<SPEC>, ArgKind<SPEC>);
pub type TypePair = (Type, Type);
pub type ArgKindPairs<SPEC> = Vec<ArgKindPair<SPEC>>;
pub type CommaPunctuatedArgs = CommaPunctuated<ArgPresentation>;
pub type SemiPunctuatedArgs = SemiPunctuated<ArgPresentation>;
pub type CommaPunctuatedArgKinds<SPEC> = PunctuatedArgKinds<SPEC, Comma>;
pub type SemiPunctuatedArgKinds<SPEC> = PunctuatedArgKinds<SPEC, Semi>;
pub type CommaPunctuatedFields = CommaPunctuated<Field>;
pub type CommaPunctuatedNestedArguments = CommaPunctuated<NestedArgument>;
pub type FieldPathResolver<SPEC> = ComposerByRef<FieldComposer<SPEC>, FieldTypeLocalContext<SPEC>>;
pub type AspectSeqKindComposer<SPEC, SEP> = Composer<AspectPresentableArguments<SPEC, SEP>, SeqKind<SPEC>>;
pub type ConversionSeqKindComposer<SPEC> = AspectSeqKindComposer<SPEC, Comma>;
pub type DropSeqKindComposer<SPEC> = AspectSeqKindComposer<SPEC, Semi>;
pub type ArgProducerByRef<SPEC, OUT> = ComposerByRef<FieldComposer<SPEC>, OUT>;
pub type ArgKindProducerByRef<SPEC> = ArgProducerByRef<SPEC, ArgKind<SPEC>>;
pub type FieldsSequenceMixer<SPEC, Link, Context, Statement> = SequenceMixer<
    Link,
    Context,
    FieldTypeLocalContext<SPEC>,
    <SPEC as Specification>::Expr,
    Statement,
    SeqKind<SPEC>,
    SeqKind<SPEC>,
    SeqKind<SPEC>
>;
pub type FFIConversionsMixer<SPEC, Link> = InterfaceSequenceMixer<SPEC, Link, Comma>;
pub type DropSequenceMixer<SPEC, Link> = InterfaceSequenceMixer<SPEC, Link, Semi>;

pub type InterfaceSequenceMixer<SPEC, Link, SEP> = FieldsSequenceMixer<
    SPEC,
    Link,
    AspectArgComposers<SPEC>,
    AspectPresentableArguments<SPEC, SEP>,
>;
pub type ArgsSequenceComposer<SPEC, Link, A, B, C, Presentable> = SequenceComposer<Link, A, FieldComposer<SPEC>, B, C, Presentable>;
pub type OwnerAspectSequenceSpecComposer<SPEC, Link, Iter, Out> = ArgsSequenceComposer<
    SPEC,
    Link,
    AspectArgComposers<SPEC>,
    <Iter as IntoIterator>::Item,
    OwnerAspectSequence<SPEC, Iter>,
    Out
>;
pub type FieldsOwnedSequenceComposer<SPEC, Link> = ArgsSequenceComposer<
    SPEC,
    Link,
    AspectArgComposers<SPEC>,
    ArgKind<SPEC>,
    AspectCommaPunctuatedArguments<SPEC>,
    SeqKind<SPEC>,
>;
pub type FieldsOwnedSequenceComposerLink<SPEC, T> = FieldsOwnedSequenceComposer<SPEC, ComposerLink<T>>;
pub type AspectSharedComposerLink<SPEC, T> = SharedComposerLink<T, AspectArgComposers<SPEC>>;
pub type SequenceSharedComposerLink<SPEC, T> = SharedComposerLink<T, SeqKind<SPEC>>;
pub type RootSequenceComposer<SPEC> = SourceComposerByRef<SeqKind<SPEC>, SeqKind<SPEC>, SeqKind<SPEC>>;

