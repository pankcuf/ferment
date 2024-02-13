use std::collections::hash_map::OccupiedEntry;
use std::collections::HashMap;
use std::hash::Hash;
use crate::context::TypeChain;
use crate::conversion::ObjectConversion;
use crate::holder::TypeHolder;
use crate::tree::ScopeTreeExportItem;

pub trait MergePolicy<K, V>: Clone + Copy + Sized {
    fn apply(&self, o: OccupiedEntry<K, V>, object: V);
}

#[derive(Copy, Clone)]
pub struct DefaultMergePolicy;
impl<K, V> MergePolicy<K, V> for DefaultMergePolicy {
    fn apply(&self, mut o: OccupiedEntry<K, V>, object: V) {
        o.insert(object);
    }
}
// impl<K, V> MergePolicy<K, V> where V: ValueReplaceScenario, Self: Sized {
//     fn apply(&self, mut o: OccupiedEntry<K, V>, object: V) {
//         if o.get().should_replace_with(&object) {
//             o.insert(object);
//         }
//     }
// }

pub trait ValueReplaceScenario {
    fn should_replace_with(&self, other: &Self) -> bool;
}

pub trait HashMapMergePolicy<K, V>
    where
        Self: Sized,
        K: Eq + Hash {
    fn insert_with_policy<P>(&mut self, key: K, value: V, policy: P) where P: MergePolicy<K, V>;
    fn extend_with_policy<M, P>(&mut self, other: M, policy: P) where M: IntoIterator<Item = (K, V)>, P: MergePolicy<K, V>;
}

impl<K, V> HashMapMergePolicy<K, V> for HashMap<K, V> where K: Eq + Hash {
    fn insert_with_policy<P>(&mut self, key: K, value: V, policy: P) where P: MergePolicy<K, V> + Clone {
        match self.entry(key) {
            std::collections::hash_map::Entry::Occupied(o) => policy.apply(o, value),
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(value);
            },
        }
    }

    fn extend_with_policy<M, P>(&mut self, other: M, policy: P) where M: IntoIterator<Item = (K, V)>, P: MergePolicy<K, V> {
        for (holder, object) in other {
            self.insert_with_policy(holder, object, policy);
        }

    }
}

pub trait MergeInto {
    fn merge_into(&self, destination: &mut Self);
    // fn merge_into_with_policy<K, V>(&self, destination: &mut Self, policy: &impl MergePolicy<K, V>);
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

impl MergeInto for TypeChain {
    fn merge_into(&self, destination: &mut Self) {
        self.inner.merge_into(&mut destination.inner);
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

