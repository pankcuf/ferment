use quote::ToTokens;
use syn::{Path, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeSlice, TypeTraitObject};
use crate::composable::{GenericBoundsModel, NestedArgument, TraitModel, TypeModel, TypeModeled};
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeInfo};
use crate::ext::{AsType, CrateBased, DictionaryType, Join, LifetimeProcessor, Pop, ReexportSeek, RefineMut, ToPath};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};

pub trait RefineInScope {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool;
}

impl RefineInScope for GenericBoundsModel {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        let mut refined = false;
        self.chain.iter_mut().for_each(|(_bounded_ty, bounds)| {
            // TODO: should refine key as well, since it can be particular type or contains QSelf
            bounds.iter_mut().for_each(|arg| if let Some(refined_obj) = source.maybe_refined_object(scope, arg) {
                *arg = refined_obj;
                refined = true;
            });
        });
        self.nested_arguments_iter_mut().for_each(|nested_arg| if nested_arg.refine_in_scope(scope, source) {
            refined = true;
        });
        refined
    }
}

impl RefineInScope for Path {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        let crate_name = scope.crate_ident_as_path();
        let mut refined = false;
        let mut chunks = self.clone();
        while !chunks.segments.is_empty() {
            chunks.segments = chunks.segments.popped();
            if !chunks.segments.is_empty() {
                let mod_chain = create_mod_chain(&chunks);
                if let Some(parent_imports) = source.imports.maybe_scope_imports(&mod_chain) {
                    for alias_path in parent_imports.values() {
                        let alias = alias_path.crate_named(&crate_name);
                        if let Some(merged) = refined_import(self, &alias, source) {
                            self.segments = merged.segments;
                            refined = true;
                        }
                    }
                }
            }
        }
        refined
    }
}



impl RefineInScope for ObjectKind {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        match self {
            ObjectKind::Type(tyc) =>
                tyc.refine_in_scope(scope, source),
            _ => false
        }
    }
}

