use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::Attribute;
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, Linkable, SigComposerLink};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentation::DocComposer;

#[derive(ComposerBase)]
pub struct VTableComposer<SPEC>
    where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
    pub vtable_method_composers: Vec<SigComposerLink<SPEC>>,
}

impl<SPEC> VTableComposer<SPEC>
where SPEC: Specification {
    pub fn from_trait_path(ty_context: SPEC::TYC, attrs: &Vec<Attribute>, vtable_method_composers: Vec<SigComposerLink<SPEC>>, context: ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                DocComposer::new(ty_context.to_token_stream()),
                AttrsModel::from(attrs),
                ty_context,
                GenModel::default(),
                LifetimesModel::default(),
                context),
            vtable_method_composers
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root

    }
}
