use crate::composer::r#abstract::Composer;

pub trait ParentLinker<Parent> {
    fn link(&mut self, parent: &Parent);
}

pub trait LinkedComposer<'a, Parent>: Composer<'a> + ParentLinker<Parent> + Sized {}
