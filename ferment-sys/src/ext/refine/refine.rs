use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_quote, AngleBracketedGenericArguments, BareFnArg, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use crate::ast::{Colon2Punctuated, PathHolder};
use crate::composable::{GenericBoundsModel, NestedArgument, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeInfo};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, CrateExtension, DictionaryType, LifetimeProcessor, Pop, RefineMut, ToPath};

#[allow(unused)]
pub trait RefineInScope {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool;
}
#[allow(unused)]
pub trait RefineWithNestedArg {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool;
}
#[allow(unused)]
pub trait RefineWithNestedArgs {
    fn refine_with_nested_args(&mut self, nested_arguments: &CommaPunctuatedNestedArguments) -> bool;
}
#[allow(unused)]
pub trait RefineWithFullPath {
    fn refine_with_full_path(&mut self, full_path: &Path, nested_arguments: &CommaPunctuatedNestedArguments) -> bool;
}

impl RefineInScope for GenericBoundsModel {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        let mut refined = false;
        //println!("GenericBoundsModel::refine_in_scope: {} --- {}", self, scope.fmt_short());
        self.bounds.iter_mut().for_each(|arg| {
            if let Some(refined_obj) = source.maybe_refined_object(scope, arg) {
                *arg = refined_obj;
                refined = true;
            }
        });
        self.predicates.iter_mut().for_each(|(_ty, args)| {
            args.iter_mut().for_each(|arg| {
                if let Some(refined_obj) = source.maybe_refined_object(scope, arg) {
                    *arg = refined_obj;
                    refined = true;
                }
            })
        });
        self.nested_arguments_iter_mut()
            .for_each(|nested_arg| {
                if nested_arg.refine_in_scope(scope, source) {
                    refined = true;
                }
            });
        //println!("GenericBoundsModel::refine_in_scope: RESULT ({}): {} --- {}", refined, self, scope.fmt_short());
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
                    for (PathHolder(_ident), alias_path) in parent_imports {
                        let alias = alias_path.crate_named(&crate_name);
                        if let Some(merged) = refined_import(&self, &alias, source) {
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

fn determine_scope_item<'a>(new_ty_to_replace: &mut TypeModel, ty_path: Path, scope: &ScopeChain, source: &'a GlobalContext) -> Option<&'a ScopeItemKind> {
    // There are 2 cases:
    // 1. it's from non-fermented crate
    // 2. it's not full scope:
    // - It's reexported somewhere?
    //     - It's child scope?
    //     - It's neighbour scope?
    match scope {
        ScopeChain::CrateRoot { info, .. } |
        ScopeChain::Mod { info, .. } => {
            // self -> neighbour mod
            let self_path = info.self_path();
            let child_scope: Path = parse_quote!(#self_path::#ty_path);
            // child -> self
            // If it's nested mod?
            source.maybe_scope_item_ref_obj_first(&child_scope)
                .map(|item| {
                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                    item
                })
                .or_else(|| {
                    // it also can be re-exported in child tree so we should check it
                    //println!("\t... (not found -> check reexport): {}", format_token_stream(&child_scope));
                    ReexportSeek::Relative
                        .maybe_reexport(&child_scope, source)
                        .and_then(|reexport| {
                            //println!("\t\t... (reexport found): [{}]", format_token_stream(&reexport));
                            //maybe_scope_item_conversion(&mut new_ty_to_replace, &reexport, source)
                            source.maybe_scope_item_ref_obj_first(&reexport)
                                .map(|item| {
                                    //println!("\t... (item found -> refine it): {}", format_token_stream(item.path()));
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                    //println!("\t... (item found -> refined): {}", item);
                                    item
                                })

                        })
                        .or_else(|| {
                            //println!("\t\t\t\t... (no: maybe item at self_path?): [{}]", format_token_stream(self_path));
                            source.maybe_scope_item_ref_obj_first(self_path)
                        })
                })
        }
        ScopeChain::Impl { parent_scope_chain, .. } |
        ScopeChain::Trait { parent_scope_chain, .. } |
        ScopeChain::Object { parent_scope_chain, .. } => {
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
            let parent_path = parent_scope_chain.self_path();
            // check parent + local

            let child_scope: Path = parse_quote!(#parent_path::#ty_path);
            //println!("... (check as relative): {}", format_token_stream(&child_scope));
            source.maybe_scope_item_ref_obj_first(&child_scope)
                .map(|item| {
                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                    item
                })
                .or_else(|| {
                    //println!("\t... (not found -> check reexport): {}", format_token_stream(&child_scope));
                    // it also can be re-exported in child tree so we should check it
                    ReexportSeek::Relative
                        .maybe_reexport(&child_scope, source)
                        .and_then(|reexport| {
                            //println!("\t\t... (reexport found): [{}]", format_token_stream(&reexport));
                            source.maybe_scope_item_ref_obj_first(&reexport)
                                .map(|item| {
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                    item
                                })

                        })
                        // .or_else(|| {
                        //     println!("\t\t\t... (reexport not found -> maybe item at self path?): [{}]", format_token_stream(self_path));
                        //     source.maybe_item(self_path)
                        // })
                        .or_else(|| {
                            //println!("\t\t\t\t... (no: maybe item at parent path?): [{}]", format_token_stream(parent_path));
                            source.maybe_scope_item_ref_obj_first(parent_path)
                                .map(|item| {
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                    item
                                })

                        })
                        .or_else(|| {
                            //println!("\t\t\t\t\t... (no maybe item at parent path + type path): [{}] + [{}]", format_token_stream(parent_path), format_token_stream(&ty_path));
                            let scope: Path = parse_quote!(#parent_path::#ty_path);
                            source.maybe_scope_item_ref_obj_first(&scope)
                                .map(|item| {
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                    item
                                })
                        })
                })
        }
        ScopeChain::Fn { info, parent_scope_chain, .. } => {
            // - Check fn scope
            // - if scope.parent is [mod | crate | impl] then lookup their child mods
            // - if scope.parent is [object | trait] then check scope.parent.parent
            source.maybe_scope_item_ref_obj_first(info.self_path())
                .or_else(|| match &**parent_scope_chain {
                    ScopeChain::CrateRoot { info, .. } |
                    ScopeChain::Mod { info, .. } => {
                        let parent_path = info.self_path();
                        let scope: Path = parse_quote!(#parent_path::#ty_path);
                        source.maybe_scope_item_ref_obj_first(&scope)
                            .map(|item| {
                                refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                item
                            })

                    },
                    ScopeChain::Trait { parent_scope_chain, .. } |
                    ScopeChain::Object { parent_scope_chain, .. } |
                    ScopeChain::Impl { parent_scope_chain, .. } => {
                        let parent_path = parent_scope_chain.self_path();
                        let scope: Path = parse_quote!(#parent_path::#ty_path);
                        source.maybe_scope_item_ref_obj_first(&scope)
                            .map(|item| {
                                refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                item
                            })

                    },
                    ScopeChain::Fn { .. } => {
                        // TODO: support nested function when necessary
                        //println!("nested function::: {} --- [{}]", info.self_scope, parent_scope_chain);
                        None
                    }
                })
        }
    }
}

pub fn refine_ty_with_import_path(ty: &mut Type, crate_named_import_path: &Path) -> bool {
    let mut refined = false;
    match ty {
        Type::Path(TypePath { path, .. }) => {
            *path = if let Some(last_segment) = path.segments.last() {
                let mut full_path_with_args = crate_named_import_path.clone();
                full_path_with_args.segments.last_mut().unwrap().arguments = last_segment.arguments.clone();
                full_path_with_args
            } else {
                crate_named_import_path.clone()
            };
            refined = true;
        }
        Type::Array(_) => {}
        Type::BareFn(_) => {}
        Type::Group(_) => {}
        Type::ImplTrait(_) => {}
        Type::Infer(_) => {}
        Type::Macro(_) => {}
        Type::Never(_) => {}
        Type::Paren(_) => {}
        Type::Ptr(_) => {}
        Type::Reference(_) => {}
        Type::Slice(_) => {}
        Type::TraitObject(_) => {}
        Type::Tuple(_) => {}
        Type::Verbatim(_) => {}
        _ => {}
    }
    refined
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
        } else if matches!(ident.to_string().as_str(), "i128") {
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(model.clone())))
        } else if matches!(ident.to_string().as_str(), "u128") {
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
        } else if ident.is_special_std_trait()  {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Other(model.clone())))
        } else if matches!(ident.to_string().as_str(), "FromIterator" | "From" | "Into" | "Sized") {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Other(model.clone())))
        } else if ident.is_map() || ident.is_special_generic() {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Other(model.clone())))
        } else if ident.is_box() {
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model.clone()))))
        // } else if ident.is_smart_ptr() {
        //     refine_ty_with_import_path(&mut model.ty, crate_named_import_path);
        //
        //     Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(model.clone())))
        } else {
            refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
            match ident.to_string().as_str() {
                "Arc" => Some(SmartPointerModelKind::Arc(model.clone())),
                "Mutex" => Some(SmartPointerModelKind::Mutex(model.clone())),
                "Pin" => Some(SmartPointerModelKind::Pin(model.clone())),
                "Rc" => Some(SmartPointerModelKind::Rc(model.clone())),
                "RefCell" => Some(SmartPointerModelKind::RefCell(model.clone())),
                "RwLock" => Some(SmartPointerModelKind::RwLock(model.clone())),
                _ => None
            }.map(|smart_ptr_model| {
                refine_ty_with_import_path(model.ty_mut(), crate_named_import_path);
                DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(smart_ptr_model))
            })
        }
    })
}

