use crate::composable::{AttrsModel, GenModel};
use crate::composer::{AttrsComposer, Composer, DocsComposable, GenericsComposer, NameContext, ComposerLink, Linkable, SourceAccessible, SourceFermentable2, TypeComposer, TypeContextComposer, AttrComposable, GenericsComposable};
use crate::context::ScopeContext;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::Context;
use crate::presentation::DocPresentation;
use crate::shared::SharedAccess;

pub struct BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    pub context: ComposerLink<ScopeContext>,
    pub attr: AttrsComposer<Link, LANG, SPEC, Gen>,
    pub doc: TypeContextComposer<Link>,
    pub ty: TypeComposer<Link, Context>,
    pub generics: GenericsComposer<Link, LANG, Gen>,
}
impl<Link, LANG, SPEC, Gen> Linkable<Link> for BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn link(&mut self, parent: &Link) {
        self.attr.link(parent);
        self.generics.link(parent);
        self.ty.link(parent);
        self.doc.link(parent);
    }
}
impl<Link, LANG, SPEC, Gen> BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn new(
        attr: AttrsComposer<Link, LANG, SPEC, Gen>,
        doc: TypeContextComposer<Link>,
        ty: TypeComposer<Link, Context>,
        generics: GenericsComposer<Link, LANG, Gen>,
        context: ComposerLink<ScopeContext>
    ) -> Self {
        Self { context, attr, doc, ty, generics }
    }

    pub fn from(attrs: AttrsModel, name_context: Context, generics: GenModel, doc: TypeContextComposer<Link>, context: ComposerLink<ScopeContext>) -> Self {
        Self::new(
            AttrsComposer::new(attrs),
            doc,
            TypeComposer::new(name_context),
            GenericsComposer::<Link, LANG, Gen>::new(generics),
            context
        )
    }
}

impl<Parent, LANG, SPEC, Gen> DocsComposable for BasicComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.doc.compose(&()))
    }
}

impl<Link, LANG, SPEC, Gen> AttrComposable<SPEC> for BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn compose_attributes(&self) -> SPEC {
        self.attr.compose(self.context())
    }
}

impl<Link, LANG, SPEC, Gen> GenericsComposable<Gen> for BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn compose_generics(&self) -> Gen {
        self.generics.compose(self.context())
    }
}
impl<'a, Link, LANG, SPEC, Gen> NameContext<Context> for BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn name_context_ref(&self) -> &Context {
        &self.ty.context
    }
}

impl<Link, LANG, SPEC, Gen> SourceAccessible for BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        &self.context
    }
}

impl<Link, LANG, SPEC, Gen> SourceFermentable2<LANG> for BasicComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {}


