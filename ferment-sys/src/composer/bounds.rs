use std::rc::Rc;
use quote::ToTokens;
use syn::Attribute;
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenericBoundsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerOwner, ComposerLink, BasicComposerLink};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentation::DocComposer;

#[derive(ComposerBase)]
pub struct BoundsComposer<SPEC>
    where SPEC: Specification + 'static {
    pub model: GenericBoundsModel,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> BoundsComposer<SPEC>
    where SPEC: Specification {
    pub fn new(
        model: &GenericBoundsModel,
        ty_context: SPEC::TYC,
        attrs: Vec<Attribute>,
        scope_context: &ScopeContextLink
    ) -> Self {
        Self {
            model: model.clone(),
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
        }
    }
}
