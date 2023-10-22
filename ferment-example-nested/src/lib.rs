mod fermented;
mod model;

extern crate ferment_macro;

#[ferment_macro::export]
pub struct SomeStruct {
    pub name: String,
}

pub mod some_package {
    use ferment_example::nested::HashID;
    use crate::model::snapshot::Snapshot;
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: Snapshot) -> HashID {
        [0u8; 32]
    }
}