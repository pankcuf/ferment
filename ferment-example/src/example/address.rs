use std::collections::BTreeMap;
use crate::nested::HashID;
use crate::chain::common::chain_type::ChainType;

#[ferment_macro::export]
pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
    Some(format_args!("{0:?}", script).to_string())
}

#[ferment_macro::export]
pub fn get_chain_type_string(chain_type: ChainType) -> String {
    chain_type.get_string()
}


#[ferment_macro::export]
pub fn get_chain_hashes_by_map(map: BTreeMap<ChainType, HashID>) -> String {
    map.iter().fold(String::new(), |mut acc, (chain_type, hash_id)| {
        acc += &chain_type.get_string();
        acc += " => ";
        acc += &String::from_utf8_lossy(hash_id);
        acc
    })
}
