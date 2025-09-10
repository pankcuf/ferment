use std::fmt::{Debug, Display};
use std::hash::Hash;
use indexmap::IndexMap;
use syn::{AngleBracketedGenericArguments, BareFnArg, Constraint, Expr, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use crate::context::EnrichScopePolicy;
use crate::ext::{HashMapMergePolicy, MaybeTraitBound, ValueReplaceScenario};

pub trait ScopeCollection<K, V>
where K: Eq + Hash,
      V: ValueReplaceScenario {
    fn scope_items(&self) -> IndexMap<K, V>;
}

impl<K, V, T, S> ScopeCollection<K, V> for Punctuated<T, S>
where T: ScopeCollection<K, V>,
      K: Eq + Hash + Display,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.iter().flat_map(T::scope_items).collect()
    }
}

impl<K, V> ScopeCollection<K, V> for AngleBracketedGenericArguments
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.args.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for BareFnArg
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.ty.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for Constraint
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.bounds.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for Expr
where K: Eq + Hash + Display,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        // TODO: Implement this if need
        Default::default()
    }
}

impl<K, V> ScopeCollection<K, V> for GenericArgument
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        match self {
            GenericArgument::Type(ty) => ty.scope_items(),
            GenericArgument::Constraint(constraint) => constraint.scope_items(),
            GenericArgument::Const(expr) => expr.scope_items(),
            _ => Default::default(),
        }
    }
}

impl<K, V> ScopeCollection<K, V> for ParenthesizedGenericArguments
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display + Debug {
    fn scope_items(&self) -> IndexMap<K, V> {
        let mut involved = IndexMap::default();
        involved.extend_with_policy(self.inputs.scope_items(), EnrichScopePolicy);
        involved.extend_with_policy(self.output.scope_items(), EnrichScopePolicy);
        involved
    }
}

impl<K, V> ScopeCollection<K, V> for Path
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display + Debug {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.segments.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for PathArguments
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display + Debug {
    fn scope_items(&self) -> IndexMap<K, V> {
        match self {
            PathArguments::AngleBracketed(args) => args.scope_items(),
            PathArguments::Parenthesized(args) => args.scope_items(),
            PathArguments::None => Default::default()
        }
    }
}

impl<K, V> ScopeCollection<K, V> for PathSegment
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.arguments.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for QSelf
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.ty.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for ReturnType
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        match self {
            ReturnType::Type(_, ty) => ty.scope_items(),
            ReturnType::Default => Default::default()
        }
    }
}
impl<K, V> ScopeCollection<K, V> for Type
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        let mut involved = IndexMap::default();
        match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                involved.extend_with_policy(elem.scope_items(), EnrichScopePolicy),
            Type::BareFn(type_bare_fn) =>
                involved.extend_with_policy(type_bare_fn.scope_items(), EnrichScopePolicy),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) =>
                involved.extend(bounds.scope_items()),
            Type::Path(TypePath { qself: Some(qself), path }) => {
                involved.extend_with_policy(path.scope_items(), EnrichScopePolicy);
                involved.extend_with_policy(qself.scope_items(), EnrichScopePolicy);
            },
            Type::Path(TypePath { path, .. }) =>
                involved.extend_with_policy(path.scope_items(), EnrichScopePolicy),
            Type::Tuple(TypeTuple { elems, .. }) =>
                involved.extend_with_policy(elems.scope_items(), EnrichScopePolicy),
            _ => {}
        }
        involved
    }
}

impl<K, V> ScopeCollection<K, V> for TypeBareFn
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        let mut involved = IndexMap::default();
        involved.extend_with_policy(self.inputs.scope_items(), EnrichScopePolicy);
        involved.extend_with_policy(self.output.scope_items(), EnrichScopePolicy);
        involved
    }
}

impl<K, V> ScopeCollection<K, V> for TypeParamBound
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.maybe_trait_bound()
            .map(TraitBound::scope_items)
            .unwrap_or_default()
    }
}

impl<K, V> ScopeCollection<K, V> for TraitBound
where K: Eq + Hash + Display + Debug,
      V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> IndexMap<K, V> {
        self.path.scope_items()
    }
}
