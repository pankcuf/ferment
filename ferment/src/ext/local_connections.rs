// use std::collections::HashSet;
// use syn::{Type, TypeArray, TypeBareFn, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
// use crate::context::ScopeChain;
// use crate::conversion::LocalTypeConversion;
//
// pub trait LocalConnections {
//     fn local_connections(&self, scope: &ScopeChain) -> HashSet<LocalTypeConversion>;
// }
//
// impl LocalConnections for Type {
//     fn local_connections(&self, scope: &ScopeChain) -> HashSet<LocalTypeConversion> {
//         // let mut involved = HashSet::from([parse_quote!(Self)]);
//         let mut involved = HashSet::from([]);
//         match self {
//             Type::Array(TypeArray { elem, .. }) |
//             Type::Ptr(TypePtr { elem, .. }) |
//             Type::Reference(TypeReference { elem, .. }) |
//             Type::Slice(TypeSlice { elem, .. }) =>
//                 involved.extend(elem.local_connections(scope)),
//             Type::BareFn(TypeBareFn { inputs, output, .. }) => {
//                 involved.extend(inputs.local_connections(scope));
//                 involved.extend(output.local_connections(scope));
//             },
//             Type::ImplTrait(TypeImplTrait { bounds, .. }) |
//             Type::TraitObject(TypeTraitObject { bounds, .. }) => {
//                 involved.extend(bounds.local_connections(scope));
//             },
//             Type::Path(TypePath { qself, path }) => {
//                 involved.insert(self.clone());
//                 involved.extend(path.local_connections(scope));
//                 if let Some(qself) = qself {
//                     involved.extend(qself.local_connections(scope));
//                 }
//             },
//             Type::Tuple(TypeTuple { elems, .. }) =>
//                 involved.extend(elems.local_connections(scope)),
//             _ => {}
//         }
//         involved
//     }
// }
