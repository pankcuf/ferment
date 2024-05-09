use std::collections::HashMap;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{ItemUse, UseRename, UseTree};
use syn::punctuated::Punctuated;
use syn::token::Semi;
use crate::builder::Crate;
use crate::composer::Depunctuated;
use crate::composition::create_item_use_with_tree;
use crate::{error, print_phase};
use crate::formatter::{format_generic_conversions, format_tree_exported_dict};
use crate::presentation::{Expansion, ScopeContextPresentable};
use crate::tree::{create_crate_root_scope_tree, ScopeTree, ScopeTreeExportItem};

#[derive(Clone, Debug)]
pub struct CrateTree {
    pub current_crate: Crate,
    pub current_tree: ScopeTree,
    pub external_crates: HashMap<Crate, ScopeTree>,
}

// Main entry point for resulting expansion
impl ToTokens for CrateTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let scope_context = self.current_tree.scope_context.borrow();
        let refined_generics = &scope_context.context.read().unwrap().refined_generics;
        print_phase!("PHASE 3: GENERICS TO EXPAND", "\t{}", format_generic_conversions(&refined_generics));
        let directives = quote!(#[allow(clippy::let_and_return, clippy::suspicious_else_formatting, clippy::redundant_field_names, dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables, unused_qualifications)]);
        Expansion::Mod {
            attrs: Depunctuated::new(),
            directives: directives.clone(),
            name: quote!(types),
            imports: Punctuated::new(),
            conversions: self.to_regular_conversions()
        }.to_tokens(tokens);
        Expansion::Mod {
            attrs: Depunctuated::new(),
            directives,
            name: quote!(generics),
            imports: Punctuated::<ItemUse, Semi>::from_iter([
                create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: self.current_tree.scope.crate_ident().clone() }))
            ]),
            conversions: refined_generics.iter().map(|generic| generic.present(&scope_context)).collect()
        }.to_tokens(tokens);
    }
}


impl CrateTree {
    pub fn new(current_crate: &Crate, current_tree: ScopeTreeExportItem, external_crates: HashMap<Crate, ScopeTreeExportItem>) -> Result<Self, error::Error> {
        match current_tree {
            ScopeTreeExportItem::Item(..) =>
                Err(error::Error::ExpansionError("Bad tree root")),
            ScopeTreeExportItem::Tree(
                scope_context,
                imported,
                exported,
                attrs) => {
                // print_phase!("PHASE 2: CRATE TREE MORPHING", "\n{}", format_tree_exported_dict(&exported));
                let current_tree = create_crate_root_scope_tree(current_crate.ident(), scope_context, imported, exported, attrs);
                let external_crates = external_crates.into_iter()
                    .map(|(external_crate, crate_root_tree_export_item)|
                        match crate_root_tree_export_item {
                            ScopeTreeExportItem::Item(..) =>
                                panic!("•• It should never happen ••"),
                            ScopeTreeExportItem::Tree(
                                scope_context,
                                imported,
                                exported,
                                attrs) => {
                                let crate_ident = external_crate.ident();
                                (external_crate, create_crate_root_scope_tree(crate_ident, scope_context, imported, exported, attrs))
                            }
                        })
                    .collect();
                // print_phase!("PHASE 2: CURRENT CRATE TREE", "\n{:?}", current_tree);
                // print_phase!("PHASE 2: EXTERNAL CRATES TREE", "\n{:?}", external_crates);
                current_tree.print_scope_tree_with_message("PHASE 2: CRATE TREE CONTEXT");
                let mut crate_tree = Self { current_crate: current_crate.clone(), current_tree, external_crates };
                crate_tree.current_tree.refine();
                Ok(crate_tree)
            }
        }
    }

    pub fn to_regular_conversions(&self) -> Depunctuated<TokenStream2> {
        let mut regular_conversions = self.external_crates
            .values()
            .map(ScopeTree::to_token_stream)
            .collect::<Depunctuated<_>>();
        regular_conversions.push(self.current_tree.to_token_stream());
        regular_conversions
    }
}