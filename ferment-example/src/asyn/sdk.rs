use crate::asyn::provider::ContextProvider;

#[ferment_macro::export]
pub struct Sdk {
    proofs: bool,
    context_provider: std::sync::Mutex<Option<Box<dyn ContextProvider>>>,
}
