mod custom;
mod fermented;

extern crate ferment_macro;
extern crate serde_json;

#[ferment_macro::export]
pub struct SPV {
    pub version: u32
}

pub mod dash {
    use dashcore::Transaction;

    #[ferment_macro::export]
    pub fn setup_dashcore(transaction: Transaction) {
        println!("setup_dashcore: {transaction:?}")
    }
}