use crate::asyn::provider::ContextProvider;

pub struct Sdk {
    proofs: bool,
    context_provider: std::sync::Mutex<Option<Box<dyn ContextProvider>>>,
}
