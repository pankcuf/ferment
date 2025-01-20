use std::marker::PhantomData;
use crate::composable::LifetimesModel;
use crate::composer::{SourceComposable, Linkable};
use crate::context::ScopeContextLink;
use crate::lang::{LangFermentable, LangLifetimeSpecification, Specification};
use crate::shared::SharedAccess;

pub struct LifetimesComposer<LANG, SPEC, Link>
where Link: SharedAccess,
      LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub parent: Option<Link>,
    pub lifetimes: LifetimesModel,
    _marker: PhantomData<(LANG, SPEC)>,

}
impl<LANG, SPEC, Link> LifetimesComposer<LANG, SPEC, Link>
where Link: SharedAccess,
      LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub fn new(lifetimes: LifetimesModel) -> Self {
        Self { parent: None, lifetimes, _marker: PhantomData }
    }
}

impl<LANG, SPEC, Link> Linkable<Link> for LifetimesComposer<LANG, SPEC, Link>
where Link: SharedAccess,
      LANG: LangFermentable,
      SPEC: Specification<LANG> {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<LANG, SPEC, Link> SourceComposable for LifetimesComposer<LANG, SPEC, Link>
where Link: SharedAccess,
      LANG: LangFermentable,
      SPEC: Specification<LANG> {
    type Source = ScopeContextLink;
    type Output = SPEC::Lt;
    fn compose(&self, _context: &Self::Source) -> Self::Output {
        SPEC::Lt::from_lifetimes(self.lifetimes.lifetimes.clone())
    }
}
