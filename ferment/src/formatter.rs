use std::collections::{HashMap, HashSet};
use quote::quote;
use syn::{Ident, ItemTrait, Path, Type};
use crate::import_conversion::{ImportConversion, ImportType};
use crate::scope::Scope;
use crate::scope_conversion::ScopeTreeExportItem;
use crate::type_conversion::TypeConversion;

pub fn format_imported_dict(dict: &HashMap<ImportType, HashSet<ImportConversion>>) -> String {
    let debug_imports = dict.iter().map(|(i, p)| {
        let import = i.as_path();
        let ppp = p.iter().map(|ImportConversion { ident: i, scope: p}| quote!(#i: #p)).collect::<Vec<_>>();
        quote!(#import: #(#ppp,)*)
    }).collect::<Vec<_>>();
    let all = quote!(#(#debug_imports,)*);
    all.to_string()
}

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

pub fn format_exported_dict(dict: &HashMap<Ident, ScopeTreeExportItem>) -> String {
    dict.iter()
        .map(|(ident, tree_item)| format!("{}: {}", ident, tree_item))
        .collect::<Vec<_>>()
        .join(", ")
}
pub fn format_types_dict(dict: &HashMap<TypeConversion, Type>) -> String {
    dict.iter()
        .map(|(tc, full_ty)| quote!(#tc: #full_ty).to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn format_types_dict_full(dict: &HashMap<Scope, HashMap<TypeConversion, Type>>) -> String {
    dict.iter()
        .map(|(scope, dict)| format!("{}: {}", quote!(#scope).to_string(), format_types_dict(dict)))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn format_used_traits(dict: &HashMap<Scope, HashMap<Ident, ItemTrait>>) -> String {
    dict.iter()
        .map(|(scope, traits)| {
            let trait_idents = traits.iter().map(|(ident, _ )| quote!(#ident { .. }));
            quote!(#scope: #(#trait_idents,) *).to_string()
        })
        .collect::<Vec<_>>()
        .join(", ")
}