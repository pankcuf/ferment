use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, TraitBound, Type, TypeParamBound, TypePtr, TypeReference, TypeTraitObject};
use crate::ast::TypeHolder;
use crate::context::{ScopeChain, TypeChain};
use crate::conversion::ObjectKind;
use crate::ext::{LifetimeProcessor, RefineMut, ToType};
use crate::formatter::types_dict;
use crate::lang::{LangFermentable, Specification};
use crate::presentation::FFIVariable;

pub type ScopeRefinement = Vec<(ScopeChain, HashMap<TypeHolder, ObjectKind>)>;

#[derive(Clone, Debug)]
pub enum ScopeSearchKey<'a> {
    PathRef(&'a Path, Option<Box<ScopeSearchKey<'a>>>),
    TypeRef(&'a Type, Option<Box<ScopeSearchKey<'a>>>),
    Type(Type, Option<Box<ScopeSearchKey<'a>>>)
}
impl<'a> Display for ScopeSearchKey<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::PathRef(path, original) => format!("PathRef({}, {})", path.to_token_stream(), original.as_ref().map(|d| d.to_string()).unwrap_or_default()),
            Self::TypeRef(ty, original) => format!("TypeRef({}, {})", ty.to_token_stream(), original.as_ref().map(|d| d.to_string()).unwrap_or_default()),
            Self::Type(ty, original) => format!("Type({}, {})", ty.to_token_stream(), original.as_ref().map(|d| d.to_string()).unwrap_or_default()),
        }.as_str())
    }
}

impl<'a> ToTokens for ScopeSearchKey<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::PathRef(path, ..) => path.to_tokens(tokens),
            Self::TypeRef(ty, ..) => ty.to_tokens(tokens),
            Self::Type(ty, ..) => ty.to_tokens(tokens)
        }
    }
}

impl<'a> ToType for ScopeSearchKey<'a> {
    fn to_type(&self) -> Type {
       match self {
           ScopeSearchKey::PathRef(path, ..) => path.to_type(),
           ScopeSearchKey::TypeRef(ty, ..) => ty.to_type(),
           ScopeSearchKey::Type(ty, ..) => ty.clone()
       }
    }
}

impl<'a> ScopeSearchKey<'a> {
    pub fn ptr_composer<LANG, SPEC, T>(&self) -> fn(T) -> FFIVariable<LANG, SPEC, T>
        where LANG: LangFermentable,
              SPEC: Specification<LANG>,
              T: ToTokens {
        if self.maybe_originally_is_const_ptr() {
            FFIVariable::const_ptr
        } else {
            FFIVariable::mut_ptr
        }
    }
}


