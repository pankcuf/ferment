use crate::composer::{BindingAccessorContext, BindingComposer, AspectArgComposers, SharedComposer, VariableComposer};
use crate::composer::r#abstract::{SourceComposable, Linkable};
use crate::context::ScopeContext;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::BindingPresentableContext;
use crate::shared::SharedAccess;

pub type AccessorMethodComposer<LANG, SPEC, Link> = MethodComposer<LANG, SPEC, Link, AspectArgComposers<LANG, SPEC>, BindingAccessorContext<LANG, SPEC>>;
pub type DtorMethodComposer<LANG, SPEC, Link> = MethodComposer<LANG, SPEC, Link, AspectArgComposers<LANG, SPEC>, AspectArgComposers<LANG, SPEC>>;

pub struct MethodComposer<LANG, SPEC, Link, LinkCtx, CTX>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    parent: Option<Link>,
    context: SharedComposer<Link, LinkCtx>,
    seq_iterator_item: BindingComposer<LANG, SPEC, CTX>
}
impl<LANG, SPEC, Link, LinkCtx, CTX> MethodComposer<LANG, SPEC, Link, LinkCtx, CTX>
    where
        Link: SharedAccess,
        LANG: LangFermentable,
        SPEC: Specification<LANG> {
    pub const fn new(
        context: SharedComposer<Link, LinkCtx>,
        seq_iterator_item: BindingComposer<LANG, SPEC, CTX>,
    ) -> Self {
        Self {
            parent: None,
            seq_iterator_item,
            context,
        }
    }
}
impl<LANG, SPEC, Link, LinkCtx, CTX> Linkable<Link> for MethodComposer<LANG, SPEC, Link, LinkCtx, CTX>
    where
        Link: SharedAccess,
        LANG: LangFermentable,
        SPEC: Specification<LANG> {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}
impl<LANG, SPEC, Link> SourceComposable for AccessorMethodComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    type Source = ScopeContext;
    type Output = Vec<BindingPresentableContext<LANG, SPEC>>;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        let ((aspect, _attrs, generics, _name_kind), context) = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        Vec::from_iter(
            context.iter()
                .map(|composer| {
                    (self.seq_iterator_item)((
                        aspect.clone(),
                        composer.attrs.clone(),
                        generics.clone(),
                        VariableComposer::<LANG, SPEC>::from(composer.ty()),
                        composer.tokenized_name()
                    ))
                })
        )
    }
}
impl<LANG, SPEC, Link> SourceComposable for DtorMethodComposer<LANG, SPEC, Link>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    type Source = ScopeContext;
    type Output = BindingPresentableContext<LANG, SPEC>;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        let context = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        (self.seq_iterator_item)(context)
    }
}
