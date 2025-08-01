mod field_type;
mod function;
mod type_model;
mod qself;
mod traits;
mod attrs;
mod generic_bounds;
mod nested_arg;
mod trait_model;

pub use attrs::*;
pub use field_type::*;
pub use function::*;
pub use generic_bounds::*;
pub use nested_arg::*;
pub use qself::*;
pub use traits::*;
pub use trait_model::*;
pub use type_model::*;

#[allow(unused)]
pub trait CompositionContext {}

#[allow(unused)]
pub trait Composition: Clone {
    type Context: CompositionContext;
    type Presentation;
    fn present(self, composition_context: Self::Context, context: &crate::context::ScopeContext) -> Self::Presentation;
}