impl<'a> ScopeSearchKey<'a> {
    pub fn maybe_original_key(&'a self) -> &'a Option<Box<ScopeSearchKey<'a>>> {
        match self {
            ScopeSearchKey::PathRef(_, original) |
            ScopeSearchKey::TypeRef(_, original) |
            ScopeSearchKey::Type(_, original) => original
        }
    }
    pub fn maybe_originally_is_ptr(&'a self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_const_ptr() || boxed_key.is_mut_ptr(),
            _ => false
        }
    }
    pub fn maybe_originally_is_const_ptr(&'a self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_const_ptr(),
            _ => false
        }
    }
    pub fn maybe_originally_is_mut_ptr(&'a self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_mut_ptr(),
            _ => false
        }
    }
    pub fn maybe_originally_is_ref(&'a self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_ref(),
            _ => false
        }
    }
    pub fn maybe_originally_is_mut_ref(&'a self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_mut_ref(),
            _ => false
        }
    }
    pub fn maybe_originally_is_dyn(&'a self) -> bool {
        match self.maybe_original_key() {
            Some(boxed_key) => boxed_key.is_dyn(),
            _ => false
        }
    }
    pub fn maybe_ptr(&'a self) -> Option<&'a TypePtr> {
        match self {
            ScopeSearchKey::TypeRef(Type::Ptr(type_ptr), ..) |
            ScopeSearchKey::Type(Type::Ptr(type_ptr), ..) => Some(type_ptr),
            _ => None
        }
    }
    pub fn maybe_dyn(&'a self) -> Option<&'a TypeTraitObject> {
        match self {
            ScopeSearchKey::TypeRef(Type::TraitObject(ty), ..) |
            ScopeSearchKey::Type(Type::TraitObject(ty), ..) => Some(ty),
            _ => None
        }
    }
    pub fn maybe_ref(&'a self) -> Option<&'a TypeReference> {
        match self {
            ScopeSearchKey::TypeRef(Type::Reference(type_ref), ..) |
            ScopeSearchKey::Type(Type::Reference(type_ref), ..) => Some(type_ref),
            _ => None
        }
    }
    pub fn is_const_ptr(&self) -> bool {
        self.maybe_ptr().is_some_and(|ty| ty.const_token.is_some())
    }
    pub fn is_mut_ptr(&self) -> bool {
        self.maybe_ptr().is_some_and(|ty| ty.mutability.is_some())
    }
    pub fn is_ref(&self) -> bool {
        self.maybe_ref().is_some()
    }
    pub fn is_mut_ref(&self) -> bool {
        match self {
            ScopeSearchKey::TypeRef(Type::Reference(TypeReference { mutability: Some(_mutable), .. }), ..) |
            ScopeSearchKey::Type(Type::Reference(TypeReference { mutability: Some(_mutable), .. }), ..) => true,
            _ => false
        }
    }
    pub fn is_dyn(&self) -> bool {
        match self {
            ScopeSearchKey::TypeRef(Type::TraitObject(TypeTraitObject { .. }), ..) |
            ScopeSearchKey::Type(Type::TraitObject(TypeTraitObject { .. }), ..) => true,
            _ => false
        }
    }
}

#[derive(Clone, Debug)]
pub enum ScopeSearch<'a> {
    KeyInScope(ScopeSearchKey<'a>, &'a ScopeChain),
    // Key(ScopeSearchKey<'a>),
    Value(ScopeSearchKey<'a>),
    ValueInScope(ScopeSearchKey<'a>, &'a ScopeChain),
}
impl<'a> Display for ScopeSearch<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::KeyInScope(key, scope) => format!("KeyInScope({} in {})", key, scope.fmt_short()),
            Self::Value(key) => format!("Value({})", key),
            Self::ValueInScope(key, scope) => format!("ValueInScope({} in {})", key, scope.fmt_short()),
        }.as_str())
    }
}

impl<'a> ScopeSearch<'a> {
    pub fn search_key(&'a self) -> &'a ScopeSearchKey<'a> {
        match self {
            ScopeSearch::KeyInScope(search_key, _) |
            ScopeSearch::Value(search_key) |
            ScopeSearch::ValueInScope(search_key, _) => search_key,
        }
    }
}
impl<'a> ScopeSearchKey<'a> {
    pub fn maybe_from_ref(ty: &'a Type) -> Option<Self> {
        let original = ScopeSearchKey::TypeRef(ty, None);
        match ty {
            Type::TraitObject(TypeTraitObject { bounds , ..}) => match bounds.first()? {
                TypeParamBound::Trait(TraitBound { path, .. }) =>
                    Some(ScopeSearchKey::PathRef(path, Some(Box::new(original)))),
                TypeParamBound::Lifetime(_) =>
                    panic!("maybe_scope_type::error")
            },
            Type::Reference(TypeReference { elem: ty, .. }) |
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                Some(ScopeSearchKey::TypeRef(ty, Some(Box::new(original)))),
            ty =>
                Some(ScopeSearchKey::TypeRef(ty, Some(Box::new(original)))),
        }

    }
    pub fn maybe_from(ty: Type) -> Option<Self> {
        let original = ScopeSearchKey::Type(ty.clone(), None);
        match ty {
            Type::TraitObject(TypeTraitObject { bounds , ..}) => match bounds.len() {
                1 => match bounds.first()? {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        Some(ScopeSearchKey::Type(path.to_type(), Some(Box::new(original)))),
                    TypeParamBound::Lifetime(_) =>
                        panic!("maybe_scope_type::error")
                },
                _ => None
            },
            Type::Reference(TypeReference { elem: ty, .. }) |
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                Some(ScopeSearchKey::Type(*ty, Some(Box::new(original)))),
            ty =>
                Some(ScopeSearchKey::Type(ty, Some(Box::new(original)))),
        }
    }
    pub fn find<K, T: Fn(&Type) -> K>(&self, finder: T) -> K {
        match self {
            ScopeSearchKey::PathRef(path, ..) => finder(&path.to_type()),
            ScopeSearchKey::TypeRef(ty, ..) => finder(&ty),
            ScopeSearchKey::Type(ty, ..) => finder(ty),
        }
    }
}


