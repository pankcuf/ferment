use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, parse_quote, Path, PathArguments, PathSegment, ReturnType, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeSlice, TypeTraitObject};
use crate::ast::{Colon2Punctuated, PathHolder};
use crate::composable::{GenericBoundComposition, NestedArgument, TypeComposition};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeInfo};
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{CrateExtension, Pop, RefineMut, ToPath};
use crate::formatter::format_token_stream;

#[allow(unused)]
pub trait RefineInScope {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool;
}

impl RefineInScope for GenericBoundComposition {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        let mut refined = false;
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

impl RefineInScope for ObjectConversion {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        match self {
            ObjectConversion::Type(tyc) =>
                tyc.refine_in_scope(scope, source),
            _ => false
        }
    }
}

impl RefineInScope for TypeCompositionConversion {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        match self {
            TypeCompositionConversion::Primitive(_) => false,
            TypeCompositionConversion::Imported(ty_composition, import_path) => {
                *self = refine_import_path(import_path, ty_composition, scope, source);
                true
            }
            TypeCompositionConversion::Unknown(composition) => {
                if let Some(known_item) = source.maybe_known_item(composition, scope) {
                    *self = known_item;
                    true
                } else {
                    false
                }
            }
            // TypeCompositionConversion::Boxed(composition) => {
            //     println!("REFINE BOXED.1: {}", composition);
            //
            //     // self.refine_nested(ty_composition, scope);
            //
            //     // let re
            //     //let conversion_replacement = self.refine_import_path(import_path, ty_composition, scope);
            //
            //     // println!("refine_nested.1: {} in {}", ty_composition, scope.fmt_short());
            //     // let mut new_ty_composition = ty_composition.clone();
            //     let mut refined: bool = false;
            //     composition.nested_arguments
            //         .iter_mut()
            //         .for_each(|arg| {
            //             let obj = arg.object_mut();
            //             let mut sub_refined: bool = false;
            //             match obj {
            //                 ObjectConversion::Type(tyc) |
            //                 ObjectConversion::Item(tyc, _) => {
            //                     let TypeComposition { ty, ..} = tyc.ty_composition().clone();
            //                     println!("REFINE BOXED NESTED: (HAS COMPO): {}", tyc);
            //                     // let nested_args = CommaPunctuatedNestedArguments::new();
            //                     match ty {
            //                         Type::TraitObject(TypeTraitObject { mut bounds, ..}) => {
            //
            //                             bounds.iter_mut().for_each(|b| match b {
            //                                 TypeParamBound::Trait(TraitBound { path, .. }) => {
            //                                     println!("REFINE BOXED NESTED: (CHECK BOUND): {}", path.to_token_stream());
            //                                     let mut refined_path = path.clone();
            //                                     sub_refined = refined_path.refine_in_scope(scope, source);
            //                                     // self.maybe_item(&refined_path)
            //                                     println!("REFINE BOXED NESTED: (REFINED COMPO): {}", refined_path.to_token_stream());
            //                                     *path = refined_path;
            //                                     // let refined = self.refine_import_path(path, ty_composition, scope);
            //                                     // *tyc = self.refine_import_path(path, ty_composition, scope);
            //                                 }
            //                                 TypeParamBound::Lifetime(_) => {}
            //                             })
            //                         }
            //                         _ => {}
            //                     }
            //                     if sub_refined {
            //
            //                     }
            //                 },
            //
            //                 ObjectConversion::Empty => {}
            //             }
            //             //
            //             if !sub_refined {
            //                 println!("REFINE BOXED NESTED: (CHECK): {}", obj);
            //                 if let Some(object_to_refine) = source.maybe_refined_object(scope, obj) {
            //                     println!("REFINE BOXED NESTED: (REFINED): {}", object_to_refine);
            //                     *obj = object_to_refine;
            //                     refined = true;
            //                 }
            //             }
            //             refined = sub_refined;
            //         });
            //     if refined {
            //         composition.ty.refine_with(composition.nested_arguments.clone());
            //     }
            //     println!("REFINE BOXED.2: {}", composition);
            //     refined
            // }
            TypeCompositionConversion::Boxed(composition) |
            TypeCompositionConversion::FnPointer(composition) |
            TypeCompositionConversion::LambdaFn(composition) |
            TypeCompositionConversion::Object(composition) |
            TypeCompositionConversion::Optional(composition) |
            // TypeCompositionConversion::SmartPointer(ty_composition) |
            TypeCompositionConversion::Trait(composition, ..) |
            TypeCompositionConversion::TraitType(composition) =>
                refine_nested_arguments(composition, scope, source),
            TypeCompositionConversion::Array(composition) |
            TypeCompositionConversion::Slice(composition) |
            TypeCompositionConversion::Tuple(composition) =>
                refine_nested_ty(composition, scope, source),
            TypeCompositionConversion::Bounds(composition) =>
                composition.refine_in_scope(scope, source),
            TypeCompositionConversion::Fn(_) |
            TypeCompositionConversion::LocalOrGlobal(_) => {
                // TODO: global generic?
                false
            }
        }
    }
}

