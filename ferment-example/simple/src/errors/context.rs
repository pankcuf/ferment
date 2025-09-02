#[ferment_macro::export]
pub enum ContextProviderError {
    Generic(String),
    Config(String),
    InvalidDataContract(String),
    InvalidQuorum(String),
}

#[ferment_macro::opaque]
#[derive(Clone)]
pub enum ContextProviderErrorOpaque {
    Generic(String),
    Config(String),
    InvalidDataContract(String),
    InvalidQuorum(String),
}
