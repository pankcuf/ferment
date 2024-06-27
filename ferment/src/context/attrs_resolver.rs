use std::collections::HashMap;
use syn::Path;
use crate::context::ScopeChain;
use crate::holder::TypePathHolder;

#[allow(unused)]
#[derive(Clone, Default)]
pub struct AttrsResolver {
    pub inner: HashMap<ScopeChain, HashMap<TypePathHolder, Vec<Path>>>,
}

