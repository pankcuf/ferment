extern crate rs_ffi_macro_derive;

use std::collections::BTreeMap;

#[allow(unused_variables)]
#[rs_ffi_macro_derive::ffi_dictionary]
pub mod ffi {
    use std::collections::BTreeMap;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type KeyID = u32;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type Hash160 = [u8; 20];

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type HashID = [u8; 32];

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type UsedKeyMatrix = Vec<bool>;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type UsedStruct = HashID;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type ArrayOfArraysOfHashes = Vec<Vec<HashID>>;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type MapOfHashes = BTreeMap<HashID, HashID>;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type MapOfVecHashes = BTreeMap<HashID, Vec<HashID>>;

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct BinaryData(pub Vec<u8>);

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct SimpleData(pub Vec<u32>);

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct IdentifierBytes32(pub [u8; 32]);

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct UnnamedPair(pub [u8; 32], pub u32);

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct Identifier(pub IdentifierBytes32);

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub enum TestEnum {
        Variant1(String),
        Variant2,
        Variant3(HashID, u32),
        Variant4(HashID, u32, String),
        Variant5(BTreeMap<String, HashID>, u32, String),
    }

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct DataContractNotPresentError {
        data_contract_id: Identifier,
    }

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub enum ProtocolError {
        IdentifierError(String),
        StringDecodeError(String),
        StringDecodeError2(String, u32),
        EmptyPublicKeyDataError,
        MaxEncodedBytesReachedError {
            max_size_kbytes: usize,
            size_hit: usize,
        },
        EncodingError(String),
        EncodingError2(&'static str),
        DataContractNotPresentError(DataContractNotPresentError),
    }

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type AddInsightCallback = fn(block_hash: HashID, context: rs_ffi_interfaces::OpaqueContext);

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type ShouldProcessDiffWithRangeCallback = fn(
        base_block_hash: HashID,
        block_hash: HashID,
        context: rs_ffi_interfaces::OpaqueContext,
    ) -> ProtocolError;

    #[rs_ffi_macro_derive::impl_ffi_fn_conv]
    pub fn address_from_hash(hash: HashID) -> String {
        format_args!("{0:?}", hash).to_string()
    }

    #[rs_ffi_macro_derive::impl_ffi_fn_conv]
    pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
        Some(format_args!("{0:?}", script).to_string())
    }

    #[rs_ffi_macro_derive::impl_ffi_fn_conv]
    pub fn address_from_hash160(hash: Hash160, pubkey: u8) -> String {
        let mut writer: Vec<u8> = Vec::new();
        writer.push(pubkey);
        writer.extend(hash);
        String::from_utf8(writer).unwrap()
    }

    #[rs_ffi_macro_derive::impl_ffi_fn_conv]
    pub fn find_hash_by_u32(key: u32, map: BTreeMap<u32, HashID>) -> Option<HashID> {
        map.get(&key).clone().copied()
    }

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct TestStruct {
        pub vec_u8: Vec<u8>,
        pub vec_u32: Vec<u32>,
        pub vec_vec_u32: Vec<Vec<u32>>,
        pub map_key_simple_value_simple: BTreeMap<u32, u32>,
        pub map_key_simple_value_complex: BTreeMap<u32, HashID>,
        pub map_key_simple_value_vec_simple: BTreeMap<u32, Vec<u32>>,
        pub map_key_simple_value_vec_complex: BTreeMap<u32, Vec<HashID>>,
        pub map_key_simple_value_map_key_simple_value_simple: BTreeMap<u32, BTreeMap<u32, u32>>,
        pub map_key_simple_value_map_key_simple_value_complex: BTreeMap<u32, BTreeMap<u32, HashID>>,
        pub map_key_simple_value_map_key_simple_value_vec_simple: BTreeMap<u32, BTreeMap<u32, Vec<u32>>>,
        pub map_key_simple_value_map_key_simple_value_vec_complex: BTreeMap<u32, BTreeMap<u32, Vec<HashID>>>,

        pub map_key_complex_value_simple: BTreeMap<HashID, u32>,
        pub map_key_complex_value_complex: BTreeMap<HashID, HashID>,
        pub map_key_complex_value_vec_simple: BTreeMap<HashID, Vec<u32>>,
        pub map_key_complex_value_vec_complex: BTreeMap<HashID, Vec<HashID>>,

        pub map_key_complex_value_map_key_simple_value_vec_simple: BTreeMap<HashID, BTreeMap<u32, Vec<u32>>>,
        pub map_key_complex_value_map_key_simple_value_vec_complex: BTreeMap<HashID, BTreeMap<u32, Vec<HashID>>>,
    }

}
