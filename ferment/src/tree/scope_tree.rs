use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::{Attribute, ItemUse, UseRename, UseTree};
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::ParentComposer;
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeContext, ScopeInfo};
use crate::conversion::ObjectKind;
use crate::ext::Join;
use crate::formatter::format_tree_item_dict;
use crate::presentation::Expansion;
use crate::tree::{ScopeTreeExportID, ScopeTreeExportItem, ScopeTreeItem};

#[derive(Clone)]
pub struct ScopeTree {
    pub scope: ScopeChain,
    pub imported: HashSet<ItemUse>,
    pub exported: HashMap<ScopeTreeExportID, ScopeTreeItem>,
    pub attrs: Vec<Attribute>,
    pub scope_context: ParentComposer<ScopeContext>,
}
impl Debug for ScopeTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("ScopeTree({})", format_tree_item_dict(&self.exported)).as_str())
    }
}


impl ScopeTree {
    // pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
    //     let mut generics = self.generics.clone();
    //     generics.extend(self.exported.values().flat_map(ScopeTreeItem::generic_conversions));
    //     // TODO: there should be refined generics
    //     println!("ScopeTree::generic_conversions: {}", format_generic_conversions(&generics));
    //     generics
    // }

    pub(crate) fn imports(&self) -> SemiPunctuated<ItemUse> {
        SemiPunctuated::from_iter(self.imported.iter().cloned())
    }

    pub(crate) fn exports(&self) -> Depunctuated<TokenStream2> {
        self.exported.values().map(ScopeTreeItem::to_token_stream).collect()
    }

    pub fn print_scope_tree_with_message(&self, message: &str) {
        self.scope_context.borrow().print_with_message(message)
    }
}

// impl std::fmt::Debug for ScopeTree {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("ScopeTree")
//             .field("scope", &self.scope)
//             .field("imported", &format_imported_dict(&self.imported))
//             .field("exported", &format_tree_item_dict(&self.exported))
//             .finish()
//     }
// }

impl ToTokens for ScopeTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let source = self.scope_context.borrow();
        let ctx = source.context.read().unwrap();
        let rename = ctx.config.current_crate.ident();

        let mut imports = SemiPunctuated::from_iter([
            create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename }))
        ]);
        imports.extend(self.imports());
        // imports
        // let imports = if source.is_from_current_crate() {
        //     let mut imports = SemiPunctuated::from_iter([
        //         create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: source.scope.crate_ident().clone() }))
        //     ]);
        //     imports.extend(self.imports());
        //     imports
        // } else {
        //     self.imports()
        // };

        let name = if self.scope.is_crate_root() {
            self.scope.crate_ident_ref().to_token_stream()
        } else {
            self.scope.head().to_token_stream()
        };
        let conversions = self.exports();
        if !conversions.is_empty() {
            Expansion::Mod {
                attrs: self.attrs.cfg_attributes(),
                name,
                imports,
                conversions
            }.to_tokens(tokens)
        }
    }
}

pub fn create_generics_scope_tree(root_scope_chain: &ScopeChain, global_context: Arc<RwLock<GlobalContext>>) -> ScopeTree {
    let crate_ident =  root_scope_chain.crate_ident_ref();
    let generics_scope_ident = format_ident!("generics");
    let generics_scope_chain = ScopeChain::Mod {
        info: ScopeInfo {
            attrs: vec![],
            crate_ident: crate_ident.clone(),
            self_scope: Scope::new(root_scope_chain.self_path_holder_ref().joined(&generics_scope_ident), ObjectKind::Empty) },
        parent_scope_chain: root_scope_chain.clone().into() };

    create_scope_tree(
        generics_scope_chain.clone(),
        Rc::new(RefCell::new(ScopeContext::with(generics_scope_chain, global_context))),
        HashSet::from_iter([
            create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: crate_ident.clone() }))
        ]),
        HashMap::new(),
        vec![]
    )
}

pub fn create_item_use_with_tree(tree: UseTree) -> ItemUse {
    ItemUse {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree,
        semi_token: Default::default(),
    }
}


pub fn create_crate_root_scope_tree(
    crate_ident: Ident,
    scope_context: ParentComposer<ScopeContext>,
    imported: HashSet<ItemUse>,
    exported: HashMap<ScopeTreeExportID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    // print_phase!("PHASE 2: SCOPE TREE MORPHING", "\n{}", format_tree_exported_dict(&exported));
    create_scope_tree(ScopeChain::crate_root(crate_ident, attrs.clone()), scope_context, imported, exported, attrs)
}

pub fn create_scope_tree(
    scope: ScopeChain,
    scope_context: ParentComposer<ScopeContext>,
    imported: HashSet<ItemUse>,
    exported: HashMap<ScopeTreeExportID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    let exported = exported.into_iter()
        .map(|(scope_id, scope_tree_export_item)| {
            let scope_tree_item = match scope_tree_export_item {
                ScopeTreeExportItem::Item(
                    scope_context,
                    item) =>
                    ScopeTreeItem::Item {
                        scope: scope.joined(&item),
                        item,
                        scope_context
                    },
                ScopeTreeExportItem::Tree(
                    scope_context,
                    imported,
                    exported,
                    attrs) =>
                    {
                        // println!("add (TREE): {}: {}", scope_context.borrow().scope.self_path_holder_ref(), attrs.iter().map(|a| a.to_token_stream()).collect::<Depunctuated<_>>().to_token_stream());
                        ScopeTreeItem::Tree {
                            tree: create_scope_tree(scope_id.create_child_scope(&scope, attrs.clone()), scope_context, imported, exported, attrs)
                        }
                    }
            };
            (scope_id, scope_tree_item)
        })
        .collect();
    // println!("ScopeTree:: {}", format_tree_item_dict(&exported));
    ScopeTree {
        exported,
        scope,
        imported,
        attrs,
        scope_context,
    }
}