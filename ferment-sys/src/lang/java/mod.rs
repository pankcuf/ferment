use std::fmt::{Display, Formatter};
use crate::Error;
use crate::lang::CrateTreeConsumer;
use crate::tree::CrateTree;

#[derive(Debug, Clone)]
pub struct Config {
    pub framework_name: String,
}
impl Config {
    pub fn new(framework_name: &str) -> Self {
        Self { framework_name: framework_name.to_string() }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[java::Config]\n\tframework_name: {}", self.framework_name))
    }
}

impl CrateTreeConsumer for Config {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), Error> {
        unimplemented!("")
    }
}