#[derive(Clone, Default)]
pub struct ScopeResolver {
    pub inner: HashMap<ScopeChain, TypeChain>,
}

impl Debug for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.inner.iter()
            .map(|(key, value)| format!("\t{}:\n\t{}", key.fmt_short(), types_dict(&value.inner).join("\n\t")))
            .collect::<Vec<String>>();
        iter.sort();
        f.write_str( iter.join("\n\n").as_str())
    }
}

impl Display for ScopeResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ScopeResolver {

    pub(crate) fn maybe_scope(&self, path: &Path) -> Option<&ScopeChain> {
        self.inner.keys()
            .find_map(|scope_chain| path.eq(scope_chain.self_path()).then_some(scope_chain))
    }
    pub(crate) fn maybe_first_obj_scope(&self, path: &Path) -> Option<&ScopeChain> {
        let mut scopes = self.inner.keys()
            .filter(|scope_chain| path.eq(scope_chain.self_path()))
            .collect::<Vec<_>>();
        scopes.sort_by(|c1, c2| {
            if c1.obj_scope_priority() == c2.obj_scope_priority() {
                Ordering::Equal
            } else if c1.obj_scope_priority() < c2.obj_scope_priority() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        scopes.first().cloned()
    }
    pub fn type_chain_mut(&mut self, scope: &ScopeChain) -> &mut TypeChain {
        let maybe_entry = self.inner.entry(scope.clone());
        maybe_entry.or_default()
    }
    pub fn maybe_object_ref_by_predicate<'a>(&'a self, predicate: ScopeSearch<'a>) -> Option<&'a ObjectKind> {
        let result = match predicate {
            ScopeSearch::KeyInScope(search_key, scope) => {
                let result = self.get(scope)
                    .and_then(|chain|
                        search_key.find(|ty|
                            chain.get_by_key(ty)
                                .or_else(|| chain.get_by_key(&ty.lifetimes_cleaned()))));
                result
            },
            ScopeSearch::ValueInScope(search_key, scope) => {
                let result = self.get(scope)
                    .and_then(|chain|
                        search_key.find(|ty|
                            chain.get_by_value(ty)
                                .or_else(|| chain.get_by_value(&ty.lifetimes_cleaned()))));
                result
            },
            ScopeSearch::Value(search_key) => {
                let result = self.inner.values()
                    .find_map(|chain|
                        search_key.find(|ty|
                            chain.get_by_value(ty)
                                .or_else(|| chain.get_by_value(&ty.lifetimes_cleaned()))));
                result
            },
        };
        result
    }

    pub fn get(&self, scope: &ScopeChain) -> Option<&TypeChain> {
        self.inner.get(scope)
    }
    pub fn scope_key_type_for_path(&self, path: &Path, scope: &ScopeChain) -> Option<Type> {
        self.get(scope)
            .and_then(|chain| chain.get_by_path(path))
    }
}

impl RefineMut for ScopeResolver {
    type Refinement = ScopeRefinement;

    fn refine_with(&mut self, refined: Self::Refinement) {
        refined.into_iter()
            .for_each(|(scope, updates)|
                self.type_chain_mut(&scope)
                    .add_many(updates.into_iter())
            );
    }
}