use std::collections::HashMap;
use syn::Path;
use crate::ast::TypePathHolder;
use crate::context::ScopeChain;

#[allow(unused)]
#[derive(Clone, Default)]
pub struct AttrsResolver {
    pub inner: HashMap<ScopeChain, HashMap<TypePathHolder, Vec<Path>>>,
}

