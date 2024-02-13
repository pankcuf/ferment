use proc_macro2::Ident;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, Constraint, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf, ReturnType, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;

pub trait Constraints {
    fn has_self(&self) -> bool;
    fn has_no_self(&self) -> bool { !self.has_self() }
}


impl<T, P> Constraints for Punctuated<T, P> where T: Constraints {
    fn has_self(&self) -> bool {
        self.iter().find(|p| p.has_self()).is_some()
    }
}

impl Constraints for Constraint {
    fn has_self(&self) -> bool {
        self.ident.has_self()
    }
}

impl Constraints for Ident {
    fn has_self(&self) -> bool {
        self.to_string().as_str() == "Self"
    }
}

impl Constraints for QSelf {
    fn has_self(&self) -> bool {
        self.ty.has_self()
    }
}

impl Constraints for TypeParamBound {
    fn has_self(&self) -> bool {
        if let TypeParamBound::Trait(bound) = self {
            bound.path.has_self()
        } else {
            false
        }
    }
}

impl Constraints for Path {
    fn has_self(&self) -> bool {
        self.segments.has_self()
    }
}

impl Constraints for GenericArgument {
    fn has_self(&self) -> bool {
        match self {
            GenericArgument::Lifetime(_) => false,
            GenericArgument::Type(ty) => ty.has_self(),
            GenericArgument::Const(_expr) => false, // TODO: Implement this
            GenericArgument::Binding(binding) => binding.has_self(),
            GenericArgument::Constraint(constraint) => constraint.has_self(),
        }
    }
}

impl Constraints for Binding {
    fn has_self(&self) -> bool {
        self.ident.to_string().as_str().eq("Self") || self.ty.has_self()
    }
}

impl Constraints for ReturnType {
    fn has_self(&self) -> bool {
        if let ReturnType::Type(_, ty) = self {
            ty.has_self()
        } else {
            false
        }
    }
}

impl Constraints for PathSegment {
    fn has_self(&self) -> bool {
        self.ident.has_self() || self.arguments.has_self()
    }
}

impl Constraints for PathArguments {
    fn has_self(&self) -> bool {
        match self {
            PathArguments::None => false,
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                args.has_self(),
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) =>
                inputs.has_self() || output.has_self()
        }
    }
}

impl Constraints for Type {
    fn has_self(&self) -> bool {
        match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Paren(TypeParen { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => elem.has_self(),
            Type::BareFn(TypeBareFn { inputs, output, .. }) =>
                inputs.iter().find(|BareFnArg { ty, .. }| ty.has_self()).is_some() || output.has_self(),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => bounds.has_self(),
            Type::Path(TypePath { qself, path }) => path.has_self() || qself.as_ref().map_or(false, Constraints::has_self),
            Type::Tuple(TypeTuple { elems, .. }) => elems.has_self(),
            _ => false,
        }
    }
}


