use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{AttrComposable, AttrsComposer, SourceComposable, ComposerLink, DocsComposable, GenericsComposable, GenericsComposer, Linkable, SourceAccessible, TypeAspect, TypeComposer, LifetimesComposable};
use crate::composer::lifetimes::LifetimesComposer;
use crate::context::ScopeContextLink;
use crate::lang::{LangFermentable, Specification};
use crate::presentation::{DocComposer, DocPresentation};
use crate::shared::SharedAccess;

pub type BasicComposerLink<LANG, SPEC, T> = BasicComposer<LANG, SPEC, ComposerLink<T>>;
pub struct BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub context: ScopeContextLink,
    pub attr: AttrsComposer<LANG, SPEC, Link>,
    pub doc: DocComposer<LANG, SPEC, Link>,
    pub ty: TypeComposer<Link, SPEC::TYC>,
    pub generics: GenericsComposer<LANG, SPEC, Link>,
    pub lifetimes: LifetimesComposer<LANG, SPEC, Link>,
}
impl<LANG, SPEC, Link> BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn new(
        attr: AttrsComposer<LANG, SPEC, Link>,
        doc: DocComposer<LANG, SPEC, Link>,
        ty: TypeComposer<Link, SPEC::TYC>,
        generics: GenericsComposer<LANG, SPEC, Link>,
        lifetimes: LifetimesComposer<LANG, SPEC, Link>,
        context: ScopeContextLink
    ) -> Self {
        Self { context, attr, doc, ty, generics, lifetimes }
    }

    pub fn from(
        doc: DocComposer<LANG, SPEC, Link>,
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
            GenericsComposer::<LANG, SPEC, Link>::new(generics),
            LifetimesComposer::<LANG, SPEC, Link>::new(lifetimes),
            context
        )
    }
}

impl<LANG, SPEC, Link> Linkable<Link> for BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn link(&mut self, parent: &Link) {
        self.attr.link(parent);
        self.generics.link(parent);
        self.lifetimes.link(parent);
        self.ty.link(parent);
        self.doc.link(parent);
    }
}

impl<LANG, SPEC, Link> DocsComposable for BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.doc.compose(self.context()))
    }
}

impl<LANG, SPEC, Link> AttrComposable<SPEC::Attr> for BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_attributes(&self) -> SPEC::Attr {
        self.attr.compose(self.context())
    }
}

impl<LANG, SPEC, Link> GenericsComposable<SPEC::Gen> for BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_generics(&self) -> SPEC::Gen {
        self.generics.compose(self.context())
    }
}
impl<LANG, SPEC, Link> LifetimesComposable<SPEC::Lt> for BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_lifetimes(&self) -> SPEC::Lt {
        self.lifetimes.compose(self.context())
    }
}
impl<'a, LANG, SPEC, Link> TypeAspect<SPEC::TYC> for BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn type_context_ref(&self) -> &SPEC::TYC {
        &self.ty.context
    }
}

impl<LANG, SPEC, Link> SourceAccessible for BasicComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn context(&self) -> &ScopeContextLink {
        &self.context
    }
}
