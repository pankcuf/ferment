use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{Attribute, parse_quote, Path, PathSegment, Type, TypePath};
use syn::punctuated::Punctuated;
use crate::Config;
use crate::ast::PathHolder;
use crate::composable::{GenericBoundComposition, GenericConversion, TraitCompositionPart1, TypeComposition};
use crate::context::{CustomResolver, GenericResolver, ImportResolver, ScopeChain, ScopeRefinement, ScopeResolver, TraitsResolver, TypeChain};
use crate::conversion::{DictionaryTypeCompositionConversion, ObjectConversion, ScopeItemConversion, TypeCompositionConversion};
use crate::ext::{GenericCollector, GenericConstraintCollector, RefineInScope, RefineMut, RefineUnrefined, ResolveAttrs, ToPath, ToType, Unrefined};
use crate::formatter::format_global_context;

#[derive(Clone)]
pub struct GlobalContext {
    pub config: Config,
    pub scope_register: ScopeResolver,
    // crate::asyn::query::Query: [T: [TransportRequest]]
    pub generics: GenericResolver,
    pub traits: TraitsResolver,
    pub custom: CustomResolver,
    // pub opaque: CustomResolver,
    pub imports: ImportResolver,
    pub refined_generics: HashMap<GenericConversion, HashSet<Option<Attribute>>>,
    pub refined_generic_constraints: HashMap<GenericBoundComposition, HashSet<Option<Attribute>>>
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
        Self { config, scope_register: ScopeResolver::default(), generics: Default::default(), traits: Default::default(), custom: Default::default(), imports: Default::default(), refined_generics: HashMap::default(), refined_generic_constraints: HashMap::default() }
    }
    pub fn fermented_mod_name(&self) -> &str {
        &self.config.mod_name
    }
    pub fn is_fermented_mod(&self, ident: &Ident) -> bool {
        format_ident!("{}", self.fermented_mod_name()).eq(ident)
    }


    pub fn resolve_trait_type(&self, from_type: &Type) -> Option<&ObjectConversion> {
        // println!("resolve_trait_type: {} ({:?})", from_type.to_token_stream(), from_type);
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
        // println!("current_scope: {}", current_scope);
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
                println!("maybe_trait_scope_pair NEW_OBJECT: {}", scope.fmt_short());
                let value = TypeCompositionConversion::Object(TypeComposition::new(scope.to_type(), Some(trait_composition.item.generics.clone()), Punctuated::new()));
                // println!("AttrsComposer: {} {} {}", trait_composition.item.ident, trait_scope, conversion);
                composition.implementors.push(value);
                (composition, trait_scope)
            })

    }

    fn maybe_obj_or_parent_scope_type(&self, self_scope: &ScopeChain, parent_chain: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        // println!("maybe_obj_or_parent_scope_type: {}", ty.to_token_stream());
        self.maybe_scope_object(ty, self_scope)
            .or_else(|| match parent_chain {
                ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                    self.maybe_scope_object(ty, parent_chain),
                _ => None,
            })
    }

    pub fn maybe_fn_type(&self, fn_scope: &ScopeChain, parent_scope: &ScopeChain, ty: &Type) -> Option<&ObjectConversion> {
        self.maybe_scope_object(ty, fn_scope)
            .or_else(|| match parent_scope {
                ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                    self.maybe_scope_object(ty, parent_scope),
                ScopeChain::Fn { parent_scope_chain, .. } =>
                    self.maybe_fn_type(parent_scope, parent_scope_chain, ty),
                ScopeChain::Trait { parent_scope_chain, .. } |
                ScopeChain::Object { parent_scope_chain, .. } |
                ScopeChain::Impl { parent_scope_chain, .. } =>
                    self.maybe_scope_object(ty, parent_scope)
                        .or_else(|| match &**parent_scope_chain {
                            ScopeChain::CrateRoot { .. } |
                            ScopeChain::Mod { ..} =>
                                self.maybe_scope_object(ty, &parent_scope_chain),
                            _ => None,
                        }),
        })
    }

    pub fn maybe_object(&self, ty: &Type, scope: &ScopeChain) -> Option<&ObjectConversion> {
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

    pub fn maybe_type_composition_conversion(&self, ty: &Type, scope: &ScopeChain) -> Option<&TypeCompositionConversion> {
        self.maybe_object(ty, scope)
            .and_then(|obj| obj.type_conversion())
    }

    pub fn maybe_item(&self, path: &Path) -> Option<&ScopeItemConversion> {
        // println!("maybe_item: {}", path.to_token_stream());
        if let Some(scope) = self.maybe_scope(path) {
            println!("[INFO] Found scope: {}", scope.fmt_short());
            let last_ident = &path.segments.last().unwrap().ident;
            let ty = last_ident.to_type();
            if let Some(ObjectConversion::Item(_, item)) = self.maybe_object(&ty, scope) {
                println!("[INFO] Found item in scope: {}", item);
                return Some(item);
            } else {
                //println!("[INFO] Scope found {} but no item: {}", scope.fmt_short(), path.to_token_stream());
            }
        } else {
            //println!("[INFO] No scope found [{}]", path.to_token_stream());
        }
        None
    }
    pub fn maybe_item_obj_first(&self, path: &Path) -> Option<&ScopeItemConversion> {
        // println!("maybe_item: {}", path.to_token_stream());
        if let Some(scope) = self.maybe_scope_obj_first(path) {
            println!("[INFO] Found obj scope: {}", scope.fmt_short());
            let last_ident = &path.segments.last().unwrap().ident;
            let ty = last_ident.to_type();
            if let Some(ObjectConversion::Item(_, item)) = self.maybe_object(&ty, scope) {
                println!("[INFO] Found item in scope: {}", item);
                return Some(item);
            } else {
                //println!("[INFO] Scope found {} but no item: {}", scope.fmt_short(), path.to_token_stream());
            }
        } else {
            //println!("[INFO] No scope found [{}]", path.to_token_stream());
        }
        None
    }


    pub fn actual_scope_for_type(&self, ty: &Type, current_scope: &ScopeChain) -> ScopeChain {
        // println!("actual_scope_for_type: {} in [{}]", format_token_stream(ty), current_scope);
        let p = parse_quote!(#ty);
        let scope = if let Some(st) = self.maybe_scope_object(ty, current_scope) {
            let self_ty = st.maybe_type().unwrap();
            let self_path: Path = self_ty.to_path();
            self.maybe_scope(&self_path).cloned()
        } else if let Some(import_path) = self.maybe_scope_import_path(current_scope, &p) {
            self.maybe_scope(import_path).cloned()
        } else {
            None
        };
        // println!("actual_scope_for_type: [{:?}]", scope);
        scope.unwrap_or(ScopeChain::crate_root(current_scope.crate_ident().clone(), vec![]))
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
            .find(|scope_chain| path.eq(scope_chain.self_path()))

    }

    pub fn maybe_import(&self, scope: &ScopeChain, path: &PathHolder) -> Option<&Path> {
        let result_opt = self.imports.maybe_import(scope, path);
        // println!("maybe_import: {path} in [{}] ---> {}", scope.self_path_holder_ref(), result_opt.to_token_stream());
        result_opt
    }

    pub fn maybe_import_scope_pair(&self, scope_path_last_segment: &PathSegment, scope_path_candidate: &Path) -> Option<(&ScopeChain, &Path)> {
        self.maybe_imports_scope(scope_path_candidate)
            .and_then(|reexport_scope| {
                let path: PathHolder = parse_quote!(#scope_path_last_segment);
                self.maybe_import(reexport_scope, &path).map(|import| (reexport_scope, import))
            })
    }



    // 1. Check whether the scope with this path exist
    // If exist then we know the type of the item
    // If not then:
    //  a) We check re-exports:
    //      - We pop last ident

    // pub(crate) fn maybe_known_import_composition(&self, ty_to_replace: &TypeComposition, scope: &ScopeChain) -> Option<TypeCompositionConversion> {
    //     // So we found a unknown path chunk in the scope
    //     // Here we trying to determine paths where the actual item located (if it's not evidently imported)
    //     // before marking this item as "possibly global"
    //     // So we build a stack with paths where it could be depending on type of the current scope
    //     // It should include now mods nested into parent mods (neighbour mods)
    //     // TODO: Support "extern crate", "super::", "self::" (paths kinda Self::AA may also require additional search routes)
    //     let path = ty_to_replace.pointer_less();
    //     // Local scopes are prioritized so we should check relative paths first
    //     // Then we should check crate-level scopes
    //     println!("\nmaybe_known_import_composition (check): {} in {}", ty_to_replace, scope.fmt_short());
    //     self.maybe_item(&path)
    //         .and_then(|scope_item| {
    //             println!("maybe_known_import_composition (found): {} in {}", path.to_token_stream(), scope_item);
    //             scope_item.update_scope_item(ty_to_replace.clone())
    //         })
    //         .or_else(|| {
    //             // There are 2 cases:
    //             // 1. it's from non-fermented crate
    //             // 2. it's not full scope:
    //             //  - It's reexported somewhere?
    //             //  - It's child scope?
    //             //  - It's neighbour scope?
    //             println!("maybe_known_import_composition (not found): {}", path.to_token_stream());
    //             self.traverse_scopes(ty_to_replace, scope)
    //         })
    // }
    // pub(crate) fn maybe_known_unknown_composition(&self, ty_to_replace: &TypeComposition, scope: &ScopeChain) -> Option<TypeCompositionConversion> {
    //     // So we found a unknown path chunk in the scope
    //     // Here we trying to determine paths where the actual item located (if it's not evidently imported)
    //     // before marking this item as "possibly global"
    //     // So we build a stack with paths where it could be depending on type of the current scope
    //     // It should include now mods nested into parent mods (neighbour mods)
    //     // TODO: Support "extern crate", "super::", "self::" (paths kinda Self::AA may also require additional search routes)
    //     let path = ty_to_replace.pointer_less();
    //     // Local scopes are prioritized so we should check relative paths first
    //     // Then we should check crate-level scopes
    //     println!("\nmaybe_known_unknown_composition (check): {} in {}", ty_to_replace, scope.fmt_short());
    //     self.maybe_item(&path)
    //         .and_then(|scope_item| {
    //             println!("maybe_known_unknown_composition (found): {} in {}", path.to_token_stream(), scope_item);
    //             scope_item.update_scope_item(ty_to_replace.clone())
    //         })
    //         .or_else(|| {
    //             // There are 2 cases:
    //             // 1. it's from non-fermented crate
    //             // 2. it's not full scope:
    //             //  - It's reexported somewhere?
    //             //  - It's child scope?
    //             //  - It's neighbour scope?
    //             println!("maybe_known_unknown_composition (not found): {}", path.to_token_stream());
    //             self.traverse_scopes(ty_to_replace, scope)
    //         })
    // }

    // pub(crate) fn maybe_item


    // We need to find full qualified paths for involved chunk and bind them to actual items
    pub(crate) fn maybe_refined_object(&self, scope: &ScopeChain, object: &ObjectConversion) -> Option<ObjectConversion> {
        // println!("maybe_refined_object --> {} \n\tin {}", object, scope.fmt_short());
        let mut refined = object.clone();
        let result = refined.refine_in_scope(scope, self)
            .then_some(refined);

        // println!("maybe_refined_object <-- {} \n\tin {}", result.as_ref().map_or("None".to_string(), |o| format!("{}", o)), scope.fmt_short());
        result
    }
    pub(crate) fn maybe_custom_conversion(&self, ty: &Type) -> Option<Type> {
        self.custom.maybe_conversion(ty)
    }

    fn should_skip_from_expanding(&self, object: &ObjectConversion) -> bool {
        let skip = match object.type_conversion() {
            Some(conversion) => {
                //println!("CHECK: {}", conversion);
                let nested_args = conversion.nested_arguments();
                let unknown_args = nested_args.iter().filter(|arg| {
                    match arg.object().type_conversion() {
                        Some(conversion) => {
                            let unknown_but_custom = self.maybe_custom_conversion(conversion.ty()).is_some();
                            //println!("CHECK NESTED: {}", conversion);
                            //conversion.is_unknown() && !unknown_but_custom
                            let skip = match conversion {
                                TypeCompositionConversion::Unknown(..) => true,
                                TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::NonPrimitiveOpaque(..)) => {
                                    //println!("CHECK NESTED NonPrimitiveOpaque: {}", ty.to_token_stream());
                                    true
                                },
                                TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::NonPrimitiveFermentable(TypeComposition { nested_arguments, .. })) => {
                                    //println!("CHECK NESTED NonPrimitiveFermentable: {}", ty.to_token_stream());
                                    let needed_types = nested_arguments.iter().filter_map(|n| match n.object().type_conversion() {
                                        Some(tyc) => {
                                            if tyc.is_unknown() && self.maybe_custom_conversion(tyc.ty()).is_none() {
                                                None
                                            } else {
                                                Some(tyc)
                                            }
                                        },
                                        None => None
                                    }).collect::<Vec<_>>();
                                    needed_types.len() != nested_arguments.len()
                                }
                                TypeCompositionConversion::Boxed(TypeComposition { nested_arguments, .. }) |
                                TypeCompositionConversion::Optional(TypeComposition { nested_arguments, .. }) => match nested_arguments.first() {
                                    Some(nested_arg) => match nested_arg.object().type_conversion() {
                                        Some(tyc) => {
                                            //println!("CHECK NESTED OPTIONAL/BOX: {}", tyc);
                                            tyc.is_unknown() && self.maybe_custom_conversion(tyc.ty()).is_none()
                                        },
                                        _ => true
                                    },
                                    _ => true
                                },
                                _ => false
                            };
                            skip && !unknown_but_custom
                        },
                        None => false
                    }

                }).collect::<Vec<_>>();
                !unknown_args.is_empty()
            }
            None => false
        };
        skip
    }
}

