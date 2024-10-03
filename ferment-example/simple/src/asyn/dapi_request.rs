use std::future::Future;
use std::pin::Pin;
use crate::asyn::dapi_client::Dapi;
use crate::asyn::query::RequestSettings;

#[ferment_macro::export]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug)]
pub enum DapiClientError<TE> {
    Transport(TE),
    NoAvailableAddresses,
}


#[ferment_macro::export]
pub trait DapiRequest {
    type Response;
    type TransportError;
    fn execute<'c, D: Dapi>(
        self,
        dapi_client: &'c D,
        settings: RequestSettings)
        -> BoxFuture<'c, Result<Self::Response, DapiClientError<Self::TransportError>>>
        where Self: 'c;
}
