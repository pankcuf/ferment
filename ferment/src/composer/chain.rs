use crate::composer::composable::Composable;
use crate::composer::ParentComposer;
use crate::presentation::Expansion;

pub trait ComposerChain<T> where T: Composable {
    fn composer(&self) -> ParentComposer<T>;
    fn setup(&self) -> ParentComposer<T>;
    fn expand(&self) -> Expansion;
}
