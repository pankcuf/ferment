use std::collections::HashSet;
use crate::ast::TypeHolder;
use crate::composable::GenericBoundsModel;
use crate::ext::visitor::{GenericCollector, TypeCollector};

pub trait GenericConstraintCollector where Self: TypeCollector {
    fn find_generic_constraints(&self) -> HashSet<TypeHolder> {
        let generics: HashSet<TypeHolder> = HashSet::new();
        generics
    }
}

impl GenericConstraintCollector for GenericBoundsModel {
    fn find_generic_constraints(&self) -> HashSet<TypeHolder> {
        let compositions = self.collect_compositions();
        let mut container: HashSet<TypeHolder> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypeHolder(field_type)|
                field_type.collect_to(&mut container));
        container
    }
}

