use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use syn::punctuated::Punctuated;
use syn::token::{Colon2, Comma};
use crate::composition::{GenericConversion, NestedArgument, TraitCompositionPart1, TypeComposition};
use crate::Config;
use crate::context::{Scope, ScopeChain, TypeChain};
use crate::context::custom_resolver::CustomResolver;
use crate::context::generic_resolver::GenericResolver;
use crate::context::import_resolver::ImportResolver;
use crate::conversion::{ObjectConversion, ScopeItemConversion, TypeCompositionConversion};
use crate::formatter::{format_global_context, format_token_stream};
use crate::holder::PathHolder;
use crate::context::{ScopeResolver, TraitsResolver};
use crate::context::scope_resolver::ScopeRefinement;
use crate::ext::{CrateExtension, visitor::GenericCollector, Pop, RefineMut, RefineUnrefined, ToPath, ToType, Unrefined};

#[derive(Clone)]
pub struct GlobalContext {
    pub config: Config,
    pub scope_register: ScopeResolver,
    // crate::asyn::query::Query: [T: [TransportRequest]]
    pub generics: GenericResolver,
    pub traits: TraitsResolver,
    pub custom: CustomResolver,
    pub imports: ImportResolver,

    pub refined_generics: HashSet<GenericConversion>
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

impl From<&Config> for GlobalContext {
    fn from(config: &Config) -> Self {
        GlobalContext::with_config(config.clone())
    }
}
impl GlobalContext {
    pub fn with_config(config: Config) -> Self {
        Self { config, scope_register: ScopeResolver::default(), generics: Default::default(), traits: Default::default(), custom: Default::default(), imports: Default::default(), refined_generics: HashSet::new() }
    }
    pub fn fermented_mod_name(&self) -> &str {
        &self.config.mod_name
    }
    pub fn is_fermented_mod(&self, ident: &Ident) -> bool {
        format_ident!("{}", self.fermented_mod_name()).eq(ident)
    }


    pub fn resolve_trait_type(&self, from_type: &Type) -> Option<&ObjectConversion> {
        // println!("resolve_trait_type: {}", from_type.to_token_stream());
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
            let ty = head.to_type();
            let root_scope = self.maybe_scope(&root.0);
            if let Some(scope) = root_scope {
                maybe_trait = self.maybe_scope_object(&ty, scope);
            }
            //maybe_trait = self.maybe_scope_type(&ty, &root);
            if i > 0 {
                match maybe_trait {
                    Some(ObjectConversion::Item(TypeCompositionConversion::Trait(_trait_ty, decomposition, _super_bounds), _)) |
                    Some(ObjectConversion::Type(TypeCompositionConversion::Trait(_trait_ty, decomposition, _super_bounds))) => {
                        let ident = &head.0.segments.last().unwrap().ident;
                        // println!("FFI (has decomposition) for: {}: {}", format_token_stream(ident), trait_ty);
                        if let Some(trait_type) = decomposition.types.get(ident) {
                            // println!("FFI (first bound) {:?}", trait_type);
                            if let Some(first_bound) = trait_type.trait_bounds.first() {
                                // println!("FFI (first bound) {}", format_token_stream(&first_bound.path));
                                let tt_type = first_bound.to_type();
                                if let Some(scope) = root_scope {
                                    maybe_trait = self.maybe_scope_object(&tt_type, scope);
                                }
                                // println!("FFI (first bound full) {:?}", maybe_trait);
                            }
                        }
                    },
                    _ => {}
                }
            }
            // println!("FFI (resolve....) for: {} in [{}] ===> {:?}", format_token_stream(&head), format_token_stream(&root), maybe_trait);
            i += 1;
        }
        maybe_trait
    }

    pub fn maybe_trait_scope_pair(&self, link: &Path, scope: &ScopeChain) -> Option<(TraitCompositionPart1, ScopeChain)> {
        let parent_scope = scope.parent_scope().unwrap();
        let trait_ty = link.to_type();
        let trait_scope = self.actual_scope_for_type(&trait_ty, parent_scope);
        // println!("find_item_trait_scope_pair: {} -> {}", link.to_token_stream(), trait_scope);
        let ident = link.get_ident().unwrap();
        self.traits
            .item_trait_with_ident_for(ident, &trait_scope)
            .map(|trait_composition| {
                let mut composition = trait_composition.clone();
                // TODO: move to full and replace nested_arguments
                let value = TypeCompositionConversion::Object(TypeComposition::new(scope.to_type(), Some(trait_composition.item.generics.clone()), Punctuated::new()));
                // println!("AttrsComposer: {} {} {}", trait_composition.item.ident, trait_scope, conversion);
                composition.implementors.push(value);
                (composition, trait_scope)
            })

    }


    // pub fn maybe_scope_generic_bounds_or_parent(&self, scope: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
    //     // println!("maybe_scope_generic_bounds_or_parent: {} in [{}]...", ident, scope);
    //     self.generics.maybe_generic_bounds(scope, ident)
    //         .and_then(|generic_bounds| {
    //             let first_bound = generic_bounds.first().unwrap();
    //             let first_bound_as_scope = PathHolder::from(first_bound);
    //             self.maybe_import(scope, &first_bound_as_scope)
    //         })
    // }



    fn maybe_obj_or_parent_scope_type(&self, self_scope: &ScopeChain, parent_chain: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        self.maybe_scope_object(ty, self_scope)
            .or(match parent_chain {
                ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                    self.maybe_scope_object(ty, parent_chain),
                _ => None,
            })
    }

