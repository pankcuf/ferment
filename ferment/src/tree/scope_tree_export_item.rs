use std::cell::RefCell;
use std::fmt::Formatter;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use proc_macro2::Ident;
use std::sync::{Arc, RwLock};
use quote::ToTokens;
use syn::{Item, ItemMod, Path, PathSegment, Type};
use crate::composer::ParentComposer;
use crate::composition::ImportComposition;
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeContext};
use crate::conversion::{ImportConversion, ObjectConversion};
use crate::formatter::{format_imported_dict, format_token_stream, format_tree_exported_dict};
use crate::helper::ItemExtension;


#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ScopeTreeExportID {
    Ident(Ident),
    Impl(Type, Option<Path>)
}

impl From<&PathSegment> for ScopeTreeExportID {
    fn from(value: &PathSegment) -> Self {
        ScopeTreeExportID::Ident(value.ident.clone())
    }
}

impl std::fmt::Debug for ScopeTreeExportID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportID::Ident(ident) =>
                f.write_str(format!("Ident({})", ident.to_token_stream()).as_str()),
            ScopeTreeExportID::Impl(ty, path) =>
                f.write_str(format!("Impl({}, {})", ty.to_token_stream(), format_token_stream(path)).as_str())
        }
    }
}

impl std::fmt::Display for ScopeTreeExportID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


impl ScopeTreeExportID {
    pub fn from_ident(ident: &Ident) -> Self {
        ScopeTreeExportID::Ident(ident.clone())
    }

    pub fn create_child_scope(&self, scope: &ScopeChain) -> ScopeChain {
        match &self {
            ScopeTreeExportID::Ident(ident) => ScopeChain::Mod {
                crate_ident: scope.crate_ident().clone(),
                self_scope: Scope::new(scope.self_path_holder_ref().joined(ident), ObjectConversion::Empty),
                parent_scope_chain: Box::new(scope.clone())
            },
            ScopeTreeExportID::Impl(_, _) =>
                panic!("impl not implemented")
        }
    }
}


#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(ParentComposer<ScopeContext>, Item),
    Tree(ParentComposer<ScopeContext>, HashMap<ImportConversion, HashSet<ImportComposition>>, HashMap<ScopeTreeExportID, ScopeTreeExportItem>),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportItem::Item(_, item) =>
                f.write_str(&format!("ScopeTreeExportItem::Item({:?})", item.maybe_ident())),
            ScopeTreeExportItem::Tree(context, imported, exported) =>
                f.debug_struct("ScopeTreeExportItem::Tree")
                    .field("context", context)
                    .field("imported", &format_imported_dict(imported))
                    .field("exported", &format_tree_exported_dict(exported))
                    .finish()
        }
    }
}

impl std::fmt::Display for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeTreeExportItem {
    pub fn tree_with_context_and_export(context: ParentComposer<ScopeContext>, export: HashMap<ScopeTreeExportID, ScopeTreeExportItem>) -> Self {
        Self::Tree(context, HashMap::default(), export)
    }
    pub fn with_global_context(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>) -> Self {
        let context = Rc::new(RefCell::new(ScopeContext::with(scope, context)));
        Self::tree_with_context_and_export(context, HashMap::default())
    }

    fn add_non_mod_item(&mut self, item: &Item, scope: &ScopeChain) {
        // println!("add_non_mod_item: {} in [{}]", item.maybe_ident().map_or(format!("None"), Ident::to_string), scope);
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(
                scope_context,
                imported,
                exported) => {
                let self_scope_context = scope_context.borrow_mut();
                let mut self_scope_context = self_scope_context.clone();
                self_scope_context.scope = scope.clone();
                self_scope_context.populate_imports(item, imported);
                // TODO: We shouldn't do this at this step since we may have not yet parsed all the items
                // self_scope_context.trait_items_from_attributes(item.attrs())
                //     .into_iter()
                //     .for_each(|(item_trait, trait_scope)| {
                //         let trait_item = ItemConversion::Trait(item_trait.item, trait_scope);
                //         self_scope_context.populate_imports_and_generics(trait_item.scope_chain(), &trait_item, imported, generics);
                //     });
                exported.insert(
                    item.scope_tree_export_id(),
                    ScopeTreeExportItem::Item(Rc::new(RefCell::new(self_scope_context)), item.clone()));
            }
        }
    }

    fn add_items(&mut self, items: &Vec<Item>, scope: &ScopeChain) {
        items.iter().for_each(|item|
            if let Item::Mod(item_mod) = item {
                self.add_mod_item(item_mod, scope)
            } else {
                self.add_non_mod_item(item, scope)
            }
        );
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &ScopeChain) {
        // println!("add TREE: [{}]: {}", scope.to_token_stream(), item_mod.to_token_stream());
        match &item_mod.content {
            Some((_, items)) => {
                let ident = &item_mod.ident;
                let inner_scope = ScopeChain::new_mod(
                    scope.crate_ident().clone(),
                    Scope::new(scope.self_path_holder_ref().joined(ident), ObjectConversion::Empty),
                    scope);
                match self {
                    ScopeTreeExportItem::Item(context, _) => {
                        let mut inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone());
                        inner_tree.add_items(items, &inner_scope);
                    },
                    ScopeTreeExportItem::Tree(context, _, exported) => {
                        let mut inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone());
                        inner_tree.add_items(items, &inner_scope);
                        exported.insert(ScopeTreeExportID::from_ident(ident), inner_tree);
                    }
                }
            },
            None => {}
        }
    }

    pub fn add_item(&mut self, item: Item, scope: ScopeChain) {
        if let ScopeTreeExportItem::Tree(..) = self {
            match &item {
                Item::Use(..) => {},
                Item::Mod(item_mod) => self.add_mod_item(item_mod, &scope),
                _ => self.add_non_mod_item(&item, &scope)
            };
        }
    }
}
