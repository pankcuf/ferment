use std::collections::HashSet;
use syn::Type;
use crate::composable::GenericBoundsModel;
use crate::ext::visitor::{GenericCollector, TypeCollector};

pub trait GenericConstraintCollector where Self: TypeCollector {
    fn find_generic_constraints(&self) -> HashSet<Type> {
        HashSet::<Type>::new()
    }
}

impl GenericConstraintCollector for GenericBoundsModel {
    fn find_generic_constraints(&self) -> HashSet<Type> {
        let compositions = self.collect_compositions();
        let mut container = HashSet::<Type>::new();
        compositions
            .iter()
            .for_each(|field_type|
                field_type.collect_to(&mut container));
        container
    }
}

