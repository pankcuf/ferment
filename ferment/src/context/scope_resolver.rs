use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use syn::{Item, Path, Type, TypeReference};
use crate::composition::GenericConversion;
use crate::context::ScopeChain;
use crate::context::type_chain::TypeChain;
use crate::conversion::ObjectConversion;
use crate::formatter::types_dict;
use crate::helper::ItemExtension;
use crate::holder::TypeHolder;

#[derive(Clone, Default)]
pub struct ScopeResolver {
    pub inner: HashMap<ScopeChain, TypeChain>,
}

impl Debug for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.inner.iter()
            .map(|(key, value)| format!("\t{}:\n\t\t{}", key, types_dict(&value.inner).join("\n\t\t")))
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
                path.eq(scope_chain.self_path())
                    .then_some(scope_chain))
    }
    pub fn scope_register_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        self.inner
            .entry(scope.clone())
            .or_default()
    }

    pub fn maybe_scope_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        let tc = match ty {
            Type::Reference(TypeReference { elem, .. }) => TypeHolder::from(elem),
            _ => TypeHolder::from(ty)
        };
        self.inner
            .get(scope)
            .and_then(|chain| chain.get(&tc))
    }

    pub fn maybe_scope_type_or_parent_type(&self, ty: &Type, scope: &ScopeChain) -> Option<ObjectConversion> {
        self.maybe_scope_type(ty, scope)
            .or(scope.parent_scope()
                .and_then(|parent_scope| self.maybe_scope_type(ty, parent_scope)))
            .cloned()
    }

    pub fn scope_type_for_path(&self, path: &Path, scope: &ScopeChain) -> Option<Type> {
        self.inner
            .get(scope)
            .and_then(|chain| chain.get_by_path(path))
            .cloned()
    }

    pub fn find_generics_fq_in(&self, item: &Item, scope: &ScopeChain) -> HashSet<GenericConversion> {
        self.inner
            .get(scope)
            .map(|chain| item.find_generics_fq(chain))
            .unwrap_or_default()
    }
}