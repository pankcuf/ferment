#[derive(Debug, Clone, PartialEq)]
#[ferment_macro::export]
pub struct InvalidDocumentTypeError {
    pub doc_type: String,
}
