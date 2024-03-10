use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{ItemUse, UseRename, UseTree};
use syn::punctuated::Punctuated;
use syn::token::Semi;
use crate::composer::{Depunctuated, ParentComposer};
use crate::composition::{create_item_use_with_tree, GenericConversion, ImportComposition};
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::ImportConversion;
use crate::formatter::{format_imported_dict, format_tree_item_dict};
use crate::presentation::expansion::Expansion;
use crate::tree::{ScopeTreeCompact, ScopeTreeExportID, ScopeTreeItem};

#[derive(Clone)]
pub struct ScopeTree {
    pub scope: ScopeChain,
    pub generics: HashSet<GenericConversion>,
    pub imported: HashMap<ImportConversion, HashSet<ImportComposition>>,
    pub exported: HashMap<ScopeTreeExportID, ScopeTreeItem>,
    pub scope_context: ParentComposer<ScopeContext>,
}

impl ScopeTree {
    pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
        let mut generics = self.generics.clone();
        generics.extend(self.inner_generics());
        generics
    }
    fn inner_generics(&self) -> HashSet<GenericConversion> {
        self.exported.values()
            .flat_map(ScopeTreeItem::generic_conversions)
            .collect()
    }

    pub(crate) fn imports(&self) -> Punctuated<ItemUse, Semi> {
        self.imported.iter()
            .flat_map(|(import_type, imports)|
                imports.iter()
                    .map(move |import| import.present(import_type)))
            .collect()
    }

    pub(crate) fn exports(&self) -> Depunctuated<TokenStream2> {
        self.exported.values().map(ScopeTreeItem::to_token_stream).collect()
    }
}

impl std::fmt::Debug for ScopeTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // f.write_str(format_tree_item_dict(&self.exported).as_str())
        f.debug_struct("ScopeTreeCompact")
            .field("scope", &self.scope)
            .field("generics", &self.generics)
            .field("imported", &format_imported_dict(&self.imported))
            .field("exported", &format_tree_item_dict(&self.exported))
            .finish()
    }
}

impl ToTokens for ScopeTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let source = self.scope_context.borrow();
        let imports = if source.is_from_current_crate() {
            let mut imports = Punctuated::<ItemUse, Semi>::from_iter([
                create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: source.scope.crate_scope().clone() }))
            ]);
            imports.extend(self.imports());
            imports
        } else {
            self.imports()
        };

        let name = if self.scope.is_crate_root() {
            self.scope.crate_scope().to_token_stream()
        } else {
            self.scope.head().to_token_stream()
        };
        // println!("ScopeTree::to_tokens [{} ({})] {}",source.scope.crate_scope(), source.is_from_current_crate(), name);
        Expansion::Mod {
            directives: quote!(),
            name,
            imports,
            conversions: self.exports()
        }.to_tokens(tokens)
    }
}


impl From<ScopeTreeCompact> for ScopeTree {
    fn from(value: ScopeTreeCompact) -> Self {
        let ScopeTreeCompact {
            scope,
            generics,
            imported,
            exported,
            scope_context
        } = value;
        ScopeTree {
            exported: exported.into_iter()
                .map(|(id, tree_item_raw)| {
                    let scope_tree_item = tree_item_raw.into_tree_item(&scope, &id);
                    (id, scope_tree_item)
                })
                .collect(),
            scope,
            imported,
            generics,
            scope_context
        }
    }
}

