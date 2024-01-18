use std::collections::HashSet;
use quote::ToTokens;
use crate::context::VisitorContext;

pub trait Conversion {
    type Item: ToTokens;
    fn nested_items_into_container(item: &Self::Item, visitor_context: &VisitorContext, container: &mut HashSet<Self::Item>);
    fn nested_items(item: &Self::Item, visitor_context: &VisitorContext) -> HashSet<Self::Item>;
    // fn nested_dictionary()
}

