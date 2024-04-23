mod constraints;
mod collection;
// mod prefix;
mod nested_arguments;
mod refine;
mod resolve;
pub mod visitor;
mod present;
mod r#abstract;

pub use self::collection::ScopeCollection;
pub use self::constraints::Constraints;
pub use self::r#abstract::{Accessory, HashMapMergePolicy, ItemHelper, Join, MergeInto, MergePolicy, Pop, ToPath, ToType, ValueReplaceScenario};
pub use self::present::{Conversion, Mangle, MangleDefault, Terminated};
pub use self::refine::{Refine, RefineMut, RefineAtScope, RefineUnrefined, Unrefined};
pub use self::resolve::{FFIResolveExtended, ffi_chunk_converted, ffi_external_chunk, FFIResolve, Resolve, ResolveTrait};
pub use self::visitor::nesting::NestingExtension;
pub use self::visitor::visit_scope::{add_trait_names, create_generics_chain, extract_trait_names, VisitScope};
pub use self::visitor::visit_scope_type::{ToObjectConversion, VisitScopeType};

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