use syn::punctuated::Punctuated;
use crate::ast::{DelimiterTrait, Depunctuated};
use crate::composer::{BindingAccessorContext, Composer, CtorSequenceComposer, DestructorContext, Linkable, LocalConversionContext, MethodComposer};
use crate::context::ScopeContext;
use crate::presentable::ScopeContextPresentable;
use crate::presentation::BindingPresentation;
use crate::shared::SharedAccess;

pub struct FFIBindingsComposer<Parent, I>
    where Parent: SharedAccess, I: DelimiterTrait + ?Sized {
    pub parent: Option<Parent>,
    pub ctor_composer: CtorSequenceComposer<Parent, I>,
    pub dtor_composer: MethodComposer<Parent, DestructorContext, DestructorContext>,
    pub getter_composer: MethodComposer<Parent, BindingAccessorContext, LocalConversionContext>,
    pub setter_composer: MethodComposer<Parent, BindingAccessorContext, LocalConversionContext>,
}

impl<Parent, I> Linkable<Parent> for FFIBindingsComposer<Parent, I>
    where Parent: SharedAccess, I: DelimiterTrait + ?Sized {
    fn link(&mut self, parent: &Parent) {
        self.getter_composer.link(parent);
        self.setter_composer.link(parent);
        self.ctor_composer.link(parent);
        self.dtor_composer.link(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent, I> FFIBindingsComposer<Parent, I>
    where Parent: SharedAccess, I: DelimiterTrait + ?Sized {
    pub const fn new(
        ctor_composer: CtorSequenceComposer<Parent, I>,
        dtor_composer: MethodComposer<Parent, DestructorContext, DestructorContext>,
        getter_composer: MethodComposer<Parent, BindingAccessorContext, LocalConversionContext>,
        setter_composer: MethodComposer<Parent, BindingAccessorContext, LocalConversionContext>
    ) -> Self {
        Self { parent: None, ctor_composer, dtor_composer, getter_composer, setter_composer }
    }
    pub fn compose_bindings(&self, source: &ScopeContext, get_set: bool) -> Depunctuated<BindingPresentation> {
        let mut bindings = Punctuated::new();
        bindings.push(self.ctor_composer.compose(&()).present(source));
        bindings.push(self.dtor_composer.compose(source));
        if get_set {
            bindings.extend(self.getter_composer.compose(source));
            bindings.extend(self.setter_composer.compose(source));
        }
        bindings
    }
}
