mod fermented;
mod model;


extern crate ferment_macro;

#[ferment_macro::export]
pub struct SomeStruct {
    pub name: String,
}

pub mod some_package {
    use ferment_example::nested::HashID;
    use crate::model::snapshot::LLMQSnapshot;
    use platform_value::types::binary_data::BinaryData;

    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: LLMQSnapshot) -> HashID {
        [0u8; 32]
    }

    #[ferment_macro::export]
    pub fn get_binary_data() -> BinaryData {
        BinaryData::new(vec![])
    }
}