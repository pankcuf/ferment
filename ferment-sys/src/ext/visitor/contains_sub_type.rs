use proc_macro2::Ident;
use syn::{Generics, GenericParam, Path, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParam, TypeParamBound, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::ast::AddPunctuated;
use crate::ext::{MaybeTraitBound, ToType};

pub trait ContainsSubType {
    fn contains_sub_type(&self, sub_type: &Type) -> bool;
}

impl ContainsSubType for Generics {
    fn contains_sub_type(&self, sub_type: &Type) -> bool {
        // Returns true if `sub_type` is exactly one of the type parameters declared
        // in these generics (e.g., `T`, `U`). Does not match composed types such as
        // `Vec<T>` or `Into<U>` — those should be considered for propagation.
        let params: Vec<Ident> = self.params.iter().filter_map(|p| if let GenericParam::Type(TypeParam { ident, .. }) = p { Some(ident.clone()) } else { None }).collect();
        match sub_type {
            Type::Path(TypePath { qself: None, path: Path { leading_colon: None, segments }, .. }) if segments.len() == 1 => {
                let ident = &segments.first().unwrap().ident;
                params.iter().any(|p| ident.eq(p))
            }
            _ => false,
        }
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
        self.iter().any(|type_param_bound| type_param_bound.maybe_trait_bound().is_some_and(|TraitBound { path, .. }| path.to_type().eq(sub_type)))
    }
}
