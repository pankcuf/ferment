use crate::composer::{DropSequenceExprMixer, FFIConversionExprMixer, SequenceOutputComposer};
use crate::composer::r#abstract::{Composer, Linkable};
use crate::lang::Specification;
use crate::presentable::{Aspect, ScopeContextPresentable, PresentableSequence, Expression};
use crate::shared::SharedAccess;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FFIAspect {
    From,
    To,
    Destroy,
    Drop,
}

pub struct FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    pub parent: Option<Link>,
    pub from_conversion_composer: FFIConversionExprMixer<Link, LANG, SPEC>,
    pub to_conversion_composer: FFIConversionExprMixer<Link, LANG, SPEC>,
    pub drop_composer: DropSequenceExprMixer<Link, LANG, SPEC>,
    pub destroy_composer: SequenceOutputComposer<Link, LANG, SPEC>,
}
impl<Link, LANG, SPEC> FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        from_conversion_composer: FFIConversionExprMixer<Link, LANG, SPEC>,
        to_conversion_composer: FFIConversionExprMixer<Link, LANG, SPEC>,
        destroy_composer: SequenceOutputComposer<Link, LANG, SPEC>,
        drop_composer: DropSequenceExprMixer<Link, LANG, SPEC>,
    ) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, parent: None }
    }
}


impl<Link, LANG, SPEC> Linkable<Link> for FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.from_conversion_composer.link(parent);
        self.to_conversion_composer.link(parent);
        self.destroy_composer.link(parent);
        self.drop_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Link, LANG, SPEC> Composer<'a> for FFIComposer<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    type Source = FFIAspect;
    type Output = PresentableSequence<LANG, SPEC>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
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