impl RefineMut for GlobalContext {
    type Refinement = ScopeRefinement;
    fn refine_with(&mut self, refined: Self::Refinement) {
        self.scope_register.refine_with(refined);
        let mut refined_generics = HashMap::<GenericConversion, HashSet<Option<Attribute>>>::new();
        let mut refined_generic_constraints = HashMap::<GenericBoundComposition, HashSet<Option<Attribute>>>::new();
        self.scope_register.inner.iter()
            .for_each(|(scope, type_chain)| {
                let scope_level_attrs = scope.resolve_attrs();
                //println!("REFINE GENERIC in {}", scope.fmt_short());
                type_chain.inner.iter().for_each(|(_conversion, object)| {
                    // println!("\t{} ---- {}", _conversion, object);
                    let object_attrs = object.resolve_attrs();
                    let mut all_attrs: HashSet<Option<Attribute>> = HashSet::from_iter(object_attrs);
                    all_attrs.extend(scope_level_attrs.clone());
                    if all_attrs.is_empty() {
                        all_attrs.insert(None);
                    }

                    if let Some(ty) = object.maybe_type() {
                        //println!("--- FIND GENERICS IN TYPE: {}", ty.to_token_stream());
                        ty.find_generics()
                            .iter()
                            .filter(|ty| self.maybe_custom_conversion(&ty.0).is_none())
                            .for_each(|_ty| {
                                // println!("CHECK")
                                let skip = self.should_skip_from_expanding(object);
                                println!("--- ADD GENERIC: OBJECT: (skip: {}) {} -- {}", skip, _ty.to_token_stream(), object);
                                if !skip {
                                    refined_generics
                                        .entry(GenericConversion::new(object.clone()))
                                        .or_insert_with(HashSet::new)
                                        .extend(all_attrs.clone());
                                }
                            });
                    }



                    if let Some(TypeCompositionConversion::Bounds(bounds)) = object.type_conversion() {
                        //println!("REFINE_SCOPE_BOUNDS: {}", bounds);
                        // refined_generic_constraints
                        //     .entry(bounds.clone())
                        //     .or_insert_with(HashSet::new)
                        //     .extend(all_attrs.clone());
                        bounds.find_generic_constraints()
                            .iter()
                            .for_each(|_ty| {
                                //println!("--- ADD GENERIC: BOUNDS: {}", bounds);
                                refined_generic_constraints
                                    .entry(bounds.clone())
                                    .or_insert_with(HashSet::new)
                                    .extend(all_attrs.clone());
                            });
                    }
                })
            });
        self.refined_generics = refined_generics;
        self.refined_generic_constraints = refined_generic_constraints;

        self.generics.inner.iter_mut()
            .for_each(|(scope, generic_chain)| {
                generic_chain.values_mut()
                    .for_each(|bounds| {
                        //println!("REFINE GENERIC BOUNDS: {}", format_path_vec(bounds));
                        bounds.iter_mut().for_each(|bound| {
                            if let Some(ty) = self.scope_register.scope_type_for_path(bound, scope) {
                                match ty {
                                    Type::Path(TypePath { path, .. }) => {
                                        //println!("REFINE BOUND: {}", path.to_token_stream());
                                        *bound = path;
                                    },
                                    _ => {
                                        //println!("NON REFINE BOUND: {}", ty.to_token_stream());
                                    }
                                }
                            }
                        });

                    });
        })
    }
}

impl Unrefined for GlobalContext {
    type Unrefinement = ScopeRefinement;
    fn unrefined(&self) -> Self::Unrefinement {
        let mut scope_updates = vec![];
        self.scope_register.inner.iter()
            .for_each(|(scope, type_chain)| {
                let scope_types_to_refine = type_chain.inner.iter()
                    .filter_map(|(holder, object)|
                        self.maybe_refined_object(scope, object)
                            .map(|object_to_refine| (holder.clone(), object_to_refine)))
                    .collect::<HashMap<_, _>>();
                if !scope_types_to_refine.is_empty() {
                    scope_updates.push((scope.clone(), scope_types_to_refine));
                }
            });
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
    pub fn maybe_scope_obj_first(&self, path: &Path) -> Option<&ScopeChain> {
        let x = self.scope_register.resolve_obj_first(path);
        //println!("maybe_scope: {} --> {}", path.to_token_stream(), x.map(ScopeChain::self_path_holder_ref).to_token_stream());
        x
    }

    // traverses import scope for re-exports (doesn't include re-exports in neighbour scopes)

}


