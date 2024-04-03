use std::collections::{HashMap, HashSet};
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{ItemUse, UseRename, UseTree};
use syn::punctuated::Punctuated;
use syn::token::Semi;
use crate::builder::Crate;
use crate::composer::Depunctuated;
use crate::composition::{create_item_use_with_tree, create_items_use_from_path, GenericConversion};
use crate::error;
use crate::formatter::format_generic_conversions;
use crate::presentation::{Expansion, ScopeContextPresentable};
use crate::tree::{create_crate_root_scope_tree, ScopeTree, ScopeTreeExportItem};

#[derive(Clone)]
pub struct CrateTree {
    pub current_crate: Crate,
    pub current_tree: ScopeTree,
    pub external_crates: HashMap<Crate, ScopeTree>,
}

// Main entry point for resulting expansion
impl ToTokens for CrateTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {

        let refined_generics = self.current_tree.scope_context.borrow().context.read().unwrap().refined_generics.clone();
        println!("CrateTree::refined_generics: {}", format_generic_conversions(&refined_generics));
        // let mut generics: HashSet<GenericConversion> = HashSet::from_iter(self.current_tree.generics.iter().cloned());
        let generics: HashSet<GenericConversion> = HashSet::from_iter(refined_generics);
        let mut generic_imports = HashSet::new();
        let mut generic_conversions = Depunctuated::new();
        let mut regular_conversions = self.external_crates
            .iter()
            .map(|(_crate, scope_tree)| {
                // generics.extend(scope_tree.generic_conversions());
                scope_tree.to_token_stream()
            })
            .collect::<Depunctuated<TokenStream2>>();

        // generics.extend(self.current_tree.generic_conversions());
        regular_conversions.push(self.current_tree.to_token_stream());

        for generic in &generics {
            generic_imports.extend(generic.used_imports());
            generic_conversions.push(generic.present(&self.current_tree.scope_context.borrow()));
        }
        let mut imports = Punctuated::<ItemUse, Semi>::from_iter([
            create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: self.current_tree.scope.crate_ident().clone() }))
        ]);
        imports.extend(generic_imports.iter().map(create_items_use_from_path));

        let directives = quote!(#[allow(clippy::let_and_return, clippy::suspicious_else_formatting, clippy::redundant_field_names, dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables, unused_qualifications)]);
        Expansion::Mod {
            directives: directives.clone(),
            name: quote!(types),
            imports: Punctuated::new(),
            conversions: regular_conversions
        }.to_tokens(tokens);
        Expansion::Mod {
            directives: directives.clone(),
            name: quote!(generics),
            imports,
            conversions: generic_conversions
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
                exported) => {
                println!("\n•• CRATE TREE MORPHING ••\n");
                let current_tree = create_crate_root_scope_tree(current_crate.ident(), scope_context, imported, exported);
                let external_crates = external_crates.into_iter()
                    .map(|(external_crate, crate_root_tree_export_item)|
                        match crate_root_tree_export_item {
                            ScopeTreeExportItem::Item(..) =>
                                panic!("•• It should never happen ••"),
                            ScopeTreeExportItem::Tree(
                                scope_context,
                                imported,
                                exported) => {
                                let crate_ident = external_crate.ident();
                                (external_crate, create_crate_root_scope_tree(crate_ident, scope_context, imported, exported))
                            }
                        })
                    .collect();
                current_tree.print_scope_tree_with_message("CRATE TREE CONTEXT");

                let mut crate_tree = Self { current_crate: current_crate.clone(), current_tree, external_crates };
                crate_tree.current_tree.refine();
                Ok(crate_tree)
            }
        }
    }
}