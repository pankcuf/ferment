use crate::composer::{ComposerPresenter, SharedComposer};
use crate::composer::r#abstract::{Composer, LinkedComposer, ParentLinker};
use crate::shared::SharedAccess;

pub struct ContextComposer<Context, Result, Parent: SharedAccess> {
    parent: Option<Parent>,
    set_output: ComposerPresenter<Context, Result>,
    get_context: SharedComposer<Parent, Context>,
}

impl<Context, Result, Parent: SharedAccess> ContextComposer<Context, Result, Parent> {
    pub const fn new(
        set_output: ComposerPresenter<Context, Result>,
        get_context: SharedComposer<Parent, Context>
    ) -> Self {
        Self { parent: None, set_output, get_context }
    }
}

impl<Context, Result, Parent: SharedAccess> ParentLinker<Parent> for ContextComposer<Context, Result, Parent> {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Context, Result, Parent> Composer<'a> for ContextComposer<Context, Result, Parent>
    where Parent: SharedAccess {
    type Source = ();
    type Result = Result;
    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.set_output)(
            self.parent.as_ref()
                .expect("no parent")
                .access(self.get_context))
    }
}

impl<'a, Context, Result, Parent: SharedAccess> LinkedComposer<'a, Parent> for ContextComposer<Context, Result, Parent> {}



