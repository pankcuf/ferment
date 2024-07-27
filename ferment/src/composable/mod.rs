mod field_type;
mod function;
mod type_composition;
mod import;
mod qself;
mod generic;
mod traits;
mod attrs;
mod context;
mod generic_bounds;
mod trait_vtable;
mod nested_arg;

pub use attrs::*;
pub use context::*;
pub use field_type::*;
pub use function::*;
pub use generic::*;
pub use generic_bounds::*;
pub use import::*;
pub use nested_arg::*;
pub use qself::*;
pub use traits::*;
#[allow(unused)]
pub use trait_vtable::*;
pub use type_composition::*;

pub trait CompositionContext {}

pub trait Composition: Clone {
    type Context: CompositionContext;
    type Presentation;
    fn present(self, composition_context: Self::Context, context: &crate::context::ScopeContext) -> Self::Presentation;
}
