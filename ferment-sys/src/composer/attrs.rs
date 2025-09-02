use std::marker::PhantomData;
use crate::composable::{AttrsModel, CfgAttributes};
use crate::composer::{SourceComposable, Linkable};
use crate::context::ScopeContextLink;
use crate::lang::{LangAttrSpecification, Specification};
use crate::shared::SharedAccess;

pub struct AttrsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    pub parent: Option<Link>,
    pub attrs: AttrsModel,
    _marker: PhantomData<SPEC>,
}
impl<SPEC, Link> AttrsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    pub fn new(attrs: AttrsModel) -> Self {
        Self { parent: None, attrs, _marker: PhantomData }
    }
}

impl<SPEC, Link> Linkable<Link> for AttrsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<SPEC, Link> SourceComposable for AttrsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    type Source = ScopeContextLink;
    type Output = SPEC::Attr;
    fn compose(&self, _context: &Self::Source) -> Self::Output {
        SPEC::Attr::from_attrs(self.attrs.cfg_attributes())
        // self.attrs.cfg_attributes()
        // TODO: currently disable trait expansion via attributes,
        // TODO: migrate onto composable RefinedTree version
        // let attrs_composition = &self.attrs;
        // let source = context.borrow();
        // source.trait_items_from_attributes(&attrs_composition.attrs)
        //     .iter_mut()
        //     .map(|(composition, trait_scope)|
        //         implement_trait_for_item((&composition.item, trait_scope), attrs_composition, context))
        //     .collect()
    }
}
