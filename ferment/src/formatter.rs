use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use proc_macro2::{Spacing, TokenTree};
use quote::{quote, ToTokens};
use syn::{Ident, Path, Signature, Type};
use crate::chunk::InitialType;
use crate::composition::{GenericConversion, ImportComposition, TraitDecompositionPart1, TraitTypeDecomposition};
use crate::context::{GlobalContext, TraitCompositionPart1};
use crate::conversion::{ImportConversion, TypeConversion};
use crate::holder::{PathHolder, TypeHolder};
use crate::tree::{ScopeTreeExportItem, ScopeTreeItem};

#[allow(unused)]
pub fn format_imported_dict(dict: &HashMap<ImportConversion, HashSet<ImportComposition>>) -> String {
    let debug_imports = dict.iter().map(|(i, p)| {
        let import = i.as_path();
        let ppp = p.iter().map(|ImportComposition { ident: i, scope: p}| quote!(#i: #p)).collect::<Vec<_>>();
        quote!(#import: #(#ppp,)*)
    }).collect::<Vec<_>>();
    let all = quote!(#(#debug_imports,)*);
    all.to_string()
}

#[allow(unused)]
pub fn format_generic_conversions(dict: &HashSet<GenericConversion>) -> String {
    dict.iter()
        .map(|item| format_token_stream(&item.0))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn format_imports(dict: &HashMap<PathHolder, HashMap<PathHolder, Path>>) -> String {
    let vec = scope_imports_dict(dict);
    let expanded = quote!(#(#vec),*);
    expanded.to_string()
}

#[allow(unused)]
pub fn format_tree_exported_dict(dict: &HashMap<Ident, ScopeTreeExportItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("{}:\n{}", ident, tree_item))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn format_tree_item_dict(dict: &HashMap<Ident, ScopeTreeItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("\t{}: {:?}", ident, quote!(#tree_item)))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn scope_type_conversion_pair(dict: (&TypeHolder, &TypeConversion)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), dict.1)
}

#[allow(unused)]
pub fn ident_type_conversion_pair(dict: (&Ident, &Type)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), format_token_stream(dict.1))
}

#[allow(unused)]
pub fn ident_signature_conversion_pair(dict: (&Ident, &Signature)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), format_token_stream(dict.1))
}

#[allow(unused)]
pub fn ident_trait_type_decomposition_conversion_pair(dict: (&Ident, &TraitTypeDecomposition)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), {
        let TraitTypeDecomposition { ident, trait_bounds } = dict.1;
        quote!(#ident: #(#trait_bounds)*)
    })
}

#[allow(unused)]
pub fn scope_chunk_conversion_pair(dict: (&InitialType, &Type)) -> String {
    format!("\t{}: {}", format_token_stream(dict.0), format_token_stream(dict.1))
}
fn format_ident_path_pair(pair: (&PathHolder, &Path)) -> String {
    format!("\t{}: {}", format_token_stream(pair.0), format_token_stream(pair.1))
}

fn format_generic_bounds_pair(pair: (&PathHolder, &Vec<Path>)) -> String {

    let v: Vec<_> = pair.1.iter().map(|p| format_token_stream(p)).collect();
    format!("\t{}: [{}]", format_token_stream(pair.0), v.join(","))
}

fn format_ident_trait_pair(pair: (&Ident, &TraitCompositionPart1)) -> String {
    let implementors = &pair.1.implementors;
    format!("\t{}: {}: [{}]", format_token_stream(pair.0), "...", quote!(#(#implementors),*))
}

#[allow(unused)]
pub fn format_chunks_dict(dict: &HashMap<InitialType, Type>) -> String {
    chunks_dict(dict)
        .join("\n")
}

#[allow(unused)]
pub fn format_types_dict(dict: &HashMap<TypeHolder, TypeConversion>) -> String {
    types_dict(dict)
        .join("\n")
}

#[allow(unused)]
pub fn format_ident_types_dict(dict: &HashMap<Ident, Type>) -> String {
    ident_types_dict(dict)
        .join("\n")
}

#[allow(unused)]
pub fn format_scope_types_dict(dict: &HashMap<PathHolder, HashMap<TypeHolder, TypeConversion>>) -> String {
    scope_types_dict(dict)
        .join("\n\n")
}

#[allow(unused)]
pub fn format_used_traits(dict: &HashMap<PathHolder, HashMap<Ident, TraitCompositionPart1>>) -> String {
    scope_traits_dict(dict).join("\n")
}

// fn format_fn_signature_decomposition(sig: &FnSignatureDecomposition) -> String {
//     let FnSignatureDecomposition { is_async, ident, scope, return_type, arguments } = sig;
// }

pub fn format_token_stream<TT: ToTokens>(token_stream: TT) -> String {
    // println!("format_token_stream2222: {}", token_stream.to_token_stream());
    let token_stream = token_stream.into_token_stream();
    let mut formatted_string = String::new();
    let mut space_needed = false;
    let mut inside_angle_brackets = 0;
    let mut last_token_was_ampersand = false;
    let mut last_token_was_comma = false;
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
                if space_needed && inside_angle_brackets == 0 {
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



/// Helpers

fn imports_dict(dict: &HashMap<PathHolder, Path>) -> Vec<String> {
    dict.iter()
        .map(format_ident_path_pair)
        .collect()
}

fn generic_bounds_dict(dict: &HashMap<PathHolder, Vec<Path>>) -> Vec<String> {
    dict.iter()
        .map(format_generic_bounds_pair)
        .collect()
}

fn chunks_dict(dict: &HashMap<InitialType, Type>) -> Vec<String> {
    dict.iter()
        .map(scope_chunk_conversion_pair)
        .collect()
}

fn types_dict(dict: &HashMap<TypeHolder, TypeConversion>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(scope_type_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}
fn ident_signatures_dict(dict: &HashMap<Ident, Signature>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(ident_signature_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}


fn ident_trait_type_decomposition_dict(dict: &HashMap<Ident, TraitTypeDecomposition>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(ident_trait_type_decomposition_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

fn ident_types_dict(dict: &HashMap<Ident, Type>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(ident_type_conversion_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

fn traits_dict(dict: &HashMap<Ident, TraitCompositionPart1>) -> Vec<String> {
    let mut iter = dict.iter()
        .map(format_ident_trait_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter
}


fn nested_scope_dict<K, K2, V2, F: Fn(&K, &HashMap<K2, V2>) -> String>(dict: &HashMap<K, HashMap<K2, V2>>, mapper: F) -> Vec<String> {
    let mut iter = dict.iter()
        .map(|(key, value)| mapper(key, value))
        .collect::<Vec<String>>();
    iter.sort();
    iter
}

fn format_scope_dict<K2, V2, F: Fn(&HashMap<K2, V2>) -> Vec<String>>(dict: &HashMap<PathHolder, HashMap<K2, V2>>, mapper: F) -> Vec<String>  {
    nested_scope_dict(dict, |scope, sub_dict|
        format!("\t{}:\n\t\t{}", scope, mapper(sub_dict).join("\n\t\t")))
}

pub fn scope_imports_dict(dict: &HashMap<PathHolder, HashMap<PathHolder, Path>>) -> Vec<String> {
    format_scope_dict(dict, imports_dict)
}

pub fn scope_generics_dict(dict: &HashMap<PathHolder, HashMap<PathHolder, Vec<Path>>>) -> Vec<String> {
    format_scope_dict(dict, generic_bounds_dict)
}

fn scope_types_dict(dict: &HashMap<PathHolder, HashMap<TypeHolder, TypeConversion>>) -> Vec<String> {
    format_scope_dict(dict, types_dict)
}

fn scope_traits_dict(dict: &HashMap<PathHolder, HashMap<Ident, TraitCompositionPart1>>) -> Vec<String> {
    format_scope_dict(dict, traits_dict)
}

fn traits_impl_dict(dict: &HashMap<PathHolder, Vec<PathHolder>>) -> Vec<String> {
    // nested_scope_dict(dict, |scope, sub_dict|
    //     format!("\t{}:\n\t\t{}", scope, mapper(sub_dict).join("\n\t\t")))
    let mut iter = dict.iter()
        .map(|(key, value)| {
            let scopes = quote!(#(#value),*);

            format!("\t{}:\n\t\t{}", format_token_stream(key), format_token_stream(&scopes))
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
    format_complex_obj(vec![
        vec!["-- types:".to_string()], scope_types_dict(&context.scope_types),
        vec!["-- traits:".to_string()], scope_traits_dict(&context.traits_dictionary),
        vec!["-- traits_impl:".to_string()], traits_impl_dict(&context.used_traits_dictionary),
        vec!["-- custom:".to_string()], scope_types_dict(&context.custom_conversions),
        vec!["-- imports:".to_string()], scope_imports_dict(&context.used_imports_at_scopes),
        vec!["-- generics:".to_string()], scope_generics_dict(&context.used_generics_at_scopes),
    ])
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

// #[allow(unused)]
// pub fn format_trait_decomposition_part2(dict: &TraitDecompositionPart2) -> String {
//     format_complex_obj(vec![
//         vec!["-- methods: ".to_string()], ident_signatures_dict(&dict.methods),
//         vec!["-- types: ".to_string()], ident_trait_type_decomposition_dict(&dict.types),
//     ])
// }

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
        println!("{}{} {}", " ".repeat($counter*2), $emoji, format!($($arg)*));
    };
}
