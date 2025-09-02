use std::marker::PhantomData;
use crate::composable::LifetimesModel;
use crate::composer::{SourceComposable, Linkable};
use crate::context::ScopeContextLink;
use crate::lang::{LangLifetimeSpecification, Specification};
use crate::shared::SharedAccess;

pub struct LifetimesComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    pub parent: Option<Link>,
    pub lifetimes: LifetimesModel,
    _marker: PhantomData<SPEC>,

}
impl<SPEC, Link> LifetimesComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    pub fn new(lifetimes: LifetimesModel) -> Self {
        Self { parent: None, lifetimes, _marker: PhantomData }
    }
}

impl<SPEC, Link> Linkable<Link> for LifetimesComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<SPEC, Link> SourceComposable for LifetimesComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    type Source = ScopeContextLink;
    type Output = SPEC::Lt;
    fn compose(&self, _context: &Self::Source) -> Self::Output {
        SPEC::Lt::from_lifetimes(self.lifetimes.lifetimes.clone())
    }
}
