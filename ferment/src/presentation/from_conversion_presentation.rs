// use quote::{quote, ToTokens};
// use proc_macro2::TokenStream as TokenStream2;
// use crate::ast::{CommaPunctuated, CommaPunctuatedTokens};
// use crate::presentation::{DictionaryExpr, DictionaryName, InterfacesMethodExpr};
//
// #[derive(Clone, Debug)]
// pub enum FromConversionPresentation {
//     Just(TokenStream2),
//     Tuple(CommaPunctuatedTokens),
//     Enum(CommaPunctuated<TokenStream2>),
//     Map(TokenStream2, TokenStream2),
//     Result(TokenStream2, TokenStream2),
//     TryInto(TokenStream2),
//     SmartPointer(TokenStream2, TokenStream2)
// }
//
// impl ToTokens for FromConversionPresentation {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         match self {
//             FromConversionPresentation::Just(conversion) =>
//                 conversion.to_token_stream(),
//             FromConversionPresentation::Enum(conversions) => {
//                 let ffi_ref = DictionaryName::FfiRef;
//                 DictionaryExpr::FromRoot(DictionaryExpr::Match(quote!(#ffi_ref { #conversions })).to_token_stream())
//                     .to_token_stream()
//             },
//             FromConversionPresentation::Map(from_key_conversion, from_value_conversion) => {
//                 let ffi_ref = DictionaryName::FfiRef;
//                 let count = DictionaryName::Count;
//                 let keys = DictionaryName::Keys;
//                 let values = DictionaryName::Values;
//                 let args = CommaPunctuated::from_iter([
//                     quote!(#ffi_ref.#count),
//                     quote!(#ffi_ref.#keys),
//                     quote!(#ffi_ref.#values),
//                     from_key_conversion.to_token_stream(),
//                     from_value_conversion.to_token_stream(),
//                 ]);
//                 DictionaryExpr::FromRoot(InterfacesMethodExpr::FoldToMap(args.to_token_stream()).to_token_stream())
//                     .to_token_stream()
//             },
//             FromConversionPresentation::Result(from_ok_conversion, from_error_conversion) => {
//                 let ffi_ref = DictionaryName::FfiRef;
//                 let ok = DictionaryName::Ok;
//                 let error = DictionaryName::Error;
//                 DictionaryExpr::FromRoot(
//                     InterfacesMethodExpr::FoldToResult(CommaPunctuated::from_iter([
//                         quote!(#ffi_ref.#ok),
//                         quote!(#ffi_ref.#error),
//                         from_ok_conversion.to_token_stream(),
//                         from_error_conversion.to_token_stream(),
//                     ]).to_token_stream()).to_token_stream())
//                     .to_token_stream()
//             },
//             FromConversionPresentation::Tuple(conversions) =>
//                 DictionaryExpr::FromRoot(quote!((#conversions))).to_token_stream(),
//             FromConversionPresentation::TryInto(expr) => quote! {
//                 #expr.try_into().unwrap()
//             },
//             FromConversionPresentation::SmartPointer(ty, conversion) => quote! {
//                 let ffi_ref = &*ffi;
//                 #ty::new(#conversion)
//             }
//         }.to_tokens(tokens)
//     }
// }
