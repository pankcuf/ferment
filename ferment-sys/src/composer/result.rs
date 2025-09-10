use std::rc::Rc;
use syn::{Attribute, Type};
use ferment_macro::ComposerBase;
use crate::composable::AttrsModel;
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, DocComposer};
use crate::context::ScopeContextLink;
use crate::lang::Specification;

#[derive(ComposerBase)]
pub struct ResultComposer<SPEC>
    where SPEC: Specification + 'static {
    pub ty: Type,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> ResultComposer<SPEC>
    where SPEC: Specification {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::from(&ty_context), AttrsModel::from(&attrs), ty_context, Default::default(), Default::default(), Rc::clone(scope_context)),
            ty: ty.clone()
        }
    }
}
