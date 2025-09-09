use syn::{AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, PathArguments, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::ext::{RefineWithNestedArg, ToPath};
use crate::ext::maybe_generic_type::MaybeGenericType;

pub trait RefineWithNestedArgs {
    fn refine_with_nested_args(&mut self, nested_arguments: &CommaPunctuatedNestedArguments) -> bool;
}

impl RefineWithNestedArgs for Type {
    fn refine_with_nested_args(&mut self, nested_arguments: &CommaPunctuatedNestedArguments) -> bool {
        let mut did_refine = false;
        match self {
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                let mut refined_nested_args = nested_arguments.clone();
                inputs.iter_mut().for_each(|inner_ty| {
                    match refined_nested_args.pop() {
                        Some(refined_nested_arg) if inner_ty.refine_with_nested_arg(refined_nested_arg.value()) =>
                            did_refine = true,
                        _ => {}
                    }
                });
                match refined_nested_args.pop() {
                    Some(refined_nested_arg) if output.refine_with_nested_arg(refined_nested_arg.value()) =>
                        did_refine = true,
                    _ => {}
                }

            }
            Type::Path(TypePath { path, .. }) => {
                if let Some(last_segment) = path.segments.last_mut() {
                    if last_segment.arguments.refine_with_nested_args(nested_arguments) {
                        did_refine = true;
                    }
                }
            }
            Type::Tuple(type_tuple) => {
                type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| if elem.refine_with_nested_arg(&nested_arguments[index]) {
                    did_refine = true;
                });
            },
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                bounds.iter_mut().enumerate().for_each(|(index, elem)| if let TypeParamBound::Trait(TraitBound { path, .. }) = elem {
                    if let Some(ty) = &nested_arguments[index].maybe_type() {
                        *path = ty.to_path();
                        did_refine = true;
                    }
                });
            }
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Array(TypeArray { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => {
                if let Some(refined_nested_arg) = &nested_arguments.first() {
                    if elem.refine_with_nested_arg(refined_nested_arg) {
                        did_refine = true;
                    }
                }
            },
            _ => {}
        }
        did_refine
    }
}

impl RefineWithNestedArgs for PathArguments {
    fn refine_with_nested_args(&mut self, nested_arguments: &CommaPunctuatedNestedArguments) -> bool {
        let mut did_refine = false;
        match self {
            PathArguments::None => {}
            PathArguments::Parenthesized(ParenthesizedGenericArguments { ref mut inputs, ref mut output, .. }) => {
                inputs.iter_mut().enumerate().for_each(|(index, inner_ty)| if inner_ty.refine_with_nested_arg(&nested_arguments[index]) {
                    did_refine = true;
                });
                if let Some(last) = nested_arguments.last() {
                    if output.refine_with_nested_arg(last) {
                        did_refine = true;
                    }
                }
            },
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { ref mut args, .. }) =>
                args.iter_mut()
                    .filter_map(GenericArgument::maybe_generic_type_mut)
                    .enumerate()
                    .for_each(|(index, arg)| if arg.refine_with_nested_arg(&nested_arguments[index]) {
                        did_refine = true;
                    })
        }
        did_refine
    }
}





