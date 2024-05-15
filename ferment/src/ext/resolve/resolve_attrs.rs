use crate::composer::Depunctuated;
use crate::presentation::Expansion;

pub trait ResolveAttrs where Self: Sized {
    fn resolve_attrs(&self) -> Depunctuated<Expansion>;
}
