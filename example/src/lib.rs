#[allow(unused_variables)]
pub mod ffi {
    use std::collections::BTreeMap;
    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type KeyID = u32;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type HashID = [u8; 32];

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type UsedKeyMatrix = Vec<bool>;

    #[rs_ffi_macro_derive::impl_ffi_ty_conv]
    pub type ArrayOfArraysOfHashes = Vec<Vec<self::HashID>>;

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct BinaryData(pub Vec<u8>);

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct IdentifierBytes32(pub [u8; 32]);

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct Identifier(pub self::IdentifierBytes32);

    #[derive(Clone)]
    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub enum TestEnum {
        Variant1(String),
        Variant2,
        Variant3(self::HashID, u32),
        Variant4(self::HashID, u32, String),
    }

    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct DataContractNotPresentError {
        data_contract_id: self::Identifier,
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
        DataContractNotPresentError(self::DataContractNotPresentError),
    }
    #[rs_ffi_macro_derive::impl_ffi_conv]
    pub struct TestStruct {
        pub map_key_simple_value_simple: BTreeMap<u32, u32>,
        pub map_key_simple_value_complex: BTreeMap<u32, self::HashID>,
        pub map_key_simple_value_vec_simple: BTreeMap<u32, Vec<u32>>,
        pub map_key_simple_value_vec_complex: BTreeMap<u32, Vec<self::HashID>>,
        pub map_key_simple_value_map_key_simple_value_simple: BTreeMap<u32, BTreeMap<u32, u32>>,
        pub map_key_simple_value_map_key_simple_value_complex:
            BTreeMap<u32, BTreeMap<u32, self::HashID>>,
        pub map_key_simple_value_map_key_simple_value_vec_simple:
            BTreeMap<u32, BTreeMap<u32, Vec<u32>>>,
        pub map_key_simple_value_map_key_simple_value_vec_complex:
            BTreeMap<u32, BTreeMap<u32, Vec<self::HashID>>>,

        pub map_key_complex_value_simple: BTreeMap<self::HashID, u32>,
        pub map_key_complex_value_complex: BTreeMap<self::HashID, self::HashID>,
        pub map_key_complex_value_vec_simple: BTreeMap<self::HashID, Vec<u32>>,
        pub map_key_complex_value_vec_complex: BTreeMap<self::HashID, Vec<self::HashID>>,

        pub map_key_complex_value_map_key_simple_value_vec_simple:
            BTreeMap<self::HashID, BTreeMap<u32, Vec<u32>>>,
        pub map_key_complex_value_map_key_simple_value_vec_complex:
            BTreeMap<self::HashID, BTreeMap<u32, Vec<self::HashID>>>,
    }

    #[rs_ffi_macro_derive::impl_ffi_callback]
    pub type AddInsightCallback = fn(block_hash: HashID, context: rs_ffi_interfaces::OpaqueContext);

    #[rs_ffi_macro_derive::impl_ffi_callback]
    pub type ShouldProcessDiffWithRangeCallback = fn(
        base_block_hash: HashID,
        block_hash: HashID,
        context: rs_ffi_interfaces::OpaqueContext,
    ) -> ProtocolError;

    #[rs_ffi_macro_derive::impl_ffi_fn_conv]
    pub fn address_from_hash(hash: HashID) -> String {
        format!("{:?}", hash)
    }

    #[rs_ffi_macro_derive::impl_ffi_fn_conv]
    pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
        Some(format!("{:?}", script))
    }
}
