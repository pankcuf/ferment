mod fermented;
mod custom;

extern crate ferment_macro;
extern crate serde_json;

#[ferment_macro::export]
pub struct SPV {
    pub version: u32
}

pub mod spv {
    use dpp::identity::identity::Identity;
    use dpp::errors::protocol_error::ProtocolError;
    use dpp::version::PlatformVersion;
    use platform_value::types::identifier::Identifier;

    #[ferment_macro::export]
    pub fn fetch_identity(identifier: Identifier) -> Result<Identity, ProtocolError> {

        Identity::create_basic_identity(identifier.into(), PlatformVersion::latest())
    }
}

pub mod dash {
    use dashcore::Transaction;

    #[ferment_macro::export]
    pub fn setup_dashcore(transaction: Transaction) {
        println!("setup_dashcore: {transaction:?}")
    }
}