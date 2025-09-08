use syn::{TraitBound, TypeParamBound};

pub trait MaybeTraitBound {
    fn maybe_trait_bound(&self) -> Option<&TraitBound>;
    fn maybe_trait_bound_mut(&mut self) -> Option<&mut TraitBound>;
}

impl MaybeTraitBound for TypeParamBound {
    fn maybe_trait_bound(&self) -> Option<&TraitBound> {
        match self {
            TypeParamBound::Trait(trait_bound) =>
                Some(trait_bound),
            _ => None
        }
    }

    fn maybe_trait_bound_mut(&mut self) -> Option<&mut TraitBound> {
        match self {
            TypeParamBound::Trait(trait_bound) =>
                Some(trait_bound),
            _ => None
        }
    }
}
