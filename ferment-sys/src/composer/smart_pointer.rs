use std::rc::Rc;
use syn::Attribute;
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, DocComposer};
use crate::context::ScopeContextLink;
use crate::kind::SmartPointerKind;
use crate::lang::Specification;

#[derive(ComposerBase)]
pub struct SmartPointerComposer<SPEC>
where SPEC: Specification + 'static {
    pub root_kind: SmartPointerKind,
    pub kind: SmartPointerKind,
    pub base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> SmartPointerComposer<SPEC>
where SPEC: Specification {
    pub fn new(root_kind: &SmartPointerKind, kind: SmartPointerKind, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::from(&ty_context), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            root_kind: root_kind.clone(),
            kind
        }
    }
}

