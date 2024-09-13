use std::marker::PhantomData;
use syn::Type;
use crate::composer::{BindingAccessorContext, DestructorContext, LocalConversionContext, SharedComposer, ComposerPresenter};
use crate::composer::r#abstract::{Composer, Linkable};
use crate::context::ScopeContext;
use crate::ext::{Resolve, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{BindingPresentableContext, OwnedItemPresentableContext, ScopeContextPresentable};
use crate::presentation::FFIVariable;
use crate::shared::SharedAccess;

pub struct MethodComposer<Link, BindingContext, SharedContext, LANG, SPEC, Gen>
    where Link: SharedAccess,
          BindingContext: Clone,
          SharedContext: Clone,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    parent: Option<Link>,
    context: SharedComposer<Link, SharedContext>,
    // seq_iterator_item: RustBindingComposer<BindingContext>,
    // seq_iterator_item: BindingComposer<BindingContext, S, SP, I, LANG, SPEC>,
    seq_iterator_item: ComposerPresenter<BindingContext, BindingPresentableContext<LANG, SPEC, Gen>>
}
impl<Link, BindingContext, SharedContext, LANG, SPEC, Gen> MethodComposer<Link, BindingContext, SharedContext, LANG, SPEC, Gen>
    where
        Link: SharedAccess,
        BindingContext: Clone,
        SharedContext: Clone,
        // I: DelimiterTrait + ?Sized,
        LANG: Clone,
        SPEC: LangAttrSpecification<LANG>,
        Gen: LangGenSpecification<LANG>,
        OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub const fn new(
        seq_iterator_item: ComposerPresenter<BindingContext, BindingPresentableContext<LANG, SPEC, Gen>>,
        context: SharedComposer<Link, SharedContext>) -> Self {
        Self {
            parent: None,
            seq_iterator_item,
            context,
        }
    }
}
impl<Link, BindingContext, SharedContext, LANG, SPEC, Gen> Linkable<Link>
for MethodComposer<Link, BindingContext, SharedContext, LANG, SPEC, Gen>
    where
        Link: SharedAccess,
        BindingContext: Clone,
        SharedContext: Clone,
        // I: DelimiterTrait + ?Sized,
        LANG: Clone,
        SPEC: LangAttrSpecification<LANG>,
        Gen: LangGenSpecification<LANG>,
        OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}
impl<'a, Link, LANG, SPEC, Gen> Composer<'a>
for MethodComposer<Link, BindingAccessorContext<LANG, SPEC, Gen>, LocalConversionContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>
    where Link: SharedAccess,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = Vec<BindingPresentableContext<LANG, SPEC, Gen>>;
    fn compose(&self, source: &Self::Source) -> Self::Output {
        let ((aspect, context), generics) = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        context.iter()
            .map(|composer| {
                (self.seq_iterator_item)((
                    aspect.present(source),
                    composer.tokenized_name(),
                    <Type as Resolve<FFIVariable>>::resolve(composer.ty(), source).to_type(),
                    composer.attrs.clone(),
                    generics.clone(),
                    PhantomData::default()
                ))
            })
            .collect()
    }
}
impl<'a, Link, LANG, SPEC, Gen> Composer<'a> for MethodComposer<Link, DestructorContext<LANG, SPEC, Gen>, DestructorContext<LANG, SPEC, Gen>, LANG, SPEC, Gen>
    where Link: SharedAccess,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = BindingPresentableContext<LANG, SPEC, Gen>;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        (self.seq_iterator_item)(
            self.parent.as_ref()
                .expect("no parent")
                .access(self.context))
    }
}
