mod chain;
mod example;
pub mod fermented;
mod traits;
mod asyn;

extern crate ferment_macro;
extern crate tokio;

#[ferment_macro::export]
pub struct RootStruct {
    pub name: String,
}

pub struct ExcludedStruct {
    pub name: String,
}

pub mod nested {
    use std::collections::BTreeMap;
    use ferment_interfaces::OpaqueContext;

    #[ferment_macro::export]
    pub type KeyID = u32;

    #[ferment_macro::export]
    pub type Hash160 = [u8; 20];

    #[ferment_macro::export]
    pub type HashID = [u8; 32];

    #[ferment_macro::export]
    pub type UsedKeyMatrix = Vec<bool>;

    #[ferment_macro::export]
    pub type UsedStruct = HashID;

    #[ferment_macro::export]
    pub type ArrayOfArraysOfHashes = Vec<Vec<HashID>>;

    #[ferment_macro::export]
    pub type MapOfHashes = BTreeMap<HashID, HashID>;

    #[ferment_macro::export]
    pub type MapOfVecHashes = BTreeMap<HashID, Vec<HashID>>;

    #[ferment_macro::export]
    pub struct BinaryData(pub Vec<u8>);

    #[ferment_macro::export]
    pub struct SimpleData(pub Vec<u32>);

    #[ferment_macro::export]
    pub struct IdentifierBytes32(pub [u8; 32]);

    #[ferment_macro::export]
    pub struct UnnamedPair(pub [u8; 32], pub u32);

    #[ferment_macro::export]
    pub struct Identifier(pub IdentifierBytes32);

    #[ferment_macro::export]
    pub enum TestEnum {
        Variant1(String),
        Variant2,
        Variant3(HashID, u32),
        Variant4(HashID, u32, String),
        Variant5(BTreeMap<String, HashID>, u32, String),
    }

    #[ferment_macro::export]
    pub struct DataContractNotPresentError {
        pub data_contract_id: Identifier,
    }

    #[ferment_macro::export]
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

    #[ferment_macro::export]
    pub type AddInsightCallback = fn(block_hash: HashID, context: OpaqueContext);

    #[ferment_macro::export]
    pub type ShouldProcessDiffWithRangeCallback = fn(
        base_block_hash: HashID,
        block_hash: HashID,
        context: OpaqueContext,
    ) -> ProtocolError;

    #[ferment_macro::export]
    pub fn find_hash_by_u32(key: u32, map: BTreeMap<u32, HashID>) -> Option<HashID> {
        map.get(&key).copied()
    }

    #[ferment_macro::export]
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
        pub map_key_simple_value_map_key_simple_value_vec_simple:
            BTreeMap<u32, BTreeMap<u32, Vec<u32>>>,
        pub map_key_simple_value_map_key_simple_value_vec_complex:
            BTreeMap<u32, BTreeMap<u32, Vec<HashID>>>,

        pub map_key_complex_value_simple: BTreeMap<HashID, u32>,
        pub map_key_complex_value_complex: BTreeMap<HashID, HashID>,
        pub map_key_complex_value_vec_simple: BTreeMap<HashID, Vec<u32>>,
        pub map_key_complex_value_vec_complex: BTreeMap<HashID, Vec<HashID>>,

        pub map_key_complex_value_map_key_simple_value_vec_simple:
            BTreeMap<HashID, BTreeMap<u32, Vec<u32>>>,
        pub map_key_complex_value_map_key_simple_value_vec_complex:
            BTreeMap<HashID, BTreeMap<u32, Vec<HashID>>>,

        pub map_key_complex_value_map_key_simple_value_map_key_complex_value_complex:
            BTreeMap<HashID, BTreeMap<u32, BTreeMap<HashID, HashID>>>,

        pub opt_primitive: Option<u8>,
        pub opt_string: Option<String>,
        // pub opt_str: Option<&'static str>,
        pub opt_vec_primitive: Option<Vec<u8>>,
        pub opt_vec_string: Option<Vec<String>>,
        pub opt_vec_complex: Option<Vec<HashID>>,
        pub opt_vec_vec_complex: Option<Vec<Vec<HashID>>>,
    }


}

