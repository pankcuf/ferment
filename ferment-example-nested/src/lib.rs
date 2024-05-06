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
    use std::collections::BTreeSet;
    use crate::model::quorum::QuorumType;
    use crate::model::Quorum;
    #[ferment_macro::export]
    pub fn get_normal_quorum() -> Quorum {
        Quorum { llmq_type: QuorumType::Normal }
    }

    #[ferment_macro::export]
    pub fn get_btree_set() -> BTreeSet<String> {
        BTreeSet::new()
    }

    #[ferment_macro::export]
    pub fn set_btree_set(set: BTreeSet<String>) {
        println!("BTreeSet: {:?}", set);
    }

    #[ferment_macro::export]
    pub struct DocumentTypeV0 {
        pub name: String,
        pub identifier_paths: BTreeSet<String>,
        pub binary_paths: BTreeSet<String>
    }
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