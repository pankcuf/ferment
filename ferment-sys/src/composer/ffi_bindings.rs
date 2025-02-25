use crate::ast::Depunctuated;
use crate::composer::{SourceComposable, Linkable, ComposerLink, AccessorMethodComposer, DtorMethodComposer, ArgKindPair, OwnerAspectSequenceSpecComposer, VariableComposer};
use crate::context::ScopeContext;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::BindingPresentableContext;
use crate::shared::SharedAccess;

pub type FFIBindingsComposerLink<LANG, SPEC, T, Iter> = FFIBindingsComposer<LANG, SPEC, ComposerLink<T>, Iter>;
pub type MaybeFFIBindingsComposerLink<LANG, SPEC, T, Iter> = Option<FFIBindingsComposerLink<LANG, SPEC, T, Iter>>;
pub struct FFIBindingsComposer<LANG, SPEC, Link, Iter>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    pub parent: Option<Link>,
    pub ctor: OwnerAspectSequenceSpecComposer<LANG, SPEC, Link, Iter, BindingPresentableContext<LANG, SPEC>>,
    pub dtor: DtorMethodComposer<LANG, SPEC, Link>,
    pub getter: AccessorMethodComposer<LANG, SPEC, Link>,
    pub setter: AccessorMethodComposer<LANG, SPEC, Link>,
    pub get_set: bool
}
impl<LANG, SPEC, Link, Iter> FFIBindingsComposer<LANG, SPEC, Link, Iter>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    pub const fn new(
        ctor: OwnerAspectSequenceSpecComposer<LANG, SPEC, Link, Iter, BindingPresentableContext<LANG, SPEC>>,
        dtor: DtorMethodComposer<LANG, SPEC, Link>,
        getter: AccessorMethodComposer<LANG, SPEC, Link>,
        setter: AccessorMethodComposer<LANG, SPEC, Link>,
        get_set: bool,
    ) -> Self {
        Self { parent: None, ctor, dtor, getter, setter, get_set }
    }

    pub fn compose_ctor(&self) -> BindingPresentableContext<LANG, SPEC> {
        self.ctor.compose(&())
    }
}

impl<LANG, SPEC, Link, Iter> Linkable<Link> for FFIBindingsComposer<LANG, SPEC, Link, Iter>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Link: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>> {
    fn link(&mut self, parent: &Link) {
        self.getter.link(parent);
        self.setter.link(parent);
        self.ctor.link(parent);
        self.dtor.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<LANG, SPEC, Link, Iter> SourceComposable for FFIBindingsComposer<LANG, SPEC, Link, Iter>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Link: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<LANG, SPEC>>,
          VariableComposer<LANG, SPEC>: SourceComposable<Source = ScopeContext, Output = SPEC::Var>,
{
    type Source = ScopeContext;
    type Output = Depunctuated<BindingPresentableContext<LANG, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut bindings = Depunctuated::new();
        bindings.push(self.ctor.compose(&()));
        bindings.push(self.dtor.compose(source));
        if self.get_set {
            bindings.extend(self.getter.compose(source));
            bindings.extend(self.setter.compose(source));
        }
        bindings
    }
}

