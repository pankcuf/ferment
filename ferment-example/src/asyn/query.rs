use std::error::Error;
use std::time::Duration;
use crate::nested::ProtocolError;

#[derive(Debug, Clone, Copy)]
#[ferment_macro::export]
pub struct RequestSettings {
    pub timeout: Option<Duration>,
    pub retries: Option<usize>,
}
#[derive(Debug, Clone, Copy)]
#[ferment_macro::export]
pub struct AppliedRequestSettings {
    pub timeout: Duration,
    pub retries: usize,
}

#[derive(Clone)]
#[ferment_macro::export]
pub struct Uri {
    pub scheme: String,
}

#[ferment_macro::export]
pub trait CanRetry {
    fn can_retry(&self) -> bool;
}

#[ferment_macro::export]
pub trait TransportClient: Send + Sized {
    type Error: CanRetry + Send;
    fn with_uri(uri: Uri) -> Self;
}


#[ferment_macro::export]
pub trait TransportResponse: Clone + Send + Sync {}

#[ferment_macro::export]
pub trait TransportRequest: Clone + Send + Sync {
    type Client: TransportClient;
    type Response: TransportResponse;
    const SETTINGS_OVERRIDES: RequestSettings;
    fn execute_transport(
        self,
        client: &mut Self::Client,
        settings: &AppliedRequestSettings
    ) -> Result<Self::Response, <Self::Client as TransportClient>::Error>;
}
