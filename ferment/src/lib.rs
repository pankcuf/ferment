pub mod composer;
pub mod error;
pub mod generics;
pub mod helper;
pub mod import_conversion;
pub mod interface;
pub mod item_conversion;
pub mod path_conversion;
pub mod presentation;
pub mod scope;
pub mod scope_conversion;
pub mod type_conversion;
pub mod visitor;
#[cfg(test)]
mod test;

use std::fs::File;
use std::io::Write;
use quote::{format_ident, quote};
use syn::{visit::Visit, ItemMod, parse_quote};
use crate::interface::Presentable;
use crate::presentation::Expansion;
use crate::visitor::{merge_visitor_trees, Visitor};
use crate::scope::Scope;
use crate::scope_conversion::ScopeTreeCompact;


/// Reads rust file and its nested dependencies
/// Creates syntax tree which we'll use later
/// to handle imports for FFI converted types
pub fn build(mod_name: &str) -> Result<(), error::Error> {
    let src = std::path::Path::new("src");
    let output_path = src.join(format!("{}.rs", mod_name));
    File::create(output_path.as_path())
        .map_err(error::Error::from)
        .and_then(|mut output| {
            let input_path = src.join("lib.rs");
            let input = input_path.as_path();
            let file_path = std::path::Path::new(input);
            let root_scope = Scope::new(parse_quote!(crate));
            let mut root_visitor = process_recursive(file_path, root_scope);
            merge_visitor_trees(&mut root_visitor);
            ScopeTreeCompact::init_with(root_visitor.tree, Scope::crate_root())
                .map_or(
                    Err(error::Error::ExpansionError("Can't expand root tree")),
                    |tree|
                        output.write_all(Expansion::from(tree)
                            .present()
                            .to_string()
                            .as_bytes())
                            .map_err(error::Error::from))
        })
}

fn read_syntax_tree(file_path: &std::path::Path) -> syn::File {
    let content = std::fs::read_to_string(file_path)
        .expect("Failed to read file");
    syn::parse_file(&content)
        .expect("Failed to parse file")
}

fn process_recursive(file_path: &std::path::Path, scope: Scope) -> Visitor {
    let syntax_tree = read_syntax_tree(file_path);
    let mut visitor = Visitor::new(scope.clone());
    visitor.visit_file(&syntax_tree);
    let items = syntax_tree.items;
    let mut visitors = vec![];
    for item in items {
        if let syn::Item::Mod(module) = item {
            if module.ident != format_ident!("fermented") {
                if let Some(visitor) = process_module(file_path, &module, scope.clone()) {
                    visitors.push(visitor);
                }
            }
        }
    }
    visitor.inner_visitors = visitors;
    visitor
}

fn process_module(base_path: &std::path::Path, module: &ItemMod, file_scope: Scope) -> Option<Visitor> {
    match &module.content {
        Some((_token, _items)) => {
            None
        },
        None => {
            let mod_name = &module.ident;
            let file_path = base_path.parent().unwrap().join(&mod_name.to_string());
            let scope = file_scope.joined(mod_name);
            if file_path.is_file() {
                return Some(process_recursive(&file_path, scope));
            } else {
                let mod_dir_path = file_path.join("mod.rs");
                if mod_dir_path.is_file() {
                    return Some(process_recursive(&mod_dir_path, scope));
                } else {
                    let file_path = file_path.parent().unwrap().join(format!("{}.rs", quote!(#mod_name)));
                    if file_path.is_file() {
                        return Some(process_recursive(&file_path, scope));
                    }
                }
            }
            None
        }
    }

}
