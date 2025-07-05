use crate::ast::Depunctuated;
use crate::composer::{SourceComposable, Linkable, ComposerLink, AccessorMethodComposer, DtorMethodComposer, ArgKindPair, OwnerAspectSequenceSpecComposer};
use crate::context::ScopeContext;
use crate::lang::Specification;
use crate::presentable::BindingPresentableContext;
use crate::shared::SharedAccess;

pub type FFIBindingsComposerLink<SPEC, T, Iter> = FFIBindingsComposer<SPEC, ComposerLink<T>, Iter>;
pub type MaybeFFIBindingsComposerLink<SPEC, T, Iter> = Option<FFIBindingsComposerLink<SPEC, T, Iter>>;
pub struct FFIBindingsComposer<SPEC, Link, Iter>
    where Link: SharedAccess,
          SPEC: Specification,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    pub parent: Option<Link>,
    pub ctor: OwnerAspectSequenceSpecComposer<SPEC, Link, Iter, BindingPresentableContext<SPEC>>,
    pub dtor: DtorMethodComposer<SPEC, Link>,
    pub getter: AccessorMethodComposer<SPEC, Link>,
    pub setter: AccessorMethodComposer<SPEC, Link>,
    pub get_set: bool
}
impl<SPEC, Link, Iter> FFIBindingsComposer<SPEC, Link, Iter>
    where Link: SharedAccess,
          SPEC: Specification,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    pub const fn new(
        ctor: OwnerAspectSequenceSpecComposer<SPEC, Link, Iter, BindingPresentableContext<SPEC>>,
        dtor: DtorMethodComposer<SPEC, Link>,
        getter: AccessorMethodComposer<SPEC, Link>,
        setter: AccessorMethodComposer<SPEC, Link>,
        get_set: bool,
    ) -> Self {
        Self { parent: None, ctor, dtor, getter, setter, get_set }
    }

    pub fn compose_ctor(&self) -> BindingPresentableContext<SPEC> {
        self.ctor.compose(&())
    }
}

impl<SPEC, Link, Iter> Linkable<Link> for FFIBindingsComposer<SPEC, Link, Iter>
    where SPEC: Specification,
          Link: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    fn link(&mut self, parent: &Link) {
        self.getter.link(parent);
        self.setter.link(parent);
        self.ctor.link(parent);
        self.dtor.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<SPEC, Link, Iter> SourceComposable for FFIBindingsComposer<SPEC, Link, Iter>
    where SPEC: Specification,
          Link: SharedAccess,
          Iter: FromIterator<Iter::Item> + IntoIterator<Item=ArgKindPair<SPEC>> {
    type Source = ScopeContext;
    type Output = Depunctuated<BindingPresentableContext<SPEC>>;

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

