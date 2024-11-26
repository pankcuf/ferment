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
pub(crate) mod r#abstract;
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
#[allow(unused)]
mod scope_search;

use std::rc::Rc;
use syn::__private::TokenStream2;
use syn::{Field, Type};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use crate::ast::CommaPunctuated;
use crate::composable::{FieldComposer, NestedArgument};
use crate::composer::r#abstract::{SequenceComposer, SequenceMixer};
use crate::composer::vtable::VTableComposer;
use crate::ext::ConversionType;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, ArgKind, SeqKind};
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
pub type SourceComposerByRef<S, T, U> = fn(source: &S, context: T) -> U;
pub type SourceContextComposerByRef<S, C, M, U> = fn(source: &S, composer: ComposerByRef<C, M>) -> U;
pub type SharedComposer<Link, U> = ComposerByRef<<Link as SharedAccess>::ImmutableAccess, U>;
pub type FieldComposerProducer<LANG, SPEC> = SourceComposerByRef<Field, usize, FieldComposer<LANG, SPEC>>;
pub type AspectArgSourceComposer<LANG, SPEC, Iter> = SourceComposerByRef<AspectArgComposers<LANG, SPEC>, ArgProducerByRef<LANG, SPEC, <Iter as IntoIterator>::Item>, OwnerAspectSequence<LANG, SPEC, Iter>>;
pub type SharedAspectArgComposer<LANG, SPEC, Link> = SharedComposer<Link, AspectArgComposers<LANG, SPEC>>;
pub type SharedComposerLink<C, U> = SharedComposer<ComposerLink<C>, U>;
pub type ItemComposerLink<LANG, SPEC, I> = ComposerLink<ItemComposer<LANG, SPEC, I>>;
pub type EnumVariantComposerLink<LANG, SPEC, I> = ComposerLink<EnumVariantComposer<LANG, SPEC, I>>;
pub type StructComposerLink<LANG, SPEC, I> = ComposerLink<StructComposer<LANG, SPEC, I>>;
pub type OpaqueStructComposerLink<LANG, SPEC, I> = ComposerLink<OpaqueStructComposer<LANG, SPEC, I>>;
pub type TypeAliasComposerLink<LANG, SPEC, I> = ComposerLink<TypeAliasComposer<LANG, SPEC, I>>;
pub type EnumComposerLink<LANG, SPEC> = ComposerLink<EnumComposer<LANG, SPEC>>;
pub type SigComposerLink<LANG, SPEC> = ComposerLink<SigComposer<LANG, SPEC>>;
pub type VTableComposerLink<LANG, SPEC> = ComposerLink<VTableComposer<LANG, SPEC>>;
pub type ImplComposerLink<LANG, SPEC> = ComposerLink<ImplComposer<LANG, SPEC>>;
pub type TraitComposerLink<LANG, SPEC> = ComposerLink<TraitComposer<LANG, SPEC>>;
// pub type TypeContextComposer<Link, TYC, U> = LinkedContextComposer<Link, Aspect<TYC>, U>;
pub type BiLinkedContextComposer<L, T> = LinkedContextComposer<L, T, T>;
pub type SeqKindComposer<LANG, SPEC, L> = BiLinkedContextComposer<L, SeqKind<LANG, SPEC>>;
pub type MaybeSequenceOutputComposer<LANG, SPEC, L> = Option<SeqKindComposer<LANG, SPEC, L>>;
pub type MaybeSequenceOutputComposerLink<LANG, SPEC, T> = Option<SeqKindComposer<LANG, SPEC, ComposerLink<T>>>;
pub type SeqKindComposerLink<LANG, SPEC, T> = SeqKindComposer<LANG, SPEC, ComposerLink<T>>;
pub type VariantComposerRef<LANG, SPEC> = ComposerByRef<AspectCommaPunctuatedArguments<LANG, SPEC>, SeqKind<LANG, SPEC>>;
pub type PresentableArgumentPairComposerRef<LANG, SPEC> = ArgProducerByRef<LANG, SPEC, ArgKindPair<LANG, SPEC>>;
pub type FieldsComposerRef<LANG, SPEC> = ComposerByRef<CommaPunctuatedFields, CommaArgComposers<LANG, SPEC>>;
pub type PresentableExprComposerRef<LANG, SPEC> = ComposerByRef<FieldTypeLocalContext<LANG, SPEC>, <SPEC as Specification<LANG>>::Expr>;
pub type PresentableArgumentComposerRef<LANG, SPEC> = ArgProducerByRef<LANG, SPEC, ArgKind<LANG, SPEC>>;
#[allow(unused)]
pub type OwnedFieldsIterator<LANG, SPEC> =
    IterativeComposer<
        OwnedArgComposers<
            LANG,
            SPEC,
            OwnerAspect<LANG, SPEC>
        >,
        FieldComposer<LANG, SPEC>,
        ArgKind<LANG, SPEC>,
        OwnerAspectSequence<LANG, SPEC, CommaPunctuatedArgKinds<LANG, SPEC>>>;
