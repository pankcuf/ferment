use quote::ToTokens;
use syn::{Path, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeSlice, TypeTraitObject};
use crate::composable::{GenericBoundsModel, NestedArgument, TraitModel, TypeModel, TypeModeled};
use crate::context::{GlobalContext, ScopeChain};
use crate::ext::{AsType, CrateBased, DictionaryType, LifetimeProcessor, ReexportSeek, RefineMut, ToPath};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, SmartPointerModelKind, TypeModelKind};

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
        // Only try to resolve the first segment through imports - don't do partial path matching
        if let Some(first_segment) = self.segments.first() {
            use crate::ext::GenericBoundKey;
            let key = GenericBoundKey::Ident(first_segment.ident.clone());

            // Priority 1: Fast O(1) lookup in resolved imports map
            let first_segment_path = first_segment.ident.to_path();
            if let Some(resolved_path) = source.imports.resolve_import_in_scope(scope, &first_segment_path) {
                // Replace first segment with resolved path, keep remaining segments
                let mut new_segments = resolved_path.segments.clone();
                for segment in self.segments.iter().skip(1) {
                    new_segments.push(segment.clone());
                }
                self.segments = new_segments;
                return true;
            }

            // Priority 2: Fall back to enhanced import resolver for backward compatibility
            if let Some(resolved_path) = source.imports.resolve_import_enhanced(scope, &key) {
                // If we have an exact match for the first segment, replace it
                if resolved_path.segments.len() > 0 {
                    let mut new_segments = resolved_path.segments.clone();
                    // Add remaining segments from the original path (skip the first one)
                    for segment in self.segments.iter().skip(1) {
                        new_segments.push(segment.clone());
                    }
                    self.segments = new_segments;
                    return true;
                }
            }
        }

        // If no import resolution worked, leave the path as-is
        // Don't do the chunking approach as it leads to incorrect path construction
        false
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
        // Debug all TypeModel processing for Error types
        let result = match self {
            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) => false,
            TypeModelKind::Imported(ty_model, import_path, original_alias) => {
                let crate_name = scope.crate_ident_as_path();
                let crate_named_import_path = import_path.crate_named(&crate_name);
                let mut model = ty_model.clone();
                let time = std::time::SystemTime::now();
                println!("[INFO] Refine Import: {} ({}) in {}", model.as_type().to_token_stream(), import_path.to_token_stream(), scope.fmt_short());

                // Debug specific imports to trace the collision
                // Always refine nested arguments first and apply them into the model type
                let _nested_refined = refine_nested_arguments(&mut model, scope, source);

                // Fast path: Try O(1) lookup in resolved imports map with scope chain traversal
                // Use the original alias if available, otherwise fall back to last segment
                let import_alias = if let Some(alias) = original_alias {
                    alias.to_path()
                } else if let Some(last_segment) = crate_named_import_path.segments.last() {
                    last_segment.ident.to_path()
                } else {
                    crate_named_import_path.clone()
                };

                let resolved_import_path = if let Some(resolved) = resolve_import_with_scope_chain(scope, &import_alias, source) {
                    resolved
                } else {
                    // Check if this looks like an external crate import before doing expensive searches
                    if is_likely_external_import(&crate_named_import_path, scope, source) {
                        crate_named_import_path.clone()
                    } else {
                        // Try the enhanced import resolver (checks parent scopes too)
                        use crate::ext::GenericBoundKey;
                        // First try with the original import_path (alias)
                        let alias_key = GenericBoundKey::Path(import_path.clone());
                        if let Some(resolved) = source.imports.resolve_import_enhanced(scope, &alias_key) {
                            resolved.clone()
                        } else {
                            // Fallback to crate-named path
                            let key = GenericBoundKey::Path(crate_named_import_path.clone());
                            if let Some(resolved) = source.imports.resolve_import_enhanced(scope, &key) {
                                resolved.clone()
                            } else {
                                // Last fallback: Use the slower resolve_absolute_path for backward compatibility
                                source.imports
                                    .resolve_absolute_path(&crate_named_import_path, scope)
                                    .unwrap_or_else(|| crate_named_import_path.clone())
                            }
                        }
                    }
                };

                model.refine(&resolved_import_path);
                if let Some(dictionary_type) = maybe_dict_type_model_kind(&crate_named_import_path, &mut model) {
                    //println!("[INFO] (Import) Dictionary item found: {}", dictionary_type);
                    *self = TypeModelKind::Dictionary(dictionary_type);

                } else {

                    println!("[INFO] Import resolved as ({} ms): {}", std::time::SystemTime::now().duration_since(time).unwrap().as_millis(), resolved_import_path.to_token_stream());
                    // Try direct resolution, then descendMayant search under nearest existing ancestor, then absolute reexport.
                    if let Some(found_item) = source.maybe_scope_item_ref_obj_first(&resolved_import_path) {
                        // .or_else(|| find_best_ancestor(&resolved_import_path, source))
                        // .or_else(|| determine_scope_item(model.ty_mut(), scope_path, scope, source))
                        // // Try resolving via absolute reexport (handles glob reexports under modules)
                        // .or_else(|| ReexportSeek::Absolute.maybe_reexport(&resolved_import_path, source).and_then(|reexport| source.maybe_scope_item_ref_obj_first(&reexport))) {
                        //println!("[INFO] (Import) Scope item found: {}", found_item);
                        // Build the full item path without duplicating the last segment.
                        // If the original type had generic arguments on the last segment,
                        // copy those arguments onto the discovered item's last segment.
                        let mut full_item_path = found_item.path().clone();
                        if let Type::Path(TypePath { path: original_path, .. }) = model.as_type() {
                            if let Some(src_last) = original_path.segments.last() {
                                if let Some(dst_last) = full_item_path.segments.last_mut() {
                                    dst_last.arguments = src_last.arguments.clone();
                                }
                            }
                        }
                        refine_ty_with_import_path(model.ty_mut(), &full_item_path);
                        if let Some(updated) = found_item.update_with(model) {
                            //println!("[INFO] (Import) Scope item refined: {}", updated);
                            *self = updated;
                        }
                        println!("[WARN] Import refined as ScopeItem ({} ms): {}", std::time::SystemTime::now().duration_since(time).unwrap().as_millis(), self.as_type().to_token_stream());
                    // } else if let Some(reexport) = ReexportSeek::Absolute.maybe_reexport(&resolved_import_path, source) {
                    //     // As a last resort, if reexport path is found but not present as an item in the scope register
                    //     // (e.g., via glob reexports), refine the model to that absolute path and treat it as an object.
                    //     refine_ty_with_import_path(model.ty_mut(), &reexport);
                    //     println!("[WARN] Import refined as External ({} ms): {}", std::time::SystemTime::now().duration_since(time).unwrap().as_millis(), reexport.to_token_stream());
                    //     *self = TypeModelKind::Object(model);
                    } else {
                        println!("[WARN] Import refined as Unknown ({} ms): {}",  std::time::SystemTime::now().duration_since(time).unwrap().as_millis(), model.as_type().to_token_stream());
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
                } else if let Some(found_item) = source.maybe_scope_item_ref_obj_first(&path) {
                //     .or_else(|| determine_scope_item(model.ty_mut(), path.clone(), scope, source))
                //     // Try absolute reexport resolution for unknown paths as well
                //     .or_else(|| ReexportSeek::Absolute.maybe_reexport(&path, source)
                //         .and_then(|reexport| source.maybe_scope_item_ref_obj_first(&reexport))) {
                    //println!("[INFO] (Unknown) Scope item found: {}", found_item);
                    refine_ty_with_import_path(model.ty_mut(), found_item.path());
                    if let Some(updated) = found_item.update_with(model.clone()) {
                        //println!("[INFO] (Unknown) Scope item refined (Unknown): {}", updated);
                        *self = updated;
                    }
                    true
                // } else if let Some(reexport) = ReexportSeek::Absolute.maybe_reexport(&path, source) {
                //     // If reexport path found but not tracked as a scope item (e.g., via glob), promote to Object
                //     refine_ty_with_import_path(model.ty_mut(), &reexport);
                //     *self = TypeModelKind::Object(model.clone());
                //     true
                } else {
                    println!("[WARN] Unknown import: {}", model.as_type().to_token_stream());
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

// fn determine_scope_item<'a>(new_ty_to_replace: &mut Type, ty_path: Path, scope: &ScopeChain, source: &'a GlobalContext) -> Option<&'a ScopeItemKind> {
//     // There are 2 cases:
//     // 1. it's from non-fermented crate
//     // 2. it's not full scope:
//     // - It's reexported somewhere?
//     //     - It's child scope?
//     //     - It's neighbour scope?
//     // println!("determine_scope_item: {} /// {} in {}", new_ty_to_replace.to_token_stream(), ty_path.to_token_stream(), scope.fmt_short());
//     match scope {
//         ScopeChain::CrateRoot { info, .. } |
//         ScopeChain::Mod { info, .. } => {
//             // self -> neighbour mod
//             let self_path = info.self_path();
//             // Respect absolute paths: if `ty_path` starts with crate ident or `crate`, do not join.
//             // For other paths, rely on import resolution to determine if they should be absolute
//             let is_absolute = ty_path.is_crate_based()
//                 || ty_path.segments.first().map(|s| s.ident.eq(&info.crate_ident)).unwrap_or_default();
//             let child_scope = if is_absolute { ty_path.clone() } else { self_path.joined(&ty_path) };
//             // child -> self
//             // If it's nested mod?
//             source.maybe_scope_item_ref_obj_first(&child_scope)
//                 .inspect(|item| refine_ty_with_import_path(new_ty_to_replace, item.path()))
//                 .or_else(|| ReexportSeek::new(is_absolute)
//                     .maybe_reexport(&child_scope, source)
//                     .and_then(|reexport| source.maybe_scope_item_ref_obj_first(&reexport)
//                         .inspect(|item| refine_ty_with_import_path(new_ty_to_replace, item.path())))
//                     .or_else(|| source.maybe_scope_item_ref_obj_first(self_path)))
//         }
//         ScopeChain::Impl { parent, .. } |
//         ScopeChain::Trait { parent, .. } |
//         ScopeChain::Object { parent, .. } => {
//             //  -- Import Scope: [ferment_example_entry_point::entry::rnt]
//             //      -- Has Scope?: ferment_example_entry_point::entry::rnt::tokio::runtime::Runtime --- No
//             //      -- Has Scope? ferment_example_entry_point::entry::rnt::tokio::runtime --- No
//             //      -- Has Scope? ferment_example_entry_point::entry::rnt::tokio --- No
//             //      -- Not a local import, so check globals:
//             //          -- Has Scope? tokio::runtime --- No
//             //          -- Has Scope? tokio --- No
//             //          -- Not a global import, so it's from non-fermented crate -> So it's opaque
//
//             // self -> parent mod -> neighbour mod
//             // let self_path = info.self_path();
//             let parent_path = parent.self_path_ref();
//             // check parent + local
//
//             // Respect absolute paths: if `ty_path` starts with crate ident or `crate`, do not join.
//             // For other paths, rely on import resolution to determine if they should be absolute
//             let is_absolute = ty_path.is_crate_based()
//                 || ty_path.segments.first().map(|s| s.ident.eq(parent.crate_ident_ref())).unwrap_or_default();
//             let child_scope = if is_absolute { ty_path.clone() } else { parent_path.joined(&ty_path) };
//             source.maybe_scope_item_ref_obj_first(&child_scope)
//                 .inspect(|item| refine_ty_with_import_path(new_ty_to_replace, item.path()))
//                 .or_else(||
//                     ReexportSeek::new(is_absolute)
//                         .maybe_reexport(&child_scope, source)
//                         .and_then(|reexport| source.maybe_scope_item_ref_obj_first(&reexport))
//                         .or_else(|| source.maybe_scope_item_ref_obj_first(parent_path))
//                         .or_else(|| source.maybe_scope_item_ref_obj_first(&parent_path.joined(&ty_path))
//                             .inspect(|item| refine_ty_with_import_path(new_ty_to_replace, item.path()))))
//         }
//         ScopeChain::Fn { parent, .. } => {
//             // - Check parent scopes for items relative to the function scope (do not match the function itself)
//             match &**parent {
//                     ScopeChain::CrateRoot { info, .. } |
//                     ScopeChain::Mod { info, .. } => {
//                         let base = info.self_path();
//                         let is_absolute = ty_path.is_crate_based()
//                             || ty_path.segments.first().map(|s| s.ident.eq(&info.crate_ident)).unwrap_or_default();
//                         let scope = if is_absolute { ty_path.clone() } else { base.joined(&ty_path) };
//                         source.maybe_scope_item_ref_obj_first(&scope)
//                             .inspect(|item| refine_ty_with_import_path(new_ty_to_replace, item.path()))
//
//                     },
//                     ScopeChain::Trait { parent, .. } |
//                     ScopeChain::Object { parent, .. } |
//                     ScopeChain::Impl { parent, .. } => {
//                         let base = parent.self_path_ref().clone();
//                         let is_absolute = ty_path.is_crate_based()
//                             || ty_path.segments.first().map(|s| s.ident == *parent.crate_ident_ref()).unwrap_or_default();
//                         let scope = if is_absolute { ty_path.clone() } else { base.joined(&ty_path) };
//                         source.maybe_scope_item_ref_obj_first(&scope)
//                             .inspect(|item| refine_ty_with_import_path(new_ty_to_replace, item.path()))
//                     },
//                     ScopeChain::Fn { .. } => {
//                         // TODO: support nested function when necessary
//                         //println!("nested function::: {} --- [{}]", info.self_scope, parent);
//                         None
//                     }
//                 }
//         }
//     }
// }

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
            type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| if let Some(maybe_nested_type) = maybe_refined_ty_for_nested_arg_in_scope(&mut nested_arguments[index], scope, source) {
                *elem = maybe_nested_type;
                refined = true;
            });
        },
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter_mut().enumerate().for_each(|(index, elem)| if let TypeParamBound::Trait(TraitBound { path, .. }) = elem {
                if let Some(maybe_nested_type) = maybe_refined_ty_for_nested_arg_in_scope(&mut nested_arguments[index], scope, source) {
                    *path = maybe_nested_type.to_path();
                    refined = true;
                }
            });
        }
        Type::Array(TypeArray { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) => if let Some(maybe_nested_type) = maybe_refined_ty_for_nested_arg_in_scope(&mut nested_arguments[0], scope, source) {
            *elem = Box::new(maybe_nested_type);
            refined = true;
        },
        _ => {
            // What about others like Reference?
        }
    }
    refined
}

