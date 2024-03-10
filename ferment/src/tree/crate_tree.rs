use std::collections::{HashMap, HashSet};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::builder::Crate;
use crate::composer::Depunctuated;
use crate::composition::{create_items_use_from_path, GenericConversion};
use crate::context::ScopeChain;
use crate::presentation::Expansion;
use crate::tree::{ScopeTree, ScopeTreeExportItem};

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
            generic_conversions.push(generic.expand(&self.current_tree.scope_context));
        }
        let directives = quote!(#[allow(clippy::let_and_return, clippy::suspicious_else_formatting, clippy::redundant_field_names, dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables, unused_qualifications)]);

        Depunctuated::from_iter([
            Expansion::Mod {
                directives: directives.clone(),
                name: quote!(types),
                imports: Punctuated::new(),
                conversions: regular_conversions
            },
            Expansion::Mod {
                directives: directives.clone(),
                name: quote!(generics),
                imports: generic_imports.iter().map(create_items_use_from_path).collect(),
                conversions: generic_conversions
            }])
            .to_tokens(tokens)
    }
}

impl CrateTree {
    pub fn new(current_crate: Crate, current_tree: ScopeTreeExportItem, external_crates: HashMap<Crate, ScopeTreeExportItem>) -> Self {
        match current_tree {
            ScopeTreeExportItem::Item(..) => panic!("•• It should never happen ••"),
            ScopeTreeExportItem::Tree(
                scope_context,
                generics,
                imported,
                exported) => {
                // {
                //     let mut lock = context.write().unwrap();
                //     lock.inject_types_from_traits_implementation();
                // }
                println!("•• TREE 1 MORPHING ••");
                let scope = ScopeChain::crate_root(current_crate.ident());

                // let current_tree = ScopeTree::from(ScopeTreeCompact { scope, scope_context, generics, imported, exported };
                let current_tree = ScopeTree {
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
                };


                println!();
                println!("•• TREE 2 MORPHING using ScopeContext:");
                println!();
                println!("{}", current_tree.scope_context.borrow());
                // Expansion::Root { tree }

                let external_crates = external_crates
                    .into_iter()
                    .map(|(external_crate, tree_item_raw)| {
                        let scope = ScopeChain::crate_root(external_crate.ident());
                        match tree_item_raw {
                            ScopeTreeExportItem::Item(..) => panic!("•• It should never happen ••"),
                            ScopeTreeExportItem::Tree(
                                scope_context,
                                generics,
                                imported,
                                exported) => {
                                let current_tree = ScopeTree {
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
                                };
                                (external_crate, current_tree)
                            }
                        }
                        // let scope_tree_item = tree_item_raw.into_tree_item(&scope, &scope_id);
                        // // scope_tree_item.
                        // (scope_id, scope_tree_item)
                        //
                        //     (external_crate.clone(), ScopeTree::from(ScopeTreeCompact { scope, scope_context: tree_item_raw.scope_context, generics, imported, exported }))
                    }
                ).collect();

                Self { current_crate, current_tree, external_crates }
            }
        }


    }
}