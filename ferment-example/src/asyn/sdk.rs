use crate::asyn::mock::{MockRequest, MockResponse};
use crate::asyn::proof::FromProof;
use crate::asyn::provider::ContextProvider;
use crate::nested::ProtocolError;

#[ferment_macro::export]
pub struct Sdk {
    pub proofs: bool,
    pub context_provider: Option<Box<dyn ContextProvider>>,
    // pub context_provider: std::sync::Mutex<Option<Box<dyn ContextProvider>>>,
}

impl Sdk {
    pub(crate) fn parse_proof<R, O: FromProof<R> + MockResponse>(request: O::Request, response: O::Response) -> Result<Option<O>, ProtocolError>
        where O::Request: MockRequest {
        unimplemented!("request: {:?}, response: {:?}", request, response)
    }

}

