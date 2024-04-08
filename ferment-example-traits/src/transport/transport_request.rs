// use crate::nested::ProtocolError;
//
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub enum Status {
    Error,
    Success
}
#[derive(Clone, Debug)]
#[allow(dead_code)]
#[ferment_macro::export]
pub struct Uri {
    pub scheme: String,
}
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct GetDocumentsRequest { pub version: u32 }
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct GetDocumentsResponse { pub version: u32 }
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct DocumentQuery { pub version: u32 }
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct Identifier(pub u32);
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct CoreGrpcClient {
    pub uri: Uri
}

#[ferment_macro::export]
pub trait CanRetry {
    fn can_retry(&self) -> bool;
}
// #[ferment_macro::export]
pub trait SomeOtherTrait {
    fn some_other_method(&self);
}
// #[ferment_macro::export]
pub trait TransportClient: Send + Sized {
    type Error: CanRetry + Send + SomeOtherTrait;
    fn with_uri(uri: Uri) -> Self;
}
// #[ferment_macro::export]
pub trait TransportResponse: Clone + Send + Sync {}

// #[ferment_macro::export]
pub trait TransportRequest: Clone + Send + Sync {
    type Client: TransportClient;
    type Response: TransportResponse;
    fn execute_transport(self, client: &mut Self::Client) -> Result<Self::Response, <Self::Client as TransportClient>::Error>;
}


// #[ferment_macro::export]
// pub trait Query<T: TransportRequest>: Send + Clone {
//     fn query(self, prove: bool) -> Result<T, Status>;
// }
//
//
// #[ferment_macro::export]
// impl CanRetry for Status {
//     fn can_retry(&self) -> bool { true }
// }
// // #[ferment_macro::export]
// impl SomeOtherTrait for Status {
//     fn some_other_method(&self) {}
// }
//
// unsafe impl Send for Status {}
//
// impl CoreGrpcClient {
//     pub fn new(uri: Uri) -> Self { Self { uri } }
// }
//
// impl TransportClient for CoreGrpcClient {
//     type Error = Status;
//
//     fn with_uri(uri: Uri) -> Self {
//         CoreGrpcClient::new(uri)
//     }
// }
//
// impl TransportRequest for GetDocumentsRequest {
//     type Client = CoreGrpcClient;
//     type Response = GetDocumentsResponse;
//
//     fn execute_transport(self, client: &mut Self::Client) -> Result<Self::Response, <Self::Client as TransportClient>::Error> {
//         println!("GetDocumentsRequest::execute_transport: {:?}", client);
//         Ok(GetDocumentsResponse { version: 0 })
//     }
// }
//
// impl TransportRequest for DocumentQuery {
//     type Client = <GetDocumentsRequest as TransportRequest>::Client;
//     type Response = <GetDocumentsRequest as TransportRequest>::Response;
//
//     fn execute_transport(
//         self,
//         client: &mut Self::Client,
//     ) -> Result<Self::Response, <Self::Client as TransportClient>::Error> {
//         let request: GetDocumentsRequest = self
//             .try_into()
//             .expect("DocumentQuery should always be valid");
//         request.execute_transport(client)
//     }
// }
//
// impl TransportResponse for GetDocumentsResponse {}
//
//
// impl Query<GetDocumentsRequest> for Identifier {
//     fn query(self, prove: bool) -> Result<GetDocumentsRequest, Status> {
//         Ok(GetDocumentsRequest { version: u32::from(prove) })
//     }
// }
//
// impl TryFrom<DocumentQuery> for GetDocumentsRequest {
//     type Error = ProtocolError;
//     fn try_from(dapi_request: DocumentQuery) -> Result<Self, Self::Error> {
//         Ok(GetDocumentsRequest { version: dapi_request.version })
//     }
// }
