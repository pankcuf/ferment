use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{Attribute, parse_quote, Path, PathSegment, Type, TypePath};
use syn::punctuated::Punctuated;
use crate::Config;
use crate::ast::PathHolder;
use crate::composable::{GenericBoundsModel, GenericConversion, TraitModelPart1, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{CustomResolver, GenericResolver, ImportResolver, ScopeChain, ScopeRefinement, ScopeResolver, ScopeSearch, ScopeSearchKey, TraitsResolver, TypeChain};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, GenericCollector, GenericConstraintCollector, RefineInScope, RefineMut, RefineUnrefined, ResolveAttrs, ToPath, ToType, Unrefined};
use crate::formatter::format_global_context;

#[derive(Clone)]
pub struct GlobalContext {
    pub config: Config,
    pub scope_register: ScopeResolver,
    // crate::asyn::query::Query: [T: [TransportRequest]]
    pub generics: GenericResolver,
    pub traits: TraitsResolver,
    pub custom: CustomResolver,
    pub imports: ImportResolver,
    pub refined_generics: HashMap<GenericConversion, HashSet<Option<Attribute>>>,
    pub refined_generic_constraints: HashMap<GenericBoundsModel, HashSet<Option<Attribute>>>
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


    pub fn resolve_trait_type<'a>(&'a self, from_type: &'a Type) -> Option<&'a ObjectKind> {
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
        let mut maybe_trait: Option<&ObjectKind>  = None;
        while i < current_scope.len() && maybe_trait.is_none() {
            let (root, head) = current_scope.split_and_join_self(i);
            let ty = head.to_type();
            let root_scope = self.maybe_scope_ref(&root.0);
            if let Some(scope) = root_scope {
                //maybe_trait = self.maybe_local_scope_object_ref_by_key(&ty, scope);
                maybe_trait = ScopeSearchKey::maybe_from(ty)
                    .map(|key| ScopeSearch::KeyInScope(key, scope))
                    .and_then(move |predicate| self.scope_register.maybe_object_ref_by_predicate(predicate));

            }
            //maybe_trait = self.maybe_scope_type(&ty, &root);
            if i > 0 {
                match maybe_trait {
                    Some(ObjectKind::Item(TypeModelKind::Trait(_trait_ty, decomposition, _super_bounds), _)) |
                    Some(ObjectKind::Type(TypeModelKind::Trait(_trait_ty, decomposition, _super_bounds))) => {
                        let ident = &head.0.segments.last().unwrap().ident;
                        // println!("FFI (has decomposition) for: {}: {}", format_token_stream(ident), trait_ty);
                        if let Some(trait_type) = decomposition.types.get(ident) {
                            // println!("FFI (first bound) {:?}", trait_type);
                            if let Some(first_bound) = trait_type.trait_bounds.first() {
                                // println!("FFI (first bound) {}", format_token_stream(&first_bound.path));
                                let tt_type = first_bound.to_type();
                                if let Some(scope) = root_scope {
                                    maybe_trait = ScopeSearchKey::maybe_from(tt_type)
                                        .map(|key| ScopeSearch::KeyInScope(key, scope))
                                        .and_then(move |predicate| self.scope_register.maybe_object_ref_by_predicate(predicate));
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

    pub fn maybe_trait_scope_pair(&self, link: &Path, scope: &ScopeChain) -> Option<(TraitModelPart1, ScopeChain)> {
        let parent_scope = scope.parent_scope().unwrap();
        let trait_ty = link.to_type();
        let trait_scope = self.actual_scope_for_type(&trait_ty, parent_scope).unwrap();
        // println!("find_item_trait_scope_pair: {} -> {}", link.to_token_stream(), trait_scope);
        let ident = link.get_ident().unwrap();
        self.traits
            .item_trait_with_ident_for(ident, trait_scope)
            .map(|trait_model| {
                let mut model = trait_model.clone();
                // TODO: move to full and replace nested_arguments
                println!("maybe_trait_scope_pair NEW_OBJECT: {}", scope.fmt_short());
                let value = TypeModelKind::Object(TypeModel::new(scope.to_type(), Some(trait_model.item.generics.clone()), Punctuated::new()));
                // println!("AttrsComposer: {} {} {}", trait_composition.item.ident, trait_scope, conversion);
                model.implementors.push(value);
                (model, trait_scope.clone())
            })

    }

    fn maybe_obj_or_parent_object_ref_by_tree_key<'a>(&'a self, self_scope: &'a ScopeChain, parent_chain: &'a ScopeChain, ty: &'a Type) -> Option<&'a ObjectKind> {
        // println!("maybe_obj_or_parent_scope_type: {}", ty.to_token_stream());
        self.maybe_local_scope_object_ref_by_key(ty, self_scope)
            .or_else(move || match parent_chain {
                ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                    self.maybe_local_scope_object_ref_by_key(ty, parent_chain),
                _ => None,
            })
    }
    fn maybe_obj_or_parent_object_ref_by_tree_search_key<'a>(&'a self, self_scope: &'a ScopeChain, parent_chain: &'a ScopeChain, search_key: ScopeSearchKey<'a>) -> Option<&'a ObjectKind> {
        // println!("maybe_obj_or_parent_scope_type: {}", ty.to_token_stream());
        self.maybe_object_ref_by_search_key_in_scope(search_key.clone(), self_scope)
            .or_else(move || match parent_chain {
                ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                    self.maybe_object_ref_by_search_key_in_scope(search_key, parent_chain),
                _ => None,
            })
    }

    fn maybe_fn_object_ref_by_tree_key<'a>(&'a self, fn_scope: &'a ScopeChain, parent_scope: &'a ScopeChain, ty: &'a Type) -> Option<&'a ObjectKind> {
        self.maybe_local_scope_object_ref_by_key(ty, fn_scope)
            .or_else(move || match parent_scope {
                ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                    self.maybe_local_scope_object_ref_by_key(ty, parent_scope),
                ScopeChain::Fn { parent_scope_chain, .. } =>
                    self.maybe_fn_object_ref_by_tree_key(parent_scope, parent_scope_chain, ty),
                ScopeChain::Trait { parent_scope_chain, .. } |
                ScopeChain::Object { parent_scope_chain, .. } |
                ScopeChain::Impl { parent_scope_chain, .. } =>
                    self.maybe_local_scope_object_ref_by_key(ty, parent_scope)
                        .or_else(|| match &**parent_scope_chain {
                            ScopeChain::CrateRoot { .. } |
                            ScopeChain::Mod { ..} =>
                                self.maybe_local_scope_object_ref_by_key(ty, &parent_scope_chain),
                            _ => None,
                        }),
        })
    }
    fn maybe_fn_object_ref_by_tree_search_key<'a>(&'a self, fn_scope: &'a ScopeChain, parent_scope: &'a ScopeChain, search_key: ScopeSearchKey<'a>) -> Option<&'a ObjectKind> {
        self.maybe_object_ref_by_search_key_in_scope(search_key.clone(), fn_scope)
            .or_else(move || match parent_scope {
                ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                    self.maybe_object_ref_by_search_key_in_scope(search_key, parent_scope),
                ScopeChain::Fn { parent_scope_chain, .. } =>
                    self.maybe_fn_object_ref_by_tree_search_key(parent_scope, parent_scope_chain, search_key),
                ScopeChain::Trait { parent_scope_chain, .. } |
                ScopeChain::Object { parent_scope_chain, .. } |
                ScopeChain::Impl { parent_scope_chain, .. } =>
                    self.maybe_object_ref_by_search_key_in_scope(search_key.clone(), parent_scope)
                        .or_else(|| match &**parent_scope_chain {
                            ScopeChain::CrateRoot { .. } |
                            ScopeChain::Mod { ..} =>
                                self.maybe_object_ref_by_search_key_in_scope(search_key, &parent_scope_chain),
                            _ => None,
                        }),
        })
    }

    pub fn maybe_object_ref_by_tree_search_key<'a>(&'a self, search_key: ScopeSearchKey<'a>, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        match scope {
            ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                self.maybe_object_ref_by_search_key_in_scope(search_key, scope),
            ScopeChain::Fn { parent_scope_chain, .. } =>
                self.maybe_fn_object_ref_by_tree_search_key(scope, parent_scope_chain, search_key),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } =>
                self.maybe_obj_or_parent_object_ref_by_tree_search_key(scope, parent_scope_chain, search_key),
        }

    }
    pub fn maybe_object_ref_by_tree_key<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
         match scope {
             ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                 self.maybe_local_scope_object_ref_by_key(ty, &scope),
             ScopeChain::Fn { parent_scope_chain, .. } =>
                 self.maybe_fn_object_ref_by_tree_key(scope, parent_scope_chain, ty),
             ScopeChain::Trait { parent_scope_chain, .. } |
             ScopeChain::Object { parent_scope_chain, .. } |
             ScopeChain::Impl { parent_scope_chain, .. } =>
                 self.maybe_obj_or_parent_object_ref_by_tree_key(scope, parent_scope_chain, ty),
         }
    }

    pub fn maybe_type_model_kind_ref_by_key<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a TypeModelKind> {
        self.maybe_object_ref_by_tree_key(ty, scope)
            .and_then(ObjectKind::maybe_type_model_kind_ref)
    }

    pub fn maybe_scope_item_ref<'a>(&'a self, path: &'a Path) -> Option<&'a ScopeItemKind> {
        // println!("maybe_item: {}", path.to_token_stream());
        if let Some(scope) = self.maybe_scope_ref(path) {
            //println!("[INFO] Found scope: {}", scope.fmt_short());
            let last_ident = &path.segments.last().unwrap().ident;
            let ty = last_ident.to_type();

            if let Some(ObjectKind::Item(_, item)) = self.maybe_object_ref_by_search_key_in_scope(ScopeSearchKey::Type(ty, None), scope) {
                //println!("[INFO] Found item in scope: {}", item);
                return Some(item);
            } else {
                //println!("[INFO] Scope found {} but no item: {}", scope.fmt_short(), path.to_token_stream());
            }
        } else {
            //println!("[INFO] No scope found [{}]", path.to_token_stream());
        }
        None
    }
    pub fn maybe_scope_item_ref_obj_first(&self, path: &Path) -> Option<&ScopeItemKind> {
        // println!("maybe_item: {}", path.to_token_stream());
        if let Some(scope) = self.maybe_scope_ref_obj_first(path) {
            //println!("[INFO] Found obj scope: {}", scope.fmt_short());
            let last_ident = &path.segments.last().unwrap().ident;
            let ty = last_ident.to_type();
            if let Some(search_key) = ScopeSearchKey::maybe_from(ty) {
                if let Some(ObjectKind::Item(_, item)) = self.maybe_object_ref_by_tree_search_key(search_key, scope) {
                    //println!("[INFO] Found item in scope: {}", item);
                    return Some(item);
                }
            }
        } else {
            //println!("[INFO] No scope found [{}]", path.to_token_stream());
        }
        None
    }


    pub fn actual_scope_for_type(&self, ty: &Type, current_scope: &ScopeChain) -> Option<&ScopeChain> {
        let p = parse_quote!(#ty);
        let search_key = ScopeSearchKey::maybe_from_ref(ty).unwrap();
        let scope = if let Some(st) = self.maybe_object_ref_by_search_key_in_scope(search_key, current_scope) {
            let self_ty = st.maybe_type().unwrap();
            let self_path: Path = self_ty.to_path();
            self.maybe_scope_ref(&self_path)
        } else if let Some(import_path) = self.maybe_scope_import_path(current_scope, &p) {
            self.maybe_scope_ref(import_path)
        } else {
            None
        };
        scope
        // scope.unwrap_or(ScopeChain::crate_root(current_scope.crate_ident().clone(), vec![]))
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

    pub fn maybe_import_path_ref(&self, scope: &ScopeChain, path: &PathHolder) -> Option<&Path> {
        let result_opt = self.imports.maybe_import(scope, path);
        // println!("maybe_import: {path} in [{}] ---> {}", scope.self_path_holder_ref(), result_opt.to_token_stream());
        result_opt
    }

    pub fn maybe_import_scope_pair_ref(&self, scope_path_last_segment: &PathSegment, scope_path_candidate: &Path) -> Option<(&ScopeChain, &Path)> {
        self.maybe_imports_scope(scope_path_candidate)
            .and_then(|reexport_scope| {
                let path: PathHolder = parse_quote!(#scope_path_last_segment);
                self.maybe_import_path_ref(reexport_scope, &path).map(|import| (reexport_scope, import))
            })
    }

    // We need to find full qualified paths for involved chunk and bind them to actual items
    pub(crate) fn maybe_refined_object(&self, scope: &ScopeChain, object: &ObjectKind) -> Option<ObjectKind> {
        // println!("maybe_refined_object --> {} \n\tin {}", object, scope.fmt_short());
        let mut refined = object.clone();
        let result = refined.refine_in_scope(scope, self)
            .then_some(refined);

        // println!("maybe_refined_object <-- {} \n\tin {}", result.as_ref().map_or("None".to_string(), |o| format!("{}", o)), scope.fmt_short());
        result
    }
    pub(crate) fn maybe_custom_type(&self, ty: &Type) -> Option<Type> {
        self.custom.maybe_type(ty)
    }

    fn num_of_nested_exposable_types_for_generic<'a>(&'a self, args: &'a CommaPunctuatedNestedArguments) -> usize {
        args.iter().filter_map(|arg| {
            let ttt = match arg.object().maybe_type_model_kind_ref() {
                Some(tyc) => match tyc {
                    TypeModelKind::Unknown(..) |
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) =>
                        self.maybe_custom_type(tyc.as_type())
                            .is_some()
                            .then_some(tyc),
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(
                            DictFermentableModelKind::SmartPointer(
                                SmartPointerModelKind::Arc(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Box(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Rc(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Mutex(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::RwLock(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::RefCell(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Pin(TypeModel { nested_arguments, .. })
                            ) |
                            DictFermentableModelKind::Group(
                                GroupModelKind::BTreeSet(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::HashSet(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::Map(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::Result(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::Vec(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::IndexMap(TypeModel { nested_arguments, .. })
                            ) |
                            DictFermentableModelKind::Other(TypeModel { nested_arguments, .. }))) |
                    TypeModelKind::Optional(TypeModel { nested_arguments, .. }) => {
                        let is_custom = self.maybe_custom_type(tyc.as_type());
                        let num_of_fermentable = self.num_of_nested_exposable_types_for_generic(nested_arguments);
                        let all_of_them_are_non_fermentable = num_of_fermentable == 0 && nested_arguments.len() != 0;
                        //println!("TYC: ({}, {}, {}) ---- {}", all_of_them_are_non_fermentable, is_custom.is_some(), nested_arguments.is_empty(), tyc);
                        (!all_of_them_are_non_fermentable || is_custom.is_some() || nested_arguments.is_empty())
                            .then_some(tyc)
                    },
                        // (self.num_of_nested_fermentable_types_for_generic(nested_arguments) == nested_arguments.len() || self.maybe_custom_conversion(tyc.ty()).is_some())
                        //     .then_some(tyc),
                    tyc => Some(tyc)
                }
                _ => None
            };
            //println!("arg: {} ---- {}", arg, ttt.map_or("None".to_string(), |f| format!("{}", f)));
            ttt
        }).collect::<Vec<_>>().len()
    }

    fn should_skip_from_expanding(&self, object: &ObjectKind) -> bool {
        let skip = match object.maybe_type_model_kind_ref() {
            Some(conversion) => {
                let maybe_custom = self.maybe_custom_type(conversion.as_type());
                let nested_arguments = conversion.nested_arguments_ref();
                let num_of_fermentable = self.num_of_nested_exposable_types_for_generic(nested_arguments);
                let all_of_them_are_non_fermentable = num_of_fermentable == 0 && nested_arguments.len() != 0;
                let skip = all_of_them_are_non_fermentable && maybe_custom.is_none();
                // let skip = self.num_of_nested_fermentable_types_for_generic(nested_args) == 0;
                //println!("SKIP ({}): {}", skip, conversion);
                skip
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
        let mut refined_generic_constraints = HashMap::<GenericBoundsModel, HashSet<Option<Attribute>>>::new();
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
                            .filter(|ty| self.maybe_custom_type(&ty.0).is_none())
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



                    if let Some(TypeModelKind::Bounds(bounds)) = object.maybe_type_model_kind_ref() {
                        println!("REFINE_SCOPE_BOUNDS: {}", bounds);
                        // if bounds.bounds.len() > 1 {
                        //     refined_generic_constraints
                        //         .entry(bounds.clone())
                        //         .or_insert_with(HashSet::new)
                        //         .extend(all_attrs.clone());
                        // }

                        // refined_generic_constraints
                        //     .entry(bounds.clone())
                        //     .or_insert_with(HashSet::new)
                        //     .extend(all_attrs.clone());
                        bounds.find_generic_constraints()
                            .iter()
                            .for_each(|_ty| {
                                println!("--- ADD GENERIC: BOUNDS: {}", bounds);
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
                            if let Some(ty) = self.scope_register.scope_key_type_for_path(bound, scope) {
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
        //println!("------- GlobalContext::unrefined ----------");
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
        //println!("------- GlobalContext::unrefined ---------- {}", format_scope_refinement(&scope_updates));
        scope_updates
    }
}

impl RefineUnrefined for GlobalContext {}

/// Scope
impl GlobalContext {
    pub fn scope_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        self.scope_register.type_chain_mut(scope)
    }
    pub fn maybe_object_ref_by_predicate<'a>(&'a self, predicate: ScopeSearch<'a>) -> Option<&'a ObjectKind> {
        self.scope_register.maybe_object_ref_by_predicate(predicate)
    }
    pub fn maybe_scope_ref(&self, path: &Path) -> Option<&ScopeChain> {
        self.scope_register.maybe_scope(path)
    }
    pub fn maybe_scope_ref_obj_first(&self, path: &Path) -> Option<&ScopeChain> {
        self.scope_register.maybe_first_obj_scope(path)
    }
    pub fn maybe_object_ref_by_value<'a>(&'a self, ty: &'a Type) -> Option<&'a ObjectKind> {
        ScopeSearchKey::maybe_from_ref(ty)
            .and_then(|search_key| self.maybe_object_ref_by_search_value(search_key))
    }
    fn maybe_local_scope_object_ref_by_key<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        ScopeSearchKey::maybe_from_ref(ty)
            .and_then(|search_key| self.maybe_object_ref_by_search_key_in_scope(search_key, scope))
    }
    fn maybe_object_ref_by_search_key_in_scope<'a>(&'a self, search_key: ScopeSearchKey<'a>, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        self.maybe_object_ref_by_predicate(ScopeSearch::KeyInScope(search_key, scope))
    }
    // fn maybe_object_ref_by_search_value_in_scope<'a>(&'a self, search_key: ScopeSearchKey<'a>, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
    //     self.maybe_object_ref_by_predicate(ScopeSearch::ValueInScope(search_key, scope))
    // }
    fn maybe_object_ref_by_search_value<'a>(&'a self, search_key: ScopeSearchKey<'a>) -> Option<&'a ObjectKind> {
        self.maybe_object_ref_by_predicate(ScopeSearch::Value(search_key))
    }
}


