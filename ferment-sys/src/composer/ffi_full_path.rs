// use std::marker::PhantomData;
// use syn::Type;
// use crate::composer::SourceComposable;
// use crate::context::{ScopeContext, ScopeSearch};
// use crate::conversion::{ObjectKind, ScopeItemKind};
// use crate::ext::{Resolve, SpecialType, ToType};
// use crate::lang::RustSpecification;
// use crate::presentation::{FFIFullPath, RustFermentate};
//
// pub struct FFIFullPathComposer<'a, LANG, SPEC> {
//     pub search: ScopeSearch<'a>,
//     _marker: PhantomData<(LANG, SPEC)>,
// }
//
// // impl<'a, SPEC> SourceComposable for FFIFullPathComposer<'a, RustFermentate, SPEC>
// //     where SPEC: RustSpecification {
// //     type Source = ScopeContext;
// //     type Output = FFIFullPath<RustFermentate, SPEC>;
// //
// //     fn compose(&self, source: &Self::Source) -> Self::Output {
// //         let search_key = self.search.search_key();
// //         let maybe_obj = source.maybe_object_by_predicate_ref(&self.search);
// //         let full_ty = maybe_obj.as_ref().and_then(ObjectKind::maybe_type).unwrap_or(search_key.to_type());
// //         let maybe_special: Option<SpecialType<RustFermentate, SPEC>> = full_ty.maybe_resolve(source);
// //         match maybe_special {
// //             Some(special) => match maybe_obj {
// //                 Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => {
// //
// //                 }
// //             }
// //             None => {}
// //         }
// //
// //     }
// // }