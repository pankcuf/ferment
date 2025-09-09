use std::cell::RefCell;
use std::rc::Rc;
use syn::{Generics, Lifetime, Type};
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerOwner, BasicComposerLink, ComposerLink, DocComposer, DocsComposable, Linkable, SourceAccessible, SourceComposable};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentable::Aspect;
use crate::presentation::DocPresentation;

pub enum FnImplContext<'a, SPEC> where SPEC: Specification {
    TypeImpl { self_ty: &'a Type, aspect: Aspect<SPEC::TYC> },
    TraitImpl { self_ty: &'a Type, trait_ty: &'a Type },
}
impl<SPEC> FnImplContext<'_, SPEC> where SPEC: Specification {
    pub fn self_ty(&self) -> &Type {
        match self {
            FnImplContext::TypeImpl { self_ty, .. } |
            FnImplContext::TraitImpl { self_ty, .. } => self_ty,
        }
    }
}

#[allow(unused)]
#[derive(ComposerBase)]
pub struct TraitAsTypeComposer<SPEC>
where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> TraitAsTypeComposer<SPEC>
where SPEC: Specification {

    #[allow(unused)]
    fn new(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::from(&ty_context), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
}

impl<SPEC> DocsComposable for TraitAsTypeComposer<SPEC>
where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}
