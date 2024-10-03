pub mod v0;

use crate::data_contract::document_type::v0::DocumentTypeV0;
#[derive(Debug, Clone, PartialEq)]
#[ferment_macro::export]
pub enum DocumentType {
    V0(DocumentTypeV0),
}
