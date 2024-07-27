use syn::Type;
use crate::composer::{BindingAccessorContext, BindingComposer, DestructorContext, LocalConversionContext, SharedComposer};
use crate::composer::r#abstract::{Composer, Linkable};
use crate::context::ScopeContext;
use crate::ext::{Resolve, ToType};
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{BindingPresentation, FFIVariable};
use crate::shared::SharedAccess;

pub struct MethodComposer<Parent, BindingContext, SharedContext>
    where Parent: SharedAccess, BindingContext: Clone, SharedContext: Clone {
    parent: Option<Parent>,
    context: SharedComposer<Parent, SharedContext>,
    seq_iterator_item: BindingComposer<BindingContext>,
}
impl<Parent, BindingContext, SharedContext> MethodComposer<Parent, BindingContext, SharedContext>
    where
        Parent: SharedAccess,
        BindingContext: Clone,
        SharedContext: Clone {
    pub const fn new(
        seq_iterator_item: BindingComposer<BindingContext>,
        context: SharedComposer<Parent, SharedContext>) -> Self {
        Self {
            parent: None,
            seq_iterator_item,
            context,
        }
    }
}
impl<Parent, BindingContext, SharedContext> Linkable<Parent>
for MethodComposer<Parent, BindingContext, SharedContext>
    where
        Parent: SharedAccess,
        BindingContext: Clone,
        SharedContext: Clone {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}
impl<'a, Parent> Composer<'a>
for MethodComposer<Parent, BindingAccessorContext, LocalConversionContext>
    where Parent: SharedAccess {
    type Source = ScopeContext;
    type Result = Vec<BindingPresentation>;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let ((aspect, context), generics) = self.parent
            .as_ref()
            .expect("no parent")
            .access(self.context);
        context.iter()
            .map(|composer| {
                // println!("MethodComposer::ty: {}", field_type.ty().to_token_stream());
                (self.seq_iterator_item)((
                    aspect.present(source),
                    composer.tokenized_name(),
                    <Type as Resolve<FFIVariable>>::resolve(composer.ty(), source).to_type(),
                    composer.to_attrs(),
                    generics.clone()
                ))
            })
            .collect()
    }
}
impl<'a, Parent> Composer<'a>
for MethodComposer<Parent, DestructorContext, DestructorContext>
    where Parent: SharedAccess {
    type Source = ScopeContext;
    type Result = BindingPresentation;
    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.seq_iterator_item)(
            self.parent.as_ref()
                .expect("no parent")
                .access(self.context))
    }
}
