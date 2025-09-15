use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use indexmap::IndexMap;
use proc_macro2::{Spacing, TokenTree};
use quote::{quote, ToTokens};
use syn::{Attribute, Ident, ItemUse, Path, Signature, Type};
use crate::composable::{GenericBoundsModel, TraitModelPart1, TraitDecompositionPart1, TraitTypeModel};
use crate::context::{GlobalContext, ScopeChain, TypeChain};
use crate::kind::{MixinKind, ObjectKind};
use crate::tree::{ScopeTreeID, ScopeTreeExportItem, ScopeTreeItem};

#[allow(unused)]
pub fn format_imported_set(dict: &HashSet<ItemUse>) -> String {
    let debug_imports = dict.iter().map(|i| {
        i.to_token_stream()
    }).collect::<Vec<_>>();
    let all = quote!(#(#debug_imports,)*);
    all.to_string()
}

#[allow(unused)]
pub fn format_scope_refinement(dict: &[(ScopeChain, IndexMap<Type, ObjectKind>)]) -> String {
    let mut iter = dict.iter()
        .map(|(scope, types)|
            format!("\t{}: \n\t\t{}", format_token_stream(scope.self_path_ref()), types.iter().map(scope_type_conversion_pair).collect::<Vec<_>>()
                .join("\n\t")))
        .collect::<Vec<String>>();
    iter.sort();
    iter.join("\n")

}

#[allow(unused)]
pub fn format_types(dict: &HashSet<Type>) -> String {
    dict.iter()
        // .map(|item| format_token_stream(item))
        .map(|item| item.to_token_stream().to_string())
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn format_mixin_kinds(dict: &IndexMap<MixinKind, HashSet<Option<Attribute>>>) -> String {
    dict.iter()
        .map(|(item, attrs)| format!("{}:\t {}", item, format_unique_attrs(attrs)))
        .collect::<Vec<_>>()
        .join("\n\t")
}
#[allow(unused)]
pub fn format_mixin_conversions(dict: &IndexMap<GenericBoundsModel, HashSet<Option<Attribute>>>) -> String {
    dict.iter()
        .map(|(item, attrs)| format!("{}:\n\t {}", item, format_unique_attrs(attrs)))
        .collect::<Vec<_>>()
        .join("\n\t")
}

#[allow(unused)]
pub fn format_unique_attrs(dict: &HashSet<Option<Attribute>>) -> String {
    dict.iter()
        .map(|item| item.as_ref().map_or("[None]".to_string(), |a| a.to_token_stream().to_string()))
        .collect::<Vec<_>>()
        .join("\n\t")
}

pub fn format_attrs(dict: &[Attribute]) -> String {
    dict.iter()
        .map(|item| item.to_token_stream().to_string())
        .collect::<Vec<_>>()
        .join("\n\t")
}

#[allow(unused)]
pub fn format_imports(dict: &IndexMap<ScopeChain, IndexMap<Path, Path>>) -> String {
    let vec = scope_imports_dict(dict);
    let expanded = quote!(#(#vec),*);
    expanded.to_string()
}

#[allow(unused)]
pub fn format_tree_exported_dict(dict: &IndexMap<ScopeTreeID, ScopeTreeExportItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("{}: {}", ident, tree_item))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn format_tree_item_dict(dict: &IndexMap<ScopeTreeID, ScopeTreeItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("\t{}: {:?}", ident, tree_item))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn scope_type_conversion_pair(dict: (&Type, &ObjectKind)) -> String {
    format!("\t{}: {}", dict.0.to_token_stream(), dict.1)
    // format!("\t{}: {}", format_token_stream(dict.0), dict.1)
}

#[allow(unused)]
pub fn refinement_pair(dict: (&Type, &Vec<ObjectKind>)) -> String {
    format!("\t{}: \n\t\t{}", dict.0.to_token_stream(), dict.1.iter().map(|i| i.to_string()).collect::<Vec<_>>()
        .join("\n\t"))
    // format!("\t{}: {}", format_token_stream(dict.0), dict.1)
}
// #[allow(unused)]
// pub fn scope_refinement_pair(dict: &(&ScopeChain, HashMap<TypeHolder, ObjectKind>)) -> String {
//     format!("\t{}: \n\t\t{}", dict.0.to_token_stream(), dict.1.iter().map(scope_type_conversion_pair).collect::<Vec<_>>()
//         .join("\n\t"))
//     // format!("\t{}: {}", format_token_stream(dict.0), dict.1)
// }

#[allow(unused)]
pub fn ident_type_conversion_pair(dict: (&Ident, &Type)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), format_token_stream(dict.1))
}

#[allow(unused)]
pub fn ident_signature_conversion_pair(dict: (&Ident, &Signature)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), format_token_stream(dict.1))
}

