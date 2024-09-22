use syn::Type;
use crate::composer::{BindingAccessorContext, BindingComposer, DestructorContext, LocalConversionContext, SharedComposer};
use crate::composer::r#abstract::{Composer, Linkable};
use crate::context::ScopeContext;
use crate::ext::{Resolve, ToType};
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, ScopeContextPresentable};
use crate::presentation::FFIVariable;
use crate::shared::SharedAccess;

pub struct MethodComposer<Link, BindingContext, SharedContext, LANG, SPEC>
    where Link: SharedAccess,
          BindingContext: Clone,
          SharedContext: Clone,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    parent: Option<Link>,
    context: SharedComposer<Link, SharedContext>,
    seq_iterator_item: BindingComposer<BindingContext, LANG, SPEC>
}
impl<Link, BindingContext, SharedContext, LANG, SPEC> MethodComposer<Link, BindingContext, SharedContext, LANG, SPEC>
    where
        Link: SharedAccess,
        BindingContext: Clone,
        SharedContext: Clone,
        LANG: Clone,
        SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
        SPEC::Expr: ScopeContextPresentable,
        Aspect<SPEC::TYC>: ScopeContextPresentable,
        PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        seq_iterator_item: BindingComposer<BindingContext, LANG, SPEC>,
        context: SharedComposer<Link, SharedContext>) -> Self {
        Self {
            parent: None,
            seq_iterator_item,
            context,
        }
    }
}
impl<Link, BindingContext, SharedContext, LANG, SPEC> Linkable<Link>
for MethodComposer<Link, BindingContext, SharedContext, LANG, SPEC>
    where
        Link: SharedAccess,
        BindingContext: Clone,
        SharedContext: Clone,
        LANG: Clone,
        SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
        SPEC::Expr: ScopeContextPresentable,
        Aspect<SPEC::TYC>: ScopeContextPresentable,
        PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}
impl<'a, Link, LANG, SPEC> Composer<'a>
for MethodComposer<Link, BindingAccessorContext<LANG, SPEC>, LocalConversionContext<LANG, SPEC>, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = Vec<BindingPresentableContext<LANG, SPEC>>;
    fn compose(&self, source: &Self::Source) -> Self::Output {
        let ((aspect, context), generics) = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        context.iter()
            .map(|composer| {
                (self.seq_iterator_item)((aspect.present(source),
                    composer.tokenized_name(),
                    <Type as Resolve<FFIVariable>>::resolve(composer.ty(), source).to_type(),
                    composer.attrs.clone(),
                    generics.clone()
                ))
            })
            .collect()
    }
}
impl<'a, Link, LANG, SPEC> Composer<'a>
for MethodComposer<Link, DestructorContext<LANG, SPEC>, DestructorContext<LANG, SPEC>, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = BindingPresentableContext<LANG, SPEC>;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        (self.seq_iterator_item)(
            self.parent.as_ref()
                .expect("no parent")
                .access(self.context))
    }
}
