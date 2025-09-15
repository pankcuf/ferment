use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use indexmap::IndexMap;
use proc_macro2::Ident;
use syn::{parse_quote, Attribute, Item, ItemTrait, Path, PathSegment, Type};
use crate::Config;
use crate::composable::{TraitModelPart1, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{CustomResolver, GenericResolver, ImportResolver, ScopeChain, ScopeResolver, ScopeSearchKey, TraitsResolver, TypeChain};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, MixinKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, GenericBoundKey, RefineInScope, Split, ToPath, ToType, CrateBased, RefineMut};
use crate::formatter::{format_global_context, scope_resolved_imports_dict};

#[derive(Clone)]
pub struct GlobalContext {
    pub config: Config,
    pub scope_register: ScopeResolver,
    pub generics: GenericResolver,
    pub traits: TraitsResolver,
    pub custom: CustomResolver,
    pub imports: ImportResolver,
    pub refined_mixins: IndexMap<MixinKind, HashSet<Option<Attribute>>>
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
        Self { config, scope_register: ScopeResolver::default(), generics: Default::default(), traits: Default::default(), custom: Default::default(), imports: Default::default(), refined_mixins: IndexMap::default(), }
    }

    pub fn refine(&mut self) {
        // Refine import paths after scope creation but before type refinement
        self.imports.refine_imports(&self.scope_register);
        // Build fully qualified resolved imports without circular dependencies
        self.imports.build_fully_qualified_imports(&self.scope_register);
        println!("################# REFINED IMPORTS ###############");
        println!("{}", scope_resolved_imports_dict(&self.imports.resolved_imports).join("\n"));
        println!("################# ############### ###############");
        // Collects scope items needed to refine
        let mut scope_updates = vec![];
        self.scope_register.inner.iter()
            .for_each(|(scope, type_chain)| {
                let scope_types_to_refine = type_chain.inner.iter()
                    .filter_map(|(ty, object)|
                        self.maybe_refined_object(scope, object)
                            .map(|object_to_refine| (ty.to_owned(), object_to_refine)))
                    .collect::<HashMap<_, _>>();
                if !scope_types_to_refine.is_empty() {
                    scope_updates.push((scope.to_owned(), scope_types_to_refine));
                }
            });
        // Actual scope items refinement
        self.refine_with(scope_updates);
    }

    pub fn fermented_mod_name(&self) -> &str {
        &self.config.mod_name
    }
    pub fn is_fermented_mod(&self, ident: &Ident) -> bool {
        ident.eq(self.fermented_mod_name())
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
        let current_scope: Path = parse_quote!(#from_type);
        // println!("current_scope: {}", current_scope);
        let mut i = 0;
        let mut maybe_trait: Option<&ObjectKind>  = None;
        while i < current_scope.segments.len() && maybe_trait.is_none() {
            let (root, mut head) = current_scope.split(i);
            head = if head.segments.is_empty() {
                parse_quote!(Self)
            } else {
                parse_quote!(Self::#head)
            };

            let ty = head.to_type();
            let root_scope = self.maybe_scope_ref(&root);
            if let Some(scope) = root_scope {
                //maybe_trait = self.maybe_local_scope_object_ref_by_key(&ty, scope);
                maybe_trait = ScopeSearchKey::maybe_from(ty)
                    .and_then(|key| self.scope_register.maybe_object_ref_by_key_in_scope(key, scope));
            }
            //maybe_trait = self.maybe_scope_type(&ty, &root);
            if i > 0 {
                match maybe_trait {
                    Some(ObjectKind::Item(TypeModelKind::Trait(model), _)) |
                    Some(ObjectKind::Type(TypeModelKind::Trait(model))) => {
                        let ident = &head.segments.last()?.ident;
                        // println!("FFI (has decomposition) for: {}: {}", format_token_stream(ident), trait_ty);
                        if let Some(trait_type) = model.decomposition.types.get(ident) {
                            // println!("FFI (first bound) {:?}", trait_type);
                            if let Some(first_bound) = trait_type.trait_bounds.first() {
                                // println!("FFI (first bound) {}", format_token_stream(&first_bound.path));
                                let tt_type = first_bound.to_type();
                                if let Some(scope) = root_scope {
                                    maybe_trait = ScopeSearchKey::maybe_from(tt_type)
                                        .and_then(|key| self.scope_register.maybe_object_ref_by_key_in_scope(key, scope));
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

    pub fn item_trait_with_ident_for(&self, ident: &Ident, scope: &ScopeChain) -> Option<&TraitModelPart1> {
        self.traits
            .item_trait_with_ident_for(ident, scope)
    }

    pub fn maybe_trait_scope_pair(&self, link: &Type, scope: &ScopeChain) -> Option<(TraitModelPart1, ScopeChain)> {
        let parent_scope = scope.parent_scope()?;
        let trait_ty = link.to_type();
        let trait_scope = self.actual_scope_for_type(&trait_ty, parent_scope)?;
        let trait_path = link.to_path();
        let ident = trait_path.get_ident()?;
        self.item_trait_with_ident_for(ident, trait_scope)
            .map(|trait_model| {
                let mut model = trait_model.clone();
                // TODO: move to full and replace nested_arguments
                let value = TypeModelKind::Object(TypeModel::new_generic_scope_non_nested(scope, &trait_model.item.generics));
                model.implementors.push(value);
                (model, trait_scope.clone())
            })

    }

    fn maybe_obj_or_parent_object_ref_by_tree_key<'a>(&'a self, self_scope: &'a ScopeChain, parent_chain: &'a ScopeChain, ty: &'a Type) -> Option<&'a ObjectKind> {
        self.maybe_local_scope_object_ref_by_key(ty, self_scope)
            .or_else(move || match parent_chain {
                ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                    self.maybe_local_scope_object_ref_by_key(ty, parent_chain),
                _ => None,
            })
    }
    fn maybe_obj_or_parent_object_ref_by_tree_search_key<'a>(&'a self, self_scope: &'a ScopeChain, parent_chain: &'a ScopeChain, search_key: ScopeSearchKey) -> Option<&'a ObjectKind> {
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
                ScopeChain::Fn { parent, .. } =>
                    self.maybe_fn_object_ref_by_tree_key(parent_scope, parent, ty),
                ScopeChain::Trait { parent, .. } |
                ScopeChain::Object { parent, .. } |
                ScopeChain::Impl { parent, .. } =>
                    self.maybe_local_scope_object_ref_by_key(ty, parent_scope)
                        .or_else(|| match &**parent {
                            ScopeChain::CrateRoot { .. } |
                            ScopeChain::Mod { .. } =>
                                self.maybe_local_scope_object_ref_by_key(ty, parent),
                            _ => None,
                        }),
        })
    }
    fn maybe_fn_object_ref_by_tree_search_key<'a>(&'a self, fn_scope: &'a ScopeChain, parent_scope: &'a ScopeChain, search_key: ScopeSearchKey) -> Option<&'a ObjectKind> {
        self.maybe_object_ref_by_search_key_in_scope(search_key.clone(), fn_scope)
            .or_else(move || match parent_scope {
                ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                    self.maybe_object_ref_by_search_key_in_scope(search_key, parent_scope),
                ScopeChain::Fn { parent, .. } =>
                    self.maybe_fn_object_ref_by_tree_search_key(parent_scope, parent, search_key),
                ScopeChain::Trait { parent, .. } |
                ScopeChain::Object { parent, .. } |
                ScopeChain::Impl { parent, .. } =>
                    self.maybe_object_ref_by_search_key_in_scope(search_key.clone(), parent_scope)
                        .or_else(|| match &**parent {
                            ScopeChain::CrateRoot { .. } |
                            ScopeChain::Mod { ..} =>
                                self.maybe_object_ref_by_search_key_in_scope(search_key, parent),
                            _ => None,
                        }),
        })
    }

    pub fn maybe_object_ref_by_tree_search_key<'a>(&'a self, search_key: ScopeSearchKey, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        match scope {
            ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                self.maybe_object_ref_by_search_key_in_scope(search_key, scope),
            ScopeChain::Fn { parent, .. } =>
                self.maybe_fn_object_ref_by_tree_search_key(scope, parent, search_key),
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Object { parent, .. } |
            ScopeChain::Impl { parent, .. } =>
                self.maybe_obj_or_parent_object_ref_by_tree_search_key(scope, parent, search_key),
        }

    }
    pub fn maybe_object_ref_by_tree_key<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
         match scope {
             ScopeChain::Mod { .. } | ScopeChain::CrateRoot { .. } =>
                 self.maybe_local_scope_object_ref_by_key(ty, scope),
             ScopeChain::Fn { parent, .. } =>
                 self.maybe_fn_object_ref_by_tree_key(scope, parent, ty),
             ScopeChain::Trait { parent, .. } |
             ScopeChain::Object { parent, .. } |
             ScopeChain::Impl { parent, .. } =>
                 self.maybe_obj_or_parent_object_ref_by_tree_key(scope, parent, ty),
         }
    }

    pub fn maybe_type_model_kind_ref_by_key<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a TypeModelKind> {
        self.maybe_object_ref_by_tree_key(ty, scope)
            .and_then(ObjectKind::maybe_type_model_kind_ref)
    }

    pub fn maybe_scope_item_ref<'a>(&'a self, path: &'a Path) -> Option<&'a ScopeItemKind> {
        if let Some(scope) = self.maybe_scope_ref(path) {
            let last_ident = &path.segments.last()?.ident;
            let ty = last_ident.to_type();
            if let Some(ObjectKind::Item(_, item)) = self.maybe_object_ref_by_search_key_in_scope(ScopeSearchKey::Type(ty, None), scope) {
                return Some(item);
            }
        }
        None
    }
    pub fn maybe_scope_item_ref_obj_first(&self, path: &Path) -> Option<&ScopeItemKind> {
        if let Some(scope) = self.maybe_scope_ref_obj_first(path) {
            let last_ident = &path.segments.last()?.ident;
            let ty = last_ident.to_type();
            if let Some(search_key) = ScopeSearchKey::maybe_from(ty) {
                if let Some(ObjectKind::Item(_, item)) = self.maybe_object_ref_by_tree_search_key(search_key, scope) {
                    return Some(item);
                }
            }
        }
        None
    }

    pub fn maybe_item_trait(&self, trait_path: &Path) -> Option<ItemTrait> {
        match self.maybe_scope_item_ref_obj_first(trait_path) {
            Some(ScopeItemKind::Item(Item::Trait(item_trait), ..)) => Some(item_trait.clone()),
            _ => None
        }
    }

    pub fn actual_scope_for_type(&self, ty: &Type, current_scope: &ScopeChain) -> Option<&ScopeChain> {
        let p = GenericBoundKey::Path(ty.to_path());
        let search_key = ScopeSearchKey::maybe_from_ref(ty)?;
        if let Some(st) = self.maybe_object_ref_by_search_key_in_scope(search_key, current_scope) {
            let self_ty = st.maybe_type()?;
            let self_path: Path = self_ty.to_path();
            self.maybe_scope_ref(&self_path)
        } else if let Some(import_path) = self.maybe_scope_import_path_ref(current_scope, &p) {
            self.maybe_scope_ref(import_path)
        } else {
            None
        }
    }
}



/// Imports
impl GlobalContext {
    pub fn maybe_scope_import_path_ref(&self, scope: &ScopeChain, chunk: &GenericBoundKey) -> Option<&Path> {
        self.imports.maybe_path(scope, chunk)
    }

    pub fn maybe_imports_scope_ref(&self, path: &Path) -> Option<&ScopeChain> {
        self.imports
            .inner
            .keys()
            .find(|scope_chain| {
                // Normalize `crate::...` to the actual crate ident for comparison
                let normalized = path.clone().crate_named(&scope_chain.crate_ident_as_path());
                normalized.eq(scope_chain.self_path_ref())
            })

    }

    pub fn maybe_globs_scope_ref(&self, path: &Path) -> Option<&ScopeChain> {
        self.imports
            .globs
            .keys()
            .find(|scope_chain| {
                // Normalize `crate::...` to the actual crate ident for comparison
                let normalized = path.clone().crate_named(&scope_chain.crate_ident_as_path());
                normalized.eq(scope_chain.self_path_ref())
            })
    }

    pub fn maybe_import_path_ref(&self, scope: &ScopeChain, path: &GenericBoundKey) -> Option<&Path> {
        self.imports.maybe_import(scope, path)
    }

    pub fn maybe_import_scope_pair_ref(&self, scope_path_last_segment: &PathSegment, scope_path_candidate: &Path) -> Option<(&ScopeChain, &Path)> {
        self.maybe_imports_scope_ref(scope_path_candidate)
            .and_then(|reexport_scope| {
                let path = GenericBoundKey::ident(&scope_path_last_segment.ident);
                self.maybe_import_path_ref(reexport_scope, &path).map(|import| (reexport_scope, import))
            })
    }

    pub fn maybe_import_glob_path_owned(&self, _scope_path_last_segment: &PathSegment, scope_path_candidate: &Path) -> Option<(&ScopeChain, Vec<Path>)> {
        self.maybe_globs_scope_ref(scope_path_candidate)
            .and_then(|reexport_scope| {
                self.imports
                    .maybe_scope_globs(reexport_scope)
                    .map(|bases| (reexport_scope, bases.clone()))
            })
    }

    // We need to find full qualified paths for involved chunk and bind them to actual items
    pub(crate) fn maybe_refined_object(&self, scope: &ScopeChain, object: &ObjectKind) -> Option<ObjectKind> {
        let mut refined = object.clone();
        refined.refine_in_scope(scope, self)
            .then_some(refined)
    }
    pub(crate) fn maybe_custom_type(&self, ty: &Type) -> Option<Type> {
        self.custom.maybe_type(ty)
    }

    fn num_of_nested_exposable_types_for_generic<'a>(&'a self, args: &'a CommaPunctuatedNestedArguments) -> usize {
        args.iter().filter_map(|arg| {
            match arg.object().maybe_type_model_kind_ref() {
                Some(tyc) => match tyc {
                    TypeModelKind::Unknown(..) |
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) =>
                        self.maybe_custom_type(tyc.as_type())
                            .is_some()
                            .then_some(tyc),
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(
                            DictFermentableModelKind::Cow(TypeModel { nested_arguments, .. }) |
                            DictFermentableModelKind::SmartPointer(
                                SmartPointerModelKind::Arc(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Box(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Rc(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Mutex(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::OnceLock(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::RwLock(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Cell(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::RefCell(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::UnsafeCell(TypeModel { nested_arguments, .. }) |
                                SmartPointerModelKind::Pin(TypeModel { nested_arguments, .. })
                            ) |
                            DictFermentableModelKind::Group(
                                GroupModelKind::BTreeSet(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::HashSet(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::Map(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::Result(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::Vec(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::IndexMap(TypeModel { nested_arguments, .. }) |
                                GroupModelKind::IndexSet(TypeModel { nested_arguments, .. })
                            ) |
                            DictFermentableModelKind::Other(TypeModel { nested_arguments, .. }))) |
                    TypeModelKind::Optional(TypeModel { nested_arguments, .. }) => {
                        let is_custom = self.maybe_custom_type(tyc.as_type());
                        let num_of_fermentable = self.num_of_nested_exposable_types_for_generic(nested_arguments);
                        let all_of_them_are_non_fermentable = num_of_fermentable == 0 && !nested_arguments.is_empty();
                        (!all_of_them_are_non_fermentable || is_custom.is_some() || nested_arguments.is_empty())
                            .then_some(tyc)
                    },
                    TypeModelKind::Trait(..) | TypeModelKind::TraitType(..) => None,
                    tyc => Some(tyc)
                }
                _ => None
            }
        }).collect::<Vec<_>>().len()
    }

    pub(crate) fn should_skip_from_expanding(&self, object: &ObjectKind) -> bool {
        let skip = match object.maybe_type_model_kind_ref() {
            Some(conversion) => {
                let maybe_custom = self.maybe_custom_type(conversion.as_type());
                let nested_arguments = conversion.nested_arguments_ref();
                let num_of_fermentable = self.num_of_nested_exposable_types_for_generic(nested_arguments);
                let all_of_them_are_non_fermentable = num_of_fermentable == 0 && !nested_arguments.is_empty();
                let maybe_lambda = conversion.is_lambda();
                all_of_them_are_non_fermentable && maybe_custom.is_none() && !maybe_lambda
            }
            None => false
        };
        skip
    }

}




/// Scope
impl GlobalContext {
    pub fn scope_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        self.scope_register.type_chain_mut(scope)
    }
    pub fn maybe_scope_ref(&self, path: &Path) -> Option<&ScopeChain> {
        self.scope_register.maybe_scope(path)
    }
    pub fn maybe_scope_ref_obj_first(&self, path: &Path) -> Option<&ScopeChain> {
        self.scope_register.maybe_first_obj_scope(path)
    }
    pub fn maybe_object_ref_by_value<'a>(&'a self, ty: &'a Type) -> Option<&'a ObjectKind> {
        ScopeSearchKey::maybe_from_ref(ty)
            .and_then(|search_key| self.maybe_object_ref_by_search_value(search_key.clone()))
    }
    fn maybe_local_scope_object_ref_by_key<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        ScopeSearchKey::maybe_from_ref(ty)
            .and_then(|search_key| self.maybe_object_ref_by_search_key_in_scope(search_key, scope))
    }
    fn maybe_object_ref_by_search_key_in_scope<'a>(&'a self, search_key: ScopeSearchKey, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        self.scope_register.maybe_object_ref_by_key_in_scope(search_key, scope)

    }
    fn maybe_object_ref_by_search_value(&self, search_key: ScopeSearchKey) -> Option<&ObjectKind> {
        self.scope_register.maybe_object_ref_by_value(search_key)
    }

}