#[allow(unused)]
pub fn ident_trait_type_decomposition_conversion_pair(dict: (&Ident, &TraitTypeModel)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), {
        let TraitTypeModel { ident, trait_bounds } = dict.1;
        quote!(#ident: [bounds: #(#trait_bounds)*])
    })
}
fn format_ident_path_pair(pair: (&Path, &Path)) -> String {
    format!("\t{}: {}", format_token_stream(pair.0), format_token_stream(pair.1))
}

pub fn format_path_vec(vec: &[Path]) -> String {
    vec.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(",")
}
pub fn format_obj_vec(vec: &[ObjectKind]) -> String {
    vec.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(",")
}

#[allow(unused)]
pub fn type_vec_path_conversion_pair(pair: (&Type, &Vec<Path>)) -> String {
    format!("\t{}: [{}]", format_token_stream(pair.0), format_path_vec(pair.1))
}
#[allow(unused)]
pub fn type_vec_obj_conversion_pair(pair: (&Type, &Vec<ObjectKind>)) -> String {
    format!("\t{}: [{}]", format_token_stream(pair.0), format_obj_vec(pair.1))
}
#[allow(unused)]
pub fn format_predicates_dict(vec: &HashMap<Type, Vec<Path>>) -> String {
    vec.iter()
        .map(type_vec_path_conversion_pair)
        .collect::<Vec<_>>()
        .join(",")
}
#[allow(unused)]
pub fn format_predicates_obj_dict(vec: &IndexMap<Type, Vec<ObjectKind>>) -> String {
    vec.iter()
        .map(type_vec_obj_conversion_pair)
        .collect::<Vec<_>>()
        .join(",")
}

#[allow(unused)]
fn format_generic_bounds_pair(pair: (&Type, &Vec<Path>)) -> String {
    format!("\t{}: [{}]", format_token_stream(pair.0), format_path_vec(pair.1))
}

