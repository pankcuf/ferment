use crate::composer::{BindingComposable, ParentComposer};
use crate::presentation::Expansion;

#[allow(unused)]
pub trait ComposerChain<T> where T: BindingComposable {
    fn composer(&self) -> ParentComposer<T>;
    fn setup(&self) -> ParentComposer<T>;
    fn expand(&self) -> Expansion;
}
