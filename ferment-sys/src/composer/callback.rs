use std::rc::Rc;
use quote::ToTokens;
use syn::Attribute;
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, DocComposer};
use crate::context::ScopeContextLink;
use crate::kind::CallbackKind;
use crate::lang::Specification;

#[derive(ComposerBase)]
pub struct CallbackComposer<SPEC>
    where SPEC: Specification + 'static {
    pub kind: CallbackKind,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> CallbackComposer<SPEC>
    where SPEC: Specification {
    pub fn new(kind: &CallbackKind, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            kind: kind.clone()
        }
    }
}
