use crate::composer::{Composer, BindingComposer, LocalConversionContext, BindingAccessorContext, DestructorContext, SharedComposer};
use crate::context::ScopeContext;
use crate::ext::FFIResolveExtended;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::shared::{ParentLinker, SharedAccess};

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
impl<Parent, BindingContext, SharedContext> ParentLinker<Parent>
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
            .map(|field_type| {
                (self.seq_iterator_item)((
                    aspect.present(source),
                    field_type.name(),
                    field_type.ty().ffi_full_dictionary_type_presenter(source),
                    field_type.attrs(),
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
