pub mod from_proof;
pub mod transport;
pub mod fermented;

extern crate ferment_macro;
pub mod nested {

    #[ferment_macro::export]
    pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
        Some(format_args!("{0:?}", script).to_string())
    }

    #[ferment_macro::export]
    #[derive(Debug)]
    pub enum ProtocolError {
        IdentifierError(String),
        Unknown(Vec<u8>)
    }

    //
    // #[ferment_macro::export]
    // pub struct FromProofImpl {
    //     pub obj: String,
    // }
    //
    // #[ferment_macro::export]
    // #[derive(Debug)]
    // pub enum IdentityRequest {
    //     Get(String),
    // }
    // #[derive(Debug)]
    // #[ferment_macro::export]
    // pub enum IdentityResponse {
    //     Get(String)
    // }
    //
    // impl<Req> FromProof<Req> for FromProofImpl {
    //     type Request = IdentityRequest;
    //     type Response = IdentityResponse;
    //
    //     fn maybe_from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(request: I, response: O, platform_version: u32) -> Result<Option<Self>, ProtocolError> where Self: Sized + 'a {
    //         println!("request: {:?}: {:?} {}", request.into(), response.into(), platform_version);
    //         Ok(None)
    //     }
    //
    //     fn from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(request: I, response: O, platform_version: u32) -> Result<Self, ProtocolError> where Self: Sized + 'a {
    //         println!("request: {:?}: {:?} {}", request.into(), response.into(), platform_version);
    //         Err(ProtocolError::IdentifierError(format!("bad platform version {platform_version}")))
    //     }
    // }
}