pub type BindingComposer<LANG, SPEC, T> = Composer<T, BindingPresentableContext<LANG, SPEC>>;
pub type PunctuatedArgKinds<LANG, SPEC, SEP> = Punctuated<ArgKind<LANG, SPEC>, SEP>;

pub type OwnerAspect<LANG, SPEC> = (Aspect<<SPEC as Specification<LANG>>::TYC>, <SPEC as Specification<LANG>>::Attr, <SPEC as Specification<LANG>>::Gen, NameKind);
pub type OwnerAspectSequence<LANG, SPEC, T> = (OwnerAspect<LANG, SPEC>, T);
pub type OwnerAspectSequenceComposer<LANG, SPEC, T, U> = Composer<OwnerAspectSequence<LANG, SPEC, T>, U>;
pub type PresentableArgsSequenceComposer<LANG, SPEC> = OwnerAspectSequenceComposer<LANG, SPEC, CommaPunctuatedArgKinds<LANG, SPEC>, SeqKind<LANG, SPEC>>;
pub type OwnedArgComposers<LANG, SPEC, T> = (T, CommaArgComposers<LANG, SPEC>);
pub type AspectPresentableArguments<LANG, SPEC, SEP> = OwnerAspectSequence<LANG, SPEC, PunctuatedArgKinds<LANG, SPEC, SEP>>;
pub type AspectArgComposers<LANG, SPEC> = OwnerAspectSequence<LANG, SPEC, CommaArgComposers<LANG, SPEC>>;
pub type AspectCommaPunctuatedArguments<LANG, SPEC> = AspectPresentableArguments<LANG, SPEC, Comma>;
pub type AspectTerminatedArguments<LANG, SPEC> = AspectPresentableArguments<LANG, SPEC, Semi>;
pub type ArgComposers<LANG, SPEC, SEP> = Punctuated<FieldComposer<LANG, SPEC>, SEP>;
pub type CommaArgComposers<LANG, SPEC> = ArgComposers<LANG, SPEC, Comma>;

pub type BindingAccessorContext<LANG, SPEC> = (
    Aspect<<SPEC as Specification<LANG>>::TYC>,
    <SPEC as Specification<LANG>>::Attr,
    <SPEC as Specification<LANG>>::Gen,
    <SPEC as Specification<LANG>>::Var,
    TokenStream2,
);
pub type FieldTypeLocalContext<LANG, SPEC> = (
    <SPEC as Specification<LANG>>::Name,
    ConversionType<LANG, SPEC>
);
pub type ArgKindPair<LANG, SPEC> = (ArgKind<LANG, SPEC>, ArgKind<LANG, SPEC>);
pub type TypePair = (Type, Type);
pub type ArgKindPairs<LANG, SPEC> = Vec<ArgKindPair<LANG, SPEC>>;
pub type CommaPunctuatedArgs = CommaPunctuated<ArgPresentation>;
pub type CommaPunctuatedArgKinds<LANG, SPEC> = PunctuatedArgKinds<LANG, SPEC, Comma>;
pub type CommaPunctuatedFields = CommaPunctuated<Field>;
pub type CommaPunctuatedNestedArguments = CommaPunctuated<NestedArgument>;
pub type FieldPathResolver<LANG, SPEC> = ComposerByRef<FieldComposer<LANG, SPEC>, FieldTypeLocalContext<LANG, SPEC>>;
pub type AspectSeqKindComposer<LANG, SPEC, SEP> = Composer<
    AspectPresentableArguments<LANG, SPEC, SEP>,
    SeqKind<LANG, SPEC>