impl RefineInScope for TypeModelKind {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        // println!("REFINE --> {} \n\tin {}", self, scope.fmt_short());
        let result = match self {
            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) => false,
            TypeModelKind::Imported(ty_model, import_path) => {
                let crate_name = scope.crate_ident_as_path();
                let crate_named_import_path = import_path.crate_named(&crate_name);
                let mut model = ty_model.clone();
                println!("[INFO] (Import) Refine Unknown import: {} ({}) in {}", model.as_type().to_token_stream(), import_path.to_token_stream(), scope.fmt_mid());

                // Always refine nested arguments first and apply them into the model type
                let _nested_refined = refine_nested_arguments(&mut model, scope, source);

                // Prefer resolving potential reexports (handles glob and multi-hop) up front.
                let resolved_import_path = ReexportSeek::Absolute
                    .maybe_reexport(&crate_named_import_path, source)
                    .unwrap_or_else(|| crate_named_import_path.clone());
                model.refine(&resolved_import_path);
                println!("resolved_import_path: {}", resolved_import_path.to_token_stream());
                if let Some(dictionary_type) = maybe_dict_type_model_kind(&crate_named_import_path, &mut model) {
                    //println!("[INFO] (Import) Dictionary item found: {}", dictionary_type);
                    *self = TypeModelKind::Dictionary(dictionary_type);

                } else {

                    let scope_path = model.lifetimes_cleaned().pointer_less();
                    // Try direct resolution, then descendant search under nearest existing ancestor, then absolute reexport.
                    if let Some(found_item) = source.maybe_scope_item_ref_obj_first(&resolved_import_path)
                        .or_else(|| {
                            // Find the nearest existing ancestor scope and scan its descendants for a matching leaf
                            let mut anc = resolved_import_path.popped();
                            println!("Find the nearest existing ancestor: {}", anc.to_token_stream());
                            let last_seg = resolved_import_path.segments.last().cloned();
                            while !anc.segments.is_empty() {
                                if let Some(ancestor_scope) = source.maybe_scope_ref(&anc) {
                                    println!("Ancestor scope found: {}", ancestor_scope.fmt_short());
                                    if let Some(last_seg) = last_seg.as_ref() {
                                        let ancestor_path = ancestor_scope.self_path_ref();
                                        println!("Ancestor path found: {}", ancestor_path.to_token_stream());
                                        if let Some(found) = source.scope_register.inner.keys()
                                            .find_map(|scope_chain| {
                                                let scope_path = scope_chain.self_path_ref();
                                                if ancestor_path.segments.len() <= scope_path.segments.len()
                                                    && scope_path.segments.iter().zip(ancestor_path.segments.iter()).all(|(a, b)| a.ident == b.ident) {
                                                    let already_matches_last = scope_path.segments.last().map(|s| s.ident == last_seg.ident).unwrap_or_default();
                                                    let candidate = if already_matches_last { scope_path.clone() } else { scope_path.joined(&last_seg.ident.to_path()) };
                                                    source.maybe_scope_item_ref_obj_first(&candidate)
                                                } else { None }
                                            }) {
                                            println!("Found the nearest existing ancestor: {found:?}");
                                            return Some(found);
                                        }
                                    }
                                    break;
                                }
                                anc = anc.popped();
                            }
                            None
                        })
                        .or_else(|| determine_scope_item(&mut model, scope_path, scope, source))
                        .or_else(|| {
                            // Try resolving via absolute reexport (handles glob reexports under modules)
                            ReexportSeek::Absolute
                                .maybe_reexport(&resolved_import_path, source)
                                .and_then(|reexport| source.maybe_scope_item_ref_obj_first(&reexport))
                        }) {
                        println!("[INFO] (Import) Scope item found: {}", found_item);
                        // Build the full item path without duplicating the last segment.
                        // If the original type had generic arguments on the last segment,
                        // copy those arguments onto the discovered item's last segment.
                        let mut full_item_path = found_item.path().clone();
                        if let Type::Path(TypePath { path: original_path, .. }) = model.as_type() {
                            if let (Some(src_last), Some(dst_last)) = (original_path.segments.last(), full_item_path.segments.last_mut()) {
                                dst_last.arguments = src_last.arguments.clone();
                            }
                        }
                        refine_ty_with_import_path(model.ty_mut(), &full_item_path);
                        if let Some(updated) = found_item.update_with(model) {
                            //println!("[INFO] (Import) Scope item refined: {}", updated);
                            *self = updated;
                        }
                        println!("[WARN] (Import) REFINED (MAYBE): {}", self.as_type().to_token_stream());
                    } else if let Some(reexport) = ReexportSeek::Absolute.maybe_reexport(&resolved_import_path, source) {
                        // As a last resort, if reexport path is found but not present as an item in the scope register
                        // (e.g., via glob reexports), refine the model to that absolute path and treat it as an object.
                        refine_ty_with_import_path(model.ty_mut(), &reexport);
                        println!("[WARN] (Import) LAST RESORT: {}", reexport.to_token_stream());
                        *self = TypeModelKind::Object(model);
                    } else {
                        println!("[WARN] (Import) Unknown import: {}", model.as_type().to_token_stream());
                        *self = TypeModelKind::Unknown(model)
                    }
                }
                true
            }
            TypeModelKind::Unknown(model) => {
                let path = model.lifetimes_cleaned().pointer_less();
                if let Some(mut dictionary_type) = maybe_dict_type_model_kind(&path, model) {
                    //println!("[INFO] (Unknown) Dictionary item found: {}", dictionary_type);
                    refine_nested_arguments(dictionary_type.type_model_mut(), scope, source);
                    *self = TypeModelKind::Dictionary(dictionary_type);
                    true
                } else if let Some(found_item) = source.maybe_scope_item_ref_obj_first(&path)
                    .or_else(|| determine_scope_item(model, path.clone(), scope, source))
                    .or_else(|| {
                        // Try absolute reexport resolution for unknown paths as well
                        ReexportSeek::Absolute
                            .maybe_reexport(&path, source)
                            .and_then(|reexport| source.maybe_scope_item_ref_obj_first(&reexport))
                    }) {
                    //println!("[INFO] (Unknown) Scope item found: {}", found_item);
                    refine_ty_with_import_path(model.ty_mut(), found_item.path());
                    if let Some(updated) = found_item.update_with(model.clone()) {
                        //println!("[INFO] (Unknown) Scope item refined (Unknown): {}", updated);
                        *self = updated;
                    }
                    true
                } else if let Some(reexport) = ReexportSeek::Absolute.maybe_reexport(&path, source) {
                    // If reexport path found but not tracked as a scope item (e.g., via glob), promote to Object
                    refine_ty_with_import_path(model.ty_mut(), &reexport);
                    *self = TypeModelKind::Object(model.clone());
                    true
                } else {
                    println!("[WARN] (Unknown) Unknown import: {}", model.as_type().to_token_stream());
                    false
                }
            }
            TypeModelKind::Dictionary(
                DictTypeModelKind::NonPrimitiveFermentable(
                    DictFermentableModelKind::Cow(model) |
                    DictFermentableModelKind::SmartPointer(
                        SmartPointerModelKind::Arc(model) |
                        SmartPointerModelKind::Box(model) |
                        SmartPointerModelKind::Rc(model) |
                        SmartPointerModelKind::Mutex(model) |
                        SmartPointerModelKind::OnceLock(model) |
                        SmartPointerModelKind::RwLock(model) |
                        SmartPointerModelKind::Cell(model) |
                        SmartPointerModelKind::RefCell(model) |
                        SmartPointerModelKind::UnsafeCell(model) |
                        SmartPointerModelKind::Pin(model)
                    ) |
                    DictFermentableModelKind::Group(
                        GroupModelKind::BTreeSet(model) |
                        GroupModelKind::HashSet(model) |
                        GroupModelKind::Map(model) |
                        GroupModelKind::Result(model) |
                        GroupModelKind::Vec(model) |
                        GroupModelKind::IndexMap(model) |
                        GroupModelKind::IndexSet(model)
                    ) |
                    DictFermentableModelKind::Other(model) |
                    DictFermentableModelKind::I128(model) |
                    DictFermentableModelKind::U128(model) |
                    DictFermentableModelKind::Str(model) |
                    DictFermentableModelKind::String(model)) |
                DictTypeModelKind::NonPrimitiveOpaque(model) |
                DictTypeModelKind::LambdaFn(model)) |
            TypeModelKind::FnPointer(model, ..) |
            TypeModelKind::Object(model) |
            TypeModelKind::Optional(model) |
            TypeModelKind::TraitType(model) |
            TypeModelKind::Trait(TraitModel { ty: model, ..}) => {
                let mut changed = refine_model_path_via_reexport(model, source);
                if refine_nested_arguments(model, scope, source) { changed = true; }
                changed
            },
            TypeModelKind::Array(model) |
            TypeModelKind::Slice(model) |
            TypeModelKind::Tuple(model) =>
                refine_nested_ty(model, scope, source),
            TypeModelKind::Bounds(model) =>
                model.refine_in_scope(scope, source),
            TypeModelKind::Fn(_) => {
                // TODO: global generic?
                false
            }
        };
        // println!("REFINE ({}) <-- {} \n\tin {}", result, self, scope.fmt_short());
        result
    }
}

