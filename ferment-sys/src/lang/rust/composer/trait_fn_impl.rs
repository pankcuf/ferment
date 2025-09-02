// use quote::{quote, ToTokens};
// use syn::{parse_quote, FnArg, Lifetime, PatType, Receiver, ReturnType, Signature, Type, Visibility};
// use syn::token::Semi;
// use crate::ast::CommaPunctuatedTokens;
// use crate::composable::FieldComposer;
// use crate::composer::{AttrComposable, CommaPunctuatedArgKinds, ConversionFromComposer, ConversionToComposer, SourceComposable, TraitFnImplComposer, VarComposer};
// use crate::context::ScopeContext;
// use crate::kind::FieldTypeKind;
// use crate::lang::{RustSpecification, Specification};
// use crate::presentable::{ArgKind, BindingPresentableContext, Expression, SeqKind};
// use crate::presentation::{BindingPresentation, DictionaryExpr, DictionaryName, Name};
//
// impl SourceComposable for TraitFnImplComposer<RustSpecification> {
//     type Source = ScopeContext;
//     type Output = BindingPresentation;
//
//     fn compose(&self, source: &Self::Source) -> Self::Output {
//         let attrs = self.compose_attributes();
//         let mut path = path.clone();
//         let last = path.segments.pop().unwrap();
//         let last_segment = last.value();
//         let path = parse_quote!(#path<#trait_ty>::#last_segment);
//         let full_self_ty: Type = self_ty.resolve(source);
//         let full_trait_ty: Type = trait_ty.resolve(source);
//         let mut used_lifetimes = Vec::<Lifetime>::new();
//         let Signature { output, inputs, asyncness, ident, .. } = sig;
//         let (return_type_presentation, return_type_conversion) = match output {
//             ReturnType::Default => (ReturnType::Default, <RustSpecification as Specification>::Expr::simple(Semi::default().to_token_stream())),
//             ReturnType::Type(_, ty) => (
//                 ReturnType::Type(Default::default(), Box::new(VarComposer::<RustSpecification>::key_ref_in_scope(ty, &source.scope).compose(source).to_type())),
//                 ConversionToComposer::<RustSpecification>::key(<RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Obj), ty, &source.scope).compose(source)
//             )
//         };
//
//         let mut arguments = CommaPunctuatedArgKinds::<RustSpecification>::new();
//         let mut argument_names = CommaPunctuatedTokens::new();
//         let mut argument_conversions = CommaPunctuatedArgKinds::<RustSpecification>::new();
//         for arg in inputs {
//             println!("\t\targ: ({})", arg.to_token_stream());
//             match arg {
//                 FnArg::Receiver(Receiver { mutability, reference, attrs, ty, .. }) => {
//                     println!("\t\t\treceiver: {}", ty.to_token_stream());
//
//                     if let Some((_, Some(lt))) = reference {
//                         used_lifetimes.push(lt.clone());
//                     }
//                     println!("\t\t\t\t: <{} as {}>", self_ty.to_token_stream(), trait_ty.to_token_stream());
//
//                     let expr_composer = match (mutability, reference) {
//                         (Some(..), _) => |expr: <RustSpecification as Specification>::Expr| <RustSpecification as Specification>::Expr::AsMutRef(expr.into()),
//                         (_, Some(..)) => |expr: <RustSpecification as Specification>::Expr| <RustSpecification as Specification>::Expr::AsRef(expr.into()),
//                         (..) => |expr: <RustSpecification as Specification>::Expr| expr.into(),
//                     };
//
//                     let name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Self_);
//                     argument_names.push(name.to_token_stream());
//                     arguments.push(ArgKind::Named(
//                         FieldComposer::new(name.clone(), FieldTypeKind::Type(full_trait_ty.clone()), true, attrs.cfg_attributes()),
//                         Visibility::Inherited
//                     ));
//                     argument_conversions.push(ArgKind::AttrExpression(
//                         expr_composer(Expression::dict_expr(DictionaryExpr::SelfAsTrait(full_self_ty.to_token_stream(), if mutability.is_some() { quote!(mut) } else { quote!(const) }))),
//                         <RustSpecification as Specification>::Attr::default()
//                     ));
//                 },
//                 FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
//                     println!("\t\t\ttyped: {}", ty.to_token_stream());
//                     used_lifetimes.extend(ty.unique_lifetimes());
//                     let name = Name::Pat(*pat.clone());
//                     argument_names.push(name.to_token_stream());
//                     arguments.push(ArgKind::Named(FieldComposer::typed(name.clone(), ty, true, attrs), Visibility::Inherited));
//                     argument_conversions.push(ArgKind::AttrExpression(
//                         ConversionFromComposer::<RustSpecification>::key_in_scope(name.clone(), ty, &source.scope).compose(source),
//                         <RustSpecification as Specification>::Attr::default()
//                     ));
//                 }
//             }
//         }
//         let input_conversions = SeqKind::TraitImplFnCall(full_self_ty, full_trait_ty, ident.clone(), argument_conversions);
//         BindingPresentableContext::RegFn(
//             path,
//             asyncness.is_some(),
//             arguments,
//             return_type_presentation,
//             input_conversions,
//             return_type_conversion,
//             attrs,
//             used_lifetimes,
//             generics
//         )
//     }
// }