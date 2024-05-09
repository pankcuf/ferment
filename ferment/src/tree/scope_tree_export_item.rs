use std::cell::RefCell;
use std::fmt::{Formatter, Write};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use syn::{Attribute, Item, ItemMod};
use crate::composer::ParentComposer;
use crate::composition::ImportComposition;
use crate::context::{GlobalContext, ScopeChain, ScopeContext};
use crate::conversion::ImportConversion;
use crate::formatter::{format_imported_dict, format_tree_exported_dict};
use crate::helper::ItemExtension;
use crate::tree::ScopeTreeExportID;


#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(ParentComposer<ScopeContext>, Item),
    Tree(ParentComposer<ScopeContext>, HashMap<ImportConversion, HashSet<ImportComposition>>, HashMap<ScopeTreeExportID, ScopeTreeExportItem>, Vec<Attribute>),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportItem::Item(_, item) =>
                f.write_str(&format!("ScopeTreeExportItem::Item({:?})", item.maybe_ident())),
            ScopeTreeExportItem::Tree(context, imported, exported, attrs) =>
                f.debug_struct("ScopeTreeExportItem::Tree")
                    .field("context", context)
                    .field("imported", &format_imported_dict(imported))
                    .field("exported", &format_tree_exported_dict(exported))
                    .field("attrs", attrs)
                    .finish()
        }
        // match self {
        //     ScopeTreeExportItem::Item(_, item) =>
        //         f.write_str(&format!("ScopeTreeExportItem::Item({})", item.ident_string())),
        //     ScopeTreeExportItem::Tree(context, imported, exported) =>
        //         f.write_str(&format!("ScopeTreeExportItem::Tree(\n\t{})", format_tree_exported_dict(exported)))
        // }
    }
}

impl std::fmt::Display for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeTreeExportItem {
    pub fn scope(&self) -> ScopeChain {
        match self {
            ScopeTreeExportItem::Item(ctx, ..) => ctx.borrow().scope.clone(),
            ScopeTreeExportItem::Tree(ctx, ..) => ctx.borrow().scope.clone(),
        }
    }
    pub fn tree_with_context_and_export(context: ParentComposer<ScopeContext>, export: HashMap<ScopeTreeExportID, ScopeTreeExportItem>, attrs: Vec<Attribute>) -> Self {
        Self::Tree(context, HashMap::default(), export, attrs)
    }
    pub fn with_global_context(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>, attrs: Vec<Attribute>) -> Self {
        let context = Rc::new(RefCell::new(ScopeContext::with(scope, context)));
        Self::tree_with_context_and_export(context, HashMap::default(), attrs)
    }
    pub fn add_item(&mut self, item: Item, scope: ScopeChain) {
        if let ScopeTreeExportItem::Tree(..) = self {
            match &item {
                Item::Use(..) => {},
                Item::Mod(item_mod) => self.add_mod_item(item_mod, &scope),
                Item::Trait(..) |
                Item::Fn(..) |
                Item::Struct(..) |
                Item::Enum(..) |
                Item::Type(..) |
                Item::Impl(..) => self.add_non_mod_item(&item, &scope),
                _ => {}
            };
        }
    }

    fn add_non_mod_item(&mut self, item: &Item, scope: &ScopeChain) {
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(
                scope_context,
                imported,
                exported,
                _attrs) => {
                exported.insert(
                    item.scope_tree_export_id(),
                    ScopeTreeExportItem::Item(
                        Rc::new(RefCell::new(ScopeContext::populated(
                            scope.clone(),
                            item,
                            imported,
                            scope_context.borrow().context.clone()))),
                        item.clone()));
            }
        }
    }

    fn add_items(&mut self, items: &Vec<Item>, scope: &ScopeChain) {
        items.iter().for_each(|item|
            match item {
                Item::Mod(item_mod) =>
                    self.add_mod_item(item_mod, scope),
                Item::Const(_) |
                Item::Enum(_) |
                Item::Fn(_) |
                Item::Impl(_) |
                Item::Struct(_) |
                Item::Trait(_) |
                Item::Type(_) => self.add_non_mod_item(item, scope),
                _ => {}
            }
        );
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &ScopeChain) {
        let ident = &item_mod.ident;
        match &item_mod.content {
            Some((_, items)) => {
                match self {
                    ScopeTreeExportItem::Item(context, _) => {
                        let mut inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone(), item_mod.attrs.clone());
                        inner_tree.add_items(items, scope);
                    },
                    ScopeTreeExportItem::Tree(context, _, exported, _) => {
                        let mut inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone(), item_mod.attrs.clone());
                        inner_tree.add_items(items, scope);
                        exported.insert(ScopeTreeExportID::from_ident(ident), inner_tree);
                    }
                }
            },
            None => {
                match self {
                    ScopeTreeExportItem::Item(..) => {},
                    ScopeTreeExportItem::Tree(context, _, exported, _) => {
                        let inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone(), item_mod.attrs.clone());
                        exported.insert(ScopeTreeExportID::from_ident(ident), inner_tree);
                    }
                }

            }
        }
    }
}
