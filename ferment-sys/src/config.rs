use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use proc_macro2::Ident;
use crate::{Crate, Lang};

#[derive(Debug, Clone)]
pub struct Config {
    pub mod_name: String,
    pub current_crate: Crate,
    pub external_crates: Vec<Crate>,
    pub languages: Vec<Lang>,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[Config]\n\tcrate: {:?}\n\texternal: {:?}", self.current_crate, self.external_crates))
    }
}


impl Config {
    pub fn new(mod_name: &'static str, current_crate: Crate) -> Self {
        Self { mod_name: String::from(mod_name), current_crate, external_crates: vec![], languages: vec![] }
    }
    pub fn expansion_path(&self) -> PathBuf {
        self.current_crate.root_path.join(format!("{}.rs", self.mod_name))
    }
    pub(crate) fn contains_fermented_crate(&self, ident: &Ident) -> bool {
        self.external_crates.iter()
            .find(|c| c.ident().eq(ident))
            .is_some()
    }

    pub(crate) fn is_current_crate(&self, crate_name: &Ident) -> bool {
        self.current_crate.ident().eq(crate_name)
    }

}

#[cfg(feature = "objc")]
impl Config {
    pub fn maybe_objc_config(&self) -> Option<&crate::lang::objc::Config> {
        self.languages.iter().find_map(|lang| match lang {
            Lang::ObjC(config) => Some(config),
            #[cfg(feature = "java")]
            _ => None
        })
    }
}
