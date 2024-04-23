use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, Constraint, Expr, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use crate::context::EnrichScopePolicy;
use crate::ext::{HashMapMergePolicy, ValueReplaceScenario};

pub trait ScopeCollection<K, V> where K: Eq + Hash, V: ValueReplaceScenario {
    fn scope_items(&self) -> HashMap<K, V>;
}

impl<K, V, T, S> ScopeCollection<K, V> for Punctuated<T, S>
    where T: ScopeCollection<K, V>,
          K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.iter().flat_map(T::scope_items).collect()
    }
}

impl<K, V> ScopeCollection<K, V> for AngleBracketedGenericArguments
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.args.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for BareFnArg
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.ty.scope_items()
    }
}
impl<K, V> ScopeCollection<K, V> for Binding
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.ty.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for Constraint
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.bounds.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for Expr
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {

    fn scope_items(&self) -> HashMap<K, V> {
        // TODO: Implement this if need
        HashMap::default()
    }
}

impl<K, V> ScopeCollection<K, V> for GenericArgument
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        match self {
            GenericArgument::Type(ty) => ty.scope_items(),
            GenericArgument::Binding(binding) => binding.scope_items(),
            GenericArgument::Constraint(constraint) => constraint.scope_items(),
            GenericArgument::Const(expr) => expr.scope_items(),
            GenericArgument::Lifetime(_) => HashMap::default(),
        }
    }
}

impl<K, V> ScopeCollection<K, V> for ParenthesizedGenericArguments
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        let mut involved = HashMap::default();
        involved.extend_with_policy(self.inputs.scope_items(), EnrichScopePolicy);
        involved.extend_with_policy(self.output.scope_items(), EnrichScopePolicy);
        involved
    }
}

impl<K, V> ScopeCollection<K, V> for Path
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.segments.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for PathArguments
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        match self {
            PathArguments::AngleBracketed(args) => args.scope_items(),
            PathArguments::Parenthesized(args) => args.scope_items(),
            PathArguments::None => HashMap::default()
        }
    }
}

impl<K, V> ScopeCollection<K, V> for PathSegment
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.arguments.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for QSelf
    where K: Eq + Hash + Display , V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        self.ty.scope_items()
    }
}

impl<K, V> ScopeCollection<K, V> for ReturnType
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        match self {
            ReturnType::Type(_, ty) => ty.scope_items(),
            ReturnType::Default => HashMap::default()
        }
    }
}
impl<K, V> ScopeCollection<K, V> for Type
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        let mut involved = HashMap::default();
        match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                involved.extend_with_policy(elem.scope_items(), EnrichScopePolicy),
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                involved.extend_with_policy(inputs.scope_items(), EnrichScopePolicy);
                involved.extend_with_policy(output.scope_items(), EnrichScopePolicy);
            },
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                involved.extend(bounds.scope_items());
            },
            Type::Path(TypePath { qself, path }) => {
                involved.extend_with_policy(path.scope_items(), EnrichScopePolicy);
                if let Some(qself) = qself {
                    involved.extend_with_policy(qself.scope_items(), EnrichScopePolicy);
                }
            },
            Type::Tuple(TypeTuple { elems, .. }) =>
                involved.extend_with_policy(elems.scope_items(), EnrichScopePolicy),
            _ => {}
        }
        involved
    }
}

impl<K, V> ScopeCollection<K, V> for TypeParamBound
    where K: Eq + Hash + Display, V: ValueReplaceScenario + Display {
    fn scope_items(&self) -> HashMap<K, V> {
        match self {
            TypeParamBound::Trait(TraitBound { path, .. }) => path.scope_items(),
            TypeParamBound::Lifetime(_) => HashMap::default()
        }
    }
}