>;
pub type ConversionSeqKindComposer<LANG, SPEC> = AspectSeqKindComposer<LANG, SPEC, Comma>;
pub type DropSeqKindComposer<LANG, SPEC> = AspectSeqKindComposer<LANG, SPEC, Semi>;
pub type ArgProducerByRef<LANG, SPEC, OUT> = ComposerByRef<FieldComposer<LANG, SPEC>, OUT>;
pub type ArgKindProducerByRef<LANG, SPEC> = ArgProducerByRef<LANG, SPEC, ArgKind<LANG, SPEC>>;
pub type FieldsSequenceMixer<LANG, SPEC, Link, Context, Statement> = SequenceMixer<
    Link,
    Context,
    FieldTypeLocalContext<LANG, SPEC>,
    <SPEC as Specification<LANG>>::Expr,
    Statement,
    SeqKind<LANG, SPEC>,
    SeqKind<LANG, SPEC>,
    SeqKind<LANG, SPEC>
>;
pub type FFIConversionsMixer<LANG, SPEC, Link> = InterfaceSequenceMixer<LANG, SPEC, Link, Comma>;
pub type DropSequenceMixer<LANG, SPEC, Link> = InterfaceSequenceMixer<LANG, SPEC, Link, Semi>;

pub type InterfaceSequenceMixer<LANG, SPEC, Link, SEP> = FieldsSequenceMixer<
    LANG,
    SPEC,
    Link,
    AspectArgComposers<LANG, SPEC>,
    AspectPresentableArguments<LANG, SPEC, SEP>,
>;
pub type ArgsSequenceComposer<LANG, SPEC, Link, A, B, C, Presentable> = SequenceComposer<Link, A, FieldComposer<LANG, SPEC>, B, C, Presentable>;
pub type OwnerAspectSequenceSpecComposer<LANG, SPEC, Link, Iter, Out> = ArgsSequenceComposer<
    LANG,
    SPEC,
    Link,
    AspectArgComposers<LANG, SPEC>,
    <Iter as IntoIterator>::Item,
    OwnerAspectSequence<LANG, SPEC, Iter>,
    Out
>;
pub type FieldsOwnedSequenceComposer<LANG, SPEC, Link> = ArgsSequenceComposer<
    LANG,
    SPEC,
    Link,
    AspectArgComposers<LANG, SPEC>,
    ArgKind<LANG, SPEC>,
    AspectCommaPunctuatedArguments<LANG, SPEC>,
    SeqKind<LANG, SPEC>,
>;
pub type FieldsOwnedSequenceComposerLink<LANG, SPEC, T> = FieldsOwnedSequenceComposer<
    LANG,
    SPEC,
    ComposerLink<T>
>;
pub type AspectSharedComposerLink<LANG, SPEC, T> = SharedComposerLink<T, AspectArgComposers<LANG, SPEC>>;
pub type SequenceSharedComposerLink<LANG, SPEC, T> = SharedComposerLink<T, SeqKind<LANG, SPEC>>;

pub type RootSequenceComposer<LANG, SPEC> = SourceComposerByRef<SeqKind<LANG, SPEC>, SeqKind<LANG, SPEC>, SeqKind<LANG, SPEC>>;

