use crate::composer::{SourceComposable, Composer, Linkable, SharedComposer};
use crate::shared::SharedAccess;
// L: Link,
// C: Context,
// R: Result
pub struct LinkedContextComposer<L, C, R>
    where L: SharedAccess {
    link: Option<L>,
    set_output: Composer<C, R>,
    get_context: SharedComposer<L, C>,
}

impl<L, C, R> LinkedContextComposer<L, C, R>
    where L: SharedAccess {
    pub const fn new(
        set_output: Composer<C, R>,
        get_context: SharedComposer<L, C>
    ) -> Self {
        Self { link: None, set_output, get_context }
    }
}

impl<L, C, R> Linkable<L> for LinkedContextComposer<L, C, R>
    where L: SharedAccess {
    fn link(&mut self, link: &L) {
        self.link = Some(link.clone_container());
    }
}

impl<L, C, R> SourceComposable for LinkedContextComposer<L, C, R>
    where L: SharedAccess {
    type Source = ();
    type Output = R;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        (self.set_output)(
            self.link.as_ref()
                .expect("no parent")
                .access(self.get_context))
    }
}

