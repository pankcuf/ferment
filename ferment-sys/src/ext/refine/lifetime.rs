use syn::{GenericArgument, Lifetime, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, ReturnType, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::ast::CommaPunctuated;

pub trait LifetimeProcessor {
    fn clean_lifetimes(&mut self);
    fn lifetimes_cleaned(&self) -> Self where Self: Sized + Clone {
        let mut clone = self.clone();
        clone.clean_lifetimes();
        clone
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime>;
}

impl LifetimeProcessor for Type {
    fn clean_lifetimes(&mut self) {
        match self {
            Type::Path(type_path) => type_path.clean_lifetimes(),
            Type::Array(type_array) => type_array.clean_lifetimes(),
            Type::BareFn(type_bare_fn) => type_bare_fn.clean_lifetimes(),
            Type::Group(type_group) => type_group.clean_lifetimes(),
            Type::ImplTrait(type_impl_trait) => type_impl_trait.clean_lifetimes(),
            Type::Ptr(type_ptr) => type_ptr.clean_lifetimes(),
            Type::Reference(type_reference) => type_reference.clean_lifetimes(),
            Type::Slice(type_slice) => type_slice.clean_lifetimes(),
            Type::TraitObject(type_trait_object) => type_trait_object.clean_lifetimes(),
            Type::Tuple(type_tuple) => type_tuple.clean_lifetimes(),
            _ => {}
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        match self {
            Type::Array(type_array) => type_array.unique_lifetimes(),
            Type::BareFn(type_bare_fn) => type_bare_fn.unique_lifetimes(),
            Type::Group(type_group) => type_group.unique_lifetimes(),
            Type::ImplTrait(type_impl_trait) => type_impl_trait.unique_lifetimes(),
            Type::Path(type_path) => type_path.unique_lifetimes(),
            Type::Ptr(type_ptr) => type_ptr.unique_lifetimes(),
            Type::Reference(type_reference) => type_reference.unique_lifetimes(),
            Type::Slice(type_slice) => type_slice.unique_lifetimes(),
            Type::TraitObject(type_trait_object) => type_trait_object.unique_lifetimes(),
            Type::Tuple(type_tuple) => type_tuple.unique_lifetimes(),
            _ => vec![],
        }
    }
}

impl LifetimeProcessor for TypePath {
    fn clean_lifetimes(&mut self) {
        self.path.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.path.unique_lifetimes()
    }
}

impl LifetimeProcessor for Path {
    fn clean_lifetimes(&mut self) {
        if let Some(last_segment) = self.segments.last_mut() {
            last_segment.arguments.clean_lifetimes();
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.segments.last().map(|PathSegment { arguments, .. }| arguments.unique_lifetimes()).unwrap_or_default()
    }
}

impl LifetimeProcessor for TypeArray {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.elem.unique_lifetimes()
    }
}

impl LifetimeProcessor for TypeBareFn {
    fn clean_lifetimes(&mut self) {
        self.lifetimes = None;
        self.inputs.iter_mut().for_each(|arg| arg.ty.clean_lifetimes());
        self.output.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        let mut lifetimes = vec![];
        self.lifetimes.iter().for_each(|lifetime| {
            lifetimes.extend(lifetime.lifetimes.iter().map(|lt| lt.lifetime.clone()));
        });
        lifetimes.extend(self.inputs.iter().flat_map(|i| i.ty.unique_lifetimes()));
        lifetimes.extend(self.output.unique_lifetimes());
        lifetimes
    }
}

impl LifetimeProcessor for PathArguments {
    fn clean_lifetimes(&mut self) {
        let mut remove_brackets = false;
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(args) => {
                let cleaned_args = CommaPunctuated::from_iter(args.args.clone().into_iter().filter_map(|arg| if let GenericArgument::Lifetime(_) = arg { None } else { Some(arg) }));
                remove_brackets = cleaned_args.is_empty();
                args.args = cleaned_args;
            },
            PathArguments::Parenthesized(args) => {
                args.inputs.iter_mut().for_each(|i| i.clean_lifetimes());
                args.output.clean_lifetimes();
            },
        }
        if remove_brackets {
            *self = PathArguments::None;
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        match self {
            PathArguments::None => vec![],
            PathArguments::AngleBracketed(args) => {
                let mut lifetimes = vec![];
                args.args.iter().for_each(|arg| match arg {
                    GenericArgument::Lifetime(lt) => {
                        lifetimes.push(lt.clone());
                    },
                    GenericArgument::Type(ty) => {
                        lifetimes.extend(ty.unique_lifetimes());
                    }
                    GenericArgument::Const(_) => {}
                    GenericArgument::Binding(_) => {}
                    GenericArgument::Constraint(_) => {}
                });
                lifetimes
            },
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                let mut lifetimes = Vec::from_iter(inputs.iter().flat_map(LifetimeProcessor::unique_lifetimes));
                lifetimes.extend(output.unique_lifetimes());
                lifetimes
            }
        }
    }
}

impl LifetimeProcessor for ReturnType {
    fn clean_lifetimes(&mut self) {
        match self {
            ReturnType::Default => {}
            ReturnType::Type(_, ref mut ty) => ty.clean_lifetimes(),
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        match self {
            ReturnType::Default => vec![],
            ReturnType::Type(_, ty) => ty.unique_lifetimes(),
        }
    }
}

impl LifetimeProcessor for TypeGroup {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.elem.unique_lifetimes()
    }
}

impl LifetimeProcessor for TypeImplTrait {
    fn clean_lifetimes(&mut self) {
        self.bounds = self.clone().bounds.into_iter().filter_map(|arg| if let TypeParamBound::Lifetime(_) = arg { None } else { Some(arg) }).collect();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        let mut lifetimes = Vec::<Lifetime>::new();
        self.bounds.iter().for_each(|bound| {
            match bound {
                TypeParamBound::Trait(trait_bound) => {
                    if let Some(ref lts) = trait_bound.lifetimes {
                        lifetimes.extend(lts.lifetimes.iter().map(|lt| lt.lifetime.clone()));
                    }
                }
                TypeParamBound::Lifetime(lt) => {
                    lifetimes.push(lt.clone());
                }
            }
        });
        lifetimes
    }
}
impl LifetimeProcessor for TypeTraitObject {
    fn clean_lifetimes(&mut self) {
        self.bounds = self.clone().bounds.into_iter().filter_map(|arg| if let TypeParamBound::Lifetime(_) = arg { None } else { Some(arg) }).collect();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        let mut lifetimes = Vec::<Lifetime>::new();
        self.bounds.iter().for_each(|bound| {
            match bound {
                TypeParamBound::Trait(trait_bound) => {
                    if let Some(ref lts) = trait_bound.lifetimes {
                        lifetimes.extend(lts.lifetimes.iter().map(|lt| lt.lifetime.clone()));
                    }
                }
                TypeParamBound::Lifetime(lt) => {
                    lifetimes.push(lt.clone());
                }
            }
        });
        lifetimes
    }
}

impl LifetimeProcessor for TypePtr {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.elem.unique_lifetimes()
    }
}

impl LifetimeProcessor for TypeReference {
    fn clean_lifetimes(&mut self) {
        self.lifetime = None;
        self.elem.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        let mut lifetimes = self.elem.unique_lifetimes();
        if let Some(ref lifetime) = self.lifetime {
            lifetimes.push(lifetime.clone());
        }
        lifetimes
    }
}
impl LifetimeProcessor for TypeSlice {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.elem.unique_lifetimes()
    }
}
impl LifetimeProcessor for TypeTuple {
    fn clean_lifetimes(&mut self) {
        self.elems.iter_mut().for_each(|e| e.clean_lifetimes());
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.elems.iter().flat_map(|e| e.unique_lifetimes()).collect()
    }
}