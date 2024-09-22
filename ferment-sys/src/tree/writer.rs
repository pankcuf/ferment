use std::fs::File;
use std::io::Write;
use quote::ToTokens;
use crate::{Config, error};
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
    pub fn write(&self, fermentate: Depunctuated<Fermentate>) -> Result<(), error::Error> {
        for f in fermentate {
            match f {
                Fermentate::Rust(fermentate) =>
                    IWriter::write(self, fermentate)?,
                #[cfg(feature = "objc")]
                Fermentate::ObjC(fermentate) => if let Some(config) = self.config.maybe_objc_config() {
                    crate::lang::objc::ObjCWriter::new(config.clone()).write(fermentate)?
                }
                _ => {}
            }
        }
        Ok(())
    }
}