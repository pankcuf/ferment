use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use syn::{Path, TraitBound, Type, TypeParamBound, TypePtr, TypeReference, TypeTraitObject};
use crate::ast::TypeHolder;
use crate::context::{ScopeChain, TypeChain};
use crate::conversion::ObjectConversion;
use crate::ext::{RefineMut, ToType};
use crate::formatter::{format_scope_refinement, types_dict};
pub type ScopeRefinement = Vec<(ScopeChain, HashMap<TypeHolder, ObjectConversion>)>;

#[derive(Clone, Default)]
pub struct ScopeResolver {
    pub inner: HashMap<ScopeChain, TypeChain>,
}

impl Debug for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.inner.iter()
            .map(|(key, value)|
                format!("\t{}:\n\t{}", key.fmt_short(), types_dict(&value.inner)
                    .join("\n\t")))
            .collect::<Vec<String>>();
        iter.sort();
        f.write_str( iter.join("\n\n").as_str())
    }
}

impl Display for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ScopeResolver {
    pub(crate) fn resolve(&self, path: &Path) -> Option<&ScopeChain> {
        self.inner
            .keys()
            .find_map(|scope_chain| {
                // println!("resolve: {} = {} == {}", path.eq(scope_chain.self_path()), scope_chain.self_path().to_token_stream(), path.to_token_stream());
                path.eq(scope_chain.self_path())
                    .then(|| scope_chain)
            })
    }
    pub(crate) fn resolve_obj_first(&self, path: &Path) -> Option<&ScopeChain> {
        let mut scopes = self.inner
            .keys()
            .filter(|scope_chain| path.eq(scope_chain.self_path()))
            .collect::<Vec<_>>();

        scopes.sort_by(|c1, c2| {
            if c1.obj_scope_priority() == c2.obj_scope_priority() {
                Ordering::Equal
            } else if c1.obj_scope_priority() < c2.obj_scope_priority() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        scopes.first().cloned()
    }
    pub fn scope_register_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        self.inner
            .entry(scope.clone())
            .or_default()
    }

    fn maybe_scope_type_(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        // println!("maybe_scope_type_.1: {} in {}", tc, scope.fmt_short());
        let tc = TypeHolder::from(ty);
        let result = self.inner
            .get(scope)
            .and_then(|chain| chain.get(&tc));
        // println!("maybe_scope_type_.2: {} --> {}", tc, result.as_ref().map_or("None".to_string(), |r| format!("{}", r)));
        result
    }
    pub fn maybe_scope_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        // println!("maybe_scope_type.1: {} --- [{}]", ty.to_token_stream(), scope.fmt_short());
        let result = match ty {
            Type::TraitObject(TypeTraitObject { bounds , ..}) => match bounds.len() {
                1 => match bounds.first().unwrap() {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        self.maybe_scope_type_(&path.to_type(), scope),
                    TypeParamBound::Lifetime(_) =>
                        panic!("maybe_scope_type::error")
                },
                _ => None
            },
            Type::Reference(TypeReference { elem: ty, .. }) |
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                self.maybe_scope_type_(ty, scope),
            ty =>
                self.maybe_scope_type_(ty, scope),
        };
        // println!("maybe_scope_type.2: {} --- [{}]", ty.to_token_stream(), result.to_token_stream());
        result
    }

    pub fn scope_type_for_path(&self, path: &Path, scope: &ScopeChain) -> Option<Type> {
        self.inner
            .get(scope)
            .and_then(|chain| chain.get_by_path(path))
    }
}

impl RefineMut for ScopeResolver {
    type Refinement = ScopeRefinement;

    fn refine_with(&mut self, refined: Self::Refinement) {
        // println!("ScopeResolver::refine_with:\n{}", format_scope_refinement(&refined));
        refined.into_iter()
            .for_each(|(scope, updates)| {

                // println!("ScopeResolver::SCOPE: {} --- {:?}", scope.fmt_short(), updates);
                self.scope_register_mut(&scope)
                    .add_many(updates.into_iter());
                // println!("ScopeResolver::SCOPE (RESULT): {} --- {:?}", scope.fmt_short(), self.inner.get(&scope));
            });

        // println!("ScopeResolver::refine_with (RESULT):\n{:?}", self.inner);
    }
}