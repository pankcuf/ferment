use crate::composer::parent_composer::IParentComposer;
use crate::composer::ParentComposer;
use crate::presentation::Expansion;

pub trait ComposerChain<T> where T: IParentComposer {
    fn composer(&self) -> ParentComposer<T>;
    fn setup(&self) -> ParentComposer<T>;
    fn expand(&self) -> Expansion;
}
