use async_trait::async_trait;
use crate::asyn::dapi_request::DapiClientError;
use crate::asyn::mock::Mockable;
use crate::asyn::query::{RequestSettings, TransportClient, TransportRequest};

#[async_trait]
#[ferment_macro::export]
pub trait Dapi {
    async fn execute<R>(
        &self,
        request: R,
        settings: RequestSettings)
        -> Result<R::Response, DapiClientError<<R::Client as TransportClient>::Error>>
        where
            R: TransportRequest + Mockable,
            R::Response: Mockable;
}
