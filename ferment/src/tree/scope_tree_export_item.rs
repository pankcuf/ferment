use std::cell::RefCell;
use std::fmt::{Formatter, Write};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use syn::{Item, ItemMod};
use crate::composer::ParentComposer;
use crate::composition::ImportComposition;
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeContext};
use crate::conversion::ImportConversion;
use crate::formatter::format_tree_exported_dict;
use crate::helper::ItemExtension;
use crate::tree::ScopeTreeExportID;


#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(ParentComposer<ScopeContext>, Item),
    Tree(ParentComposer<ScopeContext>, HashMap<ImportConversion, HashSet<ImportComposition>>, HashMap<ScopeTreeExportID, ScopeTreeExportItem>),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // match self {
        //     ScopeTreeExportItem::Item(_, item) =>
        //         f.write_str(&format!("ScopeTreeExportItem::Item({:?})", item.maybe_ident())),
        //     ScopeTreeExportItem::Tree(context, imported, exported) =>
        //         f.debug_struct("ScopeTreeExportItem::Tree")
        //             .field("context", context)
        //             .field("imported", &format_imported_dict(imported))
        //             .field("exported", &format_tree_exported_dict(exported))
        //             .finish()
        // }
        match self {
            ScopeTreeExportItem::Item(_, item) =>
                f.write_str(&format!("ScopeTreeExportItem::Item({})", item.ident_string())),
            ScopeTreeExportItem::Tree(context, imported, exported) =>
                f.write_str(&format!("ScopeTreeExportItem::Tree(\n\t{})", format_tree_exported_dict(exported)))
        }
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
    pub fn tree_with_context_and_export(context: ParentComposer<ScopeContext>, export: HashMap<ScopeTreeExportID, ScopeTreeExportItem>) -> Self {
        Self::Tree(context, HashMap::default(), export)
    }
    pub fn with_global_context(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>) -> Self {
        let context = Rc::new(RefCell::new(ScopeContext::with(scope, context)));
        Self::tree_with_context_and_export(context, HashMap::default())
    }

    fn add_non_mod_item(&mut self, item: &Item, scope: &ScopeChain) {
        // println!("add_non_mod_item: {} in [{}]", item.maybe_ident().map_or(format!("None"), Ident::to_string), scope);
        // let b = self.scope();
        // let self_scope = b.self_path_holder_ref();
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(
                scope_context,
                imported,
                exported) => {
                let self_scope_context_ref = scope_context.borrow_mut();
                // println!("add_non_mod_item: [{}]: {}: [{}]", self_scope, item.ident_string(),  scope.self_path_holder_ref());
                let mut self_scope_context = ScopeContext::with(scope.clone(), self_scope_context_ref.context.clone());
                self_scope_context.populate_imports(item, imported);
                // TODO: We shouldn't do this at this step since we may have not yet parsed all the items
                // self_scope_context.trait_items_from_attributes(item.attrs())
                //     .into_iter()
                //     .for_each(|(item_trait, trait_scope)| {
                //         let trait_item = ItemConversion::Trait(item_trait.item, trait_scope);
                //         self_scope_context.populate_imports_and_generics(trait_item.scope_chain(), &trait_item, imported, generics);
                //     });
                // println!(" •• {}", item.scope_tree_export_id(), )
                exported.insert(item.scope_tree_export_id(), ScopeTreeExportItem::Item(Rc::new(RefCell::new(self_scope_context)), item.clone()));
            }
        }
        // println!("add_non_mod_item (added): {} in [{}]", item.ident_string(), self);

    }

    fn add_items(&mut self, items: &Vec<Item>, scope: &ScopeChain) {
        // println!("add_items: {}", scope.self_path_holder_ref());
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
                // Item::Use(item_use) =
                _ => {}
            }
        );
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &ScopeChain) {
        // println!("add_mod_item: {}: [{}]", item_mod.ident, scope.self_path_holder_ref());
        let ident = &item_mod.ident;
        // let inner_scope = ScopeChain::child_mod(scope.crate_ident().clone(), ident, scope);
        match &item_mod.content {
            Some((_, items)) => {
                match self {
                    ScopeTreeExportItem::Item(context, _) => {
                        // println!("add_mod_item.0.0: {}: [{}]", item_mod.ident, scope.self_path_holder_ref());
                        let mut inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone());
                        inner_tree.add_items(items, scope);
                    },
                    ScopeTreeExportItem::Tree(context, _, exported) => {
                        // println!("add_mod_item.0.1: {}: [{}]", item_mod.ident, scope.self_path_holder_ref());
                        let mut inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone());
                        inner_tree.add_items(items, scope);
                        exported.insert(ScopeTreeExportID::from_ident(ident), inner_tree);
                    }
                }
            },
            None => {
                match self {
                    ScopeTreeExportItem::Item(..) => {
                        // println!("add_mod_item.1.0: {}: [{}]", item_mod.ident, scope.self_path_holder_ref());
                    },
                    ScopeTreeExportItem::Tree(context, _, exported) => {
                        // println!("add_mod_item.1.1: {}: [{}]", item_mod.ident, scope.self_path_holder_ref());
                        let inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context.borrow().context.clone());
                        exported.insert(ScopeTreeExportID::from_ident(ident), inner_tree);
                    }
                }

            }
        }
    }

    pub fn add_item(&mut self, item: Item, scope: ScopeChain) {
        // println!("add_item: {}: [{}]", item.ident_string(), scope.self_path_holder_ref());
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
}
