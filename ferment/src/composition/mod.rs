mod function_composition;
mod type_composition;
mod import_composition;
mod qself_composition;
mod generic_composition;
mod traits;
mod item_composition;
mod attrs_composition;
pub mod context;
mod generic_bound_composition;
mod smart_pointer_composition;
mod trait_vtable_composition;

pub use attrs_composition::{AttrsComposition, CfgAttributes};
pub use type_composition::{TypeComposition, NestedArgument};
pub use function_composition::FnArgComposer;
pub use function_composition::FnReturnTypeComposer;
pub use function_composition::FnSignatureComposition;
pub use function_composition::FnSignatureContext;
pub use generic_bound_composition::GenericBoundComposition;
pub use generic_composition::GenericConversion;
pub use import_composition::ImportComposition;
pub use import_composition::{create_item_use_with_tree, create_items_use_from_path};
pub use qself_composition::QSelfComposition;
pub use traits::TraitCompositionPart1;
pub use traits::TraitDecompositionPart1;
pub use traits::TraitDecompositionPart2;
pub use traits::TraitTypeDecomposition;
pub use traits::TraitBoundDecomposition;
pub use trait_vtable_composition::TraitVTableComposition;
pub use trait_vtable_composition::TraitVTableMethodComposition;
use crate::context::ScopeContext;

pub trait CompositionContext {}

pub trait Composition: Clone {
    type Context: CompositionContext;
    type Presentation;
    fn present(self, composition_context: Self::Context, context: &ScopeContext) -> Self::Presentation;
}
