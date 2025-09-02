use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, TypeTuple};
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, DocComposer};
use crate::context::ScopeContextLink;
use crate::lang::Specification;

#[derive(ComposerBase)]
pub struct TupleComposer<SPEC>
    where SPEC: Specification + 'static {
    pub type_tuple: TypeTuple,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> TupleComposer<SPEC>
    where SPEC: Specification {
    pub fn new(type_tuple: &TypeTuple, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            type_tuple: type_tuple.clone(),
        }
    }
}

