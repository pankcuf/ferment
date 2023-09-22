// use crate::dash_spv_masternode_processor::chain::common::chain_type::ChainType;
// use crate::ffi::{Hash160, HashID};

// #[rs_ffi_macro_derive::impl_ffi_fn_conv]
// pub fn address_from_hash(hash: HashID) -> String {
//     format_args!("{0:?}", hash).to_string()
// }

#[rs_ffi_macro_derive::impl_ffi_fn_conv]
pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
    Some(format_args!("{0:?}", script).to_string())
}

// #[rs_ffi_macro_derive::impl_ffi_fn_conv]
// pub fn address_from_hash160(hash: Hash160, pubkey: u8) -> String {
//     let mut writer: Vec<u8> = Vec::new();
//     writer.push(pubkey);
//     writer.extend(hash);
//     String::from_utf8(writer).unwrap()
// }
//
// #[rs_ffi_macro_derive::impl_ffi_fn_conv]
// pub fn address_from_hash160_2(
//     chain_type: ChainType,
// ) -> String {
//     chain_type.get_string()
// }
