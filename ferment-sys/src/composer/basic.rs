use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{AttrComposable, AttrsComposer, SourceComposable, ComposerLink, DocsComposable, GenericsComposable, GenericsComposer, Linkable, SourceAccessible, TypeAspect, TypeComposer, LifetimesComposable};
use crate::composer::lifetimes::LifetimesComposer;
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentation::{DocComposer, DocPresentation};
use crate::shared::SharedAccess;

pub type BasicComposerLink<SPEC, T> = BasicComposer<SPEC, ComposerLink<T>>;
pub struct BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    pub context: ScopeContextLink,
    pub attr: AttrsComposer<SPEC, Link>,
    pub doc: DocComposer<SPEC, Link>,
    pub ty: TypeComposer<Link, SPEC::TYC>,
    pub generics: GenericsComposer<SPEC, Link>,
    pub lifetimes: LifetimesComposer<SPEC, Link>,
}
impl<SPEC, Link> BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn new(
        attr: AttrsComposer<SPEC, Link>,
        doc: DocComposer<SPEC, Link>,
        ty: TypeComposer<Link, SPEC::TYC>,
        generics: GenericsComposer<SPEC, Link>,
        lifetimes: LifetimesComposer<SPEC, Link>,
        context: ScopeContextLink
    ) -> Self {
        Self { context, attr, doc, ty, generics, lifetimes }
    }

    pub fn from(
        doc: DocComposer<SPEC, Link>,
        attrs: AttrsModel,
        ty_context: SPEC::TYC,
        generics: GenModel,
        lifetimes: LifetimesModel,
        context: ScopeContextLink
    ) -> Self {
        Self::new(
            AttrsComposer::new(attrs),
            doc,
            TypeComposer::new(ty_context),
            GenericsComposer::<SPEC, Link>::new(generics),
            LifetimesComposer::<SPEC, Link>::new(lifetimes),
            context
        )
    }
}

impl<SPEC, Link> Linkable<Link> for BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        self.attr.link(parent);
        self.generics.link(parent);
        self.lifetimes.link(parent);
        self.ty.link(parent);
        self.doc.link(parent);
    }
}

impl<SPEC, Link> DocsComposable for BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.doc.compose(self.context()))
    }
}

impl<SPEC, Link> AttrComposable<SPEC::Attr> for BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn compose_attributes(&self) -> SPEC::Attr {
        self.attr.compose(self.context())
    }
}

impl<SPEC, Link> GenericsComposable<SPEC::Gen> for BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn compose_generics(&self) -> SPEC::Gen {
        self.generics.compose(self.context())
    }
}
impl<SPEC, Link> LifetimesComposable<SPEC::Lt> for BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn compose_lifetimes(&self) -> SPEC::Lt {
        self.lifetimes.compose(self.context())
    }
}
impl<'a, SPEC, Link> TypeAspect<SPEC::TYC> for BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn type_context_ref(&self) -> &SPEC::TYC {
        &self.ty.context
    }
}

impl<SPEC, Link> SourceAccessible for BasicComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn context(&self) -> &ScopeContextLink {
        &self.context
    }
}
