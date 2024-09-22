pub mod from_proof;
pub mod transport;
pub mod fermented;

extern crate ferment_macro;
pub mod nested {

    // #[ferment_macro::export]
    // pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
    //     Some(format_args!("{0:?}", script).to_string())
    // }

    #[ferment_macro::export]
    #[derive(Debug)]
    pub enum ProtocolError {
        IdentifierError(String),
        Unknown(u32)
    }
}