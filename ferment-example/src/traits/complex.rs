// pub trait TransportRequest: Clone + Send + Sync {
//     type Client: TransportClient;
//     type Response: TransportResponse;
//     fn execute_transport<'c>(
//         self,
//         client: &'c mut Self::Client,
//     ) -> Result<Self::Response, <Self::Client as TransportClient>::Error>;
// }
// pub trait TransportResponse: Clone + Send + Sync + Debug {}
// pub trait Query<T: TransportRequest>: Send + Debug + Clone {
//     fn query(self, prove: bool) -> Result<T, Status>;
// }
// impl Query<GetDocumentsRequest> for Identifier {
//     fn query(self, prove: bool) -> Result<GetDocumentsRequest, Status> {
//         Ok(proto::GetDocumentsRequest {
//             version: u32::from(prove),
//         })
//     }
// }
