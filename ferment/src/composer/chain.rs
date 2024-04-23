use crate::composer::composable::BindingComposable;
use crate::composer::ParentComposer;
use crate::presentation::Expansion;

pub trait ComposerChain<T> where T: BindingComposable {
    fn composer(&self) -> ParentComposer<T>;
    fn setup(&self) -> ParentComposer<T>;
    fn expand(&self) -> Expansion;
}
