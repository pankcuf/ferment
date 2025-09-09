use proc_macro2::Ident;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, BareFnArg, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use crate::ast::Colon2Punctuated;
use crate::composable::{GenericBoundsModel, NestedArgument, TraitModel, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeInfo};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, CRATE, DictionaryType, LifetimeProcessor, Pop, RefineMut, SELF, SUPER, ToPath, Join, PathTransform, CrateBased, ToPathSepSegments};
use crate::ext::maybe_generic_type::MaybeGenericType;

#[allow(unused)]
pub trait RefineInScope {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool;
}
#[allow(unused)]
pub trait RefineWithNestedArg {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool;
}
pub trait RefineWithNestedArgs {
    fn refine_with_nested_args(&mut self, nested_arguments: &CommaPunctuatedNestedArguments) -> bool;
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
            let child_scope = self_path.joined(&ty_path);
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

            let child_scope = parent_path.joined(&ty_path);
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
                            let scope = parent_path.joined(&ty_path);
                            source.maybe_scope_item_ref_obj_first(&scope)
                                .map(|item| {
                                    refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                    item
                                })
                        })
                })
        }
        ScopeChain::Fn { info, parent, .. } => {
            // - Check fn scope
            // - if scope.parent is [mod | crate | impl] then lookup their child mods
            // - if scope.parent is [object | trait] then check scope.parent.parent
            source.maybe_scope_item_ref_obj_first(info.self_path())
                .or_else(|| match &**parent {
                    ScopeChain::CrateRoot { info, .. } |
                    ScopeChain::Mod { info, .. } => {
                        let scope = info.self_path().joined(&ty_path);
                        source.maybe_scope_item_ref_obj_first(&scope)
                            .map(|item| {
                                refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                item
                            })

                    },
                    ScopeChain::Trait { parent, .. } |
                    ScopeChain::Object { parent, .. } |
                    ScopeChain::Impl { parent, .. } => {
                        let scope = parent.self_path_ref().joined(&ty_path);
                        source.maybe_scope_item_ref_obj_first(&scope)
                            .map(|item| {
                                refine_ty_with_import_path(new_ty_to_replace.ty_mut(), item.path());
                                item
                            })

                    },
                    ScopeChain::Fn { .. } => {
                        // TODO: support nested function when necessary
                        //println!("nested function::: {} --- [{}]", info.self_scope, parent);
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
            *path = if let Some(last_popped_segment) = path.segments.last() {
                let mut full_path_with_args = crate_named_import_path.clone();
                if let Some(last_segment) = full_path_with_args.segments.last_mut() {
                    last_segment.arguments = last_popped_segment.arguments.clone();
                }
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
        } else if ident.is_cow() {
            Some(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(model.clone())))
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
                "Cell" => Some(SmartPointerModelKind::Cell(model.clone())),
                "RefCell" => Some(SmartPointerModelKind::RefCell(model.clone())),
                "UnsafeCell" => Some(SmartPointerModelKind::UnsafeCell(model.clone())),
                "OnceLock" => Some(SmartPointerModelKind::OnceLock(model.clone())),
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
            TypeModelKind::Trait(TraitModel { ty: model, ..}) =>
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
                type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| if elem.refine_with_nested_arg(&nested_arguments[index]) {
                    did_refine = true;
                });
            },
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                bounds.iter_mut().enumerate().for_each(|(index, elem)| if let TypeParamBound::Trait(TraitBound { path, .. }) = elem {
                    if let Some(ty) = &nested_arguments[index].maybe_type() {
                        *path = ty.to_path();
                        did_refine = true;
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
                inputs.iter_mut().enumerate().for_each(|(index, inner_ty)| if inner_ty.refine_with_nested_arg(&nested_arguments[index]) {
                    did_refine = true;
                });
                if let Some(last) = nested_arguments.last() {
                    if output.refine_with_nested_arg(last) {
                        did_refine = true;
                    }
                }
            },
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { ref mut args, .. }) =>
                args.iter_mut()
                    .filter_map(GenericArgument::maybe_generic_type_mut)
                    .enumerate()
                    .for_each(|(index, arg)| if arg.refine_with_nested_arg(&nested_arguments[index]) {
                        did_refine = true;
                    })
        }
        did_refine
    }
}


