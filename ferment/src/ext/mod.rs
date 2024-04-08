mod constraints;
mod nesting;
mod conversion;
mod composition;
mod merge;
mod scope;
mod collection;
mod ffi_resolver;
mod item_helper;
mod local_connections;
mod visiting;
mod join;
mod pop;
mod mangle;
mod accessory;
mod terminated;
mod resolve_trait;
mod visit_scope_type;
mod prefix;
mod nested_arguments;
mod refine;
mod to_type;

pub use self::accessory::Accessory;
pub use self::collection::ScopeCollection;
pub use self::conversion::Conversion;
pub use self::ffi_resolver::{FFIResolveExtended, ffi_chunk_converted, ffi_external_chunk, FFIResolve};
pub use self::mangle::{Mangle, MangleDefault};
pub use self::nesting::NestingExtension;
pub use self::constraints::Constraints;
pub use self::item_helper::ItemHelper;
pub use self::join::Join;
pub use self::merge::HashMapMergePolicy;
pub use self::merge::MergeInto;
pub use self::merge::MergePolicy;
pub use self::merge::ValueReplaceScenario;
pub use self::pop::Pop;
pub use self::prefix::Prefix;
pub use self::refine::{Refine, RefineMut, RefineAtScope, RefineUnrefined, Unrefined};
pub use self::terminated::Terminated;
pub use self::resolve_trait::ResolveTrait;
pub use self::to_type::{ToPath, ToType};
pub use self::visiting::add_trait_names;
pub use self::visiting::create_generics_chain;
pub use self::visiting::extract_trait_names;
pub use self::visiting::Visiting;
pub use self::visit_scope_type::{ToObjectConversion, VisitScopeType};

pub trait CrateExtension {
    fn is_crate_based(&self) -> bool;
    fn crate_less(&self) -> Self;
    fn ident_less(&self) -> Self;
    fn crate_and_ident_less(&self) -> Self;
    fn replace_first_with(&mut self, chunk: &Self);
    fn replaced_first_with_ident(&self, chunk: &Self) -> Self;
    fn replace_last_with(&mut self, chunk: &Self);
    fn replaced_last_with(&self, chunk: &Self) -> Self;

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self);
    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self;

}