use std::fmt::Formatter;
use std::collections::{HashMap, HashSet};
use proc_macro2::Ident;
use std::sync::{Arc, RwLock};
use syn::{Item, ItemMod};
use crate::composition::{GenericConversion, ImportComposition};
use crate::context::{GlobalContext, ScopeContext};
use crate::conversion::{ImportConversion, ItemConversion, trait_items_from_attributes};
use crate::formatter::{format_imported_dict, format_tree_exported_dict};
use crate::holder::PathHolder;

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(ScopeContext, Item),
    Tree(ScopeContext, HashSet<GenericConversion>, HashMap<ImportConversion, HashSet<ImportComposition>>, HashMap<Ident, ScopeTreeExportItem>),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportItem::Item(..) => f.write_str("ScopeTreeExportItem::Item"),
            ScopeTreeExportItem::Tree(context, generics, imported, exported) =>
                f.debug_struct("ScopeTreeExportItem::Tree")
                    .field("context", context)
                    .field("generics", generics)
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
    pub(crate) fn tree_with_context_and_export(context: ScopeContext, export: HashMap<Ident, ScopeTreeExportItem>) -> Self {
        Self::Tree(context, HashSet::default(), HashMap::default(), export)
    }
    pub fn with_scope_context(scope_context: ScopeContext) -> ScopeTreeExportItem {
        Self::tree_with_context_and_export(scope_context, HashMap::default())
    }
    pub fn with_global_context(scope: &PathHolder, context: Arc<RwLock<GlobalContext>>) -> ScopeTreeExportItem {
        Self::tree_with_context_and_export(ScopeContext::with(scope.clone(), context), HashMap::default())
    }
    // pub fn single_export(scope: Scope, ident: Ident, item: ScopeTreeExportItem) -> ScopeTreeExportItem {
    //     Self::tree_with_context_and_export(ScopeContext::with(scope, &mut GlobalContext::default()), HashMap::from([(ident, item)]))
    // }
    // pub fn with_context(scope: &Scope, context: Context) -> ScopeTreeExportItem {
    //     Self::tree_with_context_and_export(ScopeContext::with(scope.clone(), GlobalContext::with_context(context)), HashMap::default())
    // }

    // pub fn just_export_with_context(export: HashMap<Ident, ScopeTreeExportItem>, context: Context) -> ScopeTreeExportItem {
    //     Self::tree_with_context_and_export(GlobalContext::with_context(context), export)
    // }

    fn add_non_mod_item(&mut self, item: &ItemConversion) {
        // println!("add_non_mod_item: {}", item.ident().to_token_stream());
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(
                scope_context,
                generics,
                imported,
                exported) => {
                let mut self_scope_context = scope_context.clone();
                let scope = item.scope();
                self_scope_context.scope = scope.joined(&item.ident());
                self_scope_context.populate_imports_and_generics(&self_scope_context.scope, item, imported, generics);
                trait_items_from_attributes(item.attrs(), &mut self_scope_context)
                    .into_iter()
                    .for_each(|(item_trait, trait_scope)| {
                        let trait_item = ItemConversion::Trait(item_trait.item, trait_scope);
                        self_scope_context.populate_imports_and_generics(trait_item.scope(), &trait_item, imported, generics);
                    });
                exported.insert(item.ident().clone(), ScopeTreeExportItem::Item(self_scope_context, item.into()));
            }
        }
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &PathHolder) {
        // println!("add TREE: [{}]: {}", scope.to_token_stream(), item_mod.to_token_stream());
        let context = match self {
            ScopeTreeExportItem::Item(context, _) => context.context.clone(),
            ScopeTreeExportItem::Tree(context, _, _, _) => context.context.clone()
        };
        match &item_mod.content {
            Some((_, items)) => {
                let ident = item_mod.ident.clone();
                let inner_scope = scope.joined(&ident);
                let mut inner_tree = ScopeTreeExportItem::with_global_context(scope, context);
                items.iter().for_each(|item| {
                    match ItemConversion::try_from((item, &inner_scope)) {
                        Ok(ItemConversion::Mod(item_mod, scope)) =>
                            inner_tree.add_mod_item(&item_mod, &scope),
                        Ok(inner_item) =>
                            inner_tree.add_non_mod_item(&inner_item),
                        _ => {}
                    };
                });
                match self {
                    ScopeTreeExportItem::Item(_, _) => {},
                    ScopeTreeExportItem::Tree(_, _, _, exported) => {
                        exported.insert(ident.clone(), inner_tree);
                    }
                };
            },
            None => {}
        }
    }

    pub fn add_item(&mut self, item: ItemConversion) {
        if let ScopeTreeExportItem::Tree(..) = self {
            match &item {
                ItemConversion::Use(..) => {},
                ItemConversion::Mod(item_mod, scope) =>
                    self.add_mod_item(item_mod, scope),
                _ =>
                    self.add_non_mod_item(&item)
            };
        }
    }
}
