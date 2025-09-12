mod fermented;
pub mod aa;
pub mod zz;
pub mod blockdata;

pub use crate::blockdata::script::{self, ScriptBuf};

extern crate ferment_macro;

#[ferment_macro::export]
pub struct SPV {
    pub version: u32
}

pub mod dash {
    use crate::aa::AtAa;

    #[ferment_macro::export]
    pub fn setup_aa(transaction: AtAa) {
        println!("setup_dashcore: {transaction:?}")
    }
}