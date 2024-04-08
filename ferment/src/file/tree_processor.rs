use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use proc_macro2::Ident;
use syn::{Item, ItemMod};
use syn::visit::Visit;
use crate::builder::Crate;
use crate::context::{GlobalContext, ScopeChain};
use crate::{Config, error, print_phase};
use crate::presentation::Expansion;
use crate::tree::{CrateTree, ScopeTreeExportItem};
use crate::visitor::Visitor;
pub struct FileTreeProcessor {
    pub path: PathBuf,
    pub scope: ScopeChain,
    pub context: Arc<RwLock<GlobalContext>>,
}

impl FileTreeProcessor {
    pub fn expand(config: &Config) -> Result<Expansion, error::Error> {
        let Config { current_crate: crate_config, external_crates, .. } = config;
        let context = Arc::new(RwLock::new(GlobalContext::from(config)));
        print_phase!("PHASE 0: PROCESS CRATES", "{}", config);
        process_crates(external_crates, &context)
            .and_then(|external_crates|
                Self::process_crate_tree(crate_config, &context)
                    .and_then(|current_tree| CrateTree::new(crate_config, current_tree, external_crates))
                    .map(|tree| Expansion::Root { tree }))
    }
    pub fn process_crate_tree(crate_config: &Crate, context: &Arc<RwLock<GlobalContext>>) -> Result<ScopeTreeExportItem, error::Error> {
        let path = crate_config.root_path();
        let scope = ScopeChain::crate_root_with_ident(crate_config.ident());
        Self::new(path, scope, context)
            .process()
            .map(Visitor::into_code_tree)
    }
    pub fn new(path: PathBuf, scope: ScopeChain, context: &Arc<RwLock<GlobalContext>>) -> Self {
        Self { path, scope, context: context.clone() }
    }
    fn process(self) -> Result<Visitor, error::Error> {
        print_phase!("PHASE 1: PROCESS FILE", "{:?}", self.path);
        self.read_syntax_tree()
            .map(|syntax_tree| self.setup_visitor(syntax_tree))
    }
    fn read_syntax_tree(&self) -> Result<syn::File, error::Error> {
        std::fs::read_to_string(&self.path)
            .map_err(error::Error::from)
            .and_then(|content| syn::parse_file(&content)
                .map_err(error::Error::from))
    }
    fn to_inner_visitors(&self, items: Vec<Item>) -> Vec<Visitor> {
        let mut visitors = vec![];
        for item in items {
            if let Item::Mod(ItemMod { ident: mod_name, content, .. }) = item {
                if !self.is_fermented_mod(&mod_name) && content.is_none() {
                    if let Ok(visitor) = self.process_module(&mod_name) {
                        visitors.push(visitor);
                    }
                }
            }
        }
        visitors
    }
    fn setup_visitor(&self, syntax_tree: syn::File) -> Visitor {
        let mut visitor = Visitor::new(self.scope.clone(), &self.context);
        visitor.visit_file(&syntax_tree);
        visitor.inner_visitors = self.to_inner_visitors(syntax_tree.items);
        visitor
    }
    fn process_module(&self, mod_name: &Ident) -> Result<Visitor, error::Error> {
        let scope = ScopeChain::child_mod(self.scope.crate_ident().clone(), mod_name, &self.scope);
        let file_path = self.path.parent().unwrap().join(mod_name.to_string());
        if file_path.is_file() {
            return FileTreeProcessor::new(file_path, scope, &self.context).process();
        } else {
            let path = file_path.join(format!("mod.rs"));
            if path.is_file() {
                return FileTreeProcessor::new(path, scope, &self.context).process()
            } else {
                let path = file_path.parent().unwrap().join(format!("{mod_name}.rs"));
                if path.is_file() {
                    return FileTreeProcessor::new(path, scope, &self.context).process()
                }
            }
        }
        Err(error::Error::ExpansionError("Can't locate module file"))
    }
    fn is_fermented_mod(&self, ident: &Ident) -> bool {
        self.context.read()
            .unwrap()
            .is_fermented_mod(ident)
    }
}

fn process_crates(crates: &[Crate], context: &Arc<RwLock<GlobalContext>>) -> Result<HashMap<Crate, ScopeTreeExportItem>, error::Error> {
    crates.iter()
        .try_fold(HashMap::new(), |mut acc, crate_config| {
            acc.insert(crate_config.clone(), crate_config.process(context)?);
            Ok(acc)
        })
}