/// Refinement of the actual types with refined nested arguments
/// Nested argument should be refined before
impl RefineWithNestedArg for GenericArgument {
    fn refine_with_nested_arg(&mut self, nested_argument: &NestedArgument) -> bool {
        self.maybe_generic_type_mut().map(|inner_ty| if let Some(ty) = nested_argument.object().maybe_type() {
            *inner_ty = ty;
            true
        } else {
            false
        }).unwrap_or_default()
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
        .for_each(|nested_arg| if nested_arg.refine_in_scope(scope, source) {
            refined = true;
        });
    if refined {
        model.refine_with(model.nested_arguments_ref().clone());
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
                    if let Some(maybe_nested_type) = nested_arg.maybe_type() {
                        *elem = maybe_nested_type;
                        refined = true;
                    }
                }
            });
        },
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter_mut().enumerate().for_each(|(index, elem)| match elem {
                TypeParamBound::Trait(TraitBound { path, .. }) => {
                    let nested_arg = &mut nested_arguments[index];
                    if nested_arg.refine_in_scope(scope, source) {
                        if let Some(maybe_nested_type) = nested_arg.maybe_type() {
                            *path = maybe_nested_type.to_path();
                            refined = true;
                        }
                    }
                }
                _ => {}
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

fn refined_import(import_path: &Path, alias: &Path, source: &GlobalContext) -> Option<Path> {
    match (import_path.segments.last(), alias.segments.last()) {
        (Some(PathSegment { ident: import_ident, .. }), Some(PathSegment { ident: alias_ident, .. })) if import_ident == alias_ident =>
            ReexportSeek::Absolute.maybe_reexport(import_path, source),
        _ => None
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
        ScopeChain::root(ScopeInfo::attr_less(crate_ident.clone(), Scope::empty(parent_chunks)))
    };
    let info = ScopeInfo::attr_less(crate_ident.clone(), self_scope);
    if segments.len() == 1 {
        ScopeChain::root(info)
    } else {
        ScopeChain::r#mod(info, parent)
    }
}
fn merge_reexport_chunks(mut base: Path, extension: &Path) -> Path {
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
    let base_len = base_segments.len();
    result_segments.extend(base_segments.into_iter().take(base_len - skip));
    result_segments.extend(ext_segments.into_iter());
    base.segments = result_segments.into_iter().cloned().collect();
    base
}

pub enum ReexportSeek {
    Absolute,
    Relative,
}

impl ReexportSeek {
    fn join_reexport(&self, import_path: &Path, scope_path: &Path, crate_name: &Ident, chunk: Option<&Path>) -> Path {
        // TODO: deal with "super::super::"
        match self {
            ReexportSeek::Absolute => if let Some(PathSegment { ident, .. }) = import_path.segments.first() {
                match (ident.to_string().as_str(), chunk) {
                    (CRATE, Some(chunk_ref)) =>
                        merge_reexport_chunks(Colon2Punctuated::from_iter(import_path.replaced_first_with(&crate_name.to_path())
                            .segments
                            .into_iter()
                            .skip(scope_path.segments.len()))
                            .to_path(), chunk_ref),
                    (CRATE, None) =>
                        Colon2Punctuated::from_iter(import_path.segments
                            .replaced_first_with(&crate_name.to_segments())
                            .iter()
                            .skip(scope_path.segments.len())
                            .cloned())
                            .to_path(),
                    (SELF, _) =>
                        Colon2Punctuated::from_iter(import_path.segments
                            .iter()
                            .skip(1)
                            .cloned())
                            .to_path(),
                    (SUPER, _) =>
                        scope_path.popped()
                            .joined(import_path),
                    (_, Some(chunk_ref)) =>
                        import_path.popped()
                            .joined(chunk_ref),
                    (_, None) =>
                        import_path.clone()
                }
            } else {
                import_path.clone()
            }
            ReexportSeek::Relative => if let Some(chunk) = chunk {
                import_path.popped().joined(chunk)
            } else {
                import_path.clone()
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
                    let scope_path = scope.self_path_ref();
                    let reexport_path = self.join_reexport(import, scope_path, scope.crate_ident_ref(), chunk.as_ref());
                    result = Some(scope_path.joined(&reexport_path));
                    chunk = Some(reexport_path);
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
    let scope_path = scope.self_path_ref();
    let mut closest_scope: Option<&ScopeChain> = None;

    let mut chunk = path.popped();
    while !chunk.segments.is_empty() {
        let candidate = scope_path.joined(&chunk);
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
