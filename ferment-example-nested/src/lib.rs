mod fermented;
mod model;


extern crate ferment_macro;

#[ferment_macro::export]
pub struct SomeStruct {
    pub name: String,
}

pub mod some_inner {
    use crate::model::{Quorum, QuorumType};
    #[ferment_macro::export]
    pub fn get_normal_quorum() -> Quorum {
        Quorum { llmq_type: QuorumType::Normal }
    }
}
pub mod some_inner_2 {
    use std::collections::{BTreeMap, BTreeSet};
    use crate::model::quorum::QuorumType;
    use crate::model::Quorum;
    #[ferment_macro::export]
    pub fn get_normal_quorum() -> Quorum {
        Quorum { llmq_type: QuorumType::Normal }
    }

    #[ferment_macro::export]
    pub struct AllExamples {
        pub name: String,
        pub map_k_simple_v_simple: BTreeMap<u32, u32>,
        pub map_k_simple_v_opt_simple: BTreeMap<u32, Option<u32>>,
        pub map_k_simple_v_opt_complex: BTreeMap<u32, Option<String>>,
        pub map_k_simple_v_opt_generic_simple: BTreeMap<u32, Option<Vec<u32>>>,
        pub map_k_simple_v_opt_generic_complex: BTreeMap<u32, Option<Vec<String>>>,
        pub btreeset_opt_simple: BTreeSet<Option<u32>>,
        pub result_ok_complex_err_complex: Result<String, String>,
        pub result_ok_complex_err_opt_simple: Result<String, Option<u32>>,
        pub result_ok_complex_err_opt_complex: Result<String, Option<String>>,
        pub arr: [u8; 32],
        pub opt_complex: Option<String>,
        pub opt_map_k_simple_v_simple: Option<BTreeMap<u32, u32>>,
        pub tuple_string: (String, String),
        pub opt_arr: Option<[u8; 32]>,
        // pub indexes: Option<[u8; 32]>,
    }
    // #[ferment_macro::export]
    // pub struct DocumentTypeV2 {
    //     pub name: String,
    //     pub indexes: BTreeMap<String, Option<Vec<u8>>>,
    // }
    // #[ferment_macro::export]
    // pub struct DocumentTypeV3 {
    //     pub name: String,
    //     pub indexes: BTreeMap<String, Option<Vec<String>>>,
    // }
}


// pub mod some_package {
    // use platform_value::BinaryData;
    // use ferment_example::nested::BinaryData;
    // use ferment_example::nested::HashID;
    // use crate::model::LLMQSnapshot;
    // use platform_value::types::binary_data::BinaryData;

    // #[ferment_macro::export]
    // pub fn get_hash_id_form_snapshot(_snapshot: LLMQSnapshot) -> HashID {
    //     [0u8; 32]
    // }

    // #[ferment_macro::export]
    // pub fn get_binary_data() -> BinaryData {
    //     BinaryData::new(vec![])
    // }

    // #[ferment_macro::export]
    // pub struct StructWithTuple {
    //     pub tuple: (u32, HashID)
    // }

    // #[ferment_macro::export]
    // pub fn get_tuple_simple() -> (u32, u32) {
    //     (0, 0)
    // }
    //
    // #[ferment_macro::export]
    // pub fn get_tuple_simple_complex() -> (u32, HashID) {
    //     (0, [0u8; 32])
    // }
    //
    // #[ferment_macro::export]
    // pub fn get_tuple_complex_complex(tuple: (u32, HashID)) -> u32 {
    //     tuple.0
    // }
    //
    // #[ferment_macro::export]
    // pub fn get_tuple_generic() -> Vec<(BinaryData, LLMQSnapshot)> {
    //     vec![(BinaryData(vec![]), LLMQSnapshot::default())]
    // }
    //
// }