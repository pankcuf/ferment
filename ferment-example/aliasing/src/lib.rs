mod fermented;
pub mod aa;
pub mod zz;
pub mod blockdata;
pub mod data_contract;
pub mod errors;
pub mod error1;
pub mod error2;
pub mod error3;
pub mod identity;

pub use crate::blockdata::script::{self, ScriptBuf};
pub use errors::*;

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