use crate::composer::{Composer, ComposerPresenter, Linkable, SharedComposer};
use crate::shared::SharedAccess;



pub struct LinkedContextComposer<Link, Context, Result>
    where Link: SharedAccess {
    parent: Option<Link>,
    set_output: ComposerPresenter<Context, Result>,
    get_context: SharedComposer<Link, Context>,
}

impl<Link, Context, Result> LinkedContextComposer<Link, Context, Result>
    where Link: SharedAccess {
    pub const fn new(
        set_output: ComposerPresenter<Context, Result>,
        get_context: SharedComposer<Link, Context>
    ) -> Self {
        Self { parent: None, set_output, get_context }
    }
}

impl<Link, Context, Result> Linkable<Link> for LinkedContextComposer<Link, Context, Result>
    where Link: SharedAccess {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Link, Context, Result> Composer<'a> for LinkedContextComposer<Link, Context, Result>
    where Link: SharedAccess {
    type Source = ();
    type Output = Result;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        (self.set_output)(
            self.parent.as_ref()
                .expect("no parent")
                .access(self.get_context))
    }
}


