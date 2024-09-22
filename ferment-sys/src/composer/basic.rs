use syn::__private::TokenStream2;
use crate::composable::{AttrsModel, GenModel};
use crate::composer::{AttrComposable, AttrsComposer, Composer, ComposerLink, DocsComposable, GenericsComposable, GenericsComposer, Linkable, SourceAccessible, TypeAspect, TypeComposer, TypeContextComposer};
use crate::context::ScopeContext;
use crate::lang::Specification;
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::presentation::DocPresentation;
use crate::shared::SharedAccess;

pub struct BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub context: ComposerLink<ScopeContext>,
    pub attr: AttrsComposer<Link, LANG, SPEC>,
    pub doc: TypeContextComposer<Link, SPEC::TYC, TokenStream2>,
    pub ty: TypeComposer<Link, SPEC::TYC>,
    pub generics: GenericsComposer<Link, LANG, SPEC>,
}
impl<Link, LANG, SPEC> BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn new(
        attr: AttrsComposer<Link, LANG, SPEC>,
        doc: TypeContextComposer<Link, SPEC::TYC, TokenStream2>,
        ty: TypeComposer<Link, SPEC::TYC>,
        generics: GenericsComposer<Link, LANG, SPEC>,
        context: ComposerLink<ScopeContext>
    ) -> Self {
        Self { context, attr, doc, ty, generics }
    }

    pub fn from(
        attrs: AttrsModel,
        ty_context: SPEC::TYC,
        generics: GenModel,
        doc: TypeContextComposer<Link, SPEC::TYC, TokenStream2>,
        context: ComposerLink<ScopeContext>) -> Self {
        Self::new(
            AttrsComposer::new(attrs),
            doc,
            TypeComposer::new(ty_context),
            GenericsComposer::<Link, LANG, SPEC>::new(generics),
            context
        )
    }
}

impl<Link, LANG, SPEC> Linkable<Link> for BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.attr.link(parent);
        self.generics.link(parent);
        self.ty.link(parent);
        self.doc.link(parent);
    }
}

impl<Link, LANG, SPEC> DocsComposable for BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.doc.compose(&()))
    }
}

impl<Link, LANG, SPEC> AttrComposable<SPEC::Attr> for BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC::Attr {
        self.attr.compose(self.context())
    }
}

impl<Link, LANG, SPEC> GenericsComposable<SPEC::Gen> for BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn compose_generics(&self) -> SPEC::Gen {
        self.generics.compose(self.context())
    }
}
impl<'a, Link, LANG, SPEC> TypeAspect<SPEC::TYC> for BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn type_context_ref(&self) -> &SPEC::TYC {
        &self.ty.context
    }
}

impl<Link, LANG, SPEC> SourceAccessible for BasicComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        &self.context
    }
}
