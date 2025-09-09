use syn::{BareFnArg, GenericArgument, ReturnType, Type};
use crate::composable::NestedArgument;
use crate::ext::MaybeGenericType;

#[allow(unused)]
pub trait RefineWithNestedArg {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool;
}

/// Refinement of the actual types with refined nested arguments
/// Nested argument should be refined before
impl RefineWithNestedArg for GenericArgument {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        self.maybe_generic_type_mut().map(|inner_ty| if let Some(ty) = nested_argument.object().maybe_type() {
            *inner_ty = ty;
            true
        } else {
            false
        }).unwrap_or_default()
    }
}

impl RefineWithNestedArg for BareFnArg {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        self.ty.refine_with_nested_arg(nested_argument)
    }
}

impl RefineWithNestedArg for ReturnType {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        match self {
            ReturnType::Type(_, inner_ty) => {
                if let Some(ty) = nested_argument.maybe_type() {
                    *inner_ty = Box::new(ty);
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }
}

impl RefineWithNestedArg for Type {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        if let Some(ty) = nested_argument.maybe_type() {
            *self = ty;
            true
        } else {
            false
        }
    }
}
