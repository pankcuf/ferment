use crate::nested::HashID;

#[ferment_macro::export]
pub trait Mockable
    where
        Self: std::marker::Sized,
{
    fn mock_serialize(&self) -> Option<Vec<u8>> {
        None
    }
    fn mock_deserialize(_data: &[u8]) -> Option<Self> {
        None
    }
}

#[ferment_macro::export]
pub trait MockResponse {}
impl<T> MockResponse for T {}

#[ferment_macro::export]
pub trait MockRequest {
    /// Format the object as a key that will be used to match the request with the expectation.
    ///
    /// ## Panics
    ///
    /// Can panic on errors.
    fn mock_key(&self) -> HashID;
}
