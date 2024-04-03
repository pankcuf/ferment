mod fermented;

extern crate ferment_macro;

#[ferment_macro::export]
pub struct SPV {
    pub version: u32
}

pub mod spv {
    use dpp::identity::Identity;
    use dpp::ProtocolError;
    use dpp::version::PlatformVersion;
    use platform_value::types::Identifier;

    #[ferment_macro::export]
    pub fn fetch_identity(identifier: Identifier) -> Result<Identity, ProtocolError> {
        Identity::create_basic_identity(identifier.into(), PlatformVersion::latest())
    }
}