impl RefineInScope for TypeModelKind {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        // println!("REFINE --> {} \n\tin {}", self, scope.fmt_short());
        let result = match self {
            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) => false,
            TypeModelKind::Imported(ty_model, import_path) => {
                let crate_name = scope.crate_ident_as_path();
                let mut crate_named_import_path = import_path.crate_named(&crate_name);
                let mut model = ty_model.clone();
                let mut nested_args_refined = false;

                if !crate_named_import_path.refine_in_scope(scope, source) {
                    // Refine nested arguments first
                    model.nested_arguments_iter_mut()
                        .for_each(|nested_arg| {
                            if nested_arg.refine_in_scope(scope, source) {
                                nested_args_refined = true;
                            }
                        });
                }
                model.refine(&crate_named_import_path);

                if let Some(dictionary_type) = maybe_dict_type_model_kind(&crate_named_import_path, &mut model) {
                    //println!("[INFO] (Import) Dictionary item found: {}", dictionary_type);
                    *self = TypeModelKind::Dictionary(dictionary_type);

                } else {

                    let scope_path = model.lifetimes_cleaned().pointer_less();
                    if let Some(found_item) = source.maybe_scope_item_ref_obj_first(&crate_named_import_path)
                        .or_else(|| determine_scope_item(&mut model, scope_path, scope, source)) {
                        //println!("[INFO] (Import) Scope item found: {}", found_item);
                        refine_ty_with_import_path(model.ty_mut(), found_item.path());
                        if let Some(updated) = found_item.update_with(model) {
                            //println!("[INFO] (Import) Scope item refined: {}", updated);
                            *self = updated;
                        }
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
                    .or_else(|| determine_scope_item(model, path, scope, source)) {
                    //println!("[INFO] (Unknown) Scope item found: {}", found_item);
                    refine_ty_with_import_path(model.ty_mut(), found_item.path());
                    if let Some(updated) = found_item.update_with(model.clone()) {
                        //println!("[INFO] (Unknown) Scope item refined (Unknown): {}", updated);
                        *self = updated;
                    }
                    true
                } else {
                    println!("[WARN] (Unknown) Unknown import: {}", model.as_type().to_token_stream());
                    false
                }
            }
            TypeModelKind::Dictionary(
                DictTypeModelKind::NonPrimitiveFermentable(
                    DictFermentableModelKind::SmartPointer(
                        SmartPointerModelKind::Arc(model) |
                        SmartPointerModelKind::Box(model) |
                        SmartPointerModelKind::Rc(model) |
                        SmartPointerModelKind::Mutex(model) |
                        SmartPointerModelKind::RwLock(model) |
                        SmartPointerModelKind::RefCell(model) |
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
            TypeModelKind::Trait(model, ..) |
            TypeModelKind::TraitType(model) =>
                refine_nested_arguments(model, scope, source),
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
        let result = if let Some(refined_obj) = source.maybe_refined_object(scope, obj) {
            *obj = refined_obj;
            true
        } else {
            false
        };
        //println!("NestedArgument::refine_in_scope <-- {} \n\tin {}", self, scope.fmt_short());
        result
    }
}

impl RefineWithNestedArgs for Type {
    fn refine_with_nested_args(&mut self, nested_arguments: &CommaPunctuatedNestedArguments) -> bool {
        let mut did_refine = false;
        match self {
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                let mut refined_nested_args = nested_arguments.clone();
                inputs.iter_mut().for_each(|inner_ty| {
                    match refined_nested_args.pop() {
                        Some(refined_nested_arg) if inner_ty.refine_with_nested_arg(refined_nested_arg.value()) =>
                            did_refine = true,
                        _ => {}
                    }
                });
                match refined_nested_args.pop() {
                    Some(refined_nested_arg) if output.refine_with_nested_arg(refined_nested_arg.value()) =>
                        did_refine = true,
                    _ => {}
                }

            }
            Type::Path(TypePath { path, .. }) => {
                if let Some(last_segment) = path.segments.last_mut() {
                    if last_segment.arguments.refine_with_nested_args(nested_arguments) {
                        did_refine = true;
                    }
                }
            }
            Type::Tuple(type_tuple) => {
                type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| {
                    if elem.refine_with_nested_arg(&nested_arguments[index]) {
                        did_refine = true;
                    }
                });
            },
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                bounds.iter_mut().enumerate().for_each(|(index, elem)| {
                    match elem {
                        TypeParamBound::Trait(TraitBound { path, .. }) => {
                            if let Some(ty) = &nested_arguments[index].maybe_type() {
                                *path = ty.to_path();
                                did_refine = true;
                            }
                        }
                        _ => {}
                    }
                });
            }
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Array(TypeArray { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => {
                if let Some(refined_nested_arg) = &nested_arguments.first() {
                    if elem.refine_with_nested_arg(refined_nested_arg) {
                        did_refine = true;
                    }
                }
            },
            _ => {}
        }
        did_refine
    }
}

impl RefineWithNestedArgs for PathArguments {
    fn refine_with_nested_args(&mut self, nested_arguments: &CommaPunctuatedNestedArguments) -> bool {
        let mut did_refine = false;
        match self {
            PathArguments::None => {}
            PathArguments::Parenthesized(ParenthesizedGenericArguments { ref mut inputs, ref mut output, .. }) => {
                inputs.iter_mut().enumerate().for_each(|(index, inner_ty)| {
                    if inner_ty.refine_with_nested_arg(&nested_arguments[index]) {
                        did_refine = true;
                    }
                });
                if let Some(last) = nested_arguments.last() {
                    if output.refine_with_nested_arg(last) {
                        did_refine = true;
                    }
                }

            },
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { ref mut args, .. }) => {
                args.iter_mut()
                    .filter_map(|arg| {
                        match arg {
                            GenericArgument::Type(inner_ty) => Some(inner_ty),
                            _ => None
                        }
                    })
                    .enumerate()
                    .for_each(|(index, generic_argument)| {

                        // match generic_argument {
                        //     GenericArgument::Type(inner_ty) => {
                        //         if let Some(ty) = nested_argument.object().maybe_type() {
                        //             *inner_ty = ty;
                        //             true
                        //         } else {
                        //             false
                        //         }
                        //     },
                        //     _ => false
                        // }

                        if generic_argument.refine_with_nested_arg(&nested_arguments[index]) {
                            did_refine = true;
                        }
                    });
            }
        }
        did_refine
    }
}


/// Refinement of the actual types with refined nested arguments
/// Nested argument should be refined before
impl RefineWithNestedArg for GenericArgument {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        match self {
            GenericArgument::Type(inner_ty) => {
                if let Some(ty) = nested_argument.object().maybe_type() {
                    *inner_ty = ty;
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }
}

impl RefineWithNestedArg for BareFnArg {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
       self.ty.refine_with_nested_arg(nested_argument)
    }
}

impl RefineWithNestedArg for ReturnType {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        match self {
            ReturnType::Type(_, inner_ty) => {
                if let Some(ty) = nested_argument.maybe_type() {
                    *inner_ty = Box::new(ty);
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }
}

impl RefineWithNestedArg for Type {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        if let Some(ty) = nested_argument.maybe_type() {
            *self = ty;
            true
        } else {
            false
        }
    }
}


fn refine_nested_arguments(model: &mut TypeModel, scope: &ScopeChain, source: &GlobalContext) -> bool {
    let mut refined = false;
    model.nested_arguments_iter_mut()
        .for_each(|nested_arg| {
            if nested_arg.refine_in_scope(scope, source) {
                refined = true;
            }
        });
    if refined {
        model.refine_with(model.nested_arguments_ref().clone());
        // model.ty_mut().refine_with(model.nested_arguments_ref().clone());
    }
    refined
}
fn refine_nested_ty(new_ty_model: &mut TypeModel, scope: &ScopeChain, source: &GlobalContext) -> bool {
    let mut refined = false;
    let (ty, nested_arguments) = new_ty_model.type_model_and_nested_arguments_mut();
    match ty {
        Type::Tuple(type_tuple) => {
            type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| {
                let nested_arg = &mut nested_arguments[index];
                if nested_arg.refine_in_scope(scope, source) {
                    *elem = nested_arg.maybe_type().unwrap();
                    refined = true;
                }
            });
        },
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter_mut().enumerate().for_each(|(index, elem)| {
                match elem {
                    TypeParamBound::Trait(TraitBound { path, .. }) => {
                        let nested_arg = &mut nested_arguments[index];
                        if nested_arg.refine_in_scope(scope, source) {
                            *path = nested_arg.maybe_type().unwrap().to_path();
                            refined = true;
                        }
                    }
                    _ => {}
                }
            });
        }
        Type::Array(TypeArray { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) => {
            let nested_arg = &mut nested_arguments[0];
            if nested_arg.refine_in_scope(scope, source) {
                *elem = Box::new(nested_arg.maybe_type().unwrap());
                refined = true;
            }
        },
        _ => {
            // What about others like Reference?
        }
    }
    refined
}

fn refined_import(import_path: &Path, alias: &Path, source: &GlobalContext) -> Option<Path> {
    let last_import_segment = import_path.segments.last();
    let last_alias_segment = alias.segments.last();
    if last_import_segment.is_some() &&
        last_alias_segment.is_some() &&
        last_import_segment.unwrap().ident == last_alias_segment.unwrap().ident {
        let reexport = ReexportSeek::Absolute.maybe_reexport(import_path, source);
        // if reexport.is_some() {
        //     println!("[INFO] Re-export assigned:\n\t[{}]", format_token_stream(&reexport));
        // }
        reexport
    } else {
        None
    }
}


fn create_mod_chain(path: &Path) -> ScopeChain {
    // print!("create_mod_chain: {}", path.to_token_stream());
    let segments = &path.segments;

    let crate_ident = &segments.first().unwrap().ident;
    let self_scope = Scope::new(PathHolder::from(path), ObjectKind::Empty);
    let parent_chunks = path.popped();
    let parent_scope_chain = if parent_chunks.segments.len() > 1 {
        create_mod_chain(&parent_chunks)
    } else {
        ScopeChain::CrateRoot {
            info: ScopeInfo {
                attrs: vec![],
                crate_ident: crate_ident.clone(),
                self_scope: Scope {
                    self_scope: PathHolder(parent_chunks),
                    object: ObjectKind::Empty
                }
            }
        }
    };
    if segments.len() == 1 {
        ScopeChain::CrateRoot {
            info: ScopeInfo {
                attrs: vec![],
                crate_ident: crate_ident.clone(),
                self_scope
            }
        }
    } else {
        ScopeChain::Mod {
            info: ScopeInfo {
                attrs: vec![],
                crate_ident: crate_ident.clone(),
                self_scope,
            },
            parent_scope_chain: Box::new(parent_scope_chain.clone())
        }
    }
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

pub enum ReexportSeek {
    Absolute,
    Relative,
}

impl ReexportSeek {
    fn join_reexport(&self, import_path: &Path, scope_path: &Path, crate_name: &Ident, chunk: Option<&Path>) -> Colon2Punctuated<PathSegment> {
        match self {
            ReexportSeek::Absolute => {
                match (import_path.segments.first().unwrap().ident.to_string().as_str(), chunk) {
                    ("crate", Some(chunk_ref)) => {
                        let crate_name_chunk = crate_name.to_path();
                        let result = import_path.replaced_first_with_ident(&crate_name_chunk);
                        let new_segments_iter = result.segments.iter().skip(scope_path.segments.len());
                        let new_path: Path = parse_quote!(#(#new_segments_iter)::*);
                        merge_reexport_chunks(&new_path, chunk_ref).segments
                    },
                    ("crate", None) => {
                        let crate_name_chunk = crate_name.to_path();
                        import_path.replaced_first_with_ident(&crate_name_chunk)
                            .segments
                            .iter()
                            .skip(scope_path.segments.len())
                            .cloned()
                            .collect()
                    },
                    ("self", _) => {
                        import_path.segments.iter().skip(1).cloned().collect()
                    },
                    ("super", _) => {
                        // TODO: deal with "super::super::"
                        let super_path = scope_path.popped();
                        parse_quote!(#super_path::#import_path)
                    },
                    (_, Some(chunk_ref)) => {
                        let reexport_chunk = import_path.popped();
                        parse_quote!(#reexport_chunk::#chunk_ref)
                    }
                    (_, None) => {
                        parse_quote!(#import_path)
                    }
                }
            }
            ReexportSeek::Relative => {
                if chunk.is_some() {
                    let reexport_chunk = import_path.popped();
                    parse_quote!(#reexport_chunk::#chunk)
                } else {
                    parse_quote!(#import_path)
                }
            }
        }
    }
    pub(crate) fn maybe_reexport(&self, path: &Path, source: &GlobalContext) -> Option<Path> {
        // println!("... maybe_reexport: {}", format_token_stream(path));
        let mut candidate = path.clone();
        let mut result: Option<Path> = None;
        let mut chunk: Option<Path> = None;
        while let Some(last_segment) = candidate.segments.last().cloned() {
            candidate = candidate.popped();
            // println!("... reexport candidate: {} --- {}", format_token_stream(&last_segment), format_token_stream(&candidate));
            match source.maybe_import_scope_pair_ref(&last_segment, &candidate) {
                Some((scope, import)) => {
                    let scope_path = scope.self_path();
                    let segments = self.join_reexport(import, scope_path, scope.crate_ident_ref(), chunk.as_ref());
                    result = Some(parse_quote!(#scope_path::#segments));
                    // println!("... reexport found: {}", format_token_stream(&result));
                    chunk = Some(segments.to_path());
                }
                None => if candidate.segments.is_empty() {
                    return result;
                } else if let Some(reexport) = self.maybe_reexport(&candidate, source) {
                    result = Some(reexport);
                }
            }
        }
        result
    }
}

// Try to find the scope where item is actually defined
// assuming that 'path' is defined at 'scope' and can be shortened
#[allow(unused)]
pub(crate) fn maybe_closest_known_scope_for_import_in_scope<'a>(path: &'a Path, scope: &'a ScopeChain, source: &'a GlobalContext) -> Option<&'a ScopeChain> {
    // First assumption that it is relative import path
    let scope_path = scope.self_path();
    let mut closest_scope: Option<&ScopeChain> = None;

    let mut chunk = path.popped();
    while !chunk.segments.is_empty() {
        let candidate: Path = parse_quote!(#scope_path::#chunk);
        closest_scope = source.maybe_scope_ref(&candidate);
        if closest_scope.is_some() {
            return closest_scope;
        }
        chunk = chunk.popped();
    }
    chunk = path.popped();
    // Second assumption that it is global import path;
    while !chunk.segments.is_empty() {
        closest_scope = source.maybe_scope_ref(&chunk);
        if closest_scope.is_some() {
            return closest_scope;
        }
        chunk = chunk.popped();
    }
    None
}
