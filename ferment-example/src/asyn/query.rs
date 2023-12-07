use std::time::Duration;

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

#[allow(non_camel_case_types)]
#[ferment_macro::register(Duration)]
pub struct Duration_FFI {
    secs: u64,
    nanos: u32,
}

ferment_interfaces::impl_custom_conversion!(Duration, Duration_FFI,
    |value: &Duration_FFI| Duration::new(value.secs, value.nanos),
    |value: &Duration| Self { secs: value.as_secs(), nanos: value.subsec_nanos() }
);


#[derive(Clone)]
#[ferment_macro::export]
pub struct Uri {
    pub(crate) scheme: String,
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


// #[ferment_macro::export]
// pub trait TransportResponse: Clone + Send + Sync {}
//
//
// #[ferment_macro::export]
// pub trait TransportRequest: Clone + Send + Sync {
//     type Client: TransportClient;
//     type Response: TransportResponse;
//     const SETTINGS_OVERRIDES: RequestSettings;
//     fn execute_transport(self, client: &mut Self::Client, settings: &AppliedRequestSettings) -> Result<Self::Response, <Self::Client as TransportClient>::Error>;
// }
//
//
// #[ferment_macro::export]
// pub trait Query<T: TransportRequest>: Send + Clone {
//     fn query(self, prove: bool) -> Result<T, ProtocolError>;
// }
//
// impl<T> Query<T> for T where T: TransportRequest + Sized + Send + Sync + Clone, T::Response: Send + Sync {
//     fn query(self, prove: bool) -> Result<T, ProtocolError> {
//         if !prove {
//             unimplemented!("queries without proofs are not supported yet");
//         }
//         Ok(self)
//     }
// }
