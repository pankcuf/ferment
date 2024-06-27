use syn::Generics;
use crate::ast::Depunctuated;
use crate::composable::AttrsComposition;
use crate::composer::{AttrsComposer, BasicComposable, Composer, DocsComposable, GenericsComposer, NameContext, ParentComposer, Linkable, SourceAccessible, SourceExpandable, TypeComposer, TypeContextComposer};
use crate::context::ScopeContext;
use crate::presentable::Context;
use crate::presentation::{DocPresentation, Expansion};
use crate::shared::SharedAccess;

pub struct BasicComposer<Parent> where Parent: SharedAccess {
    pub context: ParentComposer<ScopeContext>,
    pub attr: AttrsComposer<Parent>,
    pub doc: TypeContextComposer<Parent>,
    pub ty: TypeComposer<Parent>,
    pub generics: GenericsComposer<Parent>,
}
impl<Parent> Linkable<Parent> for BasicComposer<Parent> where Parent: SharedAccess {
    fn link(&mut self, parent: &Parent) {
        self.attr.link(parent);
        self.generics.link(parent);
        self.ty.link(parent);
        self.doc.link(parent);
    }
}
impl<Parent> DocsComposable for BasicComposer<Parent> where Parent: SharedAccess {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.doc.compose(&()))
    }
}


impl<Parent> BasicComposable<Parent> for BasicComposer<Parent> where Parent: SharedAccess {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.attr.compose(self.context())
    }
    fn compose_generics(&self) -> Option<Generics> {
        self.generics.compose(self.context())
    }
}

impl<Parent> SourceAccessible for BasicComposer<Parent> where Parent: SharedAccess {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        &self.context
    }
}

impl<Parent> SourceExpandable for BasicComposer<Parent> where Parent: SharedAccess {}

impl<Parent> NameContext for BasicComposer<Parent> where Parent: SharedAccess {
    fn name_context_ref(&self) -> &Context {
        &self.ty.context
    }
}
impl<Parent> BasicComposer<Parent> where Parent: SharedAccess {
    pub fn new(
        attr: AttrsComposer<Parent>,
        doc: TypeContextComposer<Parent>,
        ty: TypeComposer<Parent>,
        generics: GenericsComposer<Parent>,
        context: ParentComposer<ScopeContext>
    ) -> BasicComposer<Parent> {
        Self { context, attr, doc, ty, generics }
    }

    pub fn from(attrs: AttrsComposition, name_context: Context, generics: Option<Generics>, doc: TypeContextComposer<Parent>, context: ParentComposer<ScopeContext>) -> BasicComposer<Parent> {
        Self::new(
            AttrsComposer::new(attrs),
            doc,
            TypeComposer::new(name_context),
            GenericsComposer::new(generics),
            context
        )
    }
}
