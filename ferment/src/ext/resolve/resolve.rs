use syn::Type;
use crate::context::ScopeContext;

pub trait Resolve {
    fn resolve(&self, source: &ScopeContext) -> Self;
}

impl Resolve for Type {
    fn resolve(&self, source: &ScopeContext) -> Self {
        source.full_type_for(self)
    }
}