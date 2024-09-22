use std::collections::HashSet;
use std::hash::Hash;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, Constraint, Expr, GenericArgument, ParenthesizedGenericArguments, parse_quote, Path, PathArguments, PathSegment, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;

pub trait UniqueNestedItems {
    type Item: ToTokens + Eq + Hash;
    fn unique_nested_items(&self) -> HashSet<Self::Item>;
}

impl<A, T, P> UniqueNestedItems for Punctuated<T, P>
    where A: ToTokens + Eq + Hash,
          T: UniqueNestedItems<Item = A> {
    type Item = A;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        HashSet::from_iter(self.iter().flat_map(T::unique_nested_items))
    }
}


impl UniqueNestedItems for AngleBracketedGenericArguments {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        self.args.unique_nested_items()
    }
}
impl UniqueNestedItems for BareFnArg {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        self.ty.unique_nested_items()
    }
}

impl UniqueNestedItems for Binding {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        self.ty.unique_nested_items()
    }
}

impl UniqueNestedItems for Constraint {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        self.bounds.unique_nested_items()
    }
}

impl UniqueNestedItems for Expr {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        // TODO: Implement this if need
        HashSet::new()
    }
}

impl UniqueNestedItems for GenericArgument {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        match self {
            GenericArgument::Type(ty) => ty.unique_nested_items(),
            GenericArgument::Binding(binding) => binding.unique_nested_items(),
            GenericArgument::Constraint(constraint) => constraint.unique_nested_items(),
            GenericArgument::Const(expr) => expr.unique_nested_items(),
            GenericArgument::Lifetime(_) => HashSet::new(),
        }
    }
}

impl UniqueNestedItems for ParenthesizedGenericArguments {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        let mut involved = self.inputs.unique_nested_items();
        involved.extend(self.output.unique_nested_items());
        involved
    }
}


impl UniqueNestedItems for Path {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        self.segments.unique_nested_items()
    }
}

impl UniqueNestedItems for PathArguments {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        match self {
            PathArguments::AngleBracketed(args) => args.unique_nested_items(),
            PathArguments::Parenthesized(args) => args.unique_nested_items(),
            PathArguments::None => HashSet::new()
        }
    }
}

impl UniqueNestedItems for PathSegment {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        self.arguments.unique_nested_items()
    }
}

impl UniqueNestedItems for QSelf {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        self.ty.unique_nested_items()
    }
}

impl UniqueNestedItems for ReturnType {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        match self {
            ReturnType::Type(_, ty) => ty.unique_nested_items(),
            ReturnType::Default => HashSet::new()
        }
    }
}
impl UniqueNestedItems for Type {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        // let mut involved = HashSet::from([parse_quote!(Self)]);
        let mut involved = HashSet::from([]);
        match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => {
                involved.insert(self.clone());
                involved.extend(elem.unique_nested_items());
            }
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) =>
                involved.extend(elem.unique_nested_items()),
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                involved.extend(inputs.unique_nested_items());
                involved.extend(output.unique_nested_items());
            },
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                involved.extend(bounds.unique_nested_items());
            },
            Type::Path(TypePath { qself, path }) => {
                involved.insert(self.clone());
                involved.extend(path.unique_nested_items());
                if let Some(qself) = qself {
                    involved.extend(qself.unique_nested_items());
                }
            },
            Type::Tuple(TypeTuple { elems, .. }) => {
                involved.insert(self.clone());
                involved.extend(elems.unique_nested_items());
            },
            Type::Group(TypeGroup { elem, .. }) => {
                involved.insert(self.clone());
                involved.extend(elem.unique_nested_items());
            },
            _ => {}
        }
        involved
    }
}

impl UniqueNestedItems for TypeParamBound {
    type Item = Type;

    fn unique_nested_items(&self) -> HashSet<Self::Item> {
        match self {
            TypeParamBound::Trait(TraitBound { path, .. }) => {
                let mut involved = HashSet::from([]);
                let self_ty = parse_quote!(#path);
                involved.insert(self_ty);
                involved.extend(path.unique_nested_items());
                involved
            },
            TypeParamBound::Lifetime(_) => HashSet::new()
        }
    }
}