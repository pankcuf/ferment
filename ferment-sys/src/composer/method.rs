use crate::composer::{BindingAccessorContext, BindingComposer, AspectArgComposers, SharedComposer, VarComposer};
use crate::composer::r#abstract::{SourceComposable, Linkable};
use crate::context::ScopeContext;
use crate::lang::Specification;
use crate::presentable::BindingPresentableContext;
use crate::shared::SharedAccess;

pub type AccessorMethodComposer<SPEC, Link> = MethodComposer<SPEC, Link, AspectArgComposers<SPEC>, BindingAccessorContext<SPEC>>;
pub type DtorMethodComposer<SPEC, Link> = MethodComposer<SPEC, Link, AspectArgComposers<SPEC>, AspectArgComposers<SPEC>>;

pub struct MethodComposer<SPEC, Link, LinkCtx, CTX>
    where Link: SharedAccess,
          SPEC: Specification {
    parent: Option<Link>,
    context: SharedComposer<Link, LinkCtx>,
    seq_iterator_item: BindingComposer<SPEC, CTX>
}
impl<SPEC, Link, LinkCtx, CTX> MethodComposer<SPEC, Link, LinkCtx, CTX>
    where
        Link: SharedAccess,
        SPEC: Specification {
    pub const fn new(
        context: SharedComposer<Link, LinkCtx>,
        seq_iterator_item: BindingComposer<SPEC, CTX>,
    ) -> Self {
        Self {
            parent: None,
            seq_iterator_item,
            context,
        }
    }
}
impl<SPEC, Link, LinkCtx, CTX> Linkable<Link> for MethodComposer<SPEC, Link, LinkCtx, CTX>
    where
        Link: SharedAccess,
        SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}
impl<SPEC, Link> SourceComposable for AccessorMethodComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification + 'static {
    type Source = ScopeContext;
    type Output = Vec<BindingPresentableContext<SPEC>>;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        let ((aspect, (_attrs, lifetimes, generics), _name_kind), context) = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        Vec::from_iter(
            context.iter()
                .map(|composer| {
                    (self.seq_iterator_item)((
                        aspect.clone(),
                        (composer.attrs.clone(), lifetimes.clone(), generics.clone()),
                        VarComposer::<SPEC>::key_ref_in_composer_scope(composer.ty()),
                        composer.tokenized_name()
                    ))
                })
        )
    }
}
impl<SPEC, Link> SourceComposable for DtorMethodComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    type Source = ScopeContext;
    type Output = BindingPresentableContext<SPEC>;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        let context = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        (self.seq_iterator_item)(context)
    }
}
