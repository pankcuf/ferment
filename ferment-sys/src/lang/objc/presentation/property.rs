// use std::fmt::{Display, Formatter};
// use quote::{quote, ToTokens};
// use syn::__private::TokenStream2;
// use crate::composable::FieldComposer;
// use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
// use crate::presentable::{Aspect, ScopeContextPresentable};
//
//
// #[derive(Clone, Debug)]
// pub enum Property {
//     NonatomicReadwrite { ty: TokenStream2, name: TokenStream2 },
//     NonatomicAssign { ty: TokenStream2, name: TokenStream2 },
//     Initializer { field_name: TokenStream2, field_initializer: TokenStream2 },
//     // AccInitialized { field_name: TokenStream2, var: TokenStream2, field_initializer: TokenStream2 }
// }
//
// impl Property {
//     pub fn nonatomic_readwrite<SPEC>(composer: &FieldComposer<ObjCFermentate, SPEC>) -> Self
//         where SPEC: ObjCSpecification {
//         let FieldComposer { kind, name, .. } = composer;
//         Property::NonatomicReadwrite {
//             ty: kind.to_token_stream(),
//             name: name.to_token_stream()
//         }
//     }
//     pub fn nonatomic_assign<SPEC>(composer: &FieldComposer<ObjCFermentate, SPEC>) -> Self
//         where SPEC: ObjCSpecification {
//         let FieldComposer { kind, name, .. } = composer;
//         Property::NonatomicAssign {
//             ty: kind.to_token_stream(),
//             name: name.to_token_stream()
//         }
//     }
//     pub fn initializer<SPEC>(composer: &FieldComposer<ObjCFermentate, SPEC>) -> Self
//         where SPEC: ObjCSpecification {
//         Property::Initializer {
//             field_name: composer.tokenized_name(),
//             field_initializer: composer.to_token_stream()
//         }
//     }
// }
//
// // impl<SPEC> ToTokens for FieldComposer<ObjCFermentate, SPEC>
// //     where SPEC: Specification<ObjCFermentate, Attr=AttrWrapper, Gen=Option<Generics>> {
// //     #[allow(unused_variables)]
// //     fn to_tokens(&self, tokens: &mut TokenStream2) {
// //         let Self { name, kind, attrs, .. } = self;
// //
// //         let template = quote! {
// //             //#ifdef SMTH
// //             //#(#attrs)*
// //             #name: #kind
// //             //#endif SMTH
// //
// //         };
// //         template.to_tokens(tokens)
// //     }
// // }
// //
//
//
// impl ToTokens for Property {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         match self {
//             Property::NonatomicReadwrite { ty, name } => {
//                 quote! {
//                     @property (nonatomic, readwrite) #ty #name
//                 }
//             }
//             Property::NonatomicAssign { ty, name } => {
//                 quote! {
//                     @property (nonatomic, assign) #ty #name
//                 }
//             }
//             Property::Initializer { field_name, field_initializer } => {
//                 quote! {
//                     obj.#field_name = #field_initializer
//                 }
//             }
//             // Property::AccInitialized { field_name, var, field_initializer } => {
//             //     quote! {
//             //         #var #field_name = #field_initializer
//             //     }
//             // }
//         }.to_tokens(tokens)
//     }
// }
//
// impl Display for Property {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(match self {
//             Property::NonatomicReadwrite { ty, name } =>
//                 format!("@property (nonatomic, readwrite) {} {}", ty.to_string(), name.to_string()),
//             Property::NonatomicAssign { ty, name } =>
//                 format!("@property (nonatomic, assign) {} {}", ty.to_string(), name.to_string()),
//             Property::Initializer { field_name, field_initializer } =>
//                 format!("obj.{} = {}", field_name.to_string(), field_initializer.to_string()),
//             // Property::AccInitialized { field_name, var, field_initializer } =>
//             //     format!("{} {} = {}", var.to_string(), field_name.to_string(), field_initializer.to_string()),
//         }.as_str())
//     }
// }
//
// // #[derive(Clone, Debug)]
// // pub enum MethodDeclaration {
// //     InitWith { c_type: TokenStream2 }
// // }
//
// impl<SPEC> From<&FieldComposer<ObjCFermentate, SPEC>> for Property
//     where SPEC: ObjCSpecification,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     fn from(value: &FieldComposer<ObjCFermentate, SPEC>) -> Self {
//         Property::NonatomicReadwrite {
//             ty: value.ty().to_token_stream(),
//             name: value.name.to_token_stream()
//         }
//     }
// }