impl RefineInScope for NestedArgument {
    fn refine_in_scope(&mut self, scope: &ScopeChain, source: &GlobalContext) -> bool {
        let obj = self.object_mut();
        if let Some(refined) = source.maybe_refined_object(scope, obj) {
            *obj = refined;
            true
        } else {
            false
        }
    }
}


fn refine_nested_arguments(composition: &mut TypeComposition, scope: &ScopeChain, source: &GlobalContext) -> bool {
    let mut refined = false;
    composition.nested_arguments
        .iter_mut()
        .for_each(|arg| {
            if arg.refine_in_scope(scope, source) {
                refined = true;
            }
        });
    if refined {
        composition.ty.refine_with(composition.nested_arguments.clone());
    }
    refined
}
fn refine_nested_ty(new_ty_composition: &mut TypeComposition, scope: &ScopeChain, source: &GlobalContext) -> bool {
    let mut refined = false;
    match &mut new_ty_composition.ty {
        Type::Tuple(type_tuple) => {
            type_tuple.elems.iter_mut().enumerate().for_each(|(index, elem)| {
                let nested_arg = &mut new_ty_composition.nested_arguments[index];
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
                        let nested_arg = &mut new_ty_composition.nested_arguments[index];
                        if nested_arg.refine_in_scope(scope, source) {
                            *path = nested_arg.maybe_type().unwrap().to_path();
                            refined = true;
                        }
                    }
                    TypeParamBound::Lifetime(_) => {}
                }
            });
        }
        Type::Array(TypeArray { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) => {
            if let Some(nested_arg) = new_ty_composition.nested_arguments.first_mut() {
                if nested_arg.refine_in_scope(scope, source) {
                    *elem = Box::new(nested_arg.maybe_type().unwrap());
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
fn refine_import_path(import_path: &Path, ty_composition: &TypeComposition, scope: &ScopeChain, source: &GlobalContext) -> TypeCompositionConversion {
    let mut import_type_path: TypePath = parse_quote!(#import_path);
    let crate_name = scope.crate_ident_as_path();
    import_type_path.path = import_path.crate_named(&crate_name);
    let mut ty_replacement = ty_composition.clone();
    if !import_type_path.path.refine_in_scope(scope, source) {
        maybe_refine_args(import_type_path.path.segments.last_mut().unwrap(), &mut ty_replacement.nested_arguments, scope, source);
    }
    let dict_path = import_type_path.path.clone();
    ty_replacement.ty = Type::Path(import_type_path);
    if let Some(dictionary_type) = scope.maybe_dictionary_composition(&dict_path, source) {
        dictionary_type
    } else if let Some(found_item) = source.maybe_known_item(&ty_replacement, scope) {
        // println!("[INFO] Known item found: [{}]", found_item.to_token_stream());
        found_item
    } else {
        println!("[WARN] Unknown import: [{}]", ty_replacement.ty.to_token_stream());
        TypeCompositionConversion::Unknown(ty_replacement)
    }
}
fn maybe_refine_args(segment: &mut PathSegment, nested_arguments: &mut CommaPunctuatedNestedArguments, scope: &ScopeChain, source: &GlobalContext) -> bool {
    // println!("maybe_refine_args::: {} ---- {:?}", segment.to_token_stream(), nested_arguments);
    let mut refined = false;
    match &mut segment.arguments {
        PathArguments::None => {
            // TODO: what if it's actually lambda?
            if !nested_arguments.is_empty() {
                // Nested args here can be unrefined if their owner is not refined
                segment.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: nested_arguments.into_iter().map(|nested_arg| {
                        nested_arg.refine_in_scope(scope, source);
                        GenericArgument::Type(nested_arg.maybe_type().unwrap())
                    }).collect(),
                    gt_token: Default::default(),
                });
                refined = true;
            }
        }
        PathArguments::Parenthesized(ParenthesizedGenericArguments { ref mut inputs, ref mut output, .. }) => {
            // panic!("Parenthesized args: {} -> {}", inputs.to_token_stream(), output.to_token_stream())
            inputs.iter_mut().for_each(|inner_ty| match nested_arguments.pop() {
                None => {}
                Some(nested_arg) => match nested_arg.into_value() {
                    NestedArgument::Object(obj) => {
                        *inner_ty = obj.maybe_type().unwrap();
                        refined = true;
                    },
                    NestedArgument::Constraint(obj) => {
                        *inner_ty = obj.maybe_type().unwrap();
                        refined = true;
                    },
                }
            });
            match output {
                ReturnType::Default => {}
                ReturnType::Type(_, inner_ty) => match nested_arguments.pop() {
                    None => {}
                    Some(nested_arg) => {
                        match nested_arg.into_value() {
                            NestedArgument::Object(obj) =>
                                *inner_ty = Box::new(obj.maybe_type().unwrap()),
                            NestedArgument::Constraint(obj) =>
                                *inner_ty = Box::new(obj.maybe_type().unwrap()),
                        }
                        refined = true;
                    }
                }
            }
        },
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { ref mut args, .. }) => {
            args.iter_mut()
                .for_each(|arg| match arg {
                    GenericArgument::Type(inner_ty) => match nested_arguments.pop() {
                        None => {}
                        Some(nested_arg) => {
                            match nested_arg.into_value() {
                                NestedArgument::Object(obj) => *inner_ty = obj.maybe_type().unwrap(),
                                NestedArgument::Constraint(obj) => *inner_ty = obj.maybe_type().unwrap(),
                            }
                            refined = true;
                        }
                    }
                    GenericArgument::Lifetime(_) => {}
                    GenericArgument::Const(_) => {}
                    GenericArgument::Binding(_) => {}
                    GenericArgument::Constraint(_) => {}
                });
        }
    };
    refined
}

// fn refine_trait_bounds(&self, bounds: &mut AddPunctuated<TypeParamBound>, scope: &ScopeChain) -> bool {
//     let mut refined = false;
//     bounds.iter_mut().for_each(|b| match b {
//         TypeParamBound::Trait(TraitBound { path, .. }) => {
//             println!("REFINE TRAIT BOUND: (CHECK): {}", path.to_token_stream());
//             let mut refined_path = path.clone();
//             refined = refined_path.refine_in_scope(scope, self);
//             println!("REFINE TRAIT BOUND: (REFINED): {}", refined_path.to_token_stream());
//             *path = refined_path;
//         }
//         TypeParamBound::Lifetime(_) => {}
//     });
//     refined
// }

fn refined_import(import_path: &Path, alias: &Path, source: &GlobalContext) -> Option<Path> {
    let last_import_segment = import_path.segments.last();
    let last_alias_segment = alias.segments.last();
    if last_import_segment.is_some() &&
        last_alias_segment.is_some() &&
        last_import_segment.unwrap().ident == last_alias_segment.unwrap().ident {
        let reexport = maybe_reexport(import_path, source);
        if reexport.is_some() {
            println!("[INFO] Re-export assigned:\n\t[{}]", format_token_stream(&reexport));
        }
        reexport
    } else {
        None
    }
}


fn create_mod_chain(path: &Path) -> ScopeChain {
    // print!("create_mod_chain: {}", path.to_token_stream());
    let segments = &path.segments;

    let crate_ident = &segments.first().unwrap().ident;
    let self_scope = Scope::new(PathHolder::from(path), ObjectConversion::Empty);
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
                    object: ObjectConversion::Empty
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
fn maybe_reexport(import_path: &Path, source: &GlobalContext) -> Option<Path> {
    let mut scope_path_candidate = import_path.clone();
    let mut result: Option<Path> = None;
    let mut chunk: Option<Path> = None;
    while let Some(scope_path_last_segment) = import_path.segments.last() {
        scope_path_candidate = scope_path_candidate.popped();
        match source.maybe_imports_scope(&scope_path_candidate) {
            Some(reexport_scope) => {
                let path: PathHolder = parse_quote!(#scope_path_last_segment);
                match source.maybe_import(reexport_scope, &path) {
                    Some(reexport_import) => {
                        let reexport_scope_path = reexport_scope.self_path_holder_ref();
                        // println!("[INFO] Re-export found: \n\t[{}] +\n\t[{}]\n\t[{}]",
                        //          format_token_stream(reexport_scope_path),
                        //          format_token_stream(reexport_import),
                        //          format_token_stream(&chunk));
                        let segments: Colon2Punctuated<PathSegment> = match (reexport_import.segments.first().unwrap().ident.to_string().as_str(), chunk.as_ref()) {
                            ("crate", Some(chunk_ref)) => {
                                let crate_name_chunk = reexport_scope.crate_ident().to_path();
                                let result = reexport_import.replaced_first_with_ident(&crate_name_chunk);
                                let new_segments_iter = result.segments.iter().skip(reexport_scope_path.len());
                                let new_path: Path = parse_quote!(#(#new_segments_iter)::*);
                                let re_result = merge_reexport_chunks(&new_path, chunk_ref);
                                parse_quote!(#re_result)
                            },
                            ("crate", None) => {
                                let crate_name_chunk = reexport_scope.crate_ident().to_path();
                                reexport_import.replaced_first_with_ident(&crate_name_chunk)
                                    .segments
                                    .iter()
                                    .skip(reexport_scope_path.len())
                                    .cloned()
                                    .collect()
                            },
                            ("self", _) => {
                                reexport_import.segments.iter().skip(1).cloned().collect()
                            },
                            ("super", _) => {
                                let super_path = reexport_scope_path.popped();
                                parse_quote!(#super_path::#reexport_import)
                            },
                            (_, Some(chunk_ref)) => {
                                let reexport_chunk = reexport_import.popped();
                                parse_quote!(#reexport_chunk::#chunk_ref)
                            }
                            (_, None) => {
                                parse_quote!(#reexport_import)
                            }
                        };
                        result = Some(parse_quote!(#reexport_scope_path::#segments));
                        chunk = Some(Path { segments, leading_colon: None });
                    },
                    None => {
                        if scope_path_candidate.segments.is_empty() {
                            return result;
                        } else if let Some(reexport) = maybe_reexport(&scope_path_candidate, source) {
                            result = Some(reexport);
                        }
                    }
                }
            },
            None => {
                if scope_path_candidate.segments.is_empty() {
                    return result;
                } else if let Some(reexport) = maybe_reexport(&scope_path_candidate, source) {
                    result = Some(reexport);
                }
            }
        }
    }
    result
}
