use std::collections::{HashMap, HashSet};
use quote::ToTokens;
use syn::{Item, Path, Type, TypeReference};
use crate::composition::GenericConversion;
use crate::context::ScopeChain;
use crate::conversion::ObjectConversion;
use crate::helper::ItemExtension;
use crate::holder::TypeHolder;

#[derive(Clone, Default)]
pub struct ScopeResolver {
    pub inner: HashMap<ScopeChain, HashMap<TypeHolder, ObjectConversion>>,
}

impl ScopeResolver {
    pub(crate) fn resolve(&self, path: &Path) -> Option<&ScopeChain> {
        self.inner
            .keys()
            .find_map(|scope_chain|
                path.eq(scope_chain.self_path())
                    .then_some(scope_chain))
    }
    pub fn scope_register_mut(&mut self, scope: &ScopeChain) -> &mut HashMap<TypeHolder, ObjectConversion> {
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
            .and_then(|dict| dict.get(&tc))
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
            .and_then(|scope_types|
                scope_types.iter()
                    .find_map(|(TypeHolder { 0: other}, full_type)| {
                        if path.to_token_stream().to_string().eq(other.to_token_stream().to_string().as_str()) {
                            full_type.ty()
                        } else {
                            None
                        }
                    }))
            .cloned()
    }

    pub fn find_generics_fq_in(&self, item: &Item, scope: &ScopeChain) -> HashSet<GenericConversion> {
        self.inner
            .get(scope)
            .map(|scope_types| item.find_generics_fq(scope_types))
            .unwrap_or_default()
    }
}