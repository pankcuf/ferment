use std::collections::hash_map::OccupiedEntry;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use crate::context::TypeChain;
use crate::conversion::ObjectConversion;
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

impl<K, V> HashMapMergePolicy<K, V> for HashMap<K, V> where K: Eq + Hash + Display, V: Display {
    fn insert_with_policy<P>(&mut self, key: K, value: V, policy: P)
        where P: MergePolicy<K, V> + Clone {
        match self.entry(key) {
            std::collections::hash_map::Entry::Occupied(o) => {
                // println!("insert_with_policy: (Occupied) holder: {}", value);
                policy.apply(o, value)
            },
            std::collections::hash_map::Entry::Vacant(v) => {
                // println!("insert_with_policy: (Vacant) holder: {}", value);
                v.insert(value);
            },
        }
    }

    fn extend_with_policy<M, P>(&mut self, other: M, policy: P)
        where M: IntoIterator<Item = (K, V)>,
              P: MergePolicy<K, V> {
        for (key, value) in other {
            // println!("extend_with_policy: holder: {}: {}", key, value);
            self.insert_with_policy(key, value, policy);
        }

    }
}

pub trait MergeInto {
    fn merge_into(&self, destination: &mut Self);
    // fn merge_into_with_policy<K, V>(&self, destination: &mut Self, policy: &impl MergePolicy<K, V>);
}

impl ValueReplaceScenario for ScopeTreeExportItem {
    fn should_replace_with(&self, other: &Self) -> bool {
        println!("ScopeTreeExportItem ::: should_replace_with:::: {}: {}", self, other);
        match (self, other) {
            // (ObjectConversion::Type(..), ObjectConversion::Item(..)) => true,
            (ScopeTreeExportItem::Tree(..), ScopeTreeExportItem::Tree(..)) => true,
            _ => false
        }
    }
}


impl MergeInto for ScopeTreeExportItem {
    fn merge_into(&self, destination: &mut Self) {
        if let (ScopeTreeExportItem::Tree(_dest_ctx, _, ref mut dest_exports, _dest_attrs),
            ScopeTreeExportItem::Tree(_src_ctx, _, source_exports, _source_attrs), ) = (destination, &self) {
            // println!("•• merge_trees: source: {}: {}", src_ctx.borrow().scope.self_path_holder_ref(), source_attrs.iter().map(|a| a.to_token_stream()).collect::<Depunctuated<TokenStream>>().to_token_stream());
            // println!("•• merge_trees: destination: {}: {}", dest_ctx.borrow().scope.self_path_holder_ref(), dest_attrs.iter().map(|a| a.to_token_stream()).collect::<Depunctuated<TokenStream>>().to_token_stream());
            for (name, source_tree) in source_exports {
                match dest_exports.entry(name.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        source_tree.merge_into(o.get_mut())
                    },
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(source_tree.clone());
                    }
                }
            }
        }
    }
}



impl<T> MergeInto for HashMap<T, ObjectConversion> where T: Hash + Eq + Clone + Display {
    fn merge_into(&self, destination: &mut Self) {
        for (holder, object) in self {
            match destination.entry(holder.clone()) {
                std::collections::hash_map::Entry::Occupied(mut o) => match (o.get_mut(), &object) {
                    (ObjectConversion::Type(..), ObjectConversion::Item(..)) => {
                        o.insert(object.clone());
                    },
                    (ObjectConversion::Type(occupied_ty), ObjectConversion::Type(candidate_ty)) => {
                        if !occupied_ty.is_refined() && candidate_ty.is_refined() {
                            o.insert(object.clone());
                        }
                    }
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
            (ObjectConversion::Type(candidate_ty), ObjectConversion::Type(occupied_ty)) => {
                if !occupied_ty.is_refined() && candidate_ty.is_refined() {
                    *destination = self.clone()
                }
            }
            _ => {}
        }
    }
}

// impl MergeInto for Path {
//     fn merge_into(&self, destination: &mut Self) {
//         let full_segments = destination.segments.clone();
//         let alias_segments = &self.segments;
//         full_segments.iter().enumerate().for_each(|(index, segment)| {
//             if let Some(alias_segment) = alias_segments.first() {
//                 if segment.ident == alias_segment.ident {
//                     let mut new_segments = Punctuated::new();
//                     new_segments.extend(full_segments.iter().take(index + 1).cloned());
//                     if alias_segments.len() > 1 {
//                         new_segments.extend(alias_segments.iter().skip(1).cloned());
//                     }
//                     println!("MERGED: {}", new_segments.to_token_stream());
//                     destination.segments = new_segments;
//                 }
//             }
//         })
//     }
// }
