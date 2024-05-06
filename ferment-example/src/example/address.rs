// use std::collections::BTreeMap;
// use crate::nested::{HashID, ProtocolError};
// use crate::chain::common::chain_type::{ChainType, IHaveChainSettings};

#[ferment_macro::export]
pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
    Some(format_args!("{0:?}", script).to_string())
}

// #[ferment_macro::export]
// pub fn get_chain_type_string(chain_type: ChainType) -> String {
//     chain_type.name()
// }
//
//
// #[ferment_macro::export]
// pub fn get_chain_hashes_by_map(map: BTreeMap<ChainType, HashID>) -> String {
//     map.iter().fold(String::new(), |mut acc, (chain_type, hash_id)| {
//         acc += &chain_type.name();
//         acc += " => ";
//         acc += &String::from_utf8_lossy(hash_id);
//         acc
//     })
// }

#[ferment_macro::export]
pub fn address_simple_result(script: Vec<u32>) -> Result<u32, u32> {
    Ok(script[0])
}

// #[ferment_macro::export]
// pub fn address_simple_complex_result(script: Vec<u32>) -> Result<u32, ProtocolError> {
//     Ok(script[0])
// }
//
// #[ferment_macro::export]
// pub fn address_complex_simple_result(script: Vec<u8>) -> Result<HashID, u32> {
//     Ok(script[..32].try_into().unwrap())
// }
//
// #[ferment_macro::export]
// pub fn address_complex_result(script: Vec<u8>) -> Result<HashID, ProtocolError> {
//     Ok(script[..32].try_into().unwrap())
// }

#[allow(unused)]
pub enum TestChainType {

}
