use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use quote::quote;
use syn::{parse_quote, Path};
use crate::ast::CommaPunctuated;
use crate::composable::ImportModel;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ImportConversion {
    Original,
    External,
    // full or partial import
    ExternalChunk,
    // external crate that uses `ferment-sys`
    // FfiExternal,
    // FfiGeneric,
    FfiType,
    Inner,
    // OriginalTrait,
    // FfiTrait,
    None,
}

impl ImportConversion {

    pub fn as_path(&self) -> Path {
        match self {
            ImportConversion::Original => parse_quote!(Original),
            ImportConversion::External => parse_quote!(External),
            ImportConversion::ExternalChunk => parse_quote!(ExternalChunk),
            // ImportConversion::FfiExternal => parse_quote!(FfiExternal),
            ImportConversion::FfiType => parse_quote!(FfiType),
            // ImportConversion::FfiGeneric => parse_quote!(FfiGeneric),
            ImportConversion::Inner => parse_quote!(Inner),
            ImportConversion::None => parse_quote!(None),

            // ImportConversion::OriginalTrait => parse_quote!(OriginalTrait),
            // ImportConversion::FfiTrait => parse_quote!(FfiTrait),
        }
    }

    pub fn get_imports_for(self, used_imports: HashSet<ImportModel>) -> Option<(ImportConversion, HashSet<ImportModel>)> {
        match self {
            ImportConversion::Inner | ImportConversion::None => None,
            _ => Some((self, used_imports))
        }
    }
}

#[allow(unused)]
pub fn format_imported_dict(dict: &HashMap<ImportConversion, HashSet<ImportModel>>) -> String {
    let debug_imports = dict.iter().map(|(i, p)| {
        let import = i.as_path();
        let ppp = p.iter().map(|ImportModel { ident: i, scope: p}| quote!(#i: #p)).collect::<CommaPunctuated<_>>();
        quote!(#import: #ppp)
    }).collect::<Vec<_>>();
    let all = quote!(#(#debug_imports,)*);
    all.to_string()
}
