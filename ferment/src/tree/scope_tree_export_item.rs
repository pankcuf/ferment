use std::cell::RefCell;
use std::fmt::Formatter;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use proc_macro2::Ident;
use std::sync::{Arc, RwLock};
use quote::ToTokens;
use syn::{Item, ItemMod, Path, Type};
use crate::composition::{GenericConversion, ImportComposition};
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeContext};
use crate::conversion::{ImportConversion, ObjectConversion};
use crate::formatter::{format_imported_dict, format_token_stream, format_tree_exported_dict};
use crate::helper::ItemExtension;


#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ScopeTreeExportID {
    Ident(Ident),
    Impl(Type, Option<Path>)
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


#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(Rc<RefCell<ScopeContext>>, Item),
    Tree(Rc<RefCell<ScopeContext>>, HashSet<GenericConversion>, HashMap<ImportConversion, HashSet<ImportComposition>>, HashMap<ScopeTreeExportID, ScopeTreeExportItem>),
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
    pub(crate) fn tree_with_context_and_export(context: Rc<RefCell<ScopeContext>>, export: HashMap<ScopeTreeExportID, ScopeTreeExportItem>) -> Self {
        Self::Tree(context, HashSet::default(), HashMap::default(), export)
    }
    pub fn with_scope_context(scope_context: Rc<RefCell<ScopeContext>>) -> ScopeTreeExportItem {
        Self::tree_with_context_and_export(scope_context, HashMap::default())
    }
    pub fn with_global_context(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>) -> ScopeTreeExportItem {
        let context = Rc::new(RefCell::new(ScopeContext::with(scope, context)));
        Self::tree_with_context_and_export(context, HashMap::default())
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

    fn add_non_mod_item(&mut self, item: &Item, scope: &ScopeChain) {
        // println!("add_non_mod_item: {} in [{}]", item.maybe_ident().map_or(format!("None"), Ident::to_string), scope);
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(
                scope_context,
                generics,
                imported,
                exported) => {
                let self_scope_context = scope_context.borrow_mut();
                let mut self_scope_context = self_scope_context.clone();
                // let scope = item.scope_chain();
                // self_scope_context.scope = scope.joined(&item.ident());
                self_scope_context.scope = scope.clone();
                self_scope_context.populate_imports_and_generics(scope, item, imported, generics);
                // TODO: We shouldn't do this at this step since we may have not yet parsed all the items
                // self_scope_context.trait_items_from_attributes(item.attrs())
                //     .into_iter()
                //     .for_each(|(item_trait, trait_scope)| {
                //         let trait_item = ItemConversion::Trait(item_trait.item, trait_scope);
                //         self_scope_context.populate_imports_and_generics(trait_item.scope_chain(), &trait_item, imported, generics);
                //     });
                exported.insert(item.scope_tree_export_id(), ScopeTreeExportItem::Item(Rc::new(RefCell::new(self_scope_context)), item.clone()));
            }
        }
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &ScopeChain) {
        // println!("add TREE: [{}]: {}", scope.to_token_stream(), item_mod.to_token_stream());
        let context = match self {
            ScopeTreeExportItem::Item(context, _) => context.borrow().context.clone(),
            ScopeTreeExportItem::Tree(context, _, _, _) => context.borrow().context.clone()
        };
        match &item_mod.content {
            Some((_, items)) => {
                let ident = item_mod.ident.clone();
                let self_scope = scope.self_scope();
                let inner_scope = ScopeChain::Mod {
                    self_scope: Scope::new(
                        self_scope.self_scope.joined(&ident),
                        ObjectConversion::Empty)
                };
                let mut inner_tree = ScopeTreeExportItem::with_global_context(scope.clone(), context);
                items.iter().for_each(|item| {
                    if let Item::Mod(item_mod) = item {
                        inner_tree.add_mod_item(&item_mod, &inner_scope)
                    } else {
                        inner_tree.add_non_mod_item(item, &inner_scope)
                    }
                });
                match self {
                    ScopeTreeExportItem::Item(_, _) => {},
                    ScopeTreeExportItem::Tree(_, _, _, exported) => {
                        exported.insert(ScopeTreeExportID::Ident(ident.clone()), inner_tree);
                    }
                };
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
