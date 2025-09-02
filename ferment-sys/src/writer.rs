use crate::{Config, Error};
use crate::lang::Specification;
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


impl Writer {

    pub(crate) fn write_all(&self) -> Result<(), Error> {
        #[cfg(not(feature = "cbindgen_only"))]
        {
            let crate_tree = crate::tree::FileTreeProcessor::build(&self.config)?;
            CrateTreeWrite::<crate::lang::RustSpecification>::write(self, &crate_tree)?;
            #[cfg(feature = "objc")]
            {
                CrateTreeWrite::<crate::lang::objc::ObjCSpecification>::write(self, &crate_tree)?;
            }
        }
        #[cfg(feature = "cbindgen")]
        self.write_headers()
    }
}
