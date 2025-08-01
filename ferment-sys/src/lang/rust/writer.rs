use std::fs::File;
use std::io::Write;
use quote::ToTokens;
use crate::composer::SourceFermentable;
use crate::Error;
use crate::lang::RustSpecification;
use crate::presentation::RustFermentate;
use crate::tree::CrateTree;
use crate::writer::{CrateTreeWrite, Writer};

impl CrateTreeWrite<RustSpecification> for Writer {
    fn write(&self, crate_tree: &CrateTree) -> Result<(), Error> {
        let mut output = File::create(self.config.expansion_path())
            .map_err(Error::from)?;
        let fermentate = SourceFermentable::<RustFermentate>::ferment(crate_tree);
        output.write_all(fermentate.to_token_stream().to_string().as_bytes())
            .map_err(Error::from)
    }
}
