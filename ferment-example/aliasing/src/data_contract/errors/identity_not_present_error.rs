#[derive(Debug, Clone, PartialEq, Eq)]
#[ferment_macro::export]
pub struct IdentityNotPresentError {
    pub id: String,
}