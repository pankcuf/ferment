use syn::{BareFnArg, GenericParam, Generics, Path, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParam, TypeParamBound, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::ast::{AddPunctuated, CommaPunctuated};
use crate::ext::{ToPath, ToType};

pub trait ContainsBound {
    fn contains_bound(&self, bound: &Path) -> bool;
}
pub trait ContainsSubType {
    fn contains_sub_type(&self, sub_type: &Type) -> bool;
}

impl ContainsBound for Generics {
    fn contains_bound(&self, bound: &Path) -> bool {
        self.params.iter()
            .any(|generic_param| match generic_param {
                GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                    ident.to_path().eq(bound) || bounds.contains_bound(bound)
                },
                GenericParam::Lifetime(_) => false,
                GenericParam::Const(_) => false
            })
    }
}

impl ContainsBound for GenericParam {
    fn contains_bound(&self, bound: &Path) -> bool {
        match self {
            GenericParam::Type(type_param) => type_param.contains_bound(bound),
            GenericParam::Lifetime(_) => false,
            GenericParam::Const(_) => false
        }
    }
}

impl ContainsBound for TypeParam {
    fn contains_bound(&self, bound: &Path) -> bool {
        self.ident.to_path().eq(bound) || self.bounds.contains_bound(bound)
    }
}

impl ContainsBound for CommaPunctuated<GenericParam> {
    fn contains_bound(&self, bound: &Path) -> bool {
        self.iter().any(|generic_param| generic_param.contains_bound(bound))
    }
}
impl ContainsBound for AddPunctuated<TypeParamBound> {
    fn contains_bound(&self, bound: &Path) -> bool {
        self.iter().any(|type_param_bound| match type_param_bound {
            TypeParamBound::Trait(TraitBound { path, .. }) =>
                path.eq(bound),
            TypeParamBound::Lifetime(_) => false
        })
    }
}

impl ContainsBound for BareFnArg {
    fn contains_bound(&self, bound: &Path) -> bool {
        self.ty.contains_bound(bound)
    }
}

impl ContainsBound for CommaPunctuated<BareFnArg> {
    fn contains_bound(&self, bound: &Path) -> bool {
        self.iter().any(|arg| arg.contains_bound(bound))
    }
}
impl ContainsBound for ReturnType {
    fn contains_bound(&self, bound: &Path) -> bool {
        match self {
            ReturnType::Default => false,
            ReturnType::Type(_, ty) => ty.contains_bound(bound)
        }
    }
}

impl ContainsBound for QSelf {
    fn contains_bound(&self, bound: &Path) -> bool {
        self.ty.contains_bound(bound)
    }
}

impl ContainsBound for Type {
    fn contains_bound(&self, bound: &Path) -> bool {
        match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Group(TypeGroup { elem , .. }) |
            Type::Paren(TypeParen { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                elem.contains_bound(bound),
            Type::BareFn(TypeBareFn { inputs, output, .. }) =>
                inputs.contains_bound(bound) ||
                    output.contains_bound(bound),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) =>
                bounds.contains_bound(bound),
            Type::Path(TypePath { path, qself}) =>
                path.eq(bound) ||
                    qself.as_ref().map(|q| q.contains_bound(bound)).unwrap_or_default(),
            Type::Tuple(TypeTuple { elems, .. }) =>
                elems.iter().any(|ty| ty.contains_bound(bound)),
            _ => false
        }
    }
}

impl ContainsSubType for Generics {
    fn contains_sub_type(&self, _sub_type: &Type) -> bool {
        false
        // match sub_type {
        //     Type::Array(TypeArray { elem, .. }) |
        //     Type::Group(TypeGroup { elem , .. }) |
        //     Type::Paren(TypeParen { elem, .. }) |
        //     Type::Ptr(TypePtr { elem, .. }) |
        //     Type::Reference(TypeReference { elem, .. }) |
        //     Type::Slice(TypeSlice { elem, .. }) =>
        //         elem.contains_sub_type(sub_type),
        //     _ => false
        // }
    }
}

impl ContainsSubType for Type {
    fn contains_sub_type(&self, sub_type: &Type) -> bool {
        self.eq(sub_type) || match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Group(TypeGroup { elem , .. }) |
            Type::Paren(TypeParen { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                elem.contains_sub_type(sub_type),
            Type::BareFn(TypeBareFn { inputs, output, .. }) =>
                inputs.iter().any(|i| i.ty.eq(sub_type)) ||
                    output.contains_sub_type(sub_type),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) =>
                bounds.contains_sub_type(sub_type),
            Type::Path(TypePath { qself, ..}) =>
                qself.as_ref().map(|q| q.ty.contains_sub_type(sub_type)).unwrap_or_default(),
            Type::Tuple(TypeTuple { elems, .. }) =>
                elems.iter().any(|ty| ty.contains_sub_type(sub_type)),
            _ => false
        }
    }
}

impl ContainsSubType for ReturnType {
    fn contains_sub_type(&self, sub_type: &Type) -> bool {
        match self {
            ReturnType::Default => false,
            ReturnType::Type(_, ty) => ty.contains_sub_type(sub_type)
        }
    }
}

impl ContainsSubType for AddPunctuated<TypeParamBound> {
    fn contains_sub_type(&self, sub_type: &Type) -> bool {
        self.iter().any(|type_param_bound| match type_param_bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => path.to_type().eq(sub_type),
            TypeParamBound::Lifetime(_) => false
        })
    }
}