use syn::{AngleBracketedGenericArguments, GenericArgument, GenericParam, Lifetime, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::ast::{AddPunctuated, CommaPunctuated};
use crate::kind::{CallbackKind, GenericTypeKind, SmartPointerKind, TypeKind};
use crate::ext::ToType;

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
            lifetimes.extend(lifetime.lifetimes.iter().filter_map(|lt| match lt {
                GenericParam::Lifetime(lt) => Some(lt.lifetime.clone()),
                _ => None
            }
            ));
        });
        lifetimes.extend(self.inputs.iter().flat_map(|i| i.ty.unique_lifetimes()));
        lifetimes.extend(self.output.unique_lifetimes());
        lifetimes
    }
}

impl LifetimeProcessor for PathArguments {
    fn clean_lifetimes(&mut self) {
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(args) => {
                args.clean_lifetimes();
                if args.args.is_empty() {
                    *self = PathArguments::None;
                }
            },
            PathArguments::Parenthesized(args) => {
                args.inputs.iter_mut().for_each(|i| i.clean_lifetimes());
                args.output.clean_lifetimes();
            },
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        match self {
            PathArguments::None => vec![],
            PathArguments::AngleBracketed(args) => {
                args.unique_lifetimes()
            },
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                let mut lifetimes = Vec::from_iter(inputs.iter().flat_map(LifetimeProcessor::unique_lifetimes));
                lifetimes.extend(output.unique_lifetimes());
                lifetimes
            }
        }
    }
}
impl LifetimeProcessor for AngleBracketedGenericArguments {
    fn clean_lifetimes(&mut self) {
        self.args = CommaPunctuated::from_iter(self.args.iter().filter_map(|arg| match arg {
            GenericArgument::Type(ty) => Some(GenericArgument::Type(ty.lifetimes_cleaned())),
            _ => None,
        }));
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        let mut lifetimes = vec![];
        self.args.iter().for_each(|arg| match arg {
            GenericArgument::Lifetime(lt) => {
                lifetimes.push(lt.clone());
            },
            GenericArgument::Type(ty) => {
                lifetimes.extend(ty.unique_lifetimes());
            }
            _ => {}
        });
        lifetimes
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
        self.bounds.iter().for_each(|bound| match bound {
            TypeParamBound::Trait(trait_bound) => {
                lifetimes.extend(trait_bound.unique_lifetimes());
            }
            TypeParamBound::Lifetime(lt) => {
                lifetimes.push(lt.clone());
            }
            _ => {}
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
                        lifetimes.extend(lts.lifetimes.iter().filter_map(|lt| match lt {
                            GenericParam::Lifetime(lt) => Some(lt.lifetime.clone()),
                            _ => None
                        }));
                    }
                }
                TypeParamBound::Lifetime(lt) => {
                    lifetimes.push(lt.clone());
                }
                _ => {}
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

impl LifetimeProcessor for TraitBound {
    fn clean_lifetimes(&mut self) {
        self.lifetimes = None;
        self.path.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        let mut lifetimes = self.path.unique_lifetimes();
        if let Some(ref lts) = self.lifetimes {
            lifetimes.extend(lts.lifetimes.iter().filter_map(|lt| match lt {
                GenericParam::Lifetime(lt) => Some(lt.lifetime.clone()),
                _ => None
            }));
        }
        lifetimes
    }
}

impl LifetimeProcessor for TypeKind {
    fn clean_lifetimes(&mut self) {
        match self {
            TypeKind::Primitive(ty) |
            TypeKind::Complex(ty) => ty.clean_lifetimes(),
            TypeKind::Generic(ty) => ty.clean_lifetimes()
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.to_type().unique_lifetimes()
    }
}
impl LifetimeProcessor for GenericTypeKind {
    fn clean_lifetimes(&mut self) {
        match self {
            GenericTypeKind::Map(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Group(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Result(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Box(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Cow(ty) => ty.clean_lifetimes(),
            GenericTypeKind::SmartPointer(kind) => kind.clean_lifetimes(),
            GenericTypeKind::AnyOther(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Array(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Slice(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Tuple(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Optional(ty) => ty.clean_lifetimes(),
            GenericTypeKind::Callback(kind) => kind.ty_mut().clean_lifetimes(),
            GenericTypeKind::TraitBounds(bounds) => {
                *bounds = bounds.iter().filter_map(|b| match b {
                    TypeParamBound::Trait(trait_bound) =>
                        Some(TypeParamBound::Trait(trait_bound.lifetimes_cleaned())),
                    _ =>
                        None,
                }).collect();
            },
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        match self {
            GenericTypeKind::Map(ty) |
            GenericTypeKind::Group(ty) |
            GenericTypeKind::Result(ty) |
            GenericTypeKind::Box(ty) |
            GenericTypeKind::AnyOther(ty) |
            GenericTypeKind::Array(ty) |
            GenericTypeKind::Slice(ty) |
            GenericTypeKind::Tuple(ty) |
            GenericTypeKind::Cow(ty) |
            GenericTypeKind::Optional(ty) => ty.unique_lifetimes(),
            GenericTypeKind::SmartPointer(kind) => kind.unique_lifetimes(),
            GenericTypeKind::Callback(kind) => kind.unique_lifetimes(),
            GenericTypeKind::TraitBounds(bounds) => bounds.unique_lifetimes()
        }
    }
}

impl LifetimeProcessor for AddPunctuated<TypeParamBound> {
    fn clean_lifetimes(&mut self) {
        for type_param_bound in self.iter_mut() {
            match type_param_bound {
                TypeParamBound::Trait(trait_bound) => trait_bound.clean_lifetimes(),
                _ => {}
            }
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.iter().flat_map(|e| match e {
            TypeParamBound::Trait(trait_bound) => trait_bound.unique_lifetimes(),
            TypeParamBound::Lifetime(lifetime) => vec![lifetime.clone()],
            _ => vec![]
        }).collect()
    }
}

impl LifetimeProcessor for CallbackKind {
    fn clean_lifetimes(&mut self) {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty.clean_lifetimes(),
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.to_type().unique_lifetimes()
    }
}
impl LifetimeProcessor for SmartPointerKind {
    fn clean_lifetimes(&mut self) {
        match self {
            SmartPointerKind::Box(ty) |
            SmartPointerKind::Rc(ty) |
            SmartPointerKind::Arc(ty) |
            SmartPointerKind::Cell(ty) |
            SmartPointerKind::RefCell(ty) |
            SmartPointerKind::UnsafeCell(ty) |
            SmartPointerKind::Mutex(ty) |
            SmartPointerKind::OnceLock(ty) |
            SmartPointerKind::RwLock(ty) |
            SmartPointerKind::Pin(ty) => ty.clean_lifetimes(),
        }
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.to_type().unique_lifetimes()
    }
}