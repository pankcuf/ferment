use std::fmt::{Debug, Display};
use std::hash::Hash;
use indexmap::IndexMap;
use indexmap::map::{Entry, OccupiedEntry};
use crate::kind::ObjectKind;
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

pub trait ValueReplaceScenario: Debug + Display {
    fn should_replace_with(&self, other: &Self) -> bool;
}

pub trait HashMapMergePolicy<K, V>
    where
        Self: Sized,
        K: Eq + Hash {
    fn insert_with_policy<P>(&mut self, key: K, value: V, policy: P) where P: MergePolicy<K, V>;
    fn extend_with_policy<M, P>(&mut self, other: M, policy: P) where M: IntoIterator<Item = (K, V)>, P: MergePolicy<K, V>;
}

impl<K, V> HashMapMergePolicy<K, V> for IndexMap<K, V>
    where K: Eq + Hash + Debug, V: Display + Debug {
    fn insert_with_policy<P>(&mut self, key: K, value: V, policy: P)
        where P: MergePolicy<K, V> + Clone {
        match self.entry(key) {
            Entry::Occupied(o) => {
                policy.apply(o, value)
            },
            Entry::Vacant(v) => {
                v.insert(value);
            },
        }
    }

    fn extend_with_policy<M, P>(&mut self, other: M, policy: P)
        where M: IntoIterator<Item = (K, V)>,
              P: MergePolicy<K, V> {
        for (key, value) in other {
            self.insert_with_policy(key, value, policy);
        }
    }
}

pub trait MergeInto {
    fn merge_into(&self, destination: &mut Self);
}

impl MergeInto for ScopeTreeExportItem {
    fn merge_into(&self, destination: &mut Self) {
        if let (ScopeTreeExportItem::Tree(_dest_ctx, _, ref mut dest_exports, _dest_attrs),
            ScopeTreeExportItem::Tree(_src_ctx, _, source_exports, _source_attrs), ) = (destination, &self) {
            for (name, source_tree) in source_exports {
                match dest_exports.entry(name.clone()) {
                    Entry::Occupied(mut o) => {
                        source_tree.merge_into(o.get_mut())
                    },
                    Entry::Vacant(v) => {
                        v.insert(source_tree.clone());
                    }
                }
            }
        }
    }
}

impl MergeInto for ObjectKind {
    fn merge_into(&self, destination: &mut Self) {
        match (&self, &destination) {
            (ObjectKind::Item(..), ObjectKind::Type(..)) => {
                *destination = self.clone();
            },
            (ObjectKind::Type(candidate_ty), ObjectKind::Type(occupied_ty)) => {
                if !occupied_ty.is_refined() && candidate_ty.is_refined() {
                    *destination = self.clone();
                }
            }
            _ => {}
        }
    }
}

