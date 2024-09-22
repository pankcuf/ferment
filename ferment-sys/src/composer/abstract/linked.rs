pub trait Linkable<Parent> {
    fn link(&mut self, parent: &Parent);
}
