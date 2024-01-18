use std::collections::HashSet;
use std::hash::Hash;
use syn::{parse_quote, Path};
use crate::composition::ImportComposition;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ImportConversion {
    Original,
    External,
    // full or partial import
    ExternalChunk,
    // external crate that uses `ferment`
    FfiExternal,
    FfiGeneric,
    FfiType,
    Inner,
    OriginalTrait,
    FfiTrait,
    None,
}

impl ImportConversion {

    pub fn as_path(&self) -> Path {
        match self {
            ImportConversion::Original => parse_quote!(Original),
            ImportConversion::External => parse_quote!(External),
            ImportConversion::ExternalChunk => parse_quote!(ExternalChunk),
            ImportConversion::FfiExternal => parse_quote!(FfiExternal),
            ImportConversion::FfiType => parse_quote!(FfiType),
            ImportConversion::FfiGeneric => parse_quote!(FfiGeneric),
            ImportConversion::Inner => parse_quote!(Inner),
            ImportConversion::None => parse_quote!(None),

            ImportConversion::OriginalTrait => parse_quote!(OriginalTrait),
            ImportConversion::FfiTrait => parse_quote!(FfiTrait),
        }
    }

    pub fn get_imports_for(self, used_imports: HashSet<ImportComposition>) -> Option<(ImportConversion, HashSet<ImportComposition>)> {
        match self {
            ImportConversion::Inner | ImportConversion::None => None,
            _ => Some((self, used_imports))
        }
    }
}
