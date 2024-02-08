use std::collections::HashSet;
use std::hash::Hash;
use quote::ToTokens;

pub trait Conversion {
    type Item: ToTokens + Eq + Hash;
    fn nested_items(item: &Self::Item) -> HashSet<Self::Item>;
    fn nested_items_into_container(item: &Self::Item, container: &mut HashSet<Self::Item>) {
        container.extend(Self::nested_items(item));
    }
}

