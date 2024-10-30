use std::fmt::Debug;
use crate::composer::ComposerLink;
use crate::composer::r#abstract::{SourceComposable, Linkable};
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable, SeqKind, Expression, InterfaceKind, ArgKind};
use crate::shared::SharedAccess;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FFIAspect {
    From,
    To,
    Destroy,
    Drop,
}

pub type FFIComposerLink<T, LANG, SPEC> = FFIComposer<ComposerLink<T>, LANG, SPEC>;
pub type MaybeFFIComposerLink<T, LANG, SPEC> = Option<FFIComposerLink<T, LANG, SPEC>>;
pub struct FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    pub parent: Option<Link>,
    //
    // pub interface_composers: HashMap<FFIAspect, InterfaceKind<Link, LANG, SPEC>>,

    pub from_conversion_composer: InterfaceKind<Link, LANG, SPEC>,
    pub to_conversion_composer: InterfaceKind<Link, LANG, SPEC>,
    pub drop_composer: InterfaceKind<Link, LANG, SPEC>,
    pub destroy_composer: InterfaceKind<Link, LANG, SPEC>,
}
impl<Link, LANG, SPEC> FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        // composers: HashMap<FFIAspect, InterfaceKind<Link, LANG, SPEC>>,
        from_conversion_composer: InterfaceKind<Link, LANG, SPEC>,
        to_conversion_composer: InterfaceKind<Link, LANG, SPEC>,
        destroy_composer: InterfaceKind<Link, LANG, SPEC>,
        drop_composer: InterfaceKind<Link, LANG, SPEC>,
    ) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, parent: None }
    }
}


impl<Link, LANG, SPEC> Linkable<Link> for FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.from_conversion_composer.link(parent);
        self.to_conversion_composer.link(parent);
        self.destroy_composer.link(parent);
        self.drop_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Link, LANG, SPEC> SourceComposable for FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    type Source = FFIAspect;
    type Output = SeqKind<LANG, SPEC>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match source {
            FFIAspect::From =>
                self.from_conversion_composer.compose(&()),
            FFIAspect::To =>
                self.to_conversion_composer.compose(&()),
            FFIAspect::Destroy =>
                self.destroy_composer.compose(&()),
            FFIAspect::Drop =>
                self.drop_composer.compose(&())
        }
    }
}
