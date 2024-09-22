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
    fn mock_key(&self) -> HashID;
}
