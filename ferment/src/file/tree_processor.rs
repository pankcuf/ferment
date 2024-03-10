use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use proc_macro2::Ident;
use syn::visit::Visit;
use crate::builder::Crate;
use crate::context::{GlobalContext, ScopeChain};
use crate::{Config, error};
use crate::presentation::Expansion;
use crate::tree::{CrateTree, ScopeTreeExportItem};
use crate::visitor::Visitor;

fn process_crates(crates: &[Crate], context: &Arc<RwLock<GlobalContext>>) -> HashMap<Crate, ScopeTreeExportItem> {
    crates
        .iter()
        .filter_map(|crate_config|
            match crate_config.process(context) {
                Ok(crate_root) => Some((crate_config.clone(), crate_root)),
                Err(err) => {
                    println!("•• CRATE TREE PROCESSING ERROR ({}) •• [{}]", crate_config.name, err);
                    None
                },
            }).collect()
}

pub struct FileTreeProcessor {
    pub path: PathBuf,
    pub scope: ScopeChain,
    pub context: Arc<RwLock<GlobalContext>>,
}

impl FileTreeProcessor {

    pub fn expand(config: &Config) -> Result<Expansion, error::Error> {
        let Config { current_crate: crate_config, external_crates, .. } = config;
        let context = Arc::new(RwLock::new(GlobalContext::from(config)));
        let external_crates = process_crates(external_crates, &context);
        Self::process_crate_tree(crate_config, &context)
            .map(|current_tree| Expansion::Root { tree: CrateTree::new(crate_config, current_tree, external_crates) })
    }

    pub fn process_crate_tree(crate_config: &Crate, context: &Arc<RwLock<GlobalContext>>) -> Result<ScopeTreeExportItem, error::Error> {
        Self::from_crate(&crate_config, &context)
            .process_()
            .map(Visitor::into_code_tree)
    }
    fn from_crate(crate_config: &Crate, context: &Arc<RwLock<GlobalContext>>) -> Self {
        Self::new(crate_config.root_path(), ScopeChain::crate_root_with_name(crate_config.ident(), crate_config.ident()), context)
    }
    pub fn new(path: PathBuf, scope: ScopeChain, context: &Arc<RwLock<GlobalContext>>) -> Self {
        Self { path, scope, context: context.clone() }
    }
    fn process_(self) -> Result<Visitor, error::Error> {
        self.read_syntax_tree()
            .map(|syntax_tree| self.setup_visitor(syntax_tree))
    }

    fn read_syntax_tree(&self) -> Result<syn::File, error::Error> {
        std::fs::read_to_string(&self.path)
            .map_err(error::Error::from)
            .and_then(|content| syn::parse_file(&content)
                .map_err(error::Error::from))
    }

    fn setup_visitor(&self, syntax_tree: syn::File) -> Visitor {
        let mut visitor = Visitor::new(self.scope.clone(), &self.context);
        visitor.visit_file(&syntax_tree);
        let mut visitors = vec![];
        for item in syntax_tree.items {
            if let syn::Item::Mod(module) = item {
                if !self.is_fermented_mod(&module.ident) && module.content.is_none() {
                    if let Ok(visitor) = self.process_module(&module.ident) {
                        visitors.push(visitor);
                    }
                }
            }
        }
        visitor.inner_visitors = visitors;
        visitor
    }

    fn process_module(&self, mod_name: &Ident) -> Result<Visitor, error::Error> {
        let scope = ScopeChain::child_mod(self.scope.crate_scope().clone(), mod_name, &self.scope);
        let file_path = self.path.parent().unwrap().join(mod_name.to_string());
        if file_path.is_file() {
            return FileTreeProcessor::new(file_path, scope, &self.context).process_();
        } else {
            let path = file_path.join(format!("mod.rs"));
            if path.is_file() {
                return FileTreeProcessor::new(path, scope, &self.context).process_()
            } else {
                let path = file_path.parent().unwrap().join(format!("{mod_name}.rs"));
                if path.is_file() {
                    return FileTreeProcessor::new(path, scope, &self.context).process_()
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
