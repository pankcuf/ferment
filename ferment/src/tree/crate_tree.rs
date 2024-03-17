use std::collections::{HashMap, HashSet};
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::builder::Crate;
use crate::composer::{Depunctuated, ParentComposer};
use crate::composition::{create_items_use_from_path, GenericConversion, ImportComposition};
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::ImportConversion;
use crate::presentation::{Expansion, ScopeContextPresentable};
use crate::tree::{ScopeTree, ScopeTreeExportID, ScopeTreeExportItem};

#[derive(Clone)]
pub struct CrateTree {
    pub current_crate: Crate,
    pub current_tree: ScopeTree,
    pub external_crates: HashMap<Crate, ScopeTree>,
}
impl ToTokens for CrateTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut generics: HashSet<GenericConversion> = HashSet::from_iter(self.current_tree.generics.iter().cloned());
        let mut generic_imports = HashSet::new();
        let mut generic_conversions = Depunctuated::new();
        let mut regular_conversions = self.external_crates
            .iter()
            .map(|(_crate, scope_tree)| {
                generics.extend(scope_tree.generic_conversions());
                scope_tree.to_token_stream()
            })
            .collect::<Depunctuated<TokenStream2>>();

        generics.extend(self.current_tree.generic_conversions());
        regular_conversions.push(self.current_tree.to_token_stream());

        for generic in &generics {
            generic_imports.extend(generic.used_imports());
            generic_conversions.push(generic.present(&self.current_tree.scope_context.borrow()));
        }
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
            imports: generic_imports.iter().map(create_items_use_from_path).collect(),
            conversions: generic_conversions
        }.to_tokens(tokens);
    }
}

fn create_scope_tree(
    crate_ident: Ident,
    scope_context: ParentComposer<ScopeContext>,
    generics: HashSet<GenericConversion>,
    imported: HashMap<ImportConversion, HashSet<ImportComposition>>,
    exported: HashMap<ScopeTreeExportID, ScopeTreeExportItem>
) -> ScopeTree {
    let scope = ScopeChain::crate_root(crate_ident);
    ScopeTree {
        exported: exported.into_iter()
            .map(|(scope_id, tree_item_raw)| {
                let scope_tree_item = tree_item_raw.into_tree_item(&scope, &scope_id);
                (scope_id, scope_tree_item)
            })
            .collect(),
        scope,
        imported,
        generics,
        scope_context
    }
}

impl CrateTree {
    pub fn new(current_crate: &Crate, current_tree: ScopeTreeExportItem, external_crates: HashMap<Crate, ScopeTreeExportItem>) -> Self {
        match current_tree {
            ScopeTreeExportItem::Item(..) => panic!("•• It should never happen ••"),
            ScopeTreeExportItem::Tree(
                scope_context,
                generics,
                imported,
                exported) => {
                println!();
                println!("•• CRATE TREE MORPHING ••");
                println!();
                let current_tree = create_scope_tree(current_crate.ident(), scope_context, generics, imported, exported);
                let external_crates = external_crates.into_iter()
                    .map(|(external_crate, tree_item_raw)|
                        match tree_item_raw {
                            ScopeTreeExportItem::Item(..) => panic!("•• It should never happen ••"),
                            ScopeTreeExportItem::Tree(
                                scope_context,
                                generics,
                                imported,
                                exported) => {
                                let current_tree = create_scope_tree(external_crate.ident(), scope_context, generics, imported, exported);
                                (external_crate, current_tree)
                            }
                        })
                    .collect();
                println!();
                println!("•• CRATE TREE CONTEXT ••");
                println!();
                println!("{}", current_tree.scope_context.borrow());

                Self { current_crate: current_crate.clone(), current_tree, external_crates }
            }
        }
    }
}