use std::fmt::{Debug, Display, Formatter};
use indexmap::IndexMap;
use indexmap::map::OccupiedEntry;
use quote::ToTokens;
use syn::{parse_quote, Generics, Path, Type, TypePath, Path as SynPath};
use crate::kind::ObjectKind;
use crate::ext::{AsType, Constraints, ContainsSubType, HashMapMergePolicy, MergePolicy, ValueReplaceScenario};
use crate::formatter::format_types_dict;

#[derive(Copy, Clone)]
pub struct EnrichScopePolicy;

impl<K, V> MergePolicy<K, V> for EnrichScopePolicy
where V: ValueReplaceScenario + Debug + Display {
    fn apply(&self, mut o: OccupiedEntry<K, V>, object: V) {
        if o.get().should_replace_with(&object) {
            o.insert(object);
        }
    }
}
#[derive(Clone, Default)]
pub struct TypeChain {
    pub inner: IndexMap<Type, ObjectKind>
}

impl<T: Iterator<Item = (Type, ObjectKind)>> From<T> for TypeChain {
    fn from(value: T) -> Self {
        Self { inner: IndexMap::from_iter(value) }
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

    pub fn add_self(&mut self, obj: ObjectKind) {
        self.insert(parse_quote!(Self), obj);
    }
    pub fn insert(&mut self, ty: Type, obj: ObjectKind) {
        self.inner.insert(ty, obj);
    }
    pub fn get(&self, ty: &Type) -> Option<&ObjectKind> {
        self.inner.get(ty)
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
        Self::from(self.inner.clone().into_iter().filter(|(th, _)| th.has_no_self()))
    }
    pub fn excluding_self_and_bounds(&self, generics: &Generics) -> Self {
        // Keep only entries that do NOT reference method-level generic params
        // and do not involve `Self`.
        Self::from(self.inner.clone().into_iter().filter(|(th, _)| th.has_no_self() && !generics.contains_sub_type(th)))
    }

    pub fn only_self_associated(&self) -> Self {
        // Keep entries that involve `Self` but are not the plain `Self` type itself,
        // e.g., `Self::Item`, `<Self::Item as Trait>::Assoc`.
        fn is_plain_self(ty: &Type) -> bool {
            match ty {
                Type::Path(TypePath { qself: None, path: SynPath { leading_colon: None, segments }, .. })
                    if segments.len() == 1 => segments.first().map(|s| s.ident == "Self").unwrap_or_default(),
                _ => false,
            }
        }
        Self::from(self.inner.clone().into_iter().filter(|(th, _)| th.has_self() && !is_plain_self(th)))
    }
    pub fn get_by_path(&self, path: &Path) -> Option<Type> {
        self.inner.iter()
            .find_map(|(other, full_type)| if path.to_token_stream().to_string().eq(other.to_token_stream().to_string().as_str()) {
                full_type.maybe_type()
            } else {
                None
            })
    }
    pub fn add_one(&mut self, holder: Type, object: ObjectKind) {
        self.inner.insert_with_policy(holder, object, EnrichScopePolicy);
    }
    pub fn add_many<I>(&mut self, types: I) where I: Iterator<Item = (Type, ObjectKind)> {
        self.inner.extend_with_policy(types, EnrichScopePolicy);
    }
}
