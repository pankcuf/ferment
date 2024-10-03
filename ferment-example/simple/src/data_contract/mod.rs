pub mod v0;
pub mod v1;
pub mod document_type;

use crate::data_contract::v0::data_contract::DataContractV0;
pub use v1::DataContractV1;

#[derive(Debug, Clone, PartialEq)]
#[ferment_macro::export]
pub enum DataContract {
    V0(DataContractV0),
    V1(DataContractV1),
    #[cfg(test)]
    Test
}
