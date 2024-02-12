use std::collections::HashMap;
use std::fmt::Formatter;
use syn::{parse_quote, Path, Type};
use crate::Config;
use crate::context::ScopeChain;
use crate::composition::TraitCompositionPart1;
use crate::context::custom_resolver::CustomResolver;
use crate::context::generic_resolver::GenericResolver;
use crate::context::import_resolver::ImportResolver;
use crate::conversion::{ObjectConversion, TypeConversion};
use crate::formatter::{format_global_context, format_token_stream};
use crate::holder::{PathHolder, TypeHolder};
use crate::context::ScopeResolver;
use crate::context::traits_resolver::TraitsResolver;

#[derive(Clone, Default)]
pub struct GlobalContext {
    pub config: Config,
    pub scope_register: ScopeResolver,
    // crate::asyn::query::Query: [T: [TransportRequest]]
    pub generics: GenericResolver,
    pub traits: TraitsResolver,
    pub custom: CustomResolver,
    pub imports: ImportResolver,
}

impl std::fmt::Debug for GlobalContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_global_context(self))
    }
}

impl std::fmt::Display for GlobalContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl GlobalContext {
    pub fn with_config(config: Config) -> Self {
        Self { config, ..Default::default() }
    }
    pub fn fermented_mod_name(&self) -> &str {
        &self.config.mod_name
    }


