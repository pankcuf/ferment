use std::collections::HashMap;
use crate::conversion::ObjectConversion;
use crate::holder::TypeHolder;
use crate::tree::ScopeTreeExportItem;
use crate::visitor::Visitor;

pub trait MergeInto {
    fn merge_into(&self, destination: &mut Self);
}

impl MergeInto for ScopeTreeExportItem {
    fn merge_into(&self, destination: &mut Self) {
        if let (ScopeTreeExportItem::Tree(_dest_context, _, _, ref mut dest_exports),
            ScopeTreeExportItem::Tree(_source_context, _, _, source_exports), ) = (destination, &self) {
            // println!("merge_trees: source: {}", source_context);
            // println!("merge_trees: destination: {}", dest_context);
            for (name, source_tree) in source_exports {
                match dest_exports.entry(name.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut o) =>
                        source_tree.merge_into(o.get_mut()),
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(source_tree.clone());
                    }
                }
            }
        }
    }
}
impl MergeInto for HashMap<TypeHolder, ObjectConversion> {
    fn merge_into(&self, destination: &mut Self) {
        for (holder, object) in self {
            match destination.entry(holder.clone()) {
                std::collections::hash_map::Entry::Occupied(mut o) => match (o.get_mut(), &object) {
                    (ObjectConversion::Type(..), ObjectConversion::Item(..)) => {
                        o.insert(object.clone());
                    },
                    _ => {}
                },
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(object.clone());
                }
            }
        }
    }
}

impl MergeInto for ObjectConversion {
    fn merge_into(&self, destination: &mut Self) {
        match (&self, &destination) {
            (ObjectConversion::Item(..), ObjectConversion::Type(..)) => {
                *destination = self.clone();
            },
            _ => {}
        }
    }
}

pub fn merge_scope_type(destination: &mut HashMap<TypeHolder, ObjectConversion>, holder: TypeHolder, object: ObjectConversion) {
    match destination.entry(holder) {
        std::collections::hash_map::Entry::Occupied(mut o) => {
            match (o.get_mut(), &object) {
                (ObjectConversion::Type(..), ObjectConversion::Item(..)) => {
                    o.insert(object);
                },
                _ => {}
            }
        },
        std::collections::hash_map::Entry::Vacant(v) => {
            v.insert(object);
        }
    }
}


pub fn merge_visitor_trees(visitor: &mut Visitor) {
    // Merge the trees of the inner visitors first.
    for inner_visitor in &mut visitor.inner_visitors {
        merge_visitor_trees(inner_visitor);
    }
    // Now merge the trees of the inner visitors into the current visitor's tree.
    for Visitor { tree, .. } in &visitor.inner_visitors {
        tree.merge_into(&mut visitor.tree);
    }
}