impl RefineInScope for NestedArgument {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        //println!("NestedArgument::refine_in_scope --> {} \n\tin {}", self, scope.fmt_short());
        let obj = self.object_mut();
        if let Some(refined_obj) = source.maybe_refined_object(scope, obj) {
            *obj = refined_obj;
            true
        } else {
            false
        }
    }
}

fn create_mod_chain(path: &Path) -> ScopeChain {
    let segments = &path.segments;
    let crate_ident = &segments.first().expect("Mod path should have at least one segment").ident;
    let self_scope = Scope::empty(path.clone());
    let parent_chunks = path.popped();
    let parent = if parent_chunks.segments.len() > 1 {
        create_mod_chain(&parent_chunks)
    } else {
        ScopeChain::root(ScopeInfo::attr_less(crate_ident, Scope::empty(parent_chunks)))
    };
    let info = ScopeInfo::attr_less(crate_ident, self_scope);
    if segments.len() == 1 {
        ScopeChain::root(info)
    } else {
        ScopeChain::r#mod(info, parent)
    }
}

fn refined_import(import_path: &Path, alias: &Path, source: &GlobalContext) -> Option<Path> {
    match (import_path.segments.last(), alias.segments.last()) {
        (Some(PathSegment { ident, .. }), Some(PathSegment { ident: alias_ident, .. })) if ident == alias_ident =>
            ReexportSeek::Absolute.maybe_reexport(import_path, source),
        _ => None
    }
}

