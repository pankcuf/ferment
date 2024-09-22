use std::collections::hash_map::OccupiedEntry;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Generics, Path, Type};
use crate::ast::TypeHolder;
use crate::conversion::ObjectKind;
use crate::ext::{AsType, Constraints, ContainsSubType, HashMapMergePolicy, MergePolicy, ValueReplaceScenario};
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

#[derive(Clone, Default)]
pub struct TypeChain {
    pub inner: HashMap<TypeHolder, ObjectKind>
}

impl<T: Iterator<Item = (TypeHolder, ObjectKind)>> From<T> for TypeChain {
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

impl TypeChain {
    pub fn insert(&mut self, ty: TypeHolder, obj: ObjectKind) {
        self.inner.insert(ty, obj);
    }
    pub fn get(&self, ty: &TypeHolder) -> Option<&ObjectKind> {
        self.inner.get(ty)
    }
    pub fn get_by_key(&self, ty: &Type) -> Option<&ObjectKind> {
        let holder = TypeHolder::from(ty);
        self.get(&holder)
    }
    pub fn get_by_value(&self, ty: &Type) -> Option<&ObjectKind> {
        self.inner.values()
            .find(|obj| match obj {
                ObjectKind::Type(model) |
                ObjectKind::Item(model, ..) => model.as_type().eq(ty),
                ObjectKind::Empty => false
            })
    }
    pub fn selfless(&self) -> Self {
        Self::from(self.inner.clone().into_iter().filter(|(th, _)| th.0.has_no_self()))
    }
    pub fn excluding_self_and_bounds(&self, generics: &Generics) -> Self {
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
    pub fn add_one(&mut self, holder: TypeHolder, object: ObjectKind) {
        self.inner.insert_with_policy(holder, object, EnrichScopePolicy);
    }
    pub fn add_many<I>(&mut self, types: I) where I: Iterator<Item = (TypeHolder, ObjectKind)> {
        self.inner.extend_with_policy(types, EnrichScopePolicy);
    }
}

