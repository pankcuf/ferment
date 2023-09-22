pub mod composer;
pub mod file_visitor;
pub mod generics;
pub mod helper;
pub mod interface;
pub mod item_conversion;
pub mod path_conversion;
pub mod presentation;
#[cfg(test)]
mod test;

use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::Write;
use syn::{visit::Visit, ItemMod, PathSegment, parse_quote};
use crate::file_visitor::FileVisitor;

#[derive(Debug)]
pub enum Error {
    FileError(io::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::FileError(value)
    }
}

impl std::error::Error for Error {}


/// Reads rust file and its nested dependencies
/// Creates syntax tree which we'll use later
/// to handle imports for FFI converted types
pub fn process(input: &std::path::Path, output: &mut File) -> Result<(), Error> {
    let parent: syn::Path = parse_quote!(ffi_expansion);
    let file_visitors = process_recursive(std::path::Path::new(input), parent);
    for visitor in file_visitors {
        for expansion in visitor.make_expansions() {
            output.write_all(expansion.to_string().as_bytes())
                .map_err(Error::from)?;
        }
    }
    Ok(())
}

fn process_recursive(file_path: &std::path::Path, parent: syn::Path) -> Vec<FileVisitor> {
    let content = std::fs::read_to_string(file_path)
        .expect("Failed to read file");
    let syntax_tree = syn::parse_file(&content)
        .expect("Failed to parse file");

    let mut visitors = vec![];
    let mut visitor = FileVisitor::new(parent.clone());
    visitor.visit_file(&syntax_tree);
    visitors.push(visitor);
    // After visiting the current file, process its modules
    for item in syntax_tree.items {
        if let syn::Item::Mod(module) = item {
            visitors.extend(process_module(file_path, &module, parent.clone()));
        }
    }
    visitors
}

fn process_module(base_path: &std::path::Path, module: &ItemMod, parent: syn::Path) -> Vec<FileVisitor> {
    println!("process_module: {:?} {:?}", base_path, module);
    // Only continue if this is an inline module declaration
    if module.content.is_some() {
        return vec![];
    }

    let mod_name = &module.ident;
    let mod_path = base_path.parent().unwrap().join(&mod_name.to_string());
    let mut current_parent = parent.clone();
    current_parent.segments.push(PathSegment::from(mod_name.clone()));
    // Check if `<module_name>.rs` exists
    if mod_path.is_file() {
        return process_recursive(&mod_path, current_parent);
    } else {
        // Check for `<module_name>/mod.rs`
        let mod_dir_path = mod_path.join("mod.rs");
        if mod_dir_path.is_file() {
            return process_recursive(&mod_dir_path, current_parent);
        }
    }
    vec![]
}