// Unknown: There are 2 cases:
// 1. it's from non-fermented crate
// 2. it's not full scope:
//  - It's reexported somewhere?
//  - It's child scope?
//  - It's neighbour scope?

// Import: There are 2 cases:
// 1. it's from non-fermented crate
// 2. it's not full scope:
//  - It's reexported somewhere?
//  - It's child scope?
//  - It's neighbour scope?
// println!("(Imported) (not found): {}", crate_named_import_path.to_token_stream());

// We are here if we have no scope item with the import path, so we should do the following:
// 1. Check whether the import is local
//  - Check if import starts with "crate", "self", "super"
//      "crate" => replace "crate" with crate ident and pop chunks until found a scope
//          if [scope found] => [check reexports in the scope] else [raise exception]
//      "self" => replace "self" with the scope
//      "super" (can be chained) =>

fn maybe_dict_type_model_kind(crate_named_import_path: &Path, model: &mut TypeModel) -> Option<DictTypeModelKind> {
    crate_named_import_path.segments.last().and_then(|last_segment| {
        let ident = &last_segment.ident;
        if ident.is_primitive() {
            Some(DictTypeModelKind::Primitive(model.clone()))
        } else if ident.eq("i128") {
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(model.clone())))
        } else if ident.eq("u128") {
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(model.clone())))
        } else if ident.is_str() {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(model.clone())))
        } else if ident.is_string() {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::String(model.clone())))
        } else if ident.is_lambda_fn()  {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            Some(DictTypeModelKind::LambdaFn(model.clone()))
        } else if matches!(ident.to_string().as_str(), "FromIterator" | "From" | "Into") || ident.is_special_std_trait() || ident.is_map() || ident.is_special_generic(){
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Other(model.clone())))
        } else if ident.is_box() {
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model.clone()))))
        } else if ident.is_cow() {
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(model.clone())))
        } else {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            match ident.to_string().as_str() {
                "Arc" => Some(SmartPointerModelKind::Arc(model.clone())),
                "Mutex" => Some(SmartPointerModelKind::Mutex(model.clone())),
                "Pin" => Some(SmartPointerModelKind::Pin(model.clone())),
                "Rc" => Some(SmartPointerModelKind::Rc(model.clone())),
                "Cell" => Some(SmartPointerModelKind::Cell(model.clone())),
                "RefCell" => Some(SmartPointerModelKind::RefCell(model.clone())),
                "UnsafeCell" => Some(SmartPointerModelKind::UnsafeCell(model.clone())),
                "OnceLock" => Some(SmartPointerModelKind::OnceLock(model.clone())),
                "RwLock" => Some(SmartPointerModelKind::RwLock(model.clone())),
                _ => None
            }.map(|smart_ptr_model| {
                refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
                DictTypeModelKind::smart_pointer(smart_ptr_model)
            })
        }
    })
}

