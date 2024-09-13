use syn::punctuated::Punctuated;
use crate::ast::Depunctuated;
use crate::composer::{BindingAccessorContext, Composer, CtorSequenceComposer, DestructorContext, Linkable, LocalConversionContext, MethodComposer};
use crate::context::ScopeContext;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{BindingPresentableContext, OwnedItemPresentableContext, ScopeContextPresentable};
use crate::shared::SharedAccess;

pub struct FFIBindingsComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub parent: Option<Parent>,
    pub ctor_composer: CtorSequenceComposer<Parent, LANG, SPEC, Gen>,
    pub dtor_composer: MethodComposer<Parent, DestructorContext<LANG, SPEC, Gen>, DestructorContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
    pub getter_composer: MethodComposer<Parent, BindingAccessorContext<LANG, SPEC, Gen>, LocalConversionContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
    pub setter_composer: MethodComposer<Parent, BindingAccessorContext<LANG, SPEC, Gen>, LocalConversionContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
    pub get_set: bool
}

impl<Parent, LANG, SPEC, Gen> Linkable<Parent> for FFIBindingsComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Parent) {
        self.getter_composer.link(parent);
        self.setter_composer.link(parent);
        self.ctor_composer.link(parent);
        self.dtor_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Parent, LANG, SPEC, Gen> Composer<'a> for FFIBindingsComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = Depunctuated<BindingPresentableContext<LANG, SPEC, Gen>>;

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

impl<Parent, LANG, SPEC, Gen> FFIBindingsComposer<Parent, LANG, SPEC, Gen>
    where Parent: SharedAccess,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        ctor_composer: CtorSequenceComposer<Parent, LANG, SPEC, Gen>,
        dtor_composer: MethodComposer<Parent, DestructorContext<LANG, SPEC, Gen>, DestructorContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
        getter_composer: MethodComposer<Parent, BindingAccessorContext<LANG, SPEC, Gen>, LocalConversionContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
        setter_composer: MethodComposer<Parent, BindingAccessorContext<LANG, SPEC, Gen>, LocalConversionContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
        get_set: bool,
    ) -> Self {
        Self { parent: None, ctor_composer, dtor_composer, getter_composer, setter_composer, get_set }
    }
}
