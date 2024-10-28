use std::fmt::Debug;
use syn::Type;
use crate::composer::{BindingAccessorContext, BindingComposer, DestructorContext, LocalConversionContext, SharedComposer};
use crate::composer::r#abstract::{SourceComposable, Linkable};
use crate::context::ScopeContext;
use crate::ext::{Resolve, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, ArgKind, ScopeContextPresentable};
use crate::shared::SharedAccess;

pub type AccessorMethodComposer<Link, LANG, SPEC> = MethodComposer<Link, LocalConversionContext<LANG, SPEC>, BindingAccessorContext<LANG, SPEC>, LANG, SPEC>;
pub type DtorMethodComposer<Link, LANG, SPEC> = MethodComposer<Link, DestructorContext<LANG, SPEC>, DestructorContext<LANG, SPEC>, LANG, SPEC>;

pub struct MethodComposer<Link, LinkCtx, CTX, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    parent: Option<Link>,
    context: SharedComposer<Link, LinkCtx>,
    seq_iterator_item: BindingComposer<CTX, LANG, SPEC>
}
impl<Link, LinkCtx, CTX, LANG, SPEC> MethodComposer<Link, LinkCtx, CTX, LANG, SPEC>
    where
        Link: SharedAccess,
        LANG: LangFermentable,
        SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
        SPEC::Expr: ScopeContextPresentable,
        Aspect<SPEC::TYC>: ScopeContextPresentable,
        ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        context: SharedComposer<Link, LinkCtx>,
        seq_iterator_item: BindingComposer<CTX, LANG, SPEC>,
    ) -> Self {
        Self {
            parent: None,
            seq_iterator_item,
            context,
        }
    }
}
impl<Link, LinkCtx, CTX, LANG, SPEC> Linkable<Link> for MethodComposer<Link, LinkCtx, CTX, LANG, SPEC>
    where
        Link: SharedAccess,
        LANG: LangFermentable,
        SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
        SPEC::Expr: ScopeContextPresentable,
        Aspect<SPEC::TYC>: ScopeContextPresentable,
        ArgKind<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}
impl<Link, LANG, SPEC> SourceComposable for AccessorMethodComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Type: Resolve<<SPEC as Specification<LANG>>::Var> {
    type Source = ScopeContext;
    type Output = Vec<BindingPresentableContext<LANG, SPEC>>;
    fn compose(&self, source: &Self::Source) -> Self::Output {
        let ((aspect, generics), context) = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        context.iter()
            .map(|composer|
                (self.seq_iterator_item)((
                    aspect.present(source),
                    composer.tokenized_name(),
                    <Type as Resolve<SPEC::Var>>::resolve(composer.ty(), source),
                    composer.attrs.clone(),
                    generics.clone())))
            .collect()
    }
}
impl<Link, LANG, SPEC> SourceComposable for DtorMethodComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
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
