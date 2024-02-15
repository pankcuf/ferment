use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::visit::Visit;
use crate::context::{GlobalContext, ScopeChain};
use crate::error;
use crate::presentation::Expansion;
use crate::visitor::Visitor;

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
    pub languages: Vec<Language>,
}

#[derive(Debug, Clone)]
pub enum Language {
    ObjC,
    Java
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


impl Default for Config {
    fn default() -> Self {
        Self { mod_name: String::from("fermented"), crate_names: vec![], languages: vec![] }
    }
}

impl Config {
    pub fn new(mod_name: &'static str) -> Self {
        Self { mod_name: String::from(mod_name), crate_names: vec![], languages: vec![] }
    }
    pub fn contains_fermented_crate(&self, ident: &String) -> bool {
        self.crate_names.contains(ident)
    }

    pub fn mod_file_name(&self) -> String {
        format!("{}.rs", self.mod_name)
    }
}

pub struct FileTreeProcessor {
    pub path: PathBuf,
    pub scope: ScopeChain,
    pub context: Arc<RwLock<GlobalContext>>,
}

impl FileTreeProcessor {
    pub fn new(path: PathBuf, scope: ScopeChain, context: &Arc<RwLock<GlobalContext>>) -> Self {
        FileTreeProcessor { path, scope, context: context.clone() }
    }
    pub fn process(self) -> Result<Visitor, error::Error> {
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
        let scope = ScopeChain::child_mod(mod_name, &self.scope);
        let file_path = self.path.parent().unwrap().join(mod_name.to_string());
        if file_path.is_file() {
            return FileTreeProcessor::new(file_path, scope, &self.context).process();
        } else {
            let path = file_path.join("mod.rs");
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

    #[allow(unused)]
    pub fn with_languages(mut self, languages: Vec<Language>) -> Builder {
        self.config.languages = languages;
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
        let scope = ScopeChain::crate_root();
        let context = Arc::new(RwLock::new(GlobalContext::from(&self.config)));
        FileTreeProcessor::new(src.join("lib.rs"), scope, &context)
            .process()
            .map(|root| root.into_code_tree().into_expansion())
            .and_then(|expansion| write_expansion(src.join(self.config.mod_file_name()), expansion))
    }
}

fn write_expansion(path: PathBuf, expansion: Expansion) -> Result<(), error::Error> {
    File::create(path)
        .map_err(error::Error::from)
        .and_then(|mut output| output.write_all(expansion.to_token_stream().to_string().as_bytes())
            .map_err(error::Error::from))
}

