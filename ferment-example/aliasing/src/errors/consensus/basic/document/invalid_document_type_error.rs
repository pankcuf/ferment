#[derive(Debug, Clone, PartialEq)]
#[ferment_macro::export]
pub struct InvalidDocumentTypeError {
    pub document_type: String,
}
