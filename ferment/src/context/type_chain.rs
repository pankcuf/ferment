use std::collections::hash_map::OccupiedEntry;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Generics, Path, Type};
use crate::ast::{Holder, TypeHolder};
use crate::conversion::ObjectConversion;
use crate::ext::{Constraints, ContainsSubType, HashMapMergePolicy, MergePolicy, ToType, ValueReplaceScenario};
use crate::formatter::format_types_dict;

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

impl<T: Iterator<Item = (TypeHolder, ObjectConversion)>> From<T> for TypeChain {
    fn from(value: T) -> Self {
        Self { inner: HashMap::from_iter(value) }
    }
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

// impl From<HashMap<TypeHolder, ObjectConversion>> for TypeChain {
//     fn from(inner: HashMap<TypeHolder, ObjectConversion>) -> Self {
//         TypeChain { inner }
//     }
// }

impl TypeChain {
    pub fn insert(&mut self, ty: TypeHolder, obj: ObjectConversion) {
        self.inner.insert(ty, obj);
    }
    pub fn get(&self, ty: &TypeHolder) -> Option<&ObjectConversion> {
        let result = self.inner.get(ty);
        // println!("TypeChain::get({}) --> {}", ty.to_token_stream(), result.to_token_stream());
        result
    }
    pub fn find(&self, holder: &TypeHolder) -> Option<&ObjectConversion> {
        self.inner.values()
            .find(|obj| match obj {
                ObjectConversion::Type(ty) |
                ObjectConversion::Item(ty, ..) => ty.to_type().eq(&holder.0),
                ObjectConversion::Empty => false
            })
    }
    pub fn selfless(&self) -> Self {
        // let mut inner = HashMap::new();
        // for (ty, obj) in &self.inner {
        //     inner.insert(ty.clone(), obj.clone());
        // }
        // self.inner.iter().cloned()
        // Self::from(self.inner.iter().filter(|(th, _)| th.0.has_no_self()).cloned())
        Self::from(self.inner.clone().into_iter().filter(|(th, _)| th.0.has_no_self()))
    }
    pub fn excluding_self_and_bounds(&self, generics: &Generics) -> Self {
        // let mut inner = HashMap::new();
        // for (ty, obj) in &self.inner {
        //     inner.insert(ty.clone(), obj.clone());
        // }

        Self::from(self.inner.clone().into_iter().filter(|(th, _)| th.0.has_no_self() && generics.contains_sub_type(&th.0)))
    }
    pub fn get_by_path(&self, path: &Path) -> Option<Type> {
        self.inner.iter()
            .find_map(|(TypeHolder { 0: other}, full_type)| {
                if path.to_token_stream().to_string().eq(other.to_token_stream().to_string().as_str()) {
                    full_type.maybe_type()
                } else {
                    None
                }
            })
    }
    pub fn add_one(&mut self, holder: TypeHolder, object: ObjectConversion) {
        self.inner.insert_with_policy(holder, object, EnrichScopePolicy);
    }
    pub fn add_many<I>(&mut self, types: I) where I: Iterator<Item = (TypeHolder, ObjectConversion)> {
        self.inner.extend_with_policy(types, EnrichScopePolicy);
    }

    // pub fn find_generics_fq<G: GenericCollector>(&self, item: &G) -> HashSet<GenericConversion> {
    //     item.find_generics()
    //         .iter()
    //         .filter_map(|ty| self.get(ty))
    //         .map(|object| GenericConversion::new(object.clone(), object.resolve_attrs()))
    //         .collect()
    // }

}

