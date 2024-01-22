use std::cell::RefCell;
use std::fmt::Formatter;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use proc_macro2::Ident;
use crate::composition::{GenericConversion, ImportComposition};
use crate::context::ScopeContext;
use crate::conversion::ImportConversion;
use crate::formatter::{format_imported_dict, format_tree_exported_dict};
use crate::holder::PathHolder;
use crate::tree::ScopeTreeExportItem;

pub struct ScopeTreeCompact {
    pub scope: PathHolder,
    pub generics: HashSet<GenericConversion>,
    pub imported: HashMap<ImportConversion, HashSet<ImportComposition>>,
    pub exported: HashMap<Ident, ScopeTreeExportItem>,
    pub scope_context: Rc<RefCell<ScopeContext>>,
}

impl std::fmt::Debug for ScopeTreeCompact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopeTreeCompact")
            .field("generics", &self.generics)
            .field("imported", &format_imported_dict(&self.imported))
            .field("exported", &format_tree_exported_dict(&self.exported))
            .field("scope_context", &self.scope_context)
            .finish()
    }
}

impl ScopeTreeCompact {
    pub fn init_with(item: ScopeTreeExportItem, scope: PathHolder) -> Option<Self> {
        match item {
            ScopeTreeExportItem::Item(..) =>
                None,
            ScopeTreeExportItem::Tree(
                scope_context,
                generics,
                imported,
                exported) =>
                Some(ScopeTreeCompact { scope, scope_context, generics, imported, exported })
        }
    }
}
