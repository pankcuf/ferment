use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use crate::ast::CommaPunctuatedTokens;
use crate::composable::{FieldTypeComposition, FieldTypeConversionKind};
use crate::naming::{DictionaryExpr, DictionaryName, InterfacesMethodExpr, Name};
#[derive(Clone, Debug)]
pub enum ToConversionPresentation {
    Simple(TokenStream2),
    Enum(CommaPunctuatedTokens),
    Tuple(CommaPunctuatedTokens),
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2)
}

impl ToTokens for ToConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ToConversionPresentation::Simple(conversion) =>
                conversion.to_token_stream(),
            ToConversionPresentation::Enum(conversions) => {
                let obj = DictionaryName::Obj;
                InterfacesMethodExpr::Boxed(
                    DictionaryExpr::Match(quote!(#obj { #conversions, _ => unreachable!("Enum Variant unreachable") }))
                        .to_token_stream())
                    .to_token_stream()
            },
            ToConversionPresentation::Map(to_key_conversion, to_value_conversion) =>
                InterfacesMethodExpr::Boxed(
                    DictionaryExpr::NamedStructInit(
                        Punctuated::from_iter([
                            FieldTypeComposition::named(Name::Dictionary(DictionaryName::Count), FieldTypeConversionKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                            FieldTypeComposition::named(Name::Dictionary(DictionaryName::Keys), FieldTypeConversionKind::Conversion(to_key_conversion.clone())),
                            FieldTypeComposition::named(Name::Dictionary(DictionaryName::Values), FieldTypeConversionKind::Conversion(to_value_conversion.clone())),
                        ]))
                        .to_token_stream())
                    .to_token_stream(),
            ToConversionPresentation::Result(to_ok_conversion, to_error_conversion) => {
                let null_mut = DictionaryExpr::NullMut;
                let expr = DictionaryExpr::Match(quote!(obj { Ok(o) => (#to_ok_conversion, #null_mut), Err(o) => (#null_mut, #to_error_conversion) }));
                InterfacesMethodExpr::Boxed(quote!({
                    let (ok, error) = #expr;
                    Self { ok, error }
                })).to_token_stream()
            },
            ToConversionPresentation::Tuple(conversions) =>
                InterfacesMethodExpr::Boxed(quote!(Self { #conversions })).to_token_stream()
        }.to_tokens(tokens)
    }
}
