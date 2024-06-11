use syn::Type;
use crate::context::ScopeContext;
// use crate::conversion::{GenericTypeConversion, TypeConversion};

pub trait Resolve {
    fn resolve(&self, source: &ScopeContext) -> Self;
}

// impl Resolve for TypeConversion {
//     fn resolve(&self, source: &ScopeContext) -> Self {
//         TypeConversion::from(match self {
//             TypeConversion::Primitive(ty) => ty.clone(),
//             TypeConversion::Complex(ty) => ty.resolve(source),
//             TypeConversion::Callback(ty) => ty.resolve(source),
//             TypeConversion::Generic(ty) => ty.resolve(source)
//         })
//     }
// }

impl Resolve for Type {
    fn resolve(&self, source: &ScopeContext) -> Self {
        source.full_type_for(self)
    }
}

// impl Resolve for GenericTypeConversion {
//     fn resolve(&self, source: &ScopeContext) -> Self {
//         match self {
//             GenericTypeConversion::Map(ty) => GenericTypeConversion::Map(ty.resolve(source)),
//             GenericTypeConversion::IndexMap(ty) => GenericTypeConversion::IndexMap(ty.resolve(source)),
//             GenericTypeConversion::SerdeJsonMap(ty) => GenericTypeConversion::SerdeJsonMap(ty.resolve(source)),
//             GenericTypeConversion::Vec(ty) => GenericTypeConversion::Vec(ty.resolve(source)),
//             GenericTypeConversion::BTreeSet(ty) => GenericTypeConversion::BTreeSet(ty.resolve(source)),
//             GenericTypeConversion::HashSet(ty) => GenericTypeConversion::HashSet(ty.resolve(source)),
//             GenericTypeConversion::Result(ty) => GenericTypeConversion::Result(ty.resolve(source)),
//             GenericTypeConversion::Box(ty) => GenericTypeConversion::Box(ty.resolve(source)),
//             GenericTypeConversion::AnyOther(ty) => GenericTypeConversion::AnyOther(ty.resolve(source)),
//             GenericTypeConversion::Array(ty) => GenericTypeConversion::Array(ty.resolve(source)),
//             GenericTypeConversion::Slice(ty) => GenericTypeConversion::Slice(ty.resolve(source)),
//             GenericTypeConversion::Tuple(ty) => GenericTypeConversion::Tuple(ty.resolve(source)),
//             GenericTypeConversion::Optional(ty) => GenericTypeConversion::Optional(ty.resolve(source)),
//             GenericTypeConversion::TraitBounds(bounds) => {
//                 bounds.iter().map(|param| match param {
//                     TypeParamBound::Trait(bound) => {
//                         Type::parse(bound)
//                         bound.path.
//                             TypeParamBound
//                     }::Lifetime(_) => {}
//                 })
//                 let bounds
//
//
//             }
//         }
//     }
// }