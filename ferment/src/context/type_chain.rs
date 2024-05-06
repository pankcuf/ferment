use std::collections::hash_map::OccupiedEntry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Path, Type};
use crate::composition::GenericConversion;
use crate::conversion::ObjectConversion;
use crate::ext::{Constraints, visitor::GenericCollector, HashMapMergePolicy, MergePolicy, ValueReplaceScenario};
use crate::formatter::format_types_dict;
use crate::holder::{Holder, TypeHolder};

#[derive(Copy, Clone)]
pub struct DefaultScopePolicy;
#[derive(Copy, Clone)]
pub struct EnrichScopePolicy;
#[derive(Copy, Clone)]
pub struct ExternalModulePolicy;

impl<K, V> MergePolicy<K, V> for DefaultScopePolicy where K: Display, V: Display {
    fn apply(&self, mut o: OccupiedEntry<K, V>, object: V) {
        // println!("DefaultScopePolicy::apply: {} --> {}", o.get(), object);
        o.insert(object);
    }
}

impl<K, V> MergePolicy<K, V> for EnrichScopePolicy where V: ValueReplaceScenario + Debug + Display {
    fn apply(&self, mut o: OccupiedEntry<K, V>, object: V) {
        let should_upgrade = o.get().should_replace_with(&object);
        // println!("EnrichScopePolicy::apply: {}:: {} --> {}", should_upgrade, o.get(), object);
        if should_upgrade {
            o.insert(object);
        }
    }

}
impl<K, V> MergePolicy<K, V> for ExternalModulePolicy where V: ValueReplaceScenario + Debug + Display {
    fn apply(&self, mut o: OccupiedEntry<K, V>, object: V) {
        let should_upgrade = o.get().should_replace_with(&object);
        // println!("EnrichScopePolicy::apply: {}:: {} --> {}", should_upgrade, o.get(), object);
        if should_upgrade {
            o.insert(object);
        }
    }

}

#[derive(Clone, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum TypeChainKey {
    Object(TypeHolder),
    Constrant(TypeHolder)
}

#[allow(unused)]
impl TypeChainKey {
    pub fn ty(&self) -> &Type {
        match self {
            TypeChainKey::Object(ty) => ty.inner(),
            TypeChainKey::Constrant(ty) => ty.inner()
        }
    }
}

// impl Constraints for TypeChainKey {
//     fn has_self(&self) -> bool {
//         match self {
//             TypeChainKey::Object(holder) => holder.has_self(),
//             TypeChainKey::Constrant(holder) => holder.has_self()
//         }
//     }
// }

#[derive(Clone, Default)]
pub struct TypeChain {
    pub inner: HashMap<TypeHolder, ObjectConversion>
}

impl Debug for TypeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format_types_dict(&self.inner).as_str())
    }
}


impl Display for TypeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<HashMap<TypeHolder, ObjectConversion>> for TypeChain {
    fn from(inner: HashMap<TypeHolder, ObjectConversion>) -> Self {
        TypeChain { inner }
    }
}

impl TypeChain {
    pub fn insert(&mut self, ty: TypeHolder, obj: ObjectConversion) {
        self.inner.insert(ty, obj);
    }
    pub fn get(&self, ty: &TypeHolder) -> Option<&ObjectConversion> {
        self.inner.get(ty)
    }
    pub fn find(&self, holder: &TypeHolder) -> Option<&ObjectConversion> {
        self.inner.values()
            .find(|obj| match obj {
                ObjectConversion::Type(ty) |
                ObjectConversion::Item(ty, ..) => ty.to_ty().eq(&holder.0),
                ObjectConversion::Empty => false
            })
    }
    pub fn selfless(&self) -> Self {
        let mut inner = HashMap::new();
        for (ty, obj) in &self.inner {
            inner.insert(ty.clone(), obj.clone());
        }
        Self { inner: self.inner.clone().into_iter().filter(|(th, _)| th.0.has_no_self()).collect() }
    }
    pub fn get_by_path(&self, path: &Path) -> Option<Type> {
        self.inner.iter()
            .find_map(|(TypeHolder { 0: other}, full_type)| {
                if path.to_token_stream().to_string().eq(other.to_token_stream().to_string().as_str()) {
                    full_type.to_ty()
                } else {
                    None
                }
            })
    }
    pub fn add_one(&mut self, holder: TypeHolder, object: ObjectConversion) {
        self.inner.insert_with_policy(holder, object, EnrichScopePolicy);
    }
    pub fn add_many(&mut self, types: TypeChain) {
        self.inner.extend_with_policy(types.inner, EnrichScopePolicy);
    }

    pub fn find_generics_fq<G: GenericCollector>(&self, item: &G) -> HashSet<GenericConversion> {
        item.find_generics()
            .iter()
            .filter_map(|ty| self.get(ty))
            .map(GenericConversion::from)
            .collect()
    }

}