pub fn maybe_refined_ty_for_nested_arg_in_scope(nested_arg: &mut NestedArgument, scope: &ScopeChain, source: &GlobalContext) -> Option<Type> {
    if nested_arg.refine_in_scope(scope, source) {
        nested_arg.maybe_type()
    } else {
        None
    }

}


/// Fast O(1) import resolution with proper scope chain traversal
/// Checks current scope, then parent scopes for non-mod items
/// For traits/impl/functions, checks up to grandparent scope
fn resolve_import_with_scope_chain(scope: &ScopeChain, import_path: &Path, source: &GlobalContext) -> Option<Path> {
    // Build the complete scope chain to check
    let mut scopes_to_check = Vec::new();
    let mut current_scope = Some(scope);

    // Collect all scopes in the inheritance chain
    while let Some(scope) = current_scope {
        scopes_to_check.push(scope);

        current_scope = match scope {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => None, // Stop at module boundaries
            ScopeChain::Fn { parent, .. } |
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Object { parent, .. } |
            ScopeChain::Impl { parent, .. } => Some(parent),
        };
    }

    // Check each scope in the chain for the import
    for check_scope in scopes_to_check {
        if let Some(resolved) = source.imports.resolve_import_in_scope(check_scope, import_path) {
            println!("[INFO] Import {} found in scope chain: {} (from {})", resolved.to_token_stream(), check_scope.fmt_short(), scope.fmt_short());
            return Some(resolved.clone());
        }
    }

    None
}

/// Check if an import path is likely from an external crate or non-exported item
/// This helps avoid expensive searches for imports that won't be found in the scope resolver
fn is_likely_external_import(import_path: &Path, current_scope: &ScopeChain, source: &GlobalContext) -> bool {
    if import_path.segments.is_empty() {
        return false;
    }

    let first_segment = &import_path.segments[0].ident.to_string();
    let current_crate = current_scope.crate_ident_ref().to_string();

    // If it's the current crate, it's not external
    if first_segment == &current_crate {
        return false;
    }

    // Check if the first segment is listed in external_crates config
    source.config.external_crates.iter().any(|c| c.name.eq(first_segment))
}

pub fn refine_ty_with_import_path(ty: &mut Type, crate_named_import_path: &Path) {
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
    }
}
