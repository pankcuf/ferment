use std::collections::{HashMap, HashSet};
use quote::{quote, ToTokens};
use syn::{Ident, ItemTrait, Path, Type};
use syn::__private::TokenStream2;
use crate::import_conversion::{ImportConversion, ImportType};
use crate::scope::Scope;
use crate::scope_conversion::{ScopeTreeExportItem, ScopeTreeItem};
use crate::type_conversion::TypeConversion;

#[allow(unused)]
pub fn format_imported_dict(dict: &HashMap<ImportType, HashSet<ImportConversion>>) -> String {
    let debug_imports = dict.iter().map(|(i, p)| {
        let import = i.as_path();
        let ppp = p.iter().map(|ImportConversion { ident: i, scope: p}| quote!(#i: #p)).collect::<Vec<_>>();
        quote!(#import: #(#ppp,)*)
    }).collect::<Vec<_>>();
    let all = quote!(#(#debug_imports,)*);
    all.to_string()
}

#[allow(unused)]
pub fn format_imports(dict: &HashMap<Scope, HashMap<Ident, Path>>) -> String {
    let vec = vec![];
    let v = dict.iter().fold(vec, |mut acc, (k, scope_imports)| {
        let si = scope_imports.iter().map(|(k, v)| quote!(#k: #v)).collect::<Vec<_>>();
        acc.push(quote!(#k: #(#si),*));
        acc
    });
    let expanded = quote!(#(#v),*);
    expanded.to_string()
}

#[allow(unused)]
pub fn format_tree_exported_dict(dict: &HashMap<Ident, ScopeTreeExportItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("{}:\n{}", ident, tree_item))
        .collect::<Vec<_>>()
        .join("\n\n")
}
// self.path.to_token_stream().to_string().split_whitespace().collect::<String>().as_str(
#[allow(unused)]
pub fn format_tree_item_dict(dict: &HashMap<Ident, ScopeTreeItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("{}: {:?}", ident, quote!(#tree_item)))
        .collect::<Vec<_>>()
        .join("\n\n")
}
#[allow(unused)]
pub fn format_types_dict(dict: &HashMap<TypeConversion, Type>) -> String {
    let iter = dict.iter().map(|(tc, full_ty)| {
        let tc_str = format_token_stream(tc.to_token_stream());
        let full_ty_str = format_token_stream(full_ty.to_token_stream());
        format!("  {}: {}", tc_str, full_ty_str)
    });
    iter.collect::<Vec<_>>().join(",\n")
}

#[allow(unused)]
pub fn format_types_dict_full(dict: &HashMap<Scope, HashMap<TypeConversion, Type>>) -> String {
    dict.iter().map(|(scope, dict)| {
        let scope_str = format_token_stream(scope.to_token_stream());
        format!("{}:\n{}", scope_str, format_types_dict(dict))
    })
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn format_used_traits(dict: &HashMap<Scope, HashMap<Ident, ItemTrait>>) -> String {
    dict.iter()
        .map(|(scope, traits)| {
            let trait_idents = traits.iter().map(|(ident, _ )| quote!(#ident { .. }));
            quote!(#scope: #(#trait_idents,) *).to_string()
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn format_token_stream(token: TokenStream2) -> String {
    token.to_string()
        .split_whitespace()
        .collect::<String>()
        .to_string()
}

pub fn format_type_map(dict: &HashMap<Type, Type>) -> String {
    dict.iter()
        .map(|(ident, path )|
            format!("  {}: {}", format_token_stream(quote!(#ident)), format_token_stream(quote!(#path))))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(unused)]
pub fn format_custom_conversions(dict: &HashMap<Scope, HashMap<Type, Type>>) -> String {
    dict.iter()
        .map(|(scope, matches)|
            format!("{}:\n{}", format_token_stream(quote!(#scope)), format_type_map(matches)))
        .collect::<Vec<_>>()
        .join("\n\n")
}