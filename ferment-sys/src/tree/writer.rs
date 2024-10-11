use std::fs::File;
use std::io::Write;
use std::process::Command;
use quote::ToTokens;
use crate::{Config, error, Crate};
use crate::ast::Depunctuated;
use crate::presentation::{Fermentate, RustFermentate};

pub trait IWriter {
    type Fermentate: ToTokens;
    fn write(&self, fermentate: Self::Fermentate) -> Result<(), error::Error>;
}

pub struct Writer {
    config: Config
}

impl From<Config> for Writer {
    fn from(config: Config) -> Self {
        Self { config }
    }
}
impl From<&Config> for Writer {
    fn from(config: &Config) -> Self {
        Self { config: config.clone() }
    }
}

impl IWriter for Writer {
    type Fermentate = RustFermentate;

    fn write(&self, fermentate: Self::Fermentate) -> Result<(), error::Error> {
        File::create(self.config.expansion_path())
            .map_err(error::Error::from)
            .and_then(|mut output| output.write_all(fermentate.to_token_stream().to_string().as_bytes())
                .map_err(error::Error::from))
    }
}

impl Writer {
    pub(crate) fn write_headers(&self) -> Result<(), error::Error> {
        let Config { current_crate: Crate { name: framework, .. }, cbindgen_config, .. } = &self.config;
        Command::new("mkdir")
            .args(&["-p", "target/include"])
            .status()?;
        Command::new("cbindgen")
            .args([
                "--config", cbindgen_config,
                "-o", format!("target/include/{framework}.h").as_str()
            ])
            .status()
            .map_err(error::Error::from)
            .map(|_| ())
    }
    pub fn write(&self, fermentate: Depunctuated<Fermentate>) -> Result<(), error::Error> {
        for f in fermentate {
            match f {
                Fermentate::Rust(fermentate) =>
                    IWriter::write(self, fermentate)?,
                #[cfg(feature = "objc")]
                Fermentate::ObjC(fermentate) => if let Some(config) = self.config.maybe_objc_config() {
                    crate::lang::objc::ObjCWriter::from(config)
                        .write(fermentate)?

                }
                _ => {}
            }
        }
        self.write_headers()
    }
}

