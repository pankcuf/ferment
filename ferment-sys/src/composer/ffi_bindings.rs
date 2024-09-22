use syn::punctuated::Punctuated;
use crate::ast::Depunctuated;
use crate::composer::{BindingAccessorContext, Composer, CtorSequenceComposer, DestructorContext, Linkable, LocalConversionContext, MethodComposer};
use crate::context::ScopeContext;
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, ScopeContextPresentable};
use crate::shared::SharedAccess;

pub struct FFIBindingsComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub parent: Option<Link>,
    pub ctor_composer: CtorSequenceComposer<Link, LANG, SPEC>,
    pub dtor_composer: MethodComposer<Link, DestructorContext<LANG, SPEC>, DestructorContext<LANG, SPEC>, LANG, SPEC>,
    pub getter_composer: MethodComposer<Link, BindingAccessorContext<LANG, SPEC>, LocalConversionContext<LANG, SPEC>, LANG, SPEC>,
    pub setter_composer: MethodComposer<Link, BindingAccessorContext<LANG, SPEC>, LocalConversionContext<LANG, SPEC>, LANG, SPEC>,
    pub get_set: bool
}
impl<Link, LANG, SPEC> FFIBindingsComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        ctor_composer: CtorSequenceComposer<Link, LANG, SPEC>,
        dtor_composer: MethodComposer<Link, DestructorContext<LANG, SPEC>, DestructorContext<LANG, SPEC>, LANG, SPEC>,
        getter_composer: MethodComposer<Link, BindingAccessorContext<LANG, SPEC>, LocalConversionContext<LANG, SPEC>, LANG, SPEC>,
        setter_composer: MethodComposer<Link, BindingAccessorContext<LANG, SPEC>, LocalConversionContext<LANG, SPEC>, LANG, SPEC>,
        get_set: bool,
    ) -> Self {
        Self { parent: None, ctor_composer, dtor_composer, getter_composer, setter_composer, get_set }
    }

    pub fn compose_ctor(&self) -> BindingPresentableContext<LANG, SPEC> {
        self.ctor_composer.compose(&())
    }
}

impl<Link, LANG, SPEC> Linkable<Link> for FFIBindingsComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.getter_composer.link(parent);
        self.setter_composer.link(parent);
        self.ctor_composer.link(parent);
        self.dtor_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Link, LANG, SPEC> Composer<'a> for FFIBindingsComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = Depunctuated<BindingPresentableContext<LANG, SPEC>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let mut bindings = Punctuated::new();
        bindings.push(self.ctor_composer.compose(&()));
        bindings.push(self.dtor_composer.compose(source));
        if self.get_set {
            bindings.extend(self.getter_composer.compose(source));
            bindings.extend(self.setter_composer.compose(source));
        }
        bindings
    }
}

