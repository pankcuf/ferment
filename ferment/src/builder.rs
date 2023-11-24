use std::fs::File;
use std::io::Write;
use quote::{format_ident, quote};
use syn::{visit::Visit, ItemMod, parse_quote};
use crate::context::Context;
use crate::error;
use crate::interface::Presentable;
use crate::presentation::Expansion;
use crate::visitor::{merge_visitor_trees, Visitor};
use crate::scope::Scope;
use crate::scope_conversion::ScopeTreeCompact;

#[derive(Debug, Clone)]
pub struct Builder {
    config: Config,
}
impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub mod_name: String,
    pub crate_names: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { mod_name: String::from("fermented"), crate_names: vec![] }
    }
}

impl Config {
    pub fn new(mod_name: &'static str) -> Self {
        Self { mod_name: String::from(mod_name), crate_names: vec![] }
    }
}

impl Builder {

    pub fn new() -> Builder {
        Builder { config: Config::default() }
    }

    #[allow(unused)]
    pub fn with_mod_name<S: AsRef<str>>(mut self, mod_name: S) -> Builder {
        self.config.mod_name = String::from(mod_name.as_ref());
        self
    }

    #[allow(unused)]
    pub fn with_crates(mut self, crates: Vec<String>) -> Builder {
        self.config.crate_names = crates;
        self
    }


    /// Reads rust file and its nested dependencies
    /// Creates syntax tree which we'll use later
    /// to handle imports for FFI converted types
    /// `mod_name`: mod with this name will be created in `src/{mod_name}.rs`

    /// Recursively reads a Rust project file tree and its nested dependencies to generate a syntax tree.
    ///
    /// This function will traverse the primary Rust file and its dependencies to generate
    /// a syntax tree. This tree is later utilized to manage imports for types that are
    /// converted for FFI.
    ///
    /// The resulting code will be written into a new module file in the `src/` directory.
    ///
    /// # Arguments
    ///
    /// * `mod_name`: The name of the module to be created. The resulting file will be
    ///   named `{mod_name}.rs` and will be located inside the `src/` directory.
    ///
    /// # Errors
    ///
    /// If the function encounters any errors while reading the file, processing the syntax,
    /// or writing to the output file, it will return an `error::Error`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate ferment;
    /// ferment::Builder::new().with_mod_name("mod_name").generate().expect("Failed to process the Rust file and its dependencies");
    /// ```
    ///
    /// # Remarks
    ///
    /// This function expects the primary Rust file to be named `lib.rs` and located inside
    /// the `src/` directory. Any deviation from this naming and structure might lead to errors.
    ///
    /// The resulting module will only contain the necessary imports and types suitable for FFI conversion.
    ///
    pub fn generate(self) -> Result<(), error::Error> {
        let src = std::path::Path::new("src");
        let output_path = src.join(format!("{}.rs", self.config.mod_name));
        File::create(output_path.as_path())
            .map_err(error::Error::from)
            .and_then(|mut output| {
                let input_path = src.join("lib.rs");
                let input = input_path.as_path();
                let file_path = std::path::Path::new(input);
                let root_scope = Scope::new(parse_quote!(crate));
                let mut root_visitor = process_recursive(file_path, root_scope, &self.config);
                merge_visitor_trees(&mut root_visitor);
                ScopeTreeCompact::init_with(
                    root_visitor.tree,
                    Scope::crate_root(),
                    Context::new(self.config.crate_names))
                    .map_or(
                        Err(error::Error::ExpansionError("Can't expand root tree")),
                        |tree|
                            output.write_all(
                                Expansion::from(tree)
                                    .present()
                                    .to_string()
                                    .as_bytes())
                                    .map_err(error::Error::from))
            })

    }
}

fn read_syntax_tree(file_path: &std::path::Path) -> syn::File {
    match std::fs::read_to_string(file_path) {
        Ok(content) => match syn::parse_file(&content) {
            Ok(file) => file,
            Err(err) => panic!("Failed to parse file: {:?}: {}", file_path, err)
        },
        Err(err) => panic!("Failed to read file: {:?}: {}", file_path, err)
    }
}

fn process_recursive(file_path: &std::path::Path, scope: Scope, config: &Config) -> Visitor {
    let syntax_tree = read_syntax_tree(file_path);
    let mut visitor = Visitor::new(scope.clone(), config);
    visitor.visit_file(&syntax_tree);
    let items = syntax_tree.items;
    let mut visitors = vec![];
    for item in items {
        if let syn::Item::Mod(module) = item {
            if module.ident != format_ident!("{}", config.mod_name) {
                if let Some(visitor) = process_module(file_path, &module, scope.clone(), config) {
                    visitors.push(visitor);
                }
            }
        }
    }
    visitor.inner_visitors = visitors;
    visitor
}

fn process_module(base_path: &std::path::Path, module: &ItemMod, file_scope: Scope, config: &Config) -> Option<Visitor> {
    if module.content.is_none() {
        let mod_name = &module.ident;
        let file_path = base_path.parent().unwrap().join(mod_name.to_string());
        let scope = file_scope.joined(mod_name);
        if file_path.is_file() {
            return Some(process_recursive(&file_path, scope, config));
        } else {
            let mod_dir_path = file_path.join("mod.rs");
            if mod_dir_path.is_file() {
                return Some(process_recursive(&mod_dir_path, scope, config));
            } else {
                let file_path = file_path.parent().unwrap().join(format!("{}.rs", quote!(#mod_name)));
                if file_path.is_file() {
                    return Some(process_recursive(&file_path, scope, config));
                }
            }
        }
    }
    None
}
