use crate::{Config, Crate, error, Lang};
use crate::lang::rust::find_crates_paths;
use crate::writer::Writer;

extern crate env_logger;

pub const DEFAULT_FERMENTATE_MOD: &str = "fermented";

#[derive(Debug, Clone)]
pub struct Builder {
    config: Config,
}

impl Builder {
    pub fn new(current_crate: Crate) -> Builder {
        env_logger::init();
        Builder { config: Config::new(DEFAULT_FERMENTATE_MOD, current_crate, Default::default()) }
    }
    #[allow(unused)]
    pub fn with_crate_name(crate_name: &str) -> Builder {
        Self::new(Crate::current_with_name(crate_name))
    }

    #[allow(unused)]
    pub fn with_default_mod_name(mut self) -> Builder {
        self.config.mod_name = String::from(DEFAULT_FERMENTATE_MOD);
        self
    }
    #[allow(unused)]
    pub fn with_cbindgen_config(mut self, config: cbindgen::Config) -> Builder {
        self.config.cbindgen_config = config;
        self
    }
    #[allow(unused)]
    pub fn with_cbindgen_config_from_file(mut self, config: &'static str) -> Builder {
        self.config.cbindgen_config_from_file = Some(config.to_string());
        self
    }
    #[allow(unused)]
    pub fn with_mod_name<S: AsRef<str>>(mut self, mod_name: S) -> Builder {
        self.config.mod_name = String::from(mod_name.as_ref());
        self
    }

    #[allow(unused)]
    pub fn with_crates(mut self, crates: Vec<&str>) -> Builder {
        self.config.external_crates = find_crates_paths(crates);
        self
    }
    #[allow(unused)]
    pub fn with_external_crates(mut self, crates: Vec<&str>) -> Builder {
        self.config.external_crates = find_crates_paths(crates);
        self
    }

    #[allow(unused)]
    pub fn with_languages(mut self, languages: Vec<Lang>) -> Builder {
        self.config.languages = languages;
        self
    }

    /// Reads rust file and its nested dependencies
    /// Creates syntax tree which we'll use later
    /// to handle imports for FFI converted types
    /// `mod_name`: mod with this name will be created in `src/{mod_name}.rs`
    ///
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
    /// # extern crate ferment_sys;
    /// use ferment_sys::{Crate, Ferment, Lang};
    /// let mut languages = vec![];
    /// #[cfg(feature = "objc")]
    /// languages.push(Lang::ObjC(ferment::ObjC::new("DS", "Fermented")));
    /// #[cfg(feature = "java")]
    /// languages.push(Lang::Java(ferment::Java::new("Fermented")));
    /// Ferment::with_crate_name("your_crate_name")
    ///     .with_default_mod_name()
    ///     .with_crates(vec![])
    ///     .with_languages(languages)
    ///     .generate()
    ///     .expect("Fermentation fault");
    /// ```
    ///
    /// # Remarks
    ///
    /// This function expects the primary Rust file to be named `lib.rs` and located inside
    /// the `src/` directory. Any deviation from this naming and structure might lead to errors.
    ///
    /// The resulting module will only contain the necessary imports and types suitable for FFI kind.
    ///
    pub fn generate(self) -> Result<(), error::Error> {
        Writer::from(self.config)
            .write_all()
    }
}

