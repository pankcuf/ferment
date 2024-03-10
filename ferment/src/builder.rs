use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use crate::context::GlobalContext;
use crate::error;
use crate::file::FileTreeProcessor;
use crate::tree::ScopeTreeExportItem;

#[derive(Debug, Clone)]
pub struct Builder {
    config: Config,
}
#[derive(Debug, Clone)]
pub struct Config {
    pub mod_name: String,
    pub current_crate: Crate,
    pub external_crates: Vec<Crate>,
    pub languages: Vec<Language>,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Crate {
    pub name: String,
    pub root_path: PathBuf,
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

    pub fn process(&self, context: &Arc<RwLock<GlobalContext>>) -> Result<ScopeTreeExportItem, error::Error> {
        FileTreeProcessor::process_crate_tree(self, context)
    }
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

impl Config {
    pub fn new(mod_name: &'static str, current_crate: Crate) -> Self {
        Self { mod_name: String::from(mod_name), current_crate, external_crates: vec![], languages: vec![] }
    }
    pub(crate) fn contains_fermented_crate(&self, ident: &Ident) -> bool {
        self.external_crates.iter()
            .find(|c| c.ident().eq(ident))
            .is_some()
    }

    pub fn is_current_crate(&self, crate_name: &Ident) -> bool {
        self.current_crate.ident().eq(crate_name)
    }
    pub fn expansion_path(&self) -> PathBuf {
        self.current_crate.root_path.join(format!("{}.rs", self.mod_name))
    }
}

impl Builder {
    pub fn new(current_crate: Crate) -> Builder {
        Builder { config: Config::new("fermented", current_crate) }
    }

    #[allow(unused)]
    pub fn with_mod_name<S: AsRef<str>>(mut self, mod_name: S) -> Builder {
        self.config.mod_name = String::from(mod_name.as_ref());
        self
    }

    #[allow(unused)]
    pub fn with_crates(mut self, crates: Vec<Crate>) -> Builder {
        self.config.external_crates = crates;
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
    /// use ferment::builder::Crate;
    /// ferment::Builder::new(Crate::current_with_name("current_crate"))
    /// .with_mod_name("mod_name")
    /// .with_crates(vec![])
    /// .generate()
    /// .expect("Failed to process the Rust file and its dependencies");
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
        FileTreeProcessor::expand(&self.config)
            .and_then(|expansion| {
                File::create(self.config.expansion_path())
                    .map_err(error::Error::from)
                    .and_then(|mut output| output.write_all(expansion.to_token_stream().to_string().as_bytes())
                        .map_err(error::Error::from))
            })
    }
}