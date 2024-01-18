use crate::composition::TraitDecompositionPart1;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum VisitorContext {
    Trait(Option<TraitDecompositionPart1>),
    Object,
    Unknown,
}
