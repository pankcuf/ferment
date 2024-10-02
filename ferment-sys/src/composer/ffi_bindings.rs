use syn::Type;
use crate::ast::Depunctuated;
use crate::composer::{SourceComposable, CtorSequenceComposer, Linkable, ComposerLink, AccessorMethodComposer, DtorMethodComposer};
use crate::context::ScopeContext;
use crate::ext::{Resolve, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, PresentableArgument, ScopeContextPresentable};
use crate::shared::SharedAccess;

pub type FFIBindingsComposerLink<T, LANG, SPEC> = FFIBindingsComposer<ComposerLink<T>, LANG, SPEC>;
pub type MaybeFFIBindingsComposerLink<T, LANG, SPEC> = Option<FFIBindingsComposerLink<T, LANG, SPEC>>;
pub struct FFIBindingsComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub parent: Option<Link>,
    pub ctor_composer: CtorSequenceComposer<Link, LANG, SPEC>,
    pub dtor_composer: DtorMethodComposer<Link, LANG, SPEC>,
    pub getter_composer: AccessorMethodComposer<Link, LANG, SPEC>,
    pub setter_composer: AccessorMethodComposer<Link, LANG, SPEC>,
    pub get_set: bool
}
impl<Link, LANG, SPEC> FFIBindingsComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        ctor_composer: CtorSequenceComposer<Link, LANG, SPEC>,
        dtor_composer: DtorMethodComposer<Link, LANG, SPEC>,
        getter_composer: AccessorMethodComposer<Link, LANG, SPEC>,
        setter_composer: AccessorMethodComposer<Link, LANG, SPEC>,
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
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
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

impl<Link, LANG, SPEC> SourceComposable for FFIBindingsComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Type: Resolve<<SPEC as Specification<LANG>>::Var> {
    type Source = ScopeContext;
    type Output = Depunctuated<BindingPresentableContext<LANG, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut bindings = Depunctuated::new();
        bindings.push(self.ctor_composer.compose(&()));
        bindings.push(self.dtor_composer.compose(source));
        if self.get_set {
            bindings.extend(self.getter_composer.compose(source));
            bindings.extend(self.setter_composer.compose(source));
        }
        bindings
    }
}

