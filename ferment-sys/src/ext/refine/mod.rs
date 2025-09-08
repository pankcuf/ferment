mod refine;
mod lifetime;

pub use lifetime::*;
pub use refine::*;
use syn::{AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, parse_quote, PathArguments, ReturnType, TraitBound, Type, TypeImplTrait, TypePath, TypeTraitObject, TypeTuple, Path};
use crate::composable::NestedArgument;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeChain;
use crate::ext::MaybeTraitBound;

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
            Type::Path(TypePath { path: Path { segments, .. }, .. }) => if let Some(last_segment) = segments.last_mut() {
                last_segment.arguments.refine_with(refined)
            }
            Type::Tuple(TypeTuple { elems, .. }) => {
                let mut refinement = refined.clone();
                elems.iter_mut()
                    .rev()
                    .for_each(|inner_ty| {
                        match refinement.pop() {
                            None => {}
                            Some(nested_arg) => match nested_arg.into_value() {
                                NestedArgument::Object(obj) |
                                NestedArgument::Constraint(obj) => if let Some(obj_type) = obj.maybe_type() {
                                    *inner_ty = obj_type;
                                }
                            }
                        }
                    });
            },
            Type::TraitObject(TypeTraitObject { bounds, .. }) |
            Type::ImplTrait(TypeImplTrait { bounds, .. }) =>
                bounds.iter_mut()
                    .zip(refined.iter())
                    .for_each(|(bound, nested_arg)| {
                        bound.maybe_trait_bound_mut()
                            .map(|TraitBound { path , ..}| match nested_arg.ty() {
                                Some(Type::TraitObject(TypeTraitObject { bounds, .. }) |
                                     Type::ImplTrait(TypeImplTrait { bounds, .. })) =>
                                    *path = parse_quote!(#bounds),
                                Some(Type::Path(TypePath { path: bounds, .. })) =>
                                    *path = bounds.clone(),
                                _ => {}
                        });
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
            Some(nested_arg) => if let Some(obj_type) = nested_arg.into_value().object().maybe_type() {
                *inner_ty = obj_type;
            },
            None => {}
        };

        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                args.iter_mut()
                    .rev()
                    .for_each(|arg| if let GenericArgument::Type(inner_ty) = arg {
                        refine(inner_ty)
                    }),
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output: ReturnType::Default, .. }) =>
                inputs.iter_mut()
                    .rev()
                    .for_each(|inner_ty| refine(inner_ty)),
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output: ReturnType::Type(_, inner_ty), .. }) => {
                refine(inner_ty);
                inputs.iter_mut()
                    .rev()
                    .for_each(|inner_ty| refine(inner_ty))
            }
        }
    }
}