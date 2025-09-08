use indexmap::IndexMap;
use syn::{Path, Type};

#[derive(Clone, Default)]
pub struct GenericChain {
    pub inner: IndexMap<Type, Vec<Path>>
}

impl GenericChain {
    pub fn new(chain: IndexMap<Type, Vec<Path>>) -> Self {
        Self { inner: chain }
    }
}