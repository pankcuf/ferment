use std::fs::File;
use std::io::Write;
#[cfg(feature = "cbindgen")]
use std::process::Command;
use quote::ToTokens;
use crate::{Config, Crate, Error};
use crate::composer::SourceFermentable;
use crate::lang::{RustSpecification, Specification};
use crate::presentation::RustFermentate;
use crate::tree::CrateTree;

#[allow(unused)]
pub trait CrateTreeWrite<SPEC> where SPEC: Specification {
    fn write(&self, crate_tree: &CrateTree) -> Result<(), Error>;
}

pub struct Writer {
    pub(crate) config: Config
}

impl From<Config> for Writer {
    fn from(config: Config) -> Self {
        Self { config }
    }
}

impl CrateTreeWrite<RustSpecification> for Writer {
    fn write(&self, crate_tree: &CrateTree) -> Result<(), Error> {
        let mut output = File::create(self.config.expansion_path())
            .map_err(Error::from)?;
        let fermentate = SourceFermentable::<RustFermentate>::ferment(crate_tree);
        output.write_all(fermentate.to_token_stream().to_string().as_bytes())
            .map_err(Error::from)
    }
}

impl Writer {

    pub(crate) fn write_all(&self) -> Result<(), Error> {
        #[cfg(not(feature = "cbindgen_only"))]
        {
            let crate_tree = crate::tree::FileTreeProcessor::build(&self.config)?;
            CrateTreeWrite::<RustSpecification>::write(self, &crate_tree)?;
            #[cfg(feature = "objc")]
            {
                CrateTreeWrite::<crate::lang::objc::ObjCSpecification>::write(self, &crate_tree)?;
            }
        }
        #[cfg(feature = "cbindgen")]
        self.write_headers()
    }

    #[cfg(feature = "cbindgen")]
    pub(crate) fn write_headers(&self) -> Result<(), Error> {
        let Config { current_crate: Crate { name: framework, .. }, cbindgen_config_from_file, .. } = &self.config;
        let framework = {
            #[cfg(feature = "objc")]
            {
                self.config.maybe_objc_config().map(|config| config.xcode.header_name.clone()).unwrap_or(framework.clone())
            }
            #[cfg(not(feature = "objc"))]
            {
                framework.clone()
            }
        };
        Command::new("mkdir")
            .args(&["-p", "target/include"])
            .status()?;
        Command::new("cbindgen")
            .args([
                "--config", cbindgen_config_from_file.as_ref().map(String::as_str).unwrap_or("cbindgen.toml"),
                "-o", format!("target/include/{framework}.h").as_str()
            ])
            .status()
            .map_err(Error::from)
            .map(|_| ())
    }
}
