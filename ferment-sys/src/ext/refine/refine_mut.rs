use std::collections::HashSet;
use indexmap::IndexMap;
use syn::{AngleBracketedGenericArguments, Attribute, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, ReturnType, TraitBound, Type, TypeImplTrait, TypePath, TypeTraitObject, TypeTuple};
use crate::composable::{NestedArgument, TypeModel};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{GlobalContext, ScopeRefinement, ScopeResolver};
use crate::ext::{GenericCollector, MaybeTraitBound, ResolveAttrs, ToPath, TypeCollector};
use crate::formatter::format_mixin_kinds;
use crate::kind::{MixinKind, TypeModelKind};
use crate::print_phase;

pub trait RefineMut: Sized {
    type Refinement;
    fn refine_with(&mut self, refined: Self::Refinement);
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
                    .for_each(|(bound, nested_arg)| if let Some(TraitBound { path , .. }) = bound.maybe_trait_bound_mut() {
                        match nested_arg.ty() {
                            Some(Type::TraitObject(TypeTraitObject { bounds, .. }) |
                                 Type::ImplTrait(TypeImplTrait { bounds, .. })) =>
                                *path = bounds.to_path(),
                            Some(Type::Path(TypePath { path: bounds, .. })) =>
                                *path = bounds.clone(),
                            _ => {}
                        }
                    }),
            _ => {}
        }
    }
}
impl RefineMut for TypeModel {

    type Refinement = CommaPunctuatedNestedArguments;

    fn refine_with(&mut self, refined: Self::Refinement) {
        self.ty.refine_with(refined);
    }
}

impl RefineMut for PathArguments {
    type Refinement = CommaPunctuatedNestedArguments;

    fn refine_with(&mut self, refined: Self::Refinement) {
        let mut refinement = refined.clone();
        let mut refine = |inner_ty: &mut Type| if let Some(nested_arg) = refinement.pop() {
            if let Some(obj_type) = nested_arg.into_value().object().maybe_type() {
                *inner_ty = obj_type;
            }
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
                    .for_each(refine),
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output: ReturnType::Type(_, inner_ty), .. }) => {
                refine(inner_ty);
                inputs.iter_mut()
                    .rev()
                    .for_each(refine)
            }
        }
    }
}

impl RefineMut for ScopeResolver {
    type Refinement = ScopeRefinement;

    fn refine_with(&mut self, refined: Self::Refinement) {
        refined.into_iter()
            .for_each(|(scope, updates)|
                self.type_chain_mut(&scope)
                    .add_many(updates.into_iter())
            );
    }
}
impl RefineMut for GlobalContext {
    type Refinement = ScopeRefinement;
    fn refine_with(&mut self, refined: Self::Refinement) {
        self.scope_register.refine_with(refined);
        let mut refined_mixins = IndexMap::<MixinKind, HashSet<Option<Attribute>>>::new();
        self.scope_register.inner.iter()
            .for_each(|(scope, type_chain)| {
                let scope_level_attrs = scope.resolve_attrs();
                type_chain.inner.iter().for_each(|(_conversion, object)| {
                    let object_attrs = object.resolve_attrs();
                    let mut all_attrs: HashSet<Option<Attribute>> = HashSet::from_iter(object_attrs);
                    all_attrs.extend(scope_level_attrs.clone());
                    if all_attrs.is_empty() {
                        all_attrs.insert(None);
                    }

                    if let Some(ty) = object.maybe_type() {
                        ty.find_generics()
                            .iter()
                            .filter(|ty| self.maybe_custom_type(ty).is_none() && !self.should_skip_from_expanding(object))
                            .for_each(|_ty| if let Some(kind) = object.maybe_generic_type_kind() {
                                refined_mixins
                                    .entry(MixinKind::Generic(kind))
                                    .or_default()
                                    .extend(all_attrs.clone());
                            });
                    }

                    if let Some(TypeModelKind::Bounds(bounds)) = object.maybe_type_model_kind_ref() {
                        let mut container = HashSet::<Type>::new();
                        bounds.collect_compositions()
                            .into_iter()
                            .for_each(|field_type| field_type.collect_to(&mut container));
                        container
                            .iter()
                            .for_each(|_ty| refined_mixins
                                .entry(MixinKind::bounds(bounds))
                                .or_default()
                                .extend(all_attrs.clone()));
                    }
                })
            });
        print_phase!("PHASE 3: GENERICS TO EXPAND", "\t{}", format_mixin_kinds(&refined_mixins));
        self.refined_mixins = refined_mixins;

        self.generics.inner.iter_mut()
            .for_each(|(scope, generic_chain)| generic_chain.values_mut()
                .for_each(|bounds| bounds.iter_mut()
                    .for_each(|bound| if let Some(Type::Path(TypePath { path, .. })) = self.scope_register.scope_key_type_for_path(bound, scope) {
                        *bound = path;
                    })));
    }
}