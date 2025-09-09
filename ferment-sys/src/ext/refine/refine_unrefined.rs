use crate::context::GlobalContext;
use crate::ext::refine::{RefineMut, Unrefined};

#[allow(unused)]
pub trait RefineUnrefined: RefineMut + Unrefined<Unrefinement = Self::Refinement> {
    fn refine(&mut self) {
        let unrefined = self.unrefined();
        self.refine_with(unrefined);
    }
}

impl RefineUnrefined for GlobalContext {}
