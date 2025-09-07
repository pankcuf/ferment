mod attrs;
mod field_type;
mod function;
mod generic_bounds;
mod nested_arg;
mod type_model;
mod trait_model;
mod traits;

use syn::punctuated::IterMut;
use syn::Type;
pub use attrs::*;
pub use field_type::*;
pub use function::*;
pub use generic_bounds::*;
pub use nested_arg::*;
pub use traits::*;
pub use trait_model::*;
pub use type_model::*;
use crate::composer::CommaPunctuatedNestedArguments;

pub trait TypeModeled {
    fn type_model_mut(&mut self) -> &mut TypeModel;
    fn type_model_ref(&self) -> &TypeModel;
    fn type_model_and_nested_arguments_mut(&mut self) -> (&mut Type, &mut CommaPunctuatedNestedArguments) {
        let model = self.type_model_mut();
        (&mut model.ty, &mut model.nested_arguments)
    }
    fn nested_arguments_mut(&mut self) -> &mut CommaPunctuatedNestedArguments {
        &mut self.type_model_mut().nested_arguments
    }
    fn nested_arguments_iter_mut(&mut self) -> IterMut<NestedArgument> {
        self.nested_arguments_mut().iter_mut()
    }
    fn nested_arguments_ref(&self) -> &CommaPunctuatedNestedArguments {
        &self.type_model_ref().nested_arguments
    }
    fn ty_mut(&mut self) -> &mut Type {
        &mut self.type_model_mut().ty
    }
    fn replace_model_type(&mut self, with_ty: Type) {
        *self.ty_mut() = with_ty;
    }
}
