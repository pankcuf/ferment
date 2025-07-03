mod field_type;
mod function;
mod type_composition;
mod qself;
mod generic;
mod traits;
mod attrs;
mod generic_bounds;
mod nested_arg;

pub use attrs::*;
pub use field_type::*;
pub use function::*;
pub use generic::*;
pub use generic_bounds::*;
pub use nested_arg::*;
pub use qself::*;
pub use traits::*;
pub use type_composition::*;

#[allow(unused)]
pub trait CompositionContext {}

#[allow(unused)]
pub trait Composition: Clone {
    type Context: CompositionContext;
    type Presentation;
    fn present(self, composition_context: Self::Context, context: &crate::context::ScopeContext) -> Self::Presentation;
}