    pub fn maybe_fn_type(&self, fn_scope: &ScopeChain, parent_scope: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        self.maybe_scope_object(ty, fn_scope).or(match parent_scope {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                self.maybe_scope_object(ty, parent_scope),
            ScopeChain::Fn { parent_scope_chain, .. } =>
                self.maybe_fn_type(parent_scope, parent_scope_chain, ty),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } =>
                self.maybe_scope_object(ty, parent_scope).or(match &**parent_scope_chain {
                    ScopeChain::CrateRoot { .. } |
                    ScopeChain::Mod { ..} =>
                        self.maybe_scope_object(ty, &parent_scope_chain),
                    _ => None,
                }),
        })
    }

    pub fn maybe_type(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
         match scope {
             ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                 self.maybe_scope_object(ty, &scope),
             ScopeChain::Fn { parent_scope_chain, .. } =>
                 self.maybe_fn_type(scope, parent_scope_chain, ty),
             ScopeChain::Trait { parent_scope_chain, .. } |
             ScopeChain::Object { parent_scope_chain, .. } |
             ScopeChain::Impl { parent_scope_chain, .. } =>
                 self.maybe_obj_or_parent_scope_type(scope, parent_scope_chain, ty),
         }
    }

    pub fn maybe_item(&self, path: &Path) -> Option<&ScopeItemConversion> {
        // println!("maybe_item: {}", path.to_token_stream());
        if let Some(scope) = self.maybe_scope(path) {
            // println!("[INFO] Found scope: {}", scope);
            let last_ident = &path.segments.last().unwrap().ident;
            let ty = last_ident.to_type();
            if let Some(ObjectConversion::Item(_, item)) = self.maybe_type(&ty, scope) {
                // println!("[INFO] Found item in scope: {}", item);
                return Some(item);
            } else {
                // println!("[INFO] Scope found [{}] but no item: {}", scope, path.to_token_stream());
            }
        } else {
            // println!("[INFO] No scope found [{}]", path.to_token_stream());
        }
        None
    }

    // pub fn find_scopes_for_path(&self, path: &Path) -> Option<&ScopeChain> {
    //     self.scope_register.resolve(path)
    // }

    pub fn actual_scope_for_type(&self, ty: &Type, current_scope: &ScopeChain) -> ScopeChain {
        // println!("actual_scope_for_type: {} in [{}]", format_token_stream(ty), current_scope);
        let p = parse_quote!(#ty);
        let scope = if let Some(st) = self.maybe_scope_object(ty, current_scope) {
            // match st {
            //     ObjectConversion::Type(_) => {}
            //     ObjectConversion::Item(_, _) => {}
            //     ObjectConversion::Empty => {}
            // }
            let self_ty = st.to_ty().unwrap();
            let self_path: Path = self_ty.to_path();
            self.maybe_scope(&self_path).cloned()
        } else if let Some(import_path) = self.maybe_scope_import_path(current_scope, &p) {
            self.maybe_scope(import_path).cloned()
        } else {
            None
        };
        // println!("actual_scope_for_type: [{:?}]", scope);
        scope.unwrap_or(ScopeChain::crate_root(current_scope.crate_ident().clone()))
    }
    // pub fn maybe_trait(&self, full_ty: &Type) -> Option<TraitCompositionPart1> {
    //     let full_scope: PathHolder = parse_quote!(#full_ty);
    //     self.maybe_scope(&full_scope.0)
    //         .and_then(|scope| self.traits.maybe_trait(scope).cloned())
    // }


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
    pub fn maybe_scope_import_path(&self, scope: &ScopeChain, chunk: &PathHolder) -> Option<&Path> {
        self.imports.maybe_path(scope, chunk)
    }

    pub fn maybe_imports_scope(&self, path: &Path) -> Option<&ScopeChain> {
        self.imports
            .inner
            .keys()
            .find_map(|scope_chain|
                {
                    // println!("resolve: {} = {} == {}", path.eq(scope_chain.self_path()), scope_chain.self_path().to_token_stream(), path.to_token_stream());
                    path.eq(scope_chain.self_path())
                        .then(|| scope_chain)
                })

    }

    pub fn maybe_import(&self, scope: &ScopeChain, path: &PathHolder) -> Option<&Path> {
        let result_opt = self.imports.maybe_import(scope, path);
        // println!("maybe_import: {path} in [{}] ---> {}", scope.self_path_holder_ref(), result_opt.to_token_stream());
        result_opt
    }

    fn traverse_scopes(&self, ty_to_replace: &TypeComposition, scope: &ScopeChain) -> Option<TypeCompositionConversion> {
        let mut new_ty_to_replace = ty_to_replace.clone();
        let ty = &ty_to_replace.ty;
        let ty_path = match ty {
            Type::Reference(type_reference) => type_reference.elem.to_path(),
            _ => ty.to_path()
        };
        // println!("traverse_scope: {} \n--- [{}]", ty_to_replace, scope);
        match scope {
            ScopeChain::CrateRoot { self_scope, .. } |
            ScopeChain::Mod { self_scope, .. } => {
                // self -> neighbour mod
                let self_path = &self_scope.self_scope.0;
                let child_scope: Path = parse_quote!(#self_path::#ty_path);
                // println!("check local scope item (mod)?: {} + {}",
                //          format_token_stream(&self_path),
                //          format_token_stream(&ty_path));
                // child -> self
                // If it's nested mod?
                self.maybe_item(&child_scope)
                    .map(|item| {
                        new_ty_to_replace.ty = child_scope.to_type();
                        item
                    })
                    .or({
                        // println!("check child re-export (mod)?:\n\t[{}]", format_token_stream(&child_scope));
                        // it also can be re-exported in child tree so we should check it

                        match self.maybe_child_reexport(&child_scope) {
                            Some(child_reexport) => {
                                // println!("child re-export found (mod)?:\n\t[{}]", format_token_stream(&child_reexport));
                                self.maybe_item(&child_reexport)
                                    .map(|item| {
                                        new_ty_to_replace.ty = child_reexport.to_type();
                                        item
                                    })
                                    .or(self.maybe_item(self_path))
                            },
                            None => {
                                // println!("child re-export not found -> check self_scope: (mod):\n\t[{}]", format_token_stream(self_path));
                                self.maybe_item(self_path)
                            }
                        }
                })
                // self -> child
                // self.maybe_item(self_path).or({
                //     // let child_scope: Path = parse_quote!(#self_path::#ty_path);
                //     // println!("traverse_scopes::mod::child: {}", child_scope.to_token_stream());
                //     self.maybe_item(&child_scope)
                //         .map(|item| {
                //             // println!("traverse_scopes::mod::child::ass: {}", child_scope.to_token_stream());
                //             new_ty_to_replace.ty = child_scope.to_type();
                //             item
                //         })
                // })
            }
            ScopeChain::Impl { self_scope, parent_scope_chain, .. } |
            ScopeChain::Trait { self_scope, parent_scope_chain, .. } |
            ScopeChain::Object { self_scope, parent_scope_chain, .. } => {
                // self -> parent mod -> neighbour mod
                let self_path = &self_scope.self_scope.0;
                let parent_path = parent_scope_chain.self_path();
                // println!("check local first (obj)?: {} + {}",
                //          format_token_stream(&parent_path),
                //          format_token_stream(&ty_path));

                // check parent + local
                let child_scope = parse_quote!(#parent_path::#ty_path);
                self.maybe_item(&child_scope)
                    .map(|item| {
                        new_ty_to_replace.ty = child_scope.to_type();
                        item
                    })
                    .or({
                        // println!("check child re-export (obj)?:\n\t[{}]", format_token_stream(&child_scope));
                        // it also can be re-exported in child tree so we should check it
                        match self.maybe_child_reexport(&child_scope) {
                            Some(child_reexport) => {
                                // println!("child re-export found (obj)?:\n\t[{}]", format_token_stream(&child_reexport));
                                self.maybe_item(&child_reexport)
                                    .map(|item| {
                                        new_ty_to_replace.ty = child_reexport.to_type();
                                        item
                                    })
                                    .or(self.maybe_item(self_path))
                                    .or({
                                        self.maybe_item(parent_path).map(|item| {
                                            new_ty_to_replace.ty = parent_path.to_type();
                                            item
                                        })
                                    })
                                    .or({
                                        let neighbour_scope: Path = parse_quote!(#parent_path::#ty_path);
                                        self.maybe_item(&neighbour_scope).map(|item| {
                                            new_ty_to_replace.ty = neighbour_scope.to_type();
                                            item
                                        })

                                    })
                            },
                            None => {
                                // println!("child re-export not found -> check self_scope: (obj):\n\t[{}]", format_token_stream(self_path));
                                self.maybe_item(self_path).or({
                                    self.maybe_item(parent_path).map(|item| {
                                        new_ty_to_replace.ty = parent_path.to_type();
                                        item
                                    })
                                })
                                    .or({
                                        let neighbour_scope: Path = parse_quote!(#parent_path::#ty_path);
                                        self.maybe_item(&neighbour_scope).map(|item| {
                                            new_ty_to_replace.ty = neighbour_scope.to_type();
                                            item
                                        })

                                    })
                            }
                        }
                    })


                // self.maybe_item(self_path).or({
                //     let child_scope: Path = parse_quote!(#self_path::#ty_path);
                //     self.maybe_item(&child_scope)
                //         .map(|item| {
                //             new_ty_to_replace.ty = child_scope.to_type();
                //             item
                //         })
                //         .or({
                //             self.maybe_item(parent_path).map(|item| {
                //                 new_ty_to_replace.ty = parent_path.to_type();
                //                 item
                //             })
                //         })
                //         .or({
                //             let neighbour_scope: Path = parse_quote!(#parent_path::#ty_path);
                //             self.maybe_item(&neighbour_scope).map(|item| {
                //                 new_ty_to_replace.ty = neighbour_scope.to_type();
                //                 item
                //             })
                //
                //         })
                // })

                // old logic:
                // self.maybe_item(self_path).or({
                //     let child_scope: Path = parse_quote!(#self_path::#ty_path);
                //     self.maybe_item(&child_scope)
                //         .map(|item| {
                //             new_ty_to_replace.ty = child_scope.to_type();
                //             item
                //         })
                //         .or({
                //             self.maybe_item(parent_path).map(|item| {
                //                 new_ty_to_replace.ty = parent_path.to_type();
                //                 item
                //             })
                //         })
                //         .or({
                //             let neighbour_scope: Path = parse_quote!(#parent_path::#ty_path);
                //             self.maybe_item(&neighbour_scope).map(|item| {
                //                 new_ty_to_replace.ty = neighbour_scope.to_type();
                //                 item
                //             })
                //
                //         })
                // })

            }
            ScopeChain::Fn { self_scope, parent_scope_chain, .. } => {
                // - Check fn scope
                // - if scope.parent is [mod | crate | impl] then lookup their child mods
                // - if scope.parent is [object | trait] then check scope.parent.parent
                let self_path = &self_scope.self_scope.0;
                self.maybe_item(self_path)
                    .or({
                        match &**parent_scope_chain {
                            ScopeChain::CrateRoot { self_scope, .. } |
                            ScopeChain::Mod { self_scope, .. } => {
                                let self_path = &self_scope.self_scope.0;
                                let child_scope: Path = parse_quote!(#self_path::#ty_path);
                                self.maybe_item(&child_scope)
                                    .map(|item| {
                                        new_ty_to_replace.ty = child_scope.to_type();
                                        item
                                    })
                            }
                            ScopeChain::Trait { parent_scope_chain, .. } |
                            ScopeChain::Object { parent_scope_chain, .. } |
                            ScopeChain::Impl { parent_scope_chain, .. } => {
                                let parent_path = parent_scope_chain.self_path();
                                let neighbour_scope: Path = parse_quote!(#parent_path::#ty_path);
                                self.maybe_item(&neighbour_scope)
                                    .map(|item| {
                                        new_ty_to_replace.ty = neighbour_scope.to_type();
                                        item
                                    })
                            }
                            ScopeChain::Fn { self_scope, parent_scope_chain, .. } => {
                                // TODO: support nested function when necessary
                                println!("nested function::: {} --- [{}]", self_scope, parent_scope_chain);
                                None
                            }
                        }
                    })
            }
        }.and_then(|scope_item| scope_item.update_scope_item(new_ty_to_replace))
    }

    fn maybe_known_item(&self, ty_to_replace: &TypeComposition, scope: &ScopeChain) -> Option<TypeCompositionConversion> {
        // So we found a unknown path chunk in the scope
        // Here we trying to determine paths where the actual item located (if it's not evidently imported)
        // before marking this item as "possibly global"
        // So we build a stack with paths where it could be depending on type of the current scope
        // It should include now mods nested into parent mods (neighbour mods)
        // TODO: Support "extern crate", "super::", "self::" (paths kinda Self::AA may also require additional search routes)
        let new_ty_to_replace = ty_to_replace.clone();
        let ty = &ty_to_replace.ty;
        let path = match ty {
            Type::Reference(type_reference) => type_reference.elem.to_path(),
            _ => ty.to_path()
        };
        // println!("maybe_known_item: {} in [{}]", ty_to_replace, scope.self_path_holder_ref());
        // Local scopes are prioritized so we should check relative paths first
        // Then we should check crate-level scopes
        // let scope_path = scope.self_path_holder_ref();
        // let local_path = parse_quote!(#scope_path::#path);

        self.maybe_item(&path)
            .and_then(|scope_item| scope_item.update_scope_item(new_ty_to_replace))
            .or(self.traverse_scopes(ty_to_replace, scope))
    }

    fn maybe_refine_args(&self, segment: &mut PathSegment, nested_arguments: &mut Punctuated<NestedArgument, Comma>) {
        // println!("maybe_refine_args::: {} ---- {:?}", segment.to_token_stream(), nested_arguments);
        match &mut segment.arguments {
            PathArguments::None => {
                if !nested_arguments.is_empty() {
                    segment.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: nested_arguments.into_iter().map(|nested_arg| match nested_arg {
                            NestedArgument::Object(obj) => GenericArgument::Type(obj.to_ty().unwrap())
                        }).collect(),
                        gt_token: Default::default(),
                    });
                }
            }
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                // panic!("Parenthesized args: {} -> {}", inputs.to_token_stream(), output.to_token_stream())
            },
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { ref mut args, .. }) => {
                // println!("GENERIC::Args:: {}", args.to_token_stream());
                args.iter_mut()
                    .for_each(|arg| match arg {
                        GenericArgument::Type(inner_ty) => {
                            // println!("GENERIC::TYpe:: {}", inner_ty.to_token_stream());
                            match nested_arguments.pop() {
                                None => {}
                                Some(nested_arg) => match nested_arg.into_value() {
                                    NestedArgument::Object(obj) => {
                                        *inner_ty = obj.to_ty().unwrap();
                                    }
                                }
                            }
                        }
                        GenericArgument::Lifetime(_) => {}
                        GenericArgument::Const(_) => {}
                        GenericArgument::Binding(_) => {}
                        GenericArgument::Constraint(_) => {}
                    });
            }
        };

    }

    fn maybe_refined_object(&self, scope: &ScopeChain, object: &ObjectConversion) -> Option<ObjectConversion> {
        match object {
            ObjectConversion::Type(TypeCompositionConversion::Imported(ty_composition, import_path)) => {
                let mut ty_replacement = ty_composition.clone();
                let mut import_type_path: TypePath = parse_quote!(#import_path);

                // let last_segment_pair = import_type_path.path.segments.pop().unwrap();
                import_type_path.path = if import_path.is_crate_based() {
                    import_path.replaced_first_with_ident(&scope.crate_ident_as_path())
                } else {
                    import_path.clone()
                };
                let mut chunks = import_type_path.path.clone();
                while !chunks.segments.is_empty() {
                    chunks.segments = chunks.segments.popped();
                    if !chunks.segments.is_empty() {
                        let mod_chain = create_mod_chain(&chunks);
                        // println!("chchchc: {}",)
                        if let Some(parent_imports) = self.imports.maybe_scope_imports(&mod_chain) {
                            // println!("chchchc: {}", parent_imports)
                            for (PathHolder(_ident), alias_path) in parent_imports {
                                let alias = if alias_path.is_crate_based() {
                                    alias_path.replaced_first_with_ident(&scope.crate_ident_as_path())
                                } else {
                                    alias_path.clone()
                                };
                                if let Some(merged) = self.refined_import(&import_type_path.path, &alias, scope) {
                                    let last = import_type_path.path.segments.last().cloned();
                                    import_type_path.path.segments = merged.segments;
                                    if let Some(last) = last {
                                       // import_type_path.path.segments.last_mut().unwrap().arguments = last.arguments;
                                    }
                                }
                            }
                        }
                    }
                }
                // println!("maybe_refined_object::: {} ---- {} ----> {}", ty_composition, import_path.to_token_stream(), import_type_path.to_token_stream());
                // let mut last_segment = last_segment_pair.into_value();

                // match &mut last_segment.arguments {
                //     PathArguments::None => {}
                //     PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                //         panic!("Parenthesized args: {} -> {}", inputs.to_token_stream(), output.to_token_stream())
                //     },
                //     PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                //         println!("GENERIC::Args:: {}", args.to_token_stream());
                //         args.iter_mut()
                //             .for_each(|arg| match arg {
                //                 GenericArgument::Type(inner_ty) => {
                //                     println!("GENERIC::TYpe:: {}", inner_ty.to_token_stream());
                //                     match ty_replacement.nested_arguments.pop() {
                //                         None => {}
                //                         Some(nested_arg) => match nested_arg.into_value() {
                //                             NestedArgument::Object(obj) => {
                //                                 *inner_ty = obj.to_ty().unwrap();
                //                             }
                //                         }
                //                     }
                //                 }
                //                 GenericArgument::Lifetime(_) => {}
                //                 GenericArgument::Const(_) => {}
                //                 GenericArgument::Binding(_) => {}
                //                 GenericArgument::Constraint(_) => {}
                //             });
                //     }
                // };
                //
                // import_type_path.path.segments.last_mut().unwrap().arguments = last_segment.arguments;
                self.maybe_refine_args(import_type_path.path.segments.last_mut().unwrap(), &mut ty_replacement.nested_arguments);
                let dict_path = import_type_path.path.clone();
                ty_replacement.ty = Type::Path(import_type_path);
                let conversion_replacement = if let Some(dictionary_type) = scope.maybe_dictionary_type(&dict_path, self) {
                    dictionary_type
                } else if let Some(found_item) = self.maybe_known_item(&ty_replacement, scope) {
                    // println!("[INFO] Known item found: [{}]", found_item.to_token_stream());
                    found_item
                } else {
                    println!("[WARN] Unknown import: [{}]", format_token_stream(&ty_replacement.ty));
                    TypeCompositionConversion::Unknown(ty_replacement)
                };
                return Some(ObjectConversion::Type(conversion_replacement));
            },
            ObjectConversion::Type(TypeCompositionConversion::Unknown(ty_to_replace)) => {
                if let Some(refined) = self.maybe_known_item(ty_to_replace, scope) {
                    // println!("[INFO] Refined item found: [{}]", refined.to_token_stream());
                    return Some(ObjectConversion::Type(refined));
                }
            },
            ObjectConversion::Type(TypeCompositionConversion::Array(ty_composition)) => {
                let mut new_ty_composition = ty_composition.clone();
                self.refine_nested_ty(&mut new_ty_composition, scope);
                return Some(ObjectConversion::Type(TypeCompositionConversion::Array(new_ty_composition)));
            }
            ObjectConversion::Type(TypeCompositionConversion::Slice(ty_composition)) => {
                let mut new_ty_composition = ty_composition.clone();
                self.refine_nested_ty(&mut new_ty_composition, scope);
                return Some(ObjectConversion::Type(TypeCompositionConversion::Slice(new_ty_composition)));
            }
            ObjectConversion::Type(TypeCompositionConversion::Tuple(ty_composition)) => {
                let mut new_ty_composition = ty_composition.clone();
                self.refine_nested_ty(&mut new_ty_composition, scope);
                return Some(ObjectConversion::Type(TypeCompositionConversion::Tuple(new_ty_composition)));
            },
            ObjectConversion::Type(TypeCompositionConversion::Object(ty_composition)) => {
                if object.is_type(&parse_quote!(Option<get_identity_response_v0::Result>)) || object.is_type(&parse_quote!(get_identity_response_v0::Result)) {
                    println!("[INFO] refine [Object]: {}", ty_composition);
                }
                return Some(ObjectConversion::Type(TypeCompositionConversion::Object(self.refine_nested(ty_composition, scope))));
            }
            ObjectConversion::Type(TypeCompositionConversion::Optional(ty_composition)) => {
                // let mut new_ty_composition = ty_composition.clone();
                // self.refine_nested_ty(&mut new_ty_composition, scope);
                // return Some(ObjectConversion::Type(TypeCompositionConversion::Optional(new_ty_composition)));
                let nested_refined = self.refine_nested(ty_composition, scope);
                if object.is_type(&parse_quote!(Option<get_identity_response_v0::Result>)) || object.is_type(&parse_quote!(get_identity_response_v0::Result)) {
                    println!("Refine optional: composition: {} ----> {}", ty_composition, nested_refined);
                }
                return Some(ObjectConversion::Type(TypeCompositionConversion::Optional(nested_refined)));
            },
            ObjectConversion::Type(TypeCompositionConversion::Trait(composition, dec, paths)) => {
                return Some(ObjectConversion::Type(TypeCompositionConversion::Trait(self.refine_nested(composition, scope), dec.clone(), paths.clone())));
            },
            ObjectConversion::Type(TypeCompositionConversion::TraitType(composition)) => {
                return Some(ObjectConversion::Type(TypeCompositionConversion::TraitType(self.refine_nested(composition, scope))));
            },
            ObjectConversion::Type(TypeCompositionConversion::Bounds(_composition)) => {
                // TODO::
            },
            ObjectConversion::Type(TypeCompositionConversion::SmartPointer(_composition)) => {
                // TODO::
            },
            ObjectConversion::Type(TypeCompositionConversion::Fn(_composition)) => {
                // TODO::
            },
            _ => {}
        }
        None
    }
    fn refine_nested(&self, composition: &TypeComposition, scope: &ScopeChain) -> TypeComposition {
        let mut new_ty_composition = composition.clone();
        new_ty_composition.nested_arguments
            .iter_mut()
            .for_each(|arg| match arg {
                NestedArgument::Object(obj) => {
                    if let Some(object_to_refine) = self.maybe_refined_object(scope, obj) {
                        if composition.ty.eq(&parse_quote!(Option<get_identity_response_v0::Result>)) ||
                            composition.ty.eq(&parse_quote!(get_identity_response_v0::Result)) {
                            println!("nested refined: \n\t{}", object_to_refine);
                        }

                        *obj = object_to_refine;
                    }
                }
            });
        new_ty_composition.ty.refine_with(new_ty_composition.nested_arguments.clone());
        new_ty_composition
    }
    fn refine_nested_ty(&self, new_ty_composition: &mut TypeComposition, scope: &ScopeChain) {
        match &mut new_ty_composition.ty {
            Type::Tuple(type_tuple) => {
                type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| {
                    match &mut new_ty_composition.nested_arguments[index] {
                        NestedArgument::Object(obj) => {
                            if let Some(object_to_refine) = self.maybe_refined_object(scope, obj) {
                                let to_ty = object_to_refine.to_ty().unwrap();
                                *obj = object_to_refine;
                                *elem = to_ty;
                            }
                        },
                    }
                });
            },
            Type::Array(type_array) => {
                match &mut new_ty_composition.nested_arguments.first_mut() {
                    Some(NestedArgument::Object(obj)) => {
                        if let Some(object_to_refine) = self.maybe_refined_object(scope, obj) {
                            let to_ty = object_to_refine.to_ty().unwrap();
                            *obj = object_to_refine;
                            *type_array.elem = to_ty;
                        }
                    }
                    None => {}
                }
            },
            Type::Slice(type_slice) => {
                match &mut new_ty_composition.nested_arguments.first_mut() {
                    Some(NestedArgument::Object(obj)) => {
                        if let Some(object_to_refine) = self.maybe_refined_object(scope, obj) {
                            let to_ty = object_to_refine.to_ty().unwrap();
                            *obj = object_to_refine;
                            *type_slice.elem = to_ty;
                        }
                    }
                    None => {}
                }
            }
            _ => {}
        }

    }

}


impl RefineMut for GlobalContext {
    type Refinement = ScopeRefinement;
    // fn refine_with(&mut self, refined: Self::Refinement) {
    //     self.scope_register.refine_with(refined);
    //     let mut refined_generics = HashSet::<GenericConversion>::new();
    //     self.scope_register.inner.values()
    //         .for_each(|type_chain| {
    //             type_chain.inner.values().for_each(|conversion| {
    //                 // println!("Generic::Refine: {}", conversion);
    //                 match conversion {
    //                     ObjectConversion::Type(ty) => {
    //                         println!("REFINE:: ObjectConversion::Type:: {}", ty);
    //                         refined_generics.extend(ty
    //                             .to_ty()
    //                             .find_generics()
    //                             .iter()
    //                             .filter_map(|ty| {
    //                                 println!("--------------- {}", type_chain);
    //                                 if self.custom.maybe_conversion(&ty.0).is_some() {
    //                                     None
    //                                 } else {
    //                                     type_chain.get(ty)
    //                                 }
    //                             })
    //                             .map(GenericConversion::from));
    //                     },
    //                     ObjectConversion::Item(_, conversion) => {
    //                         println!("REFINE:: ObjectConversion::Item:: {}", conversion);
    //                         refined_generics.extend(conversion.find_generics()
    //                             .iter()
    //                             .filter_map(|ty| {
    //                                 if self.custom.maybe_conversion(&ty.0).is_some() {
    //                                     None
    //                                 } else {
    //                                     type_chain.get(ty)
    //                                 }
    //                             })
    //                             .map(GenericConversion::from));
    //                     }
    //                     ObjectConversion::Empty => {}
    //                 }
    //             })
    //         });
    //     self.refined_generics = refined_generics;
    // }
    fn refine_with(&mut self, refined: Self::Refinement) {
        self.scope_register.refine_with(refined);
        let mut refined_generics = HashSet::<GenericConversion>::new();
        self.scope_register.inner.values()
            .for_each(|type_chain| {
                type_chain.inner.iter().for_each(|(conversion, value)| {
                    refined_generics.extend(conversion.0
                        .find_generics()
                        .iter()
                        .filter_map(|ty| {
                            println!("refine_with: maybe custom?: {} --- {}", ty.to_token_stream(), value.to_token_stream());
                            if self.custom.maybe_conversion(&ty.0).is_some() {
                                None
                            } else {
                                Some(value)
                            }
                        })
                        .map(GenericConversion::from));
                })
            });
        self.refined_generics = refined_generics;
    }
}

impl Unrefined for GlobalContext {
    type Unrefinement = ScopeRefinement;
    fn unrefined(&self) -> Self::Unrefinement {
        let mut scope_updates = vec![];
        // let mut generic_updates = vec![];
        self.scope_register.inner.iter()
            .for_each(|(scope, type_chain)| {
                let scope_types_to_refine = type_chain.inner.iter()
                    .filter_map(|(holder, object)|
                        if let Some(object_to_refine) = self.maybe_refined_object(scope, object) {
                            // println!("unrefined: {}: {} --> {}", holder, object, object_to_refine);
                            Some((holder.clone(), object_to_refine))
                        } else {
                            None
                        })
                    .collect::<HashMap<_, _>>();
                // let generics_to_refine = type_chain.inner.iter().filter_map(|(holder, object)| {
                //
                // }).collect();

                if !scope_types_to_refine.is_empty() {
                    scope_updates.push((scope.clone(), scope_types_to_refine));
                }
            });

        //self.scope_register.inner.iter().for_each()

        // let mut iter = scope_updates.iter()
        //     .map(|(key, value)| format!("\t{}:\n\t\t{}", key, format_types_dict(&value)))
        //     .collect::<Vec<String>>();
        // iter.sort();
        //
        // println!("scope_updates: \n{}", iter.join("\n\n"));
        scope_updates
    }
}

impl RefineUnrefined for GlobalContext {}

/// Scope
impl GlobalContext {
    pub fn scope_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        let result = self.scope_register.scope_register_mut(scope);
        // println!("scope_mut: {} -> \n{}", scope, result);
        result
    }
    pub fn maybe_scope_object(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
        self.scope_register.maybe_scope_type(ty, scope)
    }
    pub fn maybe_scope(&self, path: &Path) -> Option<&ScopeChain> {
        let x = self.scope_register.resolve(path);
        //println!("maybe_scope: {} --> {}", path.to_token_stream(), x.map(ScopeChain::self_path_holder_ref).to_token_stream());
        x
    }

    // traverses import scope for re-exports (doesn't include re-exports in neighbour scopes)
    fn maybe_child_reexport(&self, child_scope_candidate: &Path) -> Option<Path> {
        let mut scope_path_candidate = child_scope_candidate.clone();
        let mut result: Option<Path> = None;
        let mut chunk: Option<Path> = None;
        while let Some(scope_path_last_segment) = child_scope_candidate.segments.last() {
            scope_path_candidate = scope_path_candidate.popped();
            match self.maybe_imports_scope(&scope_path_candidate) {
                Some(reexport_scope) => {
                    // println!("[INFO: {index}:{}] Child reexport_scope found: \n\t[{}]",
                    //          index_for_result.map_or(format!("None"), |index| format!("{}", index)),
                    //          format_token_stream(reexport_scope.self_path_holder_ref()));
                    let path: PathHolder = parse_quote!(#scope_path_last_segment);
                    match self.maybe_import(reexport_scope, &path) {
                        Some(reexport_import) => {
                            let reexport_scope_path = reexport_scope.self_path_holder_ref();
                            let result_chunk: Path = if chunk.is_some() {
                                let reexport_chunk = reexport_import.popped();
                                parse_quote!(#reexport_chunk::#chunk)
                            } else {
                                parse_quote!(#reexport_import)
                            };
                            // println!("[INFO: {index}:{}] Child Re-export found: \n\t[{}] +\n\t[{}]\n\tsegments: [{}]",
                            //          index_for_result.map_or(format!("None"), |index| format!("{}", index)),
                            //          format_token_stream(reexport_scope_path),
                            //          format_token_stream(reexport_import),
                            //          format_token_stream(&chunk));
                            result = Some(parse_quote!(#reexport_scope_path::#result_chunk));
                            chunk = Some(result_chunk);
                        },
                        None => {
                            // println!("[INFO: {index}:{}] Child re-export not found: [{}]",
                            //          index_for_result.map_or(format!("None"), |index| format!("{}", index)),
                            //          format_token_stream(&path));
                            if scope_path_candidate.segments.is_empty() {
                                return result;
                            } else if let Some(reexport) = self.maybe_child_reexport(&scope_path_candidate) {
                                result = Some(reexport);
                            }
                        }
                    }
                },
                None => {
                    // println!("[INFO: {index}:{}] Child reexport_scope not found: [{}]",
                    //          index_for_result.map_or(format!("None"), |index| format!("{}", index)),
                    //          format_token_stream(&scope_path_candidate));
                    if scope_path_candidate.segments.is_empty() {
                        return result;
                    } else if let Some(reexport) = self.maybe_child_reexport(&scope_path_candidate) {
                        result = Some(reexport);
                    }
                }
            }
        }
        result
    }

    fn lookup_reexport(&self, import_path: &Path) -> Option<Path> {
        let mut scope_path_candidate = import_path.clone();
        let mut result: Option<Path> = None;
        let mut chunk: Option<Path> = None;
        while let Some(scope_path_last_segment) = import_path.segments.last() {
            scope_path_candidate = scope_path_candidate.popped();
            match self.maybe_imports_scope(&scope_path_candidate) {
                Some(reexport_scope) => {
                    let path: PathHolder = parse_quote!(#scope_path_last_segment);
                    match self.maybe_import(reexport_scope, &path) {
                        Some(reexport_import) => {
                            let reexport_scope_path = reexport_scope.self_path_holder_ref();
                            println!("[INFO] Re-export found: \n\t[{}] +\n\t[{}]\n\t[{}]",
                                     format_token_stream(reexport_scope_path),
                                     format_token_stream(reexport_import),
                                     format_token_stream(&chunk));

                            let segments: Punctuated<PathSegment, Colon2> = if chunk.is_some() {
                                let reexport_chunk = reexport_import.popped();
                                match reexport_import.segments.first().unwrap().ident.to_string().as_str() {
                                    "crate" => {
                                        let crate_name_chunk = reexport_scope.crate_ident().to_path();
                                        let result = reexport_import.replaced_first_with_ident(&crate_name_chunk);
                                        let new_segments_iter = result.segments.iter().skip(reexport_scope_path.len());
                                        let new_path: Path = parse_quote!(#(#new_segments_iter)::*);


                                        // println!("----- {} + {}", new_path.to_token_stream(), chunk.to_token_stream());
                                        let re_result = merge_reexport_chunks(&new_path, &chunk.as_ref().unwrap());
                                        // println!("----- {}", re_result.to_token_stream());

                                        parse_quote!(#re_result)
                                        // result.segments.iter().skip(reexport_scope_path.len()).cloned().collect()
                                    },
                                    "self" => {
                                        reexport_import.segments.iter().skip(1).cloned().collect()
                                    },
                                    "super" => {
                                        let super_path = reexport_scope_path.popped();
                                        parse_quote!(#super_path::#reexport_import)
                                    },
                                    _ => parse_quote!(#reexport_chunk::#chunk)
                                }
                            } else {
                                match reexport_import.segments.first().unwrap().ident.to_string().as_str() {
                                    "crate" => {
                                        let crate_name_chunk = reexport_scope.crate_ident().to_path();
                                        let result = reexport_import.replaced_first_with_ident(&crate_name_chunk);
                                        result.segments.iter().skip(reexport_scope_path.len()).cloned().collect()
                                    },
                                    "self" => {
                                        reexport_import.segments.iter().skip(1).cloned().collect()
                                    },
                                    "super" => {
                                        let super_path = reexport_scope_path.popped();
                                        parse_quote!(#super_path::#reexport_import)
                                    },
                                    _ => parse_quote!(#reexport_import)
                                }
                            };



                            // if reexport_import is current-crate-based then (crate::xx::)
                            //      replace 'crate' with actual crate name and use it as is
                            // else
                            //      if reexport_import is external-crate-based (external_crate_name::xx::)
                            //          use it as is
                            //      else
                            //          join as #reexport_chunk::#chunk

                            //
                            // let result_chunk: Path = if chunk.is_some() {
                            //     let reexport_chunk = reexport_import.popped();
                            //     parse_quote!(#reexport_chunk::#chunk)
                            // } else {
                            //     parse_quote!(#reexport_import)
                            // };
                            // if reexport_import.is_crate_based() {
                            //
                            // }
                            // println!("REFINED: [{}] + [{}]", reexport_scope_path.to_token_stream(), segments.to_token_stream());
                            result = Some(parse_quote!(#reexport_scope_path::#segments));
                            chunk = Some(Path { segments, leading_colon: None });
                        },
                        None => {
                            if scope_path_candidate.segments.is_empty() {
                                return result;
                            } else if let Some(reexport) = self.lookup_reexport(&scope_path_candidate) {
                                result = Some(reexport);
                            }
                        }
                    }
                },
                None => {
                    if scope_path_candidate.segments.is_empty() {
                        return result;
                    } else if let Some(reexport) = self.lookup_reexport(&scope_path_candidate) {
                        result = Some(reexport);
                    }
                }
            }
        }
        result
    }
    // fn lookup_reexport(&self, import_path: &Path) -> Option<Path> {
    //     let mut scope_path_candidate = import_path.clone();
    //     let mut result: Option<Path> = None;
    //
    //     // Iterate through the path segments to deduce re-exports
    //     while let Some(scope_path_last_segment) = import_path.segments.last() {
    //         scope_path_candidate = scope_path_candidate.popped();  // Adjust the candidate path by popping the last segment
    //         if let Some(reexport_scope) = self.maybe_imports_scope(&scope_path_candidate) {
    //             let path_holder: PathHolder = parse_quote!(#scope_path_last_segment);
    //
    //             if let Some(reexport_import) = self.maybe_import(reexport_scope, &path_holder) {
    //
    //                 let reexport_scope_path = reexport_scope.self_path_holder_ref();
    //                 println!("[INFO] Re-export found: \n\t[{}] +\n\t[{}]", reexport_scope_path.to_token_stream(), reexport_import.to_token_stream());
    //
    //                 // Construct the full re-export path
    //                 let mut full_path = reexport_scope_path.clone();
    //                 full_path.0.segments.extend(reexport_import.segments.clone());
    //                 result = Some(full_path.0);
    //
    //                 break; // Found a valid re-export, no need to continue
    //             }
    //         }
    //         // If no segment left or no re-exports found, try to check recursively on the adjusted path
    //         if scope_path_candidate.segments.is_empty() {
    //             break; // No more segments to process
    //         }
    //     }
    //
    //     result.or_else(|| {
    //         // If result is still None and there are segments to process, try recursively
    //         scope_path_candidate = scope_path_candidate.popped();  // Adjust for recursion
    //         if !scope_path_candidate.segments.is_empty() {
    //             self.lookup_reexport(&scope_path_candidate)
    //         } else {
    //             None
    //         }
    //     })
    // }
    fn refined_import(&self, import_path: &Path, alias: &Path, scope: &ScopeChain) -> Option<Path> {
        // let mut merged_path: Option<Path> = None;
        // let mut import_segments = import_path.segments.clone();
        // let mut alias_segments = alias.segments.clone();
        // let import_path_holder = PathHolder::from(import_path);
        // println!("REEXPORT: {}", reexport.to_token_stream());
        let last_import_segment = import_path.segments.last();
        let last_alias_segment = alias.segments.last();
        // if let Some(last_import_segment) =
        if last_import_segment.is_some() &&
            last_alias_segment.is_some() &&
            last_import_segment.unwrap().ident == last_alias_segment.unwrap().ident {
            println!("[INFO] Try refine import:\n\timport: [{}]\n\talias: [{}]\n\tscope: [{}]",
                     format_token_stream(import_path),
                     format_token_stream(alias),
            scope.self_path_holder_ref());
            // println!("[INFO] Try refine import: [{}]\n\talias: [{}]\n\tscope: [{}]",
            //          format_token_stream(import_path),
            //          format_token_stream(alias),
            //          scope.self_path_holder_ref());
            let reexport = self.lookup_reexport(import_path);
            if reexport.is_some() {
                println!("[INFO] Re-export assigned:\n\t[{}]", format_token_stream(&reexport));
            }
            return reexport;
        }
        None
        // if let (Some(last_import_segment), Some(last_alias_segment)) = (import_segments.pop(), alias_segments.pop()) {
        //     let import_ident = &last_import_segment.value().ident;
        //     let alias_ident = &last_alias_segment.value().ident;
        //     if import_ident == alias_ident {
        //         println!("[INFO] Try refine import: [{}]\n\talias: [{}]\n\tscope: [{}]",
        //                  format_token_stream(import_path),
        //                  format_token_stream(alias),
        //                  scope.self_path_holder_ref());
        //         // let scope_path_candidate: Path = parse_quote!(#import_segments);
        //
        //         // match self.maybe_scope(&path.0) {
        //         //     Some(reexport_scope_candidate) => match self.maybe_import(reexport_scope_candidate, ) {
        //         //         Some() => {},
        //         //         None => {}
        //         //     },
        //         //     None => self.maybe_scope()
        //         // }
        //         //
        //         // if let Some(scope) = self.maybe_scope(&path.0) {
        //         //     self.maybe_import(scope, )
        //         // } else if let Some() = self.maybe_scope() {  }
        //         //
        //         // if let Some(refined_path) = self.maybe_import(scope, &path) {
        //         //     println!("[INFO] Re-exported in: [{}]\n\tas: [{}]",
        //         //              format_token_stream(path),
        //         //              format_token_stream(refined_path));
        //         //     return Some(refined_path.clone());
        //         // }
        //         let remaining_import_path = Path { leading_colon: import_path.leading_colon, segments: import_segments.clone() };
        //         let remaining_alias_path = Path { leading_colon: alias.leading_colon, segments: alias_segments.clone() };
        //         if remaining_import_path == remaining_alias_path {
        //             merged_path = Some(alias.clone());
        //         } else if let Some(refined_path) = self.refined_import(&remaining_import_path, &remaining_alias_path, scope) {
        //             let mut segments = refined_path.segments;
        //             segments.push(PathSegment::from(import_ident.clone()));
        //             merged_path = Some(Path { leading_colon: None, segments });
        //         }
        //     }
        // }
        // if merged_path.is_none() {
        //     if let Some(parent_scope) = scope.parent_scope() {
        //         merged_path = self.refined_import(import_path, alias, parent_scope);
        //     }
        // }
        // if merged_path.is_some() {
        //     println!("[INFO] Import refined (alias): [{}]\n\talias: [{}]\n\tscope: [{}]\n\t=====: [{}]",
        //              format_token_stream(import_path),
        //              format_token_stream(alias),
        //              scope.self_path_holder_ref(),
        //              format_token_stream(&alias));
        // }
        // merged_path
    }

}

pub fn create_mod_chain(path: &Path) -> ScopeChain {
    // print!("create_mod_chain: {}", path.to_token_stream());
    let segments = &path.segments;

    let crate_ident = &segments.first().unwrap().ident;
    let self_scope = Scope::new(PathHolder::from(path), ObjectConversion::Empty);
    let parent_chunks = path.popped();
    let parent_scope_chain = if parent_chunks.segments.len() > 1 {
        create_mod_chain(&parent_chunks)
    } else {
        ScopeChain::CrateRoot {
            crate_ident: crate_ident.clone(),
            self_scope: Scope { self_scope: PathHolder(parent_chunks), object: ObjectConversion::Empty }
        }
    };
    if segments.len() == 1 {
        ScopeChain::CrateRoot {
            crate_ident: crate_ident.clone(),
            self_scope
        }
    } else {
        ScopeChain::Mod {
            crate_ident: crate_ident.clone(),
            self_scope,
            parent_scope_chain: Box::new(parent_scope_chain.clone())
        }
    }
}

// fn refined_import(import_path: &Path, alias: &Path, scope: &ScopeChain) -> Option<Path> {
//     // TODO:: it should be more complex actually
//     // Because:
//     // - items can be aliased by multiple items
//     // - items can be aliased at the different layers
//     // - items can be re-exported across multiple crates
//     let mut last_import_segments = import_path.clone();
//     let mut last_alias_segments = alias.clone();
//     let mut merged_path: Option<Path> = None;
//     //println!("merged_import: {} <===> {}", import_path.to_token_stream(), alias.to_token_stream());
//     match (last_import_segments.segments.pop(), last_alias_segments.segments.pop()) {
//         (Some(last_import_pair), Some(last_alias_pair)) => {
//             let ident = last_import_pair.value().ident.clone();
//             let full_match = last_import_segments.eq(&last_alias_segments);
//             if !full_match && ident.eq(&last_alias_pair.value().ident) {
//                 println!("[INFO] Refine import: [{}]\n\talias: [{}]\n\tscope: [{}]\n\t=====: [{}] + [{}] + [{}]",
//                          format_token_stream(import_path),
//                          format_token_stream(alias),
//                     scope.self_path_holder_ref(),
//                     format_token_stream(&last_import_segments.segments),
//                     format_token_stream(&last_alias_segments.segments),
//                          ident);
//                 let mut path = Path { leading_colon: None, segments: last_import_segments.segments };
//                 if !full_match {
//                     path.segments.extend(last_alias_segments.segments);
//                 }
//                 path.segments.push(PathSegment::from(ident));
//                 merged_path = Some(path);
//             }
//         },
//         _ => {}
//     }
//     merged_path
// }



// fn refined_import(import_path: &Path, alias: &Path, scope: &ScopeChain) -> Option<Path> {
//     let mut merged_path: Option<Path> = None;
//     let mut import_segments = import_path.segments.clone();
//     let mut alias_segments = alias.segments.clone();
//     if let (Some(last_import_segment), Some(last_alias_segment)) = (import_segments.pop(), alias_segments.pop()) {
//         let import_ident = &last_import_segment.value().ident;
//         let alias_ident = &last_alias_segment.value().ident;
//         if import_ident == alias_ident {
//             println!("[INFO] Try refine import: [{}]\n\talias: [{}]\n\tscope: [{}]",
//                      format_token_stream(import_path),
//                      format_token_stream(alias),
//                      scope.self_path_holder_ref());
//             let remaining_import_path = Path { leading_colon: import_path.leading_colon, segments: import_segments.clone() };
//             let remaining_alias_path = Path { leading_colon: alias.leading_colon, segments: alias_segments.clone() };
//             if remaining_import_path == remaining_alias_path {
//                 println!("[INFO] Import refined (alias): [{}]\n\talias: [{}]\n\tscope: [{}]\n\t=====: [{}]",
//                          format_token_stream(import_path),
//                          format_token_stream(alias),
//                          scope.self_path_holder_ref(),
//                          format_token_stream(&alias));
//                 merged_path = Some(alias.clone());
//
//             } else {
//                 if let Some(refined_path) = refined_import(&remaining_import_path, &remaining_alias_path, scope) {
//                     let mut segments = refined_path.segments;
//                     segments.push(PathSegment::from(import_ident.clone()));
//                     println!("[INFO] Import refined (recursive): [{}]\n\talias: [{}]\n\tscope: [{}]\n\t=====: [{}]",
//                              format_token_stream(import_path),
//                              format_token_stream(alias),
//                              scope.self_path_holder_ref(),
//                              format_token_stream(&segments));
//                     merged_path = Some(Path { leading_colon: None, segments });
//                 }
//             }
//         }
//     }
//
//     merged_path
// }


fn probe_chunks<'a>(
    import_chunk: &'a mut Path,
    alias_chunk: &'a mut Path,
    merged_chunk: &'a mut Punctuated<PathSegment, Colon2>) -> Option<(&'a mut Path, &'a mut Path, &'a mut Punctuated<PathSegment, Colon2>)> {
    match (import_chunk.segments.pop(), alias_chunk.segments.pop()) {
        (Some(last_import_pair), Some(last_alias_pair)) => {
            let ident = last_import_pair.value().ident.clone();
            if ident.eq(&last_alias_pair.value().ident) {
                merged_chunk.insert(0, PathSegment::from(ident));
                return Some((import_chunk, alias_chunk, merged_chunk))
            }
        },
        _ => {}
    }
    None
}
fn refined_import2(import_path: &Path, alias: &Path, scope: &ScopeChain) -> Option<Path> {
    // TODO:: it should be more complex actually
    // Because:
    // - items can be aliased by multiple items
    // - items can be aliased at the different layers
    // - items can be re-exported across multiple crates
    let mut import_chunk = import_path.clone();
    let mut alias_chunk = alias.clone();
    let mut merged_path: Option<Path> = None;
    let mut merged_chunk = Punctuated::new();
    while let Some((import_chunk, alias_chunk, merged_chunk)) = probe_chunks(&mut import_chunk, &mut alias_chunk, &mut merged_chunk) {
        println!("[INFO] Refine import: [{}]\n\talias: [{}]\n\tscope: [{}]\n\t=====: [{}] + [{}] + [{}]",
                 format_token_stream(import_path),
                 format_token_stream(alias),
                 scope.self_path_holder_ref(),
                 format_token_stream(&import_chunk.segments),
                 format_token_stream(&alias_chunk.segments),
                 format_token_stream(&merged_chunk));
        merged_path = Some(merge_import_chunks(merged_chunk.clone(), import_chunk.clone(), alias_chunk.clone()));

    }
    merged_path
}

fn merge_import_chunks(ident: Punctuated<PathSegment, Colon2>, import_chunk: Path, alias_chunk: Path) -> Path {
    let full_match = import_chunk.eq(&alias_chunk);
    let mut path = Path { leading_colon: None, segments: import_chunk.segments };
    if !full_match {
        path.segments.extend(alias_chunk.segments);
    }
    path.segments.extend(ident);
    path
}

fn merge_reexport_chunks(base: &Path, extension: &Path) -> Path {
    let mut base_segments: Vec<_> = base.segments.iter().collect();
    let mut ext_segments: Vec<_> = extension.segments.iter().collect();
    base_segments.reverse();
    ext_segments.reverse();
    let mut result_segments = vec![];
    let mut skip = 0;
    for (base_segment, ext_segment) in base_segments.iter().zip(ext_segments.iter()) {
        if base_segment.ident == ext_segment.ident {
            skip += 1;
        } else {
            break;
        }
    }
    base_segments.reverse();
    ext_segments.reverse();
    result_segments.extend(base_segments.iter().take(base_segments.len() - skip).cloned());
    result_segments.extend(ext_segments.into_iter());
    Path {
        leading_colon: base.leading_colon,
        segments: result_segments.into_iter().cloned().collect(),
    }
}