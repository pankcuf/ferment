use crate::document::Document;

#[ferment_macro::export]
pub enum DocumentError {
    InvalidActionError(u8),
    InvalidInitialRevisionError { document: Box<Document> }
}
