mod refine;
mod lifetime;

pub use lifetime::*;
pub use refine::*;
use syn::{AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, parse_quote, PathArguments, ReturnType, TraitBound, Type, TypeImplTrait, TypeParamBound, TypePath, TypeTraitObject, TypeTuple};
use crate::composable::NestedArgument;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeChain;

pub trait RefineMut: Sized {
    type Refinement;
    fn refine_with(&mut self, refined: Self::Refinement);
}

pub trait Unrefined: Sized {
    type Unrefinement;
    fn unrefined(&self) -> Self::Unrefinement;
}

#[allow(unused)]
pub trait RefineUnrefined: RefineMut + Unrefined<Unrefinement = Self::Refinement> {
    fn refine(&mut self) {
        let unrefined = self.unrefined();
        self.refine_with(unrefined);
    }
}

#[allow(unused)]
pub trait RefineAtScope: Sized {
    fn refine_at_scope(&self, scope: &ScopeChain) -> Self;
}

impl RefineMut for Type {
    type Refinement = CommaPunctuatedNestedArguments;

    fn refine_with(&mut self, refined: Self::Refinement) {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.segments.last_mut().unwrap().arguments.refine_with(refined),
            Type::Tuple(TypeTuple { elems, .. }) => {
                let mut refinement = refined.clone();
                elems.iter_mut()
                    .rev()
                    .for_each(|inner_ty| {
                        match refinement.pop() {
                            None => {}
                            Some(nested_arg) => match nested_arg.into_value() {
                                NestedArgument::Object(obj) |
                                NestedArgument::Constraint(obj) => {
                                    *inner_ty = obj.maybe_type().unwrap();
                                }
                            }
                        }
                    });
            },
            Type::TraitObject(TypeTraitObject { bounds, .. }) |
            Type::ImplTrait(TypeImplTrait { bounds, .. }) =>
                bounds.iter_mut()
                    .zip(refined.iter())
                    .for_each(|(bound, nested_arg)| if let TypeParamBound::Trait(TraitBound { path , ..}) = bound {
                        match nested_arg.ty() {
                            Some(Type::TraitObject(TypeTraitObject { bounds, .. }) |
                                 Type::ImplTrait(TypeImplTrait { bounds, .. })) =>
                                *path = parse_quote!(#bounds),
                            Some(Type::Path(TypePath { path: bounds, .. })) =>
                                *path = bounds.clone(),
                            _ => {}
                        }
                }),
            _ => {}
        }
    }
}

impl RefineMut for PathArguments {
    type Refinement = CommaPunctuatedNestedArguments;

    fn refine_with(&mut self, refined: Self::Refinement) {
        let mut refinement = refined.clone();
        let mut refine = |inner_ty: &mut Type| match refinement.pop() {
            Some(nested_arg) =>
                *inner_ty = nested_arg.into_value().object().maybe_type().unwrap(),
            None => {}
        };

        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                args.iter_mut()
                    .rev()
                    .for_each(|arg| {
                        match arg {
                            GenericArgument::Type(inner_ty) => {
                                refine(inner_ty)
                            }
                            GenericArgument::Lifetime(_) => {}
                            GenericArgument::Const(_) => {}
                            GenericArgument::Binding(_) => {}
                            GenericArgument::Constraint(_) => {}
                        }
                    });
            }
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                match output {
                    ReturnType::Default => {}
                    ReturnType::Type(_, inner_ty) => refine(inner_ty)
                }
                inputs.iter_mut()
                    .rev()
                    .for_each(|inner_ty| refine(inner_ty))
            }
        }
    }
}