fn format_ident_trait_pair(pair: (&Ident, &TraitModelPart1)) -> String {
    let implementors = &pair.1.implementors;
    format!("\t{}: {}: [{}]", format_token_stream(pair.0), "...", quote!(#(#implementors),*))
}

#[allow(unused)]
pub fn format_types_dict(dict: &IndexMap<Type, ObjectKind>) -> String {
    types_dict(dict)
        .join("\n")
}
#[allow(unused)]
pub fn format_types_to_refine(dict: &IndexMap<Type, Vec<ObjectKind>>) -> String {
    let mut iter = dict.iter()
        .map(refinement_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter.join("\n")
}

#[allow(unused)]
pub fn format_ident_types_dict(dict: &IndexMap<Ident, Type>) -> String {
    ident_types_dict(dict)
        .join("\n")
}

#[allow(unused)]
pub fn format_scope_types_dict(dict: &HashMap<ScopeChain, TypeChain>) -> String {
    dict.iter().map(|(scope, tc)| {
        format!("{}: \n\t{}", scope.fmt_short(), format_types_dict(&tc.inner))
    }).collect::<Vec<_>>()
        .join("\n")
}
//
#[allow(unused)]
pub fn format_used_traits(dict: &IndexMap<ScopeChain, IndexMap<Ident, TraitModelPart1>>) -> String {
    scope_traits_dict(dict).join("\n")
}

// fn format_fn_signature_decomposition(sig: &FnSignatureDecomposition) -> String {
//     let FnSignatureDecomposition { is_async, ident, scope, return_type, arguments } = sig;
// }

pub fn format_token_stream<TT: ToTokens>(token_stream: TT) -> String {
    let token_stream = token_stream.into_token_stream();
    let mut formatted_string = String::new();
    let mut space_needed = false;
    let mut inside_angle_brackets = 0;
    let mut inside_round_brackets = 0;
    // let mut inside_square_brackets = 0;
    let mut last_token_was_ampersand = false;
    let mut last_token_was_comma = false;
    // let mut last_token_was_sq_bracket = false;
    // let mut last_token_was_round_bracket = false;
    for token in token_stream {
        if last_token_was_comma {
            formatted_string.push(' ');
        }
        last_token_was_comma = false;
        match token {
            TokenTree::Ident(ident) => {
                // Check for 'mut' or lifetime after '&'
                if last_token_was_ampersand && (ident == "mut" || ident.to_string().starts_with('\'')) {
                    formatted_string.pop(); // Remove the space after '&'
                } else if space_needed {
                    formatted_string.push(' ');
                }
                formatted_string.push_str(&ident.to_string());
                space_needed = true;
                last_token_was_ampersand = false;
            }
            TokenTree::Punct(punct) => {
                match punct.as_char() {
                    ';' => {
                        formatted_string.push(';');
                        space_needed = true;
                    }
                    ':' => {
                        formatted_string.push(':');
                        space_needed = false;
                    }
                    '(' => {
                        inside_round_brackets += 1;
                        formatted_string.push('(');
                        space_needed = false;
                    }
                    ')' => {
                        inside_round_brackets -= 1;
                        formatted_string.push(')');
                        space_needed = true;
                    }
                    '<' => {
                        inside_angle_brackets += 1;
                        formatted_string.push('<');
                        space_needed = false;
                    }
                    '>' => {
                        inside_angle_brackets -= 1;
                        formatted_string.push('>');
                        space_needed = true;
                    }
                    ',' => {
                        formatted_string.push(',');
                        last_token_was_comma = true;
                        space_needed = true; // Add space after comma
                    }
                    '&' => {
                        formatted_string.push('&');
                        last_token_was_ampersand = true;
                        space_needed = false;
                    }
                    _ => {
                        if space_needed {
                            formatted_string.push(' ');
                        }
                        formatted_string.push(punct.as_char());
                        space_needed = punct.spacing() == Spacing::Alone;
                    }
                }
            }
            TokenTree::Literal(literal) => {
                if space_needed {
                    formatted_string.push(' ');
                }
                formatted_string.push_str(&literal.to_string());
                space_needed = true;
                last_token_was_ampersand = false;
            }
            TokenTree::Group(group) => {
                if space_needed && (inside_angle_brackets == 0 || inside_round_brackets == 0) {
                    formatted_string.push(' ');
                }
                formatted_string.push_str(&format_token_stream(group.stream()));
                space_needed = true;
                last_token_was_ampersand = false;
            }
        }
    }

    formatted_string
}

pub fn imports_dict(dict: &IndexMap<Path, Path>) -> Vec<String> {
    dict.iter()
        .map(format_ident_path_pair)
        .collect()
}

#[allow(unused)]
pub fn generic_bounds_dict(dict: &IndexMap<Type, Vec<Path>>) -> Vec<String> {
    dict.iter()
        .map(format_generic_bounds_pair)
        .collect()
}
#[allow(unused)]
pub fn format_generic_scope_chain(dict: &IndexMap<ObjectKind, Vec<ObjectKind>>) -> String {
    dict.iter()
        .map(|(bounded_ty, bounds)| format!("{}: {}", bounded_ty.to_token_stream(), format_obj_vec(bounds)))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn types_dict(dict: &IndexMap<Type, ObjectKind>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(scope_type_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}
fn ident_signatures_dict(dict: &IndexMap<Ident, Signature>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(ident_signature_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}


fn ident_trait_type_decomposition_dict(dict: &IndexMap<Ident, TraitTypeModel>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(ident_trait_type_decomposition_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

fn ident_types_dict(dict: &IndexMap<Ident, Type>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(ident_type_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

fn traits_dict(dict: &IndexMap<Ident, TraitModelPart1>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(format_ident_trait_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}


#[allow(unused)]
fn nested_scope_dict<K, K2, V2, F: Fn(&K, &IndexMap<K2, V2>) -> String>(dict: &IndexMap<K, IndexMap<K2, V2>>, mapper: F) -> Vec<String> {
    let mut iter = dict.iter()
        .map(|(key, value)| mapper(key, value))
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

fn format_scope_dict<K2, V2, F: Fn(&IndexMap<K2, V2>) -> Vec<String>>(dict: &IndexMap<ScopeChain, IndexMap<K2, V2>>, mapper: F) -> Vec<String>  {
    let mut iter = dict.iter()
        .filter_map(|(scope, sub_dict)| {
            let lines = mapper(sub_dict);
            (!lines.is_empty()).then(|| format!("\t{}:\n\t\t{}", scope.fmt_short(), lines.join("\n\t\t")))
        })
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

pub fn scope_imports_dict(dict: &IndexMap<ScopeChain, IndexMap<Path, Path>>) -> Vec<String> {
    format_scope_dict(dict, imports_dict)
}

pub fn scope_globs_dict(dict: &IndexMap<ScopeChain, Vec<Path>>) -> Vec<String> {
    let mut iter = Vec::from_iter(dict.iter()
        .filter_map(|(scope, glob_paths)|
            (!glob_paths.is_empty())
                .then(|| format!("\t{}:{}", scope.fmt_short(), Vec::from_iter(glob_paths.iter().map(|path| format!("\n\t\t\t{}::*", format_token_stream(path)))).join(",")))));
    iter.sort();
    iter
}

pub fn scope_materialized_globs_dict(dict: &IndexMap<ScopeChain, IndexMap<Path, Path>>) -> Vec<String> {
    let mut iter = Vec::from_iter(dict.iter()
        .filter_map(|(scope, materialized_map)|
            (!materialized_map.is_empty())
                .then(|| format!("\t{}:\n\t\t\t{}", scope.fmt_short(), Vec::from_iter(materialized_map.iter().map(|(alias, full_path)| format!("{} -> {}", format_token_stream(alias), format_token_stream(full_path)))).join("\n\t\t\t")))));
    iter.sort();
    iter
}

pub fn scope_resolved_imports_dict(dict: &HashMap<(ScopeChain, Path), Path>) -> Vec<String> {
    // Group by scope for better organization
    let mut scope_groups: HashMap<ScopeChain, Vec<(&Path, &Path)>> = HashMap::new();

    for ((scope, import_path), resolved_path) in dict {
        scope_groups.entry(scope.clone()).or_default().push((import_path, resolved_path));
    }

    let mut iter = Vec::from_iter(scope_groups.iter()
        .filter_map(|(scope, import_mappings)| {
            if import_mappings.is_empty() {
                return None;
            }

            let mut sorted_mappings = import_mappings.clone();
            sorted_mappings.sort_by_key(|(import_path, _)| format_token_stream(import_path));

            let formatted_mappings = sorted_mappings.iter()
                .map(|(import_path, resolved_path)| {
                    format!("{} ⇒ {}", format_token_stream(import_path), format_token_stream(resolved_path))
                })
                .collect::<Vec<_>>()
                .join("\n\t\t\t");

            Some(format!("\t{}:\n\t\t\t{}", scope.fmt_short(), formatted_mappings))
        }));

    iter.sort();
    iter
}

pub fn format_import_resolution_summary(context: &GlobalContext) -> Vec<String> {
    let mut summary = vec![];

    // Count statistics
    let direct_imports_count: usize = context.imports.inner.values().map(|m| m.len()).sum();
    let glob_imports_count: usize = context.imports.globs.values().map(|v| v.len()).sum();
    let materialized_count: usize = context.imports.materialized_globs.values().map(|m| m.len()).sum();
    let resolved_imports_count = context.imports.resolved_imports.len();

    summary.push("Import Statistics:".to_string());
    summary.push(format!("\t- Direct imports: {}", direct_imports_count));
    summary.push(format!("\t- Glob patterns: {}", glob_imports_count));
    summary.push(format!("\t- Materialized items: {}", materialized_count));
    summary.push(format!("\t- Resolved imports: {}", resolved_imports_count));

    // Show scopes with multiple import types
    let scopes_with_multiple_types: Vec<_> = context.imports.inner.keys()
        .filter(|&scope| {
            let has_direct = context.imports.inner.get(scope).map_or(false, |m| !m.is_empty());
            let has_globs = context.imports.globs.get(scope).map_or(false, |v| !v.is_empty());
            let has_materialized = context.imports.materialized_globs.get(scope).map_or(false, |m| !m.is_empty());
            [has_direct, has_globs, has_materialized].iter().filter(|&&x| x).count() > 1
        })
        .collect();

    if !scopes_with_multiple_types.is_empty() {
        summary.push(format!("Scopes with mixed import types: {}",
            scopes_with_multiple_types.len()));
    }

    summary
}

pub fn format_glob_resolution_chains(context: &GlobalContext) -> Vec<String> {
    let mut chains = vec![];

    // Show detailed resolution chains for each scope with globs
    for (scope, glob_bases) in &context.imports.globs {
        if !glob_bases.is_empty() {
            chains.push(format!("\t{}", scope.fmt_short()));

            for glob_base in glob_bases {
                chains.push(format!("\t\t└─ Glob: {}::*", format_token_stream(glob_base)));

                // Show what was materialized from this glob
                if let Some(materialized) = context.imports.materialized_globs.get(scope) {
                    let from_this_glob: Vec<_> = materialized.iter()
                        .filter(|(_, full_path)| {
                            // Check if this materialized item could have come from this glob base
                            full_path.segments.len() > glob_base.segments.len() &&
                            full_path.segments.iter().take(glob_base.segments.len())
                                .zip(glob_base.segments.iter())
                                .all(|(a, b)| a.ident == b.ident)
                        })
                        .collect();

                    if !from_this_glob.is_empty() {
                        chains.push(format!("\t\t\t├─ Materialized: {} items", from_this_glob.len()));
                        for (alias, full_path) in from_this_glob.iter().take(3) { // Show max 3 examples
                            chains.push(format!("\t\t\t│  ├─ {} -> {}",
                                format_token_stream(alias),
                                format_token_stream(full_path)));
                        }
                        if from_this_glob.len() > 3 {
                            chains.push(format!("\t\t\t│  └─ ... and {} more", from_this_glob.len() - 3));
                        }
                    } else {
                        chains.push("\t\t\t└─ No materialized items".to_string());
                    }
                } else {
                    chains.push("\t\t\t└─ No materialization found".to_string());
                }
            }
            chains.push("".to_string()); // Empty line between scopes
        }
    }

    chains
}

pub fn format_import_conflicts(context: &GlobalContext) -> Vec<String> {
    let mut conflicts = vec![];

    // Check for conflicts between direct imports and materialized globs
    for (scope, direct_imports) in &context.imports.inner {
        if let Some(materialized) = context.imports.materialized_globs.get(scope) {
            for (direct_alias, direct_path) in direct_imports {
                if let Some(glob_path) = materialized.get(direct_alias) {
                    if direct_path != glob_path {
                        conflicts.push(format!(
                            "CONFLICT in scope {}: {} resolves to both {} (direct) and {} (glob)",
                            scope.fmt_short(),
                            format_token_stream(direct_alias),
                            format_token_stream(direct_path),
                            format_token_stream(glob_path)
                        ));
                    }
                }
            }
        }
    }

    // Check for conflicts between multiple glob materializations (same name, different paths)
    for (scope, materialized) in &context.imports.materialized_globs {
        let mut name_to_paths = HashMap::<String, Vec<&Path>>::new();
        for (alias, full_path) in materialized {
            let alias_str = format_token_stream(alias);
            name_to_paths.entry(alias_str).or_default().push(full_path);
        }

        for (alias_name, paths) in name_to_paths {
            if paths.len() > 1 {
                conflicts.push(format!(
                    "AMBIGUOUS GLOB in scope {}: {} could resolve to: {}",
                    scope.fmt_short(),
                    alias_name,
                    paths.iter().map(|p| format_token_stream(p)).collect::<Vec<_>>().join(", ")
                ));
            }
        }
    }

    if !conflicts.is_empty() {
        conflicts.insert(0, "Import Resolution Conflicts:".to_string());
    }

    conflicts
}

#[allow(unused)]
pub fn scope_generics_dict(dict: &IndexMap<ScopeChain, IndexMap<Type, Vec<Path>>>) -> Vec<String> {
    format_scope_dict(dict, generic_bounds_dict)
}


fn scope_traits_dict(dict: &IndexMap<ScopeChain, IndexMap<Ident, TraitModelPart1>>) -> Vec<String> {
    format_scope_dict(dict, traits_dict)
}



fn traits_impl_dict(dict: &HashMap<ScopeChain, Vec<Path>>) -> Vec<String> {
    let mut iter = dict.iter()
        .filter_map(|(key, value)| {
            let scopes = quote!(#(#value),*);
            (!value.is_empty()).then(|| format!("\t{}:\n\t\t{}", format_token_stream(key), format_token_stream(&scopes)))
        })
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

fn format_complex_obj(vec: Vec<Vec<String>>) -> String {
    vec.into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n\t")
}

pub fn format_global_context(context: &GlobalContext) -> String {
    let mut sections: Vec<Vec<String>> = Vec::new();

    // Types: always include
    sections.push(vec!["\n-- types:".to_string(), context.scope_register.to_string()]);

    // Traits: include only if non-empty
    let traits = scope_traits_dict(&context.traits.inner);
    if !traits.is_empty() {
        sections.push(vec!["-- traits:".to_string()]);
        sections.push(traits);
    }

    // Traits impl: include only if non-empty
    let impls = traits_impl_dict(&context.traits.used_traits_dictionary);
    if !impls.is_empty() {
        sections.push(vec!["-- traits_impl:".to_string()]);
        sections.push(impls);
    }

    // Custom: include only if non-empty
    let custom_str = context.custom.to_string();
    if !custom_str.trim().is_empty() {
        sections.push(vec!["-- custom:".to_string(), custom_str]);
    }

    // Import Summary: always show if any imports exist
    let has_any_imports = !context.imports.inner.is_empty() ||
                         !context.imports.globs.is_empty() ||
                         !context.imports.materialized_globs.is_empty();
    if has_any_imports {
        sections.push(vec!["-- import_summary:".to_string()]);
        sections.push(format_import_resolution_summary(context));
    }

    // Direct Imports: include only if non-empty
    let imports = scope_imports_dict(&context.imports.inner);
    if !imports.is_empty() {
        sections.push(vec!["-- kind:".to_string()]);
        sections.push(imports);
    }

    // Glob Imports: include only if non-empty
    let globs = scope_globs_dict(&context.imports.globs);
    if !globs.is_empty() {
        sections.push(vec!["-- glob_imports:".to_string()]);
        sections.push(globs);
    }

    // Materialized Globs: include only if non-empty
    let materialized_globs = scope_materialized_globs_dict(&context.imports.materialized_globs);
    if !materialized_globs.is_empty() {
        sections.push(vec!["-- materialized_globs:".to_string()]);
        sections.push(materialized_globs);
    }

    // Resolved Imports: include only if non-empty
    let resolved_imports = scope_resolved_imports_dict(&context.imports.resolved_imports);
    if !resolved_imports.is_empty() {
        sections.push(vec!["-- resolved_imports:".to_string()]);
        sections.push(resolved_imports);
    }

    // Import Conflicts: always show if any exist
    let conflicts = format_import_conflicts(context);
    if !conflicts.is_empty() {
        sections.push(vec!["-- import_conflicts:".to_string()]);
        sections.push(conflicts);
    }

    // Detailed Glob Resolution Chains: only if debug environment variable is set
    if std::env::var("FERMENT_DEBUG_IMPORTS").is_ok() {
        let resolution_chains = format_glob_resolution_chains(context);
        if !resolution_chains.is_empty() {
            sections.push(vec!["-- glob_resolution_chains:".to_string()]);
            sections.push(resolution_chains);
        }
    }

    // Generics: include only if non-empty (and per-scope filtered above)
    let generics = scope_generics_dict(&context.generics.inner);
    if !generics.is_empty() {
        sections.push(vec!["-- generics:".to_string()]);
        sections.push(generics);
    }

    format_complex_obj(sections)
}

#[allow(unused)]
pub fn format_trait_decomposition_part1(dict: &TraitDecompositionPart1) -> String {
    format_complex_obj(vec![
        vec!["\n-- ident:".to_string()], vec![format_token_stream(&dict.ident)],
        vec!["-- consts:".to_string()], ident_types_dict(&dict.consts),
        vec!["-- methods:".to_string()], ident_signatures_dict(&dict.methods),
        vec!["-- types:".to_string()], ident_trait_type_decomposition_dict(&dict.types),
    ])
}

#[allow(dead_code)]
pub enum Emoji {
    Branch,
    Question,
    Local,
    Nothing,
    Ok,
    Error,
    Plus,
    Node,
    Folder,
    File
}

impl Display for Emoji {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(
        match self {
            Emoji::Question => '\u{2753}',
            Emoji::Branch => '\u{1D30E}',
            Emoji::Local => '\u{1F501}',
            Emoji::Ok => '\u{2705}',
            Emoji::Error => '\u{274C}',
            Emoji::Nothing => '\u{1F502}',
            Emoji::Plus => '\u{271A}',
            Emoji::Node => '\u{1F491}',
            Emoji::Folder => '\u{1f4c1}',
            Emoji::File => '\u{1f4c4}'
        })
    }
}

#[macro_export]
macro_rules! nprint {
    ($counter:expr, $emoji:expr, $($arg:tt)*) => {
        //println!("cargo:warning={}", format!("{}{} {}", " ".repeat($counter*2), $emoji, format!($($arg)*)))

        // log::warn!("{}", ansi_term::Colour::Green.paint(format!("{}{} {}", " ".repeat($counter*2), $emoji, format!($($arg)*))))
        //ansi_term::Colour::Green.paint(format!("{}{} {}", " ".repeat($counter*2), $emoji, format!($($arg)*)))
        //println!("{}{} {}", " ".repeat($counter*2), $emoji, format!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_phase {
    ($label:expr, $($arg:tt)*) => {
        println!("\n########################################################################################################################");
        println!("# {} {:?}", $label, std::time::SystemTime::now());
        println!("########################################################################################################################");
        println!("{}", format!($($arg)*));
        println!("########################################################################################################################\n");
    }
}
