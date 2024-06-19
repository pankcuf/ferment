use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use syn::{Path, Type, TypePtr, TypeReference};
use crate::context::ScopeChain;
use crate::context::type_chain::TypeChain;
use crate::conversion::ObjectConversion;
use crate::ext::RefineMut;
use crate::formatter::types_dict;
use crate::holder::TypeHolder;
pub type ScopeRefinement = Vec<(ScopeChain, HashMap<TypeHolder, ObjectConversion>)>;

#[derive(Clone, Default)]
pub struct ScopeResolver {
    pub inner: HashMap<ScopeChain, TypeChain>,
}

impl Debug for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.inner.iter()
            .map(|(key, value)|
                format!("\t{}:\n\t\t{}", key, types_dict(&value.inner)
                    .join("\n\t\t")))
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

    pub fn maybe_scope_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        // println!("maybe_scope_type: {} --- [{}]", ty.to_token_stream(), scope);
        let tc = match ty {
            Type::Reference(TypeReference { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) => TypeHolder::from(elem),
            _ => TypeHolder::from(ty)
        };
        self.inner
            .get(scope)
            .and_then(|chain| chain.get(&tc))
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
                    .add_many(TypeChain::from(updates)));

    }
}