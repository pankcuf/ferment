pub trait Linkable<Link> {
    fn link(&mut self, parent: &Link);
}
