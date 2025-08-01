mod composer;
mod kind;
mod presentable;
mod composable;
mod ext;
mod presentation;
mod tree;
mod writer;

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use cargo_metadata::MetadataCommand;
use proc_macro2::Ident;
use quote::format_ident;
use syn::Attribute;
use crate::context::GlobalContext;
use crate::error;
use crate::tree::{FileTreeProcessor, ScopeTreeExportItem};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Crate {
    pub name: String,
    pub root_path: PathBuf,
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Crate: {} ({:?})", self.name, self.root_path).as_str())
    }
}
impl Crate {
    pub fn current_with_name(name: &str) -> Self {
        Self { name: name.to_string(), root_path: std::path::Path::new("src").to_path_buf() }
    }
    pub fn new(name: &str, root_path: PathBuf) -> Self {
        Self { name: name.to_string(), root_path }
    }
    pub fn ident(&self) -> Ident {
        format_ident!("{}", self.name)
    }
    pub fn root_path(&self) -> PathBuf {
        self.root_path.join("lib.rs")
    }

    pub fn process(&self, attrs: Vec<Attribute>, context: &Arc<RwLock<GlobalContext>>) -> Result<ScopeTreeExportItem, error::Error> {
        FileTreeProcessor::process_crate_tree(self, attrs, context)
    }
}

pub(crate) fn find_crates_paths(crate_names: Vec<&str>) -> Vec<Crate> {
    let metadata = MetadataCommand::new().exec().unwrap();
    crate_names.into_iter()
        .filter_map(|crate_name| {
            metadata.packages
                .iter()
                .find_map(|p| {
                    if p.name.as_str() == crate_name {
                        if let Some(target) = p.targets.first() {
                            return Some(
                                Crate::new(
                                    p.name
                                        .replace("-", "_")
                                        .as_str(),
                                    PathBuf::from(target.src_path.parent()
                                        .unwrap())))
                        }
                    }
                    None
                })
        })
        .collect()
}
