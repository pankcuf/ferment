use syn::{GenericArgument, PathArguments, ReturnType, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::ast::CommaPunctuated;

pub trait LifetimeCleaner {
    fn clean_lifetimes(&mut self);
    fn lifetimes_cleaned(&self) -> Self where Self: Sized + Clone {
        let mut clone = self.clone();
        clone.clean_lifetimes();
        clone
    }
}

impl LifetimeCleaner for Type {
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
}

impl LifetimeCleaner for TypePath {
    fn clean_lifetimes(&mut self) {
        if let Some(last_segment) = self.path.segments.last_mut() {
            last_segment.arguments.clean_lifetimes();
        }
    }
}
impl LifetimeCleaner for TypeArray {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }
}

impl LifetimeCleaner for TypeBareFn {
    fn clean_lifetimes(&mut self) {
        self.lifetimes = None;
        self.inputs.iter_mut().for_each(|arg| arg.ty.clean_lifetimes());
        self.output.clean_lifetimes();
    }
}

impl LifetimeCleaner for PathArguments {
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
}

impl LifetimeCleaner for ReturnType {
    fn clean_lifetimes(&mut self) {
        match self {
            ReturnType::Default => {}
            ReturnType::Type(_, ref mut ty) => ty.clean_lifetimes(),
        }
    }
}

impl LifetimeCleaner for TypeGroup {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }
}

impl LifetimeCleaner for TypeImplTrait {
    fn clean_lifetimes(&mut self) {
        self.bounds = self.clone().bounds.into_iter().filter_map(|arg| if let TypeParamBound::Lifetime(_) = arg { None } else { Some(arg) }).collect();
    }
}
impl LifetimeCleaner for TypeTraitObject {
    fn clean_lifetimes(&mut self) {
        self.bounds = self.clone().bounds.into_iter().filter_map(|arg| if let TypeParamBound::Lifetime(_) = arg { None } else { Some(arg) }).collect();
    }
}

impl LifetimeCleaner for TypePtr {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }
}

impl LifetimeCleaner for TypeReference {
    fn clean_lifetimes(&mut self) {
        self.lifetime = None;
        self.elem.clean_lifetimes();
    }
}
impl LifetimeCleaner for TypeSlice {
    fn clean_lifetimes(&mut self) {
        self.elem.clean_lifetimes();
    }
}
impl LifetimeCleaner for TypeTuple {
    fn clean_lifetimes(&mut self) {
        self.elems.iter_mut().for_each(|e| e.clean_lifetimes());
    }
}