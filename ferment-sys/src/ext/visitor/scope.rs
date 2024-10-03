use std::hash::Hash;
use quote::ToTokens;
use syn::{PredicateType, TraitBound, Type, TypeParamBound, WherePredicate};
use crate::context::ScopeChain;
use crate::ext::ToType;
use crate::visitor::Visitor;

// pub trait ScopeExtension {
//     type Item: ToTokens + Eq + Hash;
//     #[allow(unused)]
//     fn add(&self, visitor: &mut Visitor, scope: &ScopeChain);
// }
//
// impl ScopeExtension for TypeParamBound {
//     type Item = Type;
//
//     fn add(&self, visitor: &mut Visitor, scope: &ScopeChain) {
//         match self {
//             TypeParamBound::Trait(TraitBound { path, .. }) => {
//                 let ty = path.to_type();
//                 visitor.add_full_qualified_type_match(scope, &ty);
//             },
//             TypeParamBound::Lifetime(_lifetime) => {}
//         }
//     }
// }
//
// impl ScopeExtension for WherePredicate {
//     type Item = Type;
//
//     fn add(&self, visitor: &mut Visitor, scope: &ScopeChain) {
//         match self {
//             WherePredicate::Type(PredicateType { bounds, bounded_ty, .. }) => {
//                 // let mut de_bounds: Vec<Path> =  vec![];
//                 bounds.iter().for_each(|bound| {
//                     match bound {
//                         TypeParamBound::Trait(TraitBound { path, .. }) => {
//                             let ty = path.to_type();
//                             // de_bounds.push(path.clone());
//                             visitor.add_full_qualified_type_match(scope, &ty);
//                         },
//                         TypeParamBound::Lifetime(_lifetime) => {}
//                     }
//                 });
//                 // generics.insert(parse_quote!(#generic_ident), de_bounds);
//                 visitor.add_full_qualified_type_match(scope, bounded_ty);
//             },
//             WherePredicate::Lifetime(_) => {}
//             WherePredicate::Eq(_) => {}
//         }    }
// }