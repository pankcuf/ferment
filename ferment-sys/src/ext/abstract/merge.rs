use std::collections::hash_map::{Entry, OccupiedEntry};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use crate::conversion::ObjectKind;
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

impl<K, V> HashMapMergePolicy<K, V> for HashMap<K, V>
    where K: Eq + Hash + Display + Debug, V: Display + Debug {
    fn insert_with_policy<P>(&mut self, key: K, value: V, policy: P)
        where P: MergePolicy<K, V> + Clone {
        // let key_str = key.to_string();
        match self.entry(key) {
            Entry::Occupied(o) => {
                // println!("insert_with_policy: (Occupied) holder: {} -- {}", key_str, value);
                policy.apply(o, value)
            },
            Entry::Vacant(v) => {
                // println!("insert_with_policy: (Vacant) holder: {} -- {}", key_str, value);
                v.insert(value);
            },
        }
    }

    fn extend_with_policy<M, P>(&mut self, other: M, policy: P)
        where M: IntoIterator<Item = (K, V)>,
              P: MergePolicy<K, V> {

        // println!("extend_with_policy: {:?}", self);
        for (key, value) in other {
            // println!("extend_with_policy: holder: {}: {}", key, value);
            self.insert_with_policy(key, value, policy);
        }
        // println!("extend_with_policy (RESULT): {:?}", self);
    }
}

pub trait MergeInto {
    fn merge_into(&self, destination: &mut Self);
    // fn merge_into_with_policy<K, V>(&self, destination: &mut Self, policy: &impl MergePolicy<K, V>);
}

// impl ValueReplaceScenario for ScopeTreeExportItem {
//     fn should_replace_with(&self, other: &Self) -> bool {
//         println!("ScopeTreeExportItem ::: should_replace_with:::: {}: {}", self, other);
//         match (self, other) {
//             // (ObjectKind::Type(..), ObjectKind::Item(..)) => true,
//             (ScopeTreeExportItem::Tree(..), ScopeTreeExportItem::Tree(..)) => true,
//             _ => false
//         }
//     }
// }


impl MergeInto for ScopeTreeExportItem {
    fn merge_into(&self, destination: &mut Self) {
        if let (ScopeTreeExportItem::Tree(_dest_ctx, _, ref mut dest_exports, _dest_attrs),
            ScopeTreeExportItem::Tree(_src_ctx, _, source_exports, _source_attrs), ) = (destination, &self) {
            // println!("•• merge_trees: source: {}: {}", _src_ctx.borrow().scope.self_path_holder_ref(), _source_attrs.iter().map(|a| a.to_token_stream()).collect::<Depunctuated<_>>().to_token_stream());
            // println!("•• merge_trees: destination: {}: {}", _dest_ctx.borrow().scope.self_path_holder_ref(), _dest_attrs.iter().map(|a| a.to_token_stream()).collect::<Depunctuated<_>>().to_token_stream());
            for (name, source_tree) in source_exports {
                match dest_exports.entry(name.clone()) {
                    Entry::Occupied(mut o) => {
                        println!("•• (Occupied) merge_trees: {}", name);
                        source_tree.merge_into(o.get_mut())
                    },
                    Entry::Vacant(v) => {
                        println!("•• (Vacant) merge_trees: {}", name);
                        v.insert(source_tree.clone());
                    }
                }
            }
        }
    }
}



// impl<T> MergeInto for HashMap<T, ObjectKind>
//     where T: Hash + Eq + Clone + Debug + Display {
//     fn merge_into(&self, destination: &mut Self) {
//         for (holder, object) in self {
//             println!("DEST: {}: {:?}", holder.to_string(), destination.entry(holder.clone()));
//             match destination.entry(holder.clone()) {
//                 Entry::Occupied(mut o) => match (o.get_mut(), &object) {
//                     (ObjectKind::Type(..), ObjectKind::Item(..)) => {
//                         o.insert(object.clone());
//                     },
//                     (ObjectKind::Type(occupied_ty), ObjectKind::Type(candidate_ty)) if !occupied_ty.is_refined() && candidate_ty.is_refined() => {
//                         o.insert(object.clone());
//                     }
//                     _ => {}
//                 },
//                 Entry::Vacant(v) => {
//                     v.insert(object.clone());
//                 }
//             }
//         }
//     }
// }

// impl MergeInto for TypeChain {
//     fn merge_into(&self, destination: &mut Self) {
//         self.inner.merge_into(&mut destination.inner);
//     }
// }

impl MergeInto for ObjectKind {
    fn merge_into(&self, destination: &mut Self) {
        let mut merged = false;
        match (&self, &destination) {
            (ObjectKind::Item(..), ObjectKind::Type(..)) => {
                *destination = self.clone();
                merged = true;
            },
            (ObjectKind::Type(candidate_ty), ObjectKind::Type(occupied_ty)) => {
                //println!("MMMMM: {} (refined: {}) ---> {} (refined: {})", occupied_ty, occupied_ty.is_refined(), candidate_ty, candidate_ty.is_refined());
                if !occupied_ty.is_refined() && candidate_ty.is_refined() {
                    *destination = self.clone();
                    merged = true;
                }
            }
            _ => {}
        }
        println!("ObjectKind: MERGED? ({merged}) {} ~~~ {}", self, destination)
    }
}