fn determine_scope_item<'a>(new_ty_to_replace: &mut TypeModel, ty_path: Path, scope: &ScopeChain, source: &'a GlobalContext) -> Option<&'a ScopeItemKind> {
    // There are 2 cases:
    // 1. it's from non-fermented crate
    // 2. it's not full scope:
    // - It's reexported somewhere?
    //     - It's child scope?
    //     - It's neighbour scope?
    println!("determine_scope_item: {} /// {} in {}", new_ty_to_replace.ty.to_token_stream(), ty_path.to_token_stream(), scope.fmt_short());
    match scope {
        ScopeChain::CrateRoot { info, .. } |
        ScopeChain::Mod { info, .. } => {
            // self -> neighbour mod
            let self_path = info.self_path();
            // Respect absolute paths: if `ty_path` starts with crate ident or `crate`, do not join.
            let is_absolute = ty_path.is_crate_based()
                || ty_path.segments.first().map(|s| s.ident.eq(&info.crate_ident)).unwrap_or_default();
            let child_scope = if is_absolute { ty_path.clone() } else { self_path.joined(&ty_path) };
            // child -> self
            // If it's nested mod?
            source.maybe_scope_item_ref_obj_first(&child_scope)
                .inspect(|item| {
                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                })
                .or_else(|| {
                    // it also can be re-exported in child tree so we should check it
                    //println!("\t... (not found -> check reexport): {}", format_token_stream(&child_scope));
                    // Choose reexport mode based on how `child_scope` was built
                    let reexp = if is_absolute { ReexportSeek::Absolute } else { ReexportSeek::Relative };
                    reexp
                        .maybe_reexport(&child_scope, source)
                        .and_then(|reexport| {
                            //println!("\t\t... (reexport found): [{}]", format_token_stream(&reexport));
                            source.maybe_scope_item_ref_obj_first(&reexport)
                                .inspect(|item| {
                                    //println!("\t... (item found -> refine it): {}", format_token_stream(item.path()));
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                    //println!("\t... (item found -> refined): {}", item);
                                })

                        })
                        .or_else(|| {
                            //println!("\t\t\t\t... (no: maybe item at self_path?): [{}]", format_token_stream(self_path));
                            source.maybe_scope_item_ref_obj_first(self_path)
                        })
                })
        }
        ScopeChain::Impl { parent, .. } |
        ScopeChain::Trait { parent, .. } |
        ScopeChain::Object { parent, .. } => {
            //  -- Import Scope: [ferment_example_entry_point::entry::rnt]
            //      -- Has Scope?: ferment_example_entry_point::entry::rnt::tokio::runtime::Runtime --- No
            //      -- Has Scope? ferment_example_entry_point::entry::rnt::tokio::runtime --- No
            //      -- Has Scope? ferment_example_entry_point::entry::rnt::tokio --- No
            //      -- Not a local import, so check globals:
            //          -- Has Scope? tokio::runtime --- No
            //          -- Has Scope? tokio --- No
            //          -- Not a global import, so it's from non-fermented crate -> So it's opaque

            // self -> parent mod -> neighbour mod
            // let self_path = info.self_path();
            let parent_path = parent.self_path_ref();
            // check parent + local

            // Respect absolute paths: if `ty_path` starts with crate ident or `crate`, do not join.
            let is_absolute = ty_path.is_crate_based() || ty_path.segments.first().map(|s| s.ident == *parent.crate_ident_ref()).unwrap_or_default();
            let child_scope = if is_absolute { ty_path.clone() } else { parent_path.joined(&ty_path) };
            //println!("... (check as relative): {}", format_token_stream(&child_scope));
            source.maybe_scope_item_ref_obj_first(&child_scope)
                .inspect(|item| {
                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                })
                .or_else(|| {
                    //println!("\t... (not found -> check reexport): {}", format_token_stream(&child_scope));
                    // it also can be re-exported in child tree so we should check it
                    let reexp = if is_absolute { ReexportSeek::Absolute } else { ReexportSeek::Relative };
                    reexp
                        .maybe_reexport(&child_scope, source)
                        .and_then(|reexport| {
                            //println!("\t\t... (reexport found): [{}]", format_token_stream(&reexport));
                            source.maybe_scope_item_ref_obj_first(&reexport)
                                .inspect(|item| {
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                })

                        })
                        // .or_else(|| {
                        //     println!("\t\t\t... (reexport not found -> maybe item at self path?): [{}]", format_token_stream(self_path));
                        //     source.maybe_item(self_path)
                        // })
                        .or_else(|| {
                            //println!("\t\t\t\t... (no: maybe item at parent path?): [{}]", format_token_stream(parent_path));
                            source.maybe_scope_item_ref_obj_first(parent_path)
                                .inspect(|item| {
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                })

                        })
                        .or_else(|| {
                            //println!("\t\t\t\t\t... (no maybe item at parent path + type path): [{}] + [{}]", format_token_stream(parent_path), format_token_stream(&ty_path));
                            let scope = parent_path.joined(&ty_path);
                            source.maybe_scope_item_ref_obj_first(&scope)
                                .inspect(|item| {
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                })
                        })
                })
        }
        ScopeChain::Fn { parent, .. } => {
            // - Check parent scopes for items relative to the function scope (do not match the function itself)
            match &**parent {
                    ScopeChain::CrateRoot { info, .. } |
                    ScopeChain::Mod { info, .. } => {
                        let base = info.self_path();
                        let is_absolute = ty_path.is_crate_based()
                            || ty_path.segments.first().map(|s| s.ident.eq(&info.crate_ident)).unwrap_or_default();
                        let scope = if is_absolute { ty_path.clone() } else { base.joined(&ty_path) };
                        source.maybe_scope_item_ref_obj_first(&scope)
                            .inspect(|item| {
                                refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                            })

                    },
                    ScopeChain::Trait { parent, .. } |
                    ScopeChain::Object { parent, .. } |
                    ScopeChain::Impl { parent, .. } => {
                        let base = parent.self_path_ref().clone();
                        let is_absolute = ty_path.is_crate_based()
                            || ty_path.segments.first().map(|s| s.ident == *parent.crate_ident_ref()).unwrap_or_default();
                        let scope = if is_absolute { ty_path.clone() } else { base.joined(&ty_path) };
                        source.maybe_scope_item_ref_obj_first(&scope)
                            .inspect(|item| {
                                refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                            })

                    },
                    ScopeChain::Fn { .. } => {
                        // TODO: support nested function when necessary
                        //println!("nested function::: {} --- [{}]", info.self_scope, parent);
                        None
                    }
                }
        }
    }
}