    pub fn resolve_trait_type(&self, from_type: &Type) -> Option<&ObjectConversion> {
        // RESOLVE PATHS
        // Self::asyn::query::TransportRequest::Client::Error
        // ? [Self::asyn::query::TransportRequest::Client::Error] Self
        // : [Self::asyn::query::TransportRequest::Client] Self::Error
        // : [Self::asyn::query::TransportRequest] Self::Client::Error
        //  : [Self::asyn::query::TransportRequest] Self::Client -> [Self::asyn::query::TransportClient] Self::Error

        // aa::bb::cc::dd::ee
        // 1. a) [aa::bb::cc::dd::ee] Self
        // 2. a) [aa::bb::cc::dd] Self::ee
        // 3. a) [aa::bb::cc::dd] Self, [Self::ee]
        // 4. a) [aa::bb::cc] Self::dd::ee, b) [aa::bb::cc] Self::dd
        let current_scope: PathHolder = parse_quote!(#from_type);
        let mut i = 0;
        let mut maybe_trait: Option<&ObjectConversion>  = None;
        while i < current_scope.len() && maybe_trait.is_none() {
            let (root, head) = current_scope.split_and_join_self(i);
            let ty = parse_quote!(#head);
            let root_scope = self.resolve_scope(&root.0);
            if let Some(scope) = root_scope {
                maybe_trait = self.maybe_scope_type(&ty, scope);
            }
            //maybe_trait = self.maybe_scope_type(&ty, &root);
            if i > 0 {
                match maybe_trait {
                    Some(ObjectConversion::Item(TypeConversion::Trait(trait_ty, decomposition), _)) |
                    Some(ObjectConversion::Type(TypeConversion::Trait(trait_ty, decomposition))) => {
                        let ident = &head.0.segments.last().unwrap().ident;
                        println!("FFI (has decomposition) for: {}: {}", format_token_stream(ident), trait_ty);
                        if let Some(trait_type) = decomposition.types.get(ident) {
                            println!("FFI (first bound) {:?}", trait_type);
                            if let Some(first_bound) = trait_type.trait_bounds.first() {
                                println!("FFI (first bound) {}", format_token_stream(&first_bound.path));
                                let tt_type = parse_quote!(#first_bound);
                                if let Some(scope) = root_scope {
                                    maybe_trait = self.maybe_scope_type(&tt_type, scope);
                                }
                                println!("FFI (first bound full) {:?}", maybe_trait);
                            }
                        }
                    },
                    _ => {}
                }
            }
            println!("FFI (resolve....) for: {} in [{}] ===> {:?}", format_token_stream(&head), format_token_stream(&root), maybe_trait);
            i += 1;
        }
        maybe_trait
    }


    pub fn maybe_scope_generic_bounds_or_parent(&self, scope: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
        // println!("maybe_scope_generic_bounds_or_parent: {} in [{}]...", ident, scope);
        self.generics.maybe_generic_bounds(scope, ident)
            .and_then(|generic_bounds| {
                let first_bound = generic_bounds.first().unwrap();
                let first_bound_as_scope = PathHolder::from(first_bound);
                self.maybe_import(scope, &first_bound_as_scope)
            })
    }



    fn maybe_obj_or_parent_scope_type(&self, self_scope: &ScopeChain, parent_chain: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        self.maybe_scope_type(ty, self_scope)
            .or(match parent_chain {
                ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                    self.maybe_scope_type(ty, parent_chain),
                _ => None,
            })
    }

    pub fn maybe_fn_type(&self, fn_scope: &ScopeChain, parent_scope: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        self.maybe_scope_type(ty, fn_scope).or(match parent_scope {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                self.maybe_scope_type(ty, parent_scope),
            ScopeChain::Fn { parent_scope_chain, .. } =>
                self.maybe_fn_type(parent_scope, parent_scope_chain, ty),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } =>
                self.maybe_scope_type(ty, parent_scope).or(match &**parent_scope_chain {
                    ScopeChain::CrateRoot { .. } |
                    ScopeChain::Mod { ..} =>
                        self.maybe_scope_type(ty, &parent_scope_chain),
                    _ => None,
                }),
        })
    }

    pub fn maybe_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
         match scope {
             ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                 self.maybe_scope_type(ty, &scope),
             ScopeChain::Fn { parent_scope_chain, .. } =>
                 self.maybe_fn_type(scope, parent_scope_chain, ty),
             ScopeChain::Trait { parent_scope_chain, .. } |
             ScopeChain::Object { parent_scope_chain, .. } |
             ScopeChain::Impl { parent_scope_chain, .. } =>
                 self.maybe_obj_or_parent_scope_type(scope, parent_scope_chain, ty),
         }
    }


    pub fn actual_scope_for_type(&self, ty: &Type, current_scope: &ScopeChain) -> ScopeChain {
        println!("actual_scope_for_type: {} in [{}]", format_token_stream(ty), current_scope);
        let p = parse_quote!(#ty);
        let scope = if let Some(st) = self.maybe_scope_type(ty, current_scope) {
            // match st {
            //     ObjectConversion::Type(_) => {}
            //     ObjectConversion::Item(_, _) => {}
            //     ObjectConversion::Empty => {}
            // }
            let self_ty = st.ty().unwrap();
            let self_path: Path = parse_quote!(#self_ty);
            self.resolve_scope(&self_path).cloned()
        } else if let Some(import_path) = self.maybe_scope_import_path(current_scope, &p) {
            self.resolve_scope(import_path).cloned()
        } else {
            None
        };
        println!("actual_scope_for_type: [{:?}]", scope);
        scope.unwrap_or(ScopeChain::crate_root())
    }
    pub fn maybe_trait(&self, full_ty: &Type) -> Option<TraitCompositionPart1> {
        let full_scope: PathHolder = parse_quote!(#full_ty);
        self.resolve_scope(&full_scope.0)
            .and_then(|scope| self.traits.maybe_trait(scope).cloned())
    }


    // fn find_trait_item_full_paths_pair(&self, ) -> (Scope, Scope) {
    //     self.used_traits_dictionary.iter()
    //         .for_each(|(item_full_path, trait_path_chunks)| {
    //             trait_path_chunks.iter()
    //                 .for_each(|trait_ident_or_chunk| {
    //                     let trait_ident = trait_ident_or_chunk.root_ident();
    //                     // Restore full trait path using imports
    //                     // TODO: can be chunk so need to handle not only idents
    //                     let trait_scope = if let Some(import) = self.maybe_scope_import_path(item_full_path, &trait_ident) {
    //                         Scope::from(import)
    //                     } else {
    //                         item_full_path.popped().joined(&trait_ident)
    //                     };
    //                     (trait_scope, item_full_path)
    //                 });
    //
    //         });
    //
    // }

    // pub fn inject_types_from_traits_implementation(&mut self) {
    //     let self_tc = TypeHolder::new(parse_quote!(Self));
    //     self.used_traits_dictionary.iter()
    //         .for_each(|(item_full_path, trait_path_chunks)| {
    //             trait_path_chunks.iter()
    //                 .for_each(|trait_ident_or_chunk| {
    //                     let trait_ident = trait_ident_or_chunk.root_ident();
    //                     // Restore full trait path using imports
    //                     // TODO: can be chunk so need to handle not only idents
    //                     let trait_scope = if let Some(import) = self.maybe_scope_import_path(item_full_path, &trait_ident) {
    //                         Scope::from(import)
    //                     } else {
    //                         item_full_path.popped().joined(&trait_ident)
    //                     };
    //                     println!("inject_types_from_traits_implementation: [{}]: {}: {}", format_token_stream(item_full_path), format_token_stream(trait_ident_or_chunk), format_token_stream(&trait_scope));
    //                     if let Some(types_used_by_trait) = self.scope_types.get(&trait_scope).cloned() {
    //                         // Copy them to implementor's types
    //                         println!("copy types except self:\n   {}", format_types_dict(&types_used_by_trait));
    //                         // TODO: do we need to replace Self to <Self as #trait>?
    //                         let types = types_used_by_trait.into_iter().filter(|(tc, tyty)| {
    //
    //                             !self_tc.eq(tc)
    //                         });
    //                         self.scope_types.entry(item_full_path.clone())
    //                             .or_default()
    //                             .extend(types);
    //                     }
    //             });
    //
    //     });
    // }
    // pub fn inject_types_from_traits_implementation(&mut self) {
    //     let self_tc: TypeHolder = parse_quote!(Self);
    //
    //     // Collect necessary data in a Vec to avoid borrowing `self` while iterating.
    //     let mut trait_data = Vec::new();
    //
    //     for (item_full_path, trait_path_chunks) in &self.used_traits_dictionary {
    //         for trait_ident_or_chunk in trait_path_chunks {
    //
    //             // let trait_ident = trait_ident_or_chunk.root_ident();
    //             let trait_scope = if let Some(import) = self.maybe_scope_import_path(item_full_path, trait_ident_or_chunk) {
    //                 PathHolder::from(import)
    //             } else {
    //                 // item_full_path.parent_scope()
    //                 item_full_path.parent_scope().joined_path(trait_ident_or_chunk.0.clone())
    //             };
    //             println!("inject_types_from_traits_implementation: [{}]: {}: {}", format_token_stream(item_full_path), format_token_stream(trait_ident_or_chunk), format_token_stream(&trait_scope));
    //
    //             if let Some(types_used_by_trait) = self.scope_types.get(&trait_scope) {
    //                 trait_data.push((item_full_path.clone(), types_used_by_trait.clone()));
    //             }
    //         }
    //     }
    //
    //     // Now, iterate over the collected data and modify `self.scope_types`.
    //     for (item_full_path, types_used_by_trait) in trait_data {
    //         // println!("copy types except self:\n   {}", format_types_dict(&types_used_by_trait));
    //         let types = types_used_by_trait.into_iter().filter(|(tc, _)| !self_tc.eq(tc));
    //         self.scope_types
    //             .entry(item_full_path)
    //             .or_default()
    //             .extend(types);
    //     }
    // }
}

/// Imports
impl GlobalContext {
    pub fn maybe_scope_import_path(&self, scope: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
        self.imports.maybe_path(scope, ident)
    }

    pub fn maybe_import(&self, scope: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
        self.imports.maybe_import(scope, ident)
    }
}

/// Scope
impl GlobalContext {
    pub fn scope_register_mut(&mut self, scope: &ScopeChain) -> &mut HashMap<TypeHolder, ObjectConversion> {
        self.scope_register.scope_register_mut(scope)
    }
    pub fn maybe_scope_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        self.scope_register.maybe_scope_type(ty, scope)
    }
    fn resolve_scope(&self, path: &Path) -> Option<&ScopeChain> {
        self.scope_register.resolve(path)
    }

}