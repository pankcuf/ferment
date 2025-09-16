use crate::data_contract::errors::*;
use crate::error1::Error as DashCoreError;
use crate::error2::error::Error as ValueError;
use crate::error3::Error3;

#[derive(Debug)]
#[ferment_macro::export]
pub enum ProtocolError {
    ValueError(ValueError),
    DashCoreError(DashCoreError),
    NonAliasedError(Error3),
    DataContractError(DataContractError),
    DataContractNotPresentError(DataContractNotPresentError),
    IdentityNotPresentError(IdentityNotPresentError),
}