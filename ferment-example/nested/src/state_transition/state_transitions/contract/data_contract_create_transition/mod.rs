use crate::state_transition::state_transitions::contract::data_contract_create_transition::v0::DataContractCreateTransitionV0;

pub mod v0;

// pub use v0::DataContractCreateTransitionV0;

#[derive(Clone)]
#[ferment_macro::export]
pub enum DataContractCreateTransition {
    V0(DataContractCreateTransitionV0),
}


// #[ferment_macro::export]
// pub fn create_doc_ref<'a>(docs: BTreeMap<DocumentTransitionActionType,Vec<(Document, DocumentTypeRef<'a>, Bytes32)>>) {
//     println!("doc_type: {:?}", docs);
// }

#[derive(Clone)]
#[ferment_macro::export]
pub enum ExampleEnumLif<'a> {
    Varik(&'a DataContractCreateTransition)
}
#[derive(Clone)]
#[ferment_macro::export]
pub struct ExampleStructLif<'a> {
    pub varik: &'a DataContractCreateTransition
}