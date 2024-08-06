use ferment_example::nested::HashID;
use crate::model::LLMQSnapshot;
use crate::state_transition::state_transitions::contract::data_contract_create_transition::DataContractCreateTransition;

#[ferment_macro::export]
pub fn get_hash_id_form_snapshot(_snapshot: LLMQSnapshot) -> HashID {
    [0u8; 32]
}

#[ferment_macro::export]
pub struct StructWithTuple {
    pub tuple: (u32, HashID)
}

#[ferment_macro::export]
pub fn get_tuple_simple() -> (u32, u32) {
    (0, 0)
}

#[ferment_macro::export]
pub fn get_tuple_simple_complex() -> (u32, HashID) {
    (0, [0u8; 32])
}

#[ferment_macro::export]
pub fn get_tuple_complex_complex(tuple: (u32, HashID)) -> u32 {
    tuple.0
}

// #[ferment_macro::export]
// pub fn get_tuple_generic() -> Vec<(BinaryData, LLMQSnapshot)> {
//     vec![(BinaryData(vec![]), LLMQSnapshot::default())]
// }

#[ferment_macro::export]
pub struct TransUser {
    pub transition: DataContractCreateTransition,
}