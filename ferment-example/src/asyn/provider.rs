use std::sync::Arc;
use crate::nested::Identifier;

#[derive(Debug, Clone, PartialEq)]
#[ferment_macro::export]
pub struct DataContractV0 {
    pub(crate) id: Identifier,
    pub(crate) version: u32,
}

#[ferment_macro::export]
pub enum DataContract {
    V0(DataContractV0),
}

#[ferment_macro::export]
pub enum ContextProviderError {
    Generic(String),
    Config(String),
    InvalidDataContract(String),
    InvalidQuorum(String),
}

#[ferment_macro::export]
pub trait ContextProvider: Send + Sync {
    fn get_quorum_public_key(
        &self,
        quorum_type: u32,
        quorum_hash: [u8; 32], // quorum hash is 32 bytes
        core_chain_locked_height: u32,
    ) -> Result<[u8; 48], ContextProviderError>; // public key is 48 bytes
    fn get_data_contract(
        &self,
        id: &Identifier,
    ) -> Result<Option<Arc<DataContract>>, ContextProviderError>;
}