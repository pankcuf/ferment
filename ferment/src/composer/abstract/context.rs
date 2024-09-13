use crate::composer::{Composer, ComposerPresenter, Linkable, SharedComposer};
use crate::shared::SharedAccess;

pub struct ContextComposer<Context, Result, Parent> where Parent: SharedAccess {
    parent: Option<Parent>,
    set_output: ComposerPresenter<Context, Result>,
    get_context: SharedComposer<Parent, Context>,
}

impl<Context, Result, Parent> ContextComposer<Context, Result, Parent>
    where Parent: SharedAccess {
    pub const fn new(
        set_output: ComposerPresenter<Context, Result>,
        get_context: SharedComposer<Parent, Context>
    ) -> Self {
        Self { parent: None, set_output, get_context }
    }
}

impl<Context, Result, Parent> Linkable<Parent> for ContextComposer<Context, Result, Parent>
    where Parent: SharedAccess {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Context, Result, Parent> Composer<'a> for ContextComposer<Context, Result, Parent>
    where Parent: SharedAccess {
    type Source = ();
    type Output = Result;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        (self.set_output)(
            self.parent.as_ref()
                .expect("no parent")
                .access(self.get_context))
    }
}


