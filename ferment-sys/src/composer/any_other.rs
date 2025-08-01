use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Type};
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentation::DocComposer;

#[derive(ComposerBase)]
pub struct AnyOtherComposer<SPEC>
    where SPEC: Specification + 'static {
    pub ty: Type,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> AnyOtherComposer<SPEC>
    where SPEC: Specification {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            ty: ty.clone(),
        }
    }
}
