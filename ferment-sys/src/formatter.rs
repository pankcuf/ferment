use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use proc_macro2::{Spacing, TokenTree};
use quote::{quote, ToTokens};
use syn::{Attribute, Ident, ItemUse, Path, Signature, Type};
use crate::ast::{PathHolder, TypeHolder, TypePathHolder};
use crate::composable::{GenericBoundsModel, GenericConversion, TraitModelPart1, TraitDecompositionPart1, TraitTypeModel};
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::{MixinKind, ObjectKind};
use crate::tree::{ScopeTreeExportID, ScopeTreeExportItem, ScopeTreeItem};

#[allow(unused)]
pub fn format_imported_set(dict: &HashSet<ItemUse>) -> String {
    let debug_imports = dict.iter().map(|i| {
        i.to_token_stream()
    }).collect::<Vec<_>>();
    let all = quote!(#(#debug_imports,)*);
    all.to_string()
}

#[allow(unused)]
pub fn format_scope_refinement(dict: &Vec<(ScopeChain, HashMap<TypeHolder, ObjectKind>)>) -> String {
    let mut iter = dict.iter()
        .map(|(scope, types)|
            format!("\t{}: \n\t\t{}", scope.self_path_holder_ref(), types.iter().map(scope_type_conversion_pair).collect::<Vec<_>>()
                .join("\n\t")))
        .collect::<Vec<String>>();
    iter.sort();
    iter.join("\n")

}

#[allow(unused)]
pub fn format_type_holders(dict: &HashSet<TypeHolder>) -> String {
    dict.iter()
        // .map(|item| format_token_stream(&item.0))
        .map(|item| item.0.to_token_stream().to_string())
        .collect::<Vec<_>>()
        .join("\n\n")
}
#[allow(unused)]
pub fn format_type_holders_vec(dict: &Vec<TypeHolder>) -> String {
    dict.iter()
        // .map(|item| format_token_stream(&item.0))
        .map(|item| item.0.to_token_stream().to_string())
        .collect::<Vec<_>>()
        .join("\n\n")
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
pub fn format_generic_conversions(dict: &HashMap<GenericConversion, HashSet<Option<Attribute>>>) -> String {
    dict.iter()
        .map(|(item, attrs)| format!("{}: {}", format_unique_attrs(attrs), item.object.to_token_stream()))
        .collect::<Vec<_>>()
        .join("\n\t")
}
#[allow(unused)]
pub fn format_mixin_kinds(dict: &HashMap<MixinKind, HashSet<Option<Attribute>>>) -> String {
    dict.iter()
        .map(|(item, attrs)| format!("{}:\n\t {}", item, format_unique_attrs(attrs)))
        .collect::<Vec<_>>()
        .join("\n\t")
}
#[allow(unused)]
pub fn format_mixin_conversions(dict: &HashMap<GenericBoundsModel, HashSet<Option<Attribute>>>) -> String {
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

pub fn format_attrs(dict: &Vec<Attribute>) -> String {
    dict.iter()
        .map(|item| item.to_token_stream().to_string())
        .collect::<Vec<_>>()
        .join("\n\t")
}

#[allow(unused)]
pub fn format_imports(dict: &HashMap<ScopeChain, HashMap<PathHolder, Path>>) -> String {
    let vec = scope_imports_dict(dict);
    let expanded = quote!(#(#vec),*);
    expanded.to_string()
}

#[allow(unused)]
pub fn format_tree_exported_dict(dict: &HashMap<ScopeTreeExportID, ScopeTreeExportItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("{}: {}", ident, tree_item))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn format_tree_item_dict(dict: &HashMap<ScopeTreeExportID, ScopeTreeItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("\t{}: {:?}", ident, tree_item))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn scope_type_conversion_pair(dict: (&TypeHolder, &ObjectKind)) -> String {
    format!("\t{}: {}", dict.0.to_token_stream(), dict.1)
    // format!("\t{}: {}", format_token_stream(dict.0), dict.1)
}

#[allow(unused)]
pub fn refinement_pair(dict: (&TypeHolder, &Vec<ObjectKind>)) -> String {
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
        let TraitTypeModel { ident, trait_bounds, generics } = dict.1;
        quote!(#ident: [bounds: #(#trait_bounds)*, generics: #generics])
    })
}
fn format_ident_path_pair(pair: (&PathHolder, &Path)) -> String {
    format!("\t{}: {}", format_token_stream(pair.0), format_token_stream(pair.1))
}

pub fn format_path_vec(vec: &Vec<Path>) -> String {
    vec.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(",")
}
pub fn format_obj_vec(vec: &Vec<ObjectKind>) -> String {
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
pub fn format_predicates_obj_dict(vec: &HashMap<Type, Vec<ObjectKind>>) -> String {
    vec.iter()
        .map(type_vec_obj_conversion_pair)
        .collect::<Vec<_>>()
        .join(",")
}

#[allow(unused)]
fn format_generic_bounds_pair(pair: (&TypePathHolder, &Vec<Path>)) -> String {
    format!("\t{}: [{}]", format_token_stream(pair.0), format_path_vec(pair.1))
}

fn format_ident_trait_pair(pair: (&Ident, &TraitModelPart1)) -> String {
    let implementors = &pair.1.implementors;
    format!("\t{}: {}: [{}]", format_token_stream(pair.0), "...", quote!(#(#implementors),*))
}

#[allow(unused)]
pub fn format_types_dict(dict: &HashMap<TypeHolder, ObjectKind>) -> String {
    types_dict(dict)
        .join("\n")
}
#[allow(unused)]
pub fn format_types_to_refine(dict: &HashMap<TypeHolder, Vec<ObjectKind>>) -> String {
    let mut iter = dict.iter()
        .map(refinement_pair)
        .collect::<Vec<String>>();
    iter.sort();
    iter.join("\n")
}

#[allow(unused)]
pub fn format_ident_types_dict(dict: &HashMap<Ident, Type>) -> String {
    ident_types_dict(dict)
        .join("\n")
}

// #[allow(unused)]
// pub fn format_scope_types_dict(dict: &HashMap<ScopeChain, TypeChain>) -> String {
//     scope_types_dict(dict)
//         .join("\n\n")
// }
//
#[allow(unused)]
pub fn format_used_traits(dict: &HashMap<ScopeChain, HashMap<Ident, TraitModelPart1>>) -> String {
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
                    // '[' => {
                    //     inside_square_brackets += 1;
                    //     formatted_string.push('[');
                    //     space_needed = false;
                    // }
                    // ']' => {
                    //     inside_square_brackets -= 1;
                    //     formatted_string.push(']');
                    //     space_needed = false;
                    // }
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



/// Helpers

pub fn imports_dict(dict: &HashMap<PathHolder, Path>) -> Vec<String> {
    dict.iter()
        .map(format_ident_path_pair)
        .collect()
}

#[allow(unused)]
pub fn generic_bounds_dict(dict: &HashMap<TypePathHolder, Vec<Path>>) -> Vec<String> {
    dict.iter()
        .map(format_generic_bounds_pair)
        .collect()
}

pub fn types_dict(dict: &HashMap<TypeHolder, ObjectKind>) -> Vec<String> {
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


fn ident_trait_type_decomposition_dict(dict: &HashMap<Ident, TraitTypeModel>) -> Vec<String> {
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

fn traits_dict(dict: &HashMap<Ident, TraitModelPart1>) -> Vec<String> {
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

fn format_scope_dict<K2, V2, F: Fn(&HashMap<K2, V2>) -> Vec<String>>(dict: &HashMap<ScopeChain, HashMap<K2, V2>>, mapper: F) -> Vec<String>  {
    nested_scope_dict(dict, |scope, sub_dict|
        format!("\t{}:\n\t\t{}", scope.fmt_short(), mapper(sub_dict).join("\n\t\t")))
}

pub fn scope_imports_dict(dict: &HashMap<ScopeChain, HashMap<PathHolder, Path>>) -> Vec<String> {
    format_scope_dict(dict, imports_dict)
}

#[allow(unused)]
pub fn scope_generics_dict(dict: &HashMap<ScopeChain, HashMap<TypePathHolder, Vec<Path>>>) -> Vec<String> {
    format_scope_dict(dict, generic_bounds_dict)
}


fn scope_traits_dict(dict: &HashMap<ScopeChain, HashMap<Ident, TraitModelPart1>>) -> Vec<String> {
    format_scope_dict(dict, traits_dict)
}



fn traits_impl_dict(dict: &HashMap<ScopeChain, Vec<PathHolder>>) -> Vec<String> {
    // nested_scope_dict(dict, |scope, sub_dict|
    //     format!("\t{}:\n\t\t{}", scope, mapper(sub_dict).join("\n\t\t")))
    let mut iter = dict.iter()
        .filter_map(|(key, value)| {
            let scopes = quote!(#(#value),*);
            if value.is_empty() {
                None
            } else {
                Some(format!("\t{}:\n\t\t{}", format_token_stream(key), format_token_stream(&scopes)))
            }
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
        vec!["-- types:".to_string(), context.scope_register.to_string()],
        vec!["-- traits:".to_string()], scope_traits_dict(&context.traits.inner),
        vec!["-- traits_impl:".to_string()], traits_impl_dict(&context.traits.used_traits_dictionary),
        vec!["-- custom:".to_string(), context.custom.to_string()],
        vec!["-- imports:".to_string()], scope_imports_dict(&context.imports.inner),
        vec!["-- generics:".to_string()], scope_generics_dict(&context.generics.inner),
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
// pub fn format_type_composition(dict: &TypeComposition) -> String {
//     format_complex_obj(vec![
//         vec!["-- type:".to_string(), format_token_stream(&dict.ty), "-- generics:".to_string(), dict.generics.as_ref().map_or(format!("None"), |generics| format_token_stream(generics))],
//     ])
// }

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
        println!("# {}", $label);
        println!("########################################################################################################################");
        println!("{}", format!($($arg)*));
        println!("########################################################################################################################\n");
    }
}

