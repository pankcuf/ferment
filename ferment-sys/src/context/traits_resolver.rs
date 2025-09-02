use std::collections::HashMap;
use proc_macro2::Ident;
use syn::{ItemTrait, Path,};
use crate::ast::PathHolder;
use crate::composable::TraitModelPart1;
use crate::context::ScopeChain;
use crate::kind::ObjectKind;

#[derive(Clone, Default)]
pub struct TraitsResolver {
    pub inner: HashMap<ScopeChain, HashMap<Ident, TraitModelPart1>>,
    pub used_traits_dictionary: HashMap<ScopeChain, Vec<PathHolder>>,
}

impl TraitsResolver {
    pub fn add_trait(&mut self, scope: &ScopeChain, item_trait: &ItemTrait, _itself: &ObjectKind) {
        self.inner
            .entry(scope.clone())
            .or_default()
            .insert(item_trait.ident.clone(), TraitModelPart1::new(item_trait.clone()));
    }

    pub fn add_used_traits(&mut self, scope: &ScopeChain, trait_names: Vec<Path>) {
        self.used_traits_dictionary
            .entry(scope.clone())
            .or_default()
            .extend(trait_names.iter().map(|trait_name| PathHolder::from(trait_name)));
    }

    pub fn item_trait_with_ident_for(&self, ident: &Ident, scope: &ScopeChain) -> Option<&TraitModelPart1> {
        self.inner
            .get(scope)
            .and_then(|dict| dict.get(ident))
    }

}