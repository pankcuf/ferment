use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use syn::{Path, TraitBound, Type, TypeParamBound, TypePtr, TypeReference, TypeTraitObject};
use crate::ast::TypeHolder;
use crate::context::{ScopeChain, TypeChain};
use crate::conversion::ObjectConversion;
use crate::ext::{RefineMut, ToType};
use crate::formatter::types_dict;
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
            .find_map(|scope_chain|
                          {
                              // println!("resolve: {} = {} == {}", path.eq(scope_chain.self_path()), scope_chain.self_path().to_token_stream(), path.to_token_stream());
                              path.eq(scope_chain.self_path())
                                  .then(|| scope_chain)
                          })
    }
    pub fn scope_register_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        self.inner
            .entry(scope.clone())
            .or_default()
    }

    fn maybe_scope_type_(&self, tc: &TypeHolder, scope: &ScopeChain) -> Option<&ObjectConversion> {
        // println!("maybe_scope_type_.1: {} in {}", tc, scope.fmt_short());
        let result = self.inner
            .get(scope)
            .and_then(|chain| chain.get(tc));
        // println!("maybe_scope_type_.2: {} --> {}", tc, result.as_ref().map_or("None".to_string(), |r| format!("{}", r)));
        result
    }
    pub fn maybe_scope_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        // println!("maybe_scope_type.1: {} --- [{}]", ty.to_token_stream(), scope.fmt_short());
        let result = match ty {
            Type::TraitObject(TypeTraitObject { bounds , ..}) => match bounds.len() {
                1 => match bounds.first().unwrap() {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        self.maybe_scope_type_(&TypeHolder::from(&path.to_type()), scope),
                    TypeParamBound::Lifetime(_) =>
                        panic!("maybe_opaque_object::error")
                },
                _ => panic!("maybe_opaque_object::error")
            },
            Type::Reference(TypeReference { elem: ty, .. }) |
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                self.maybe_scope_type_(&TypeHolder::from(ty), scope),
            ty =>
                self.maybe_scope_type_(&TypeHolder::from(ty), scope),
        };
        // println!("maybe_scope_type.2: {} --- [{}]", ty.to_token_stream(), result.to_token_stream());
        result
    }

    pub fn scope_type_for_path(&self, path: &Path, scope: &ScopeChain) -> Option<Type> {
        self.inner
            .get(scope)
            .and_then(|chain| chain.get_by_path(path))
    }

    // pub fn find_generics_fq_in(&self, item: &Item, scope: &ScopeChain) -> HashSet<GenericConversion> {
    //     self.inner
    //         .get(scope)
    //         .map(|chain| item.find_generics_conversions(chain))
    //         .unwrap_or_default()
    // }

}

impl RefineMut for ScopeResolver {
    type Refinement = ScopeRefinement;

    fn refine_with(&mut self, refined: Self::Refinement) {
        refined.into_iter()
            .for_each(|(scope, updates)|
                self.scope_register_mut(&scope)
                    .add_many(updates.into_iter()));

    }
}