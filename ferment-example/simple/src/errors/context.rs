#[ferment_macro::export]
pub enum ContextProviderError {
    Generic(String),
    Config(String),
    InvalidDataContract(String),
    InvalidQuorum(String),
}
