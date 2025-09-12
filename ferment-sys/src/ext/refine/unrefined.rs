use std::collections::HashMap;
use crate::context::{GlobalContext, ScopeRefinement};

pub trait Unrefined: Sized {
    type Unrefinement;
    fn unrefined(&self) -> Self::Unrefinement;
}
impl Unrefined for GlobalContext {
    type Unrefinement = ScopeRefinement;
    fn unrefined(&self) -> Self::Unrefinement {
        let mut scope_updates = vec![];
        self.scope_register.inner.iter()
            .for_each(|(scope, type_chain)| {
                let scope_types_to_refine = type_chain.inner.iter()
                    .filter_map(|(ty, object)|
                        self.maybe_refined_object(scope, object)
                            .map(|object_to_refine| (ty.to_owned(), object_to_refine)))
                    .collect::<HashMap<_, _>>();
                if !scope_types_to_refine.is_empty() {
                    scope_updates.push((scope.to_owned(), scope_types_to_refine));
                }
            });
        scope_updates
    }
}

