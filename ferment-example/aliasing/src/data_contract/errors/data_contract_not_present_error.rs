#[derive(Debug)]
#[ferment_macro::export]
pub struct DataContractNotPresentError {
    pub data_contract_id: String,
}