fn refine_nested_arguments(model: &mut TypeModel, scope: &ScopeChain, source: &GlobalContext) -> bool {
    let mut refined = false;
    model.nested_arguments_iter_mut()
        .for_each(|nested_arg| if nested_arg.refine_in_scope(scope, source) {
            refined = true;
        });
    if refined {
        model.refine_with(model.nested_arguments());
    }
    refined
}

fn refine_model_path_via_reexport(model: &mut TypeModel, source: &GlobalContext) -> bool {
    let path = model.lifetimes_cleaned().pointer_less();
    if let Some(reexport) = ReexportSeek::Absolute.maybe_reexport(&path, source) {
        if reexport.to_token_stream().to_string() != path.to_token_stream().to_string() {
            refine_ty_with_import_path(model.ty_mut(), &reexport);
            return true;
        }
    }
    false
}

fn refine_nested_ty(new_ty_model: &mut TypeModel, scope: &ScopeChain, source: &GlobalContext) -> bool {
    let mut refined = false;
    let (ty, nested_arguments) = new_ty_model.type_model_and_nested_arguments_mut();
    match ty {
        Type::Tuple(type_tuple) => {
            type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| {
                let nested_arg = &mut nested_arguments[index];
                if nested_arg.refine_in_scope(scope, source) {
                    if let Some(maybe_nested_type) = nested_arg.maybe_type() {
                        *elem = maybe_nested_type;
                        refined = true;
                    }
                }
            });
        },
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter_mut().enumerate().for_each(|(index, elem)| if let TypeParamBound::Trait(TraitBound { path, .. }) = elem {
                let nested_arg = &mut nested_arguments[index];
                if nested_arg.refine_in_scope(scope, source) {
                    if let Some(maybe_nested_type) = nested_arg.maybe_type() {
                        *path = maybe_nested_type.to_path();
                        refined = true;
                    }
                }
            });
        }
        Type::Array(TypeArray { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) => {
            let nested_arg = &mut nested_arguments[0];
            if nested_arg.refine_in_scope(scope, source) {
                if let Some(maybe_nested_type) = nested_arg.maybe_type() {
                    *elem = Box::new(maybe_nested_type);
                    refined = true;
                }
            }
        },
        _ => {
            // What about others like Reference?
        }
    }
    refined
}

pub fn refine_ty_with_import_path(ty: &mut Type, crate_named_import_path: &Path) -> bool {
    let mut refined = false;
    if let Type::Path(TypePath { path, .. }) = ty {
        *path = if let Some(PathSegment { arguments: last_popped_args, .. }) = path.segments.last() {
            let mut full_path_with_args = crate_named_import_path.clone();
            if let Some(PathSegment { arguments, .. }) = full_path_with_args.segments.last_mut() {
                *arguments = last_popped_args.clone();
            }
            full_path_with_args
        } else {
            crate_named_import_path.clone()
        };
        refined = true;
    }
    refined
}
