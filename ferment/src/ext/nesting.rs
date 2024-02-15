use std::collections::HashSet;
use std::hash::Hash;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, Constraint, Expr, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;

pub trait NestingExtension {
    type Item: ToTokens + Eq + Hash;
    fn nested_items(&self) -> HashSet<Self::Item>;
}

impl<A, T, P> NestingExtension for Punctuated<T, P>
    where A: ToTokens + Eq + Hash,
          T: NestingExtension<Item = A> {
    type Item = A;

    fn nested_items(&self) -> HashSet<Self::Item> {
        HashSet::from_iter(self.iter().flat_map(|ff| ff.nested_items()))
    }
}


impl NestingExtension for AngleBracketedGenericArguments {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        self.args.nested_items()
    }
}
impl NestingExtension for BareFnArg {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        self.ty.nested_items()
    }
}

impl NestingExtension for Binding {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        self.ty.nested_items()
    }
}

impl NestingExtension for Constraint {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        self.bounds.nested_items()
    }
}

impl NestingExtension for Expr {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        // TODO: Implement this if need
        HashSet::new()
    }
}

impl NestingExtension for GenericArgument {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        match self {
            GenericArgument::Type(ty) => ty.nested_items(),
            GenericArgument::Binding(binding) => binding.nested_items(),
            GenericArgument::Constraint(constraint) => constraint.nested_items(),
            GenericArgument::Const(expr) => expr.nested_items(),
            GenericArgument::Lifetime(_) => HashSet::new(),
        }
    }
}

impl NestingExtension for ParenthesizedGenericArguments {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        let mut involved = self.inputs.nested_items();
        involved.extend(self.output.nested_items());
        involved
    }
}


impl NestingExtension for Path {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        self.segments.nested_items()
    }
}

impl NestingExtension for PathArguments {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        match self {
            PathArguments::AngleBracketed(args) => args.nested_items(),
            PathArguments::Parenthesized(args) => args.nested_items(),
            PathArguments::None => HashSet::new()
        }
    }
}

impl NestingExtension for PathSegment {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        self.arguments.nested_items()
    }
}

impl NestingExtension for QSelf {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        self.ty.nested_items()
    }
}

impl NestingExtension for ReturnType {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        match self {
            ReturnType::Type(_, ty) => ty.nested_items(),
            ReturnType::Default => HashSet::new()
        }
    }
}
impl NestingExtension for Type {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        // let mut involved = HashSet::from([parse_quote!(Self)]);
        let mut involved = HashSet::from([]);
        match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                involved.extend(elem.nested_items()),
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                involved.extend(inputs.nested_items());
                involved.extend(output.nested_items());
            },
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                involved.extend(bounds.nested_items());
            },
            Type::Path(TypePath { qself, path }) => {
                involved.insert(self.clone());
                involved.extend(path.nested_items());
                if let Some(qself) = qself {
                    involved.extend(qself.nested_items());
                }
            },
            Type::Tuple(TypeTuple { elems, .. }) =>
                involved.extend(elems.nested_items()),
            _ => {}
        }
        involved
    }
}

impl NestingExtension for TypeParamBound {
    type Item = Type;

    fn nested_items(&self) -> HashSet<Self::Item> {
        match self {
            TypeParamBound::Trait(TraitBound { path, .. }) => path.nested_items(),
            TypeParamBound::Lifetime(_) => HashSet::new()
        }
    }
}