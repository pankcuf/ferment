use std::collections::HashMap;
use indexmap::IndexMap;
use syn::{Path, TypePath};
use crate::context::ScopeChain;

#[allow(unused)]
#[derive(Clone, Default)]
pub struct AttrsResolver {
    pub inner: HashMap<ScopeChain, IndexMap<TypePath, Vec<Path>>>,
}

