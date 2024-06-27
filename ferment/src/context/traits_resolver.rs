use std::collections::HashMap;
use proc_macro2::Ident;
use syn::{ItemTrait, Path};
use crate::composable::TraitCompositionPart1;
use crate::context::ScopeChain;
use crate::conversion::ObjectConversion;
use crate::holder::PathHolder;

#[derive(Clone, Default)]
pub struct TraitsResolver {
    pub inner: HashMap<ScopeChain, HashMap<Ident, TraitCompositionPart1>>,
    pub used_traits_dictionary: HashMap<ScopeChain, Vec<PathHolder>>,
}

impl TraitsResolver {
    pub fn add_trait(&mut self, scope: &ScopeChain, item_trait: &ItemTrait, _itself: &ObjectConversion) {
        self.inner
            .entry(scope.clone())
            .or_default()
            .insert(item_trait.ident.clone(), TraitCompositionPart1::new(item_trait.clone()));
    }

    // pub fn maybe_trait(&self, scope: &ScopeChain) -> Option<&TraitCompositionPart1> {
    //     let last_ident = scope.head();
    //     self.inner.get(&scope)
    //         .and_then(|scope_traits| scope_traits.get(&last_ident))
    // }

    pub fn add_used_traits(&mut self, scope: &ScopeChain, trait_names: Vec<Path>) {
        self.used_traits_dictionary
            .entry(scope.clone())
            .or_default()
            .extend(trait_names.iter().map(|trait_name| PathHolder::from(trait_name)));
    }

    pub fn item_trait_with_ident_for(&self, ident: &Ident, scope: &ScopeChain) -> Option<&TraitCompositionPart1> {
        // println!("item_trait_with_ident_for: {} in [{}] ", format_token_stream(ident), format_token_stream(scope));
        self.inner
            .get(scope)
            .and_then(|dict| dict.get(ident))
    }

}