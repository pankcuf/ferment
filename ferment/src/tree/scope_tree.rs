use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composition::{GenericConversion, ImportComposition};
use crate::context::{Scope, ScopeChain, ScopeContext};
use crate::conversion::{ImportConversion, ObjectConversion};
use crate::formatter::{format_imported_dict, format_tree_item_dict};
use crate::helper::ItemExtension;
use crate::presentation::expansion::Expansion;
use crate::tree::{ScopeTreeCompact, ScopeTreeExportID, ScopeTreeExportItem, ScopeTreeItem};

// impl From<ScopeTreeCompact> for ScopeTreeItem {
//     fn from(value: ScopeTreeCompact) -> Self {
//         let name = value.scope.head();
//         ScopeTreeItem::Tree {
//             item: parse_quote!(pub mod #name;),
//             tree: value.into()
//         }
//     }
// }


#[derive(Clone)]
pub struct ScopeTree {
    pub scope: ScopeChain,
    pub generics: HashSet<GenericConversion>,
    pub imported: HashMap<ImportConversion, HashSet<ImportComposition>>,
    pub exported: HashMap<ScopeTreeExportID, ScopeTreeItem>,
    pub scope_context: Rc<RefCell<ScopeContext>>,
}

impl std::fmt::Debug for ScopeTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        let scope_imports = self.imported.iter()
            .flat_map(|(import_type, imports)|
                imports.iter()
                    .map(move |import| import.present(import_type)));
        if self.scope.is_crate_root() {
            // For root tree only
            let mut generics: HashSet<GenericConversion> = HashSet::from_iter(self.generics.iter().cloned());
            let scope_conversions = self.exported.values().map(|tree_item| {
                generics.extend(tree_item.generic_conversions());
                tree_item.to_token_stream()
            }).collect::<Vec<_>>();
            let mut generic_imports = HashSet::new();
            let mut generic_conversions = vec![];
            let vtable_improts = HashSet::new();
            let vtable_conversions = vec![];
            for generic in &generics {
                generic_imports.extend(generic.used_imports());
                generic_conversions.push(generic.expand(&self.scope_context));
            }
            let directives = quote!(#[allow(clippy::let_and_return, clippy::suspicious_else_formatting, clippy::redundant_field_names, dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables, unused_qualifications)]);
            let types_expansion = Expansion::Mod {
                directives: directives.clone(),
                name: quote!(types),
                imports: scope_imports.collect(),
                conversions: scope_conversions
            };
            let generics_expansion = Expansion::Mod {
                directives: directives.clone(),
                name: quote!(generics),
                imports: generic_imports.into_iter().collect(),
                conversions: generic_conversions
            };
            let vtables_expansion = Expansion::Mod {
                directives,
                name: quote!(vtables),
                imports: vtable_improts.into_iter().collect(),
                conversions: vtable_conversions
            };
            let modules = [types_expansion, generics_expansion, vtables_expansion];
            quote!(#(#modules)*)
        } else {
            Expansion::Mod {
                directives: quote!(),
                name: self.scope.head().to_token_stream(),
                imports: scope_imports.collect(),
                conversions: self.exported.values().map(ScopeTreeItem::to_token_stream).collect()
            }.to_token_stream()
        }.to_tokens(tokens)
    }}


impl From<ScopeTreeCompact> for ScopeTree {
    fn from(value: ScopeTreeCompact) -> Self {
        let ScopeTreeCompact {
            scope,
            generics,
            imported,
            exported,
            scope_context
        } = value;
        let imported = imported.clone();
        let exported = exported.into_iter().map(|(id, tree_item_raw)| {
            // let scope = scope.joined(&ident);
            let scope_tree_item = match tree_item_raw {
                ScopeTreeExportItem::Item(scope_context, item) => {
                    println!("ScopeTreeItem::Item: {} in [{}]", item.ident_string(), scope);
                    let scope = scope.joined(&item);
                    ScopeTreeItem::Item { item, scope, scope_context }
                },
                ScopeTreeExportItem::Tree(
                    scope_context,
                    generics,
                    imported,
                    exported) =>
                    ScopeTreeItem::Tree {
                        tree: {
                            println!("ScopeTreeItem::Tree: {} in [{}]", id, scope);
                            // let scope = scope.joined_mod(&ident);
                            // let scope = scope.joined(&id.ident());
                            let self_scope = match &id {
                                ScopeTreeExportID::Ident(ident) => scope.self_scope().self_scope.joined(ident),
                                ScopeTreeExportID::Impl(_, _) =>
                                    panic!("impl not implemented")
                            };

                            Self::from(ScopeTreeCompact {
                                scope: ScopeChain::Mod {
                                    self_scope: Scope::new(self_scope, ObjectConversion::Empty),
                                },
                                generics,
                                imported,
                                exported,
                                scope_context
                            })
                        }
                    }
            };

            (id, scope_tree_item)
        }).collect();
        Self {
            scope: scope.clone(),
            imported,
            exported,
            generics,
            scope_context,
        }
    }
}

impl ScopeTree {

    pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
        let mut generics = self.generics.clone();
        generics.extend(self.inner_generics());
        generics
    }
    fn inner_generics(&self) -> HashSet<GenericConversion> {
        self.exported.values()
            .flat_map(|tree| tree.generic_conversions())
            .collect()
    }
}