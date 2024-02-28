use ferment_macro::Parent;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::shared::SharedAccess;

#[derive(Parent)]
pub struct NameComposer<Parent: SharedAccess> {
    pub parent: Option<Parent>,
    pub name: Name,
}

impl<Parent: SharedAccess> NameComposer<Parent> {
    pub const fn new(name: Name) -> Self {
        Self { parent: None, name }
    }
}

impl<Parent: SharedAccess> Composer<Parent> for NameComposer<Parent> {
    type Source = ScopeContext;
    type Result = Name;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        self.name.clone()
    }
}
