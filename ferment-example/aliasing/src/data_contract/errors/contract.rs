use crate::consensus::basic::document::InvalidDocumentTypeError;

#[derive(Debug, PartialEq, Clone)]
#[ferment_macro::export]
pub enum DataContractError {
    InvalidDocumentTypeError(InvalidDocumentTypeError),
}