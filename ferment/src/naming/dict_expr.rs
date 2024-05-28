use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::CommaPunctuated;
use crate::conversion::FieldTypeConversion;
use crate::naming::DictionaryName;

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum DictionaryExpr {
    // FromPrimitiveArray(TokenStream2, TokenStream2),
    // FromPrimitiveOptArray(TokenStream2, TokenStream2),
    // FromComplexArray(TokenStream2, TokenStream2),
    // FromComplexOptArray(TokenStream2, TokenStream2),
    // MapKeysCloned(TokenStream2),
    // MapValuesCloned(TokenStream2),
    NamedStructInit(CommaPunctuated<FieldTypeConversion>),
    ObjLen,
    ObjIntoIter,
    ObjToVec,
    FfiDeref,
    FfiDerefAsRef,
    LetFfiRef,
    Deref(TokenStream2),
    AsRef(TokenStream2),
    AsMutRef(TokenStream2),
    Mapper(TokenStream2, TokenStream2),
    SelfProp(TokenStream2),
    AsMut_(TokenStream2),
    IfNotNull(TokenStream2, TokenStream2),
    IfThen(TokenStream2, TokenStream2),
    // IfNotNullThen(TokenStream2, TokenStream2),
    // MapOr(TokenStream2, TokenStream2, TokenStream2),
    NullMut,
    CChar,
    AsSlice(TokenStream2),
    FromRawParts(TokenStream2, TokenStream2),
    ToVec(TokenStream2),
    MapCollect(TokenStream2, TokenStream2),
    Match(TokenStream2),
    FromRoot(TokenStream2),
    UnwrapOr(TokenStream2, TokenStream2),
    CountRange,
    Range(TokenStream2),
    NewBox(TokenStream2),
    Add(TokenStream2, TokenStream2),
    CastAs(TokenStream2, TokenStream2),
    CallMethod(TokenStream2, TokenStream2),
}


impl std::fmt::Display for DictionaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl ToTokens for DictionaryExpr {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            Self::ObjLen => {
                let obj = DictionaryName::Obj;
                quote!(#obj.len())
            },
            Self::ObjToVec =>
                Self::ToVec(DictionaryName::Obj.to_token_stream())
                    .to_token_stream(),
            Self::ObjIntoIter => {
                let obj = DictionaryName::Obj;
                quote!(#obj.into_iter())
            },
            Self::FfiDeref =>
                Self::Deref(DictionaryName::Ffi.to_token_stream())
                    .to_token_stream(),
            Self::FfiDerefAsRef =>
                Self::AsRef(Self::FfiDeref.to_token_stream())
                    .to_token_stream(),
            Self::LetFfiRef => {
                let ffi_ref = DictionaryName::FfiRef;
                let ffi_deref = Self::FfiDerefAsRef;
                quote!(let #ffi_ref = #ffi_deref;)
            }
            Self::Deref(expr) =>
                quote!(*#expr),
            Self::AsRef(expr) =>
                quote!(&#expr),
            Self::AsMutRef(expr) =>
                quote!(&mut #expr),
            Self::NamedStructInit(fields) =>
                quote!(Self { #fields }),
            // Self::FromPrimitiveArray(values, count) => {
            //     let let_ffi_ref = Self::LetFfiRef;
            //     let ffi_ref = DictionaryName::FfiRef;
            //     let from_raw = Self::FromRawParts(
            //         quote!(#ffi_ref.#values),
            //         quote!(#ffi_ref.#count));
            //     quote! {
            //         #let_ffi_ref
            //         #from_raw
            //             .try_into()
            //             .expect("Array Length mismatch")
            //     }
            // }
            // Self::FromPrimitiveOptArray(values, count) => {
            //     let let_ffi_ref = Self::LetFfiRef;
            //     let ffi_ref = DictionaryName::FfiRef;
            //     let collector = InterfacesMethodExpr::FromOptPrimitiveGroup(CommaPunctuated::from_iter([
            //         Self::SelfProp(quote!(values)),
            //         Self::SelfProp(quote!(count))]).to_token_stream());
            //     quote! {
            //         #let_ffi_ref
            //         let count = #ffi_ref.#count;
            //         let values = #ffi_ref.#values;
            //         #collector
            //     }
            // }
            // Self::FromComplexArray(values, count) => {
            //     let let_ffi_ref = Self::LetFfiRef;
            //     let ffi_ref = DictionaryName::FfiRef;
            //     let mapper = Self::Mapper(
            //         DictionaryName::I.to_token_stream(),
            //         FFIConversionMethodExpr::FfiFromConst(
            //             Self::Add(
            //                 quote!(*#ffi_ref.#values),
            //                 DictionaryName::I.to_token_stream())
            //                 .to_token_stream())
            //             .to_token_stream());
            //
            //     quote! {
            //         #let_ffi_ref
            //         (0..#ffi_ref.#count)
            //             .into_iter()
            //             .map(#mapper)
            //             .collect::<Vec<_>>()
            //             .try_into()
            //             .expect("Array Length mismatch")
            //     }
            // }
            // Self::FromComplexOptArray(values, count) => {
            //     let let_ffi_ref = Self::LetFfiRef;
            //     let ffi_ref = DictionaryName::FfiRef;
            //     let mapper = Self::Mapper(
            //         DictionaryName::I.to_token_stream(),
            //         FFIConversionMethodExpr::FfiFromOpt(
            //             Self::Add(
            //                 quote!(*#ffi_ref.#values),
            //                 DictionaryName::I.to_token_stream())
            //                 .to_token_stream())
            //             .to_token_stream());
            //
            //     quote! {
            //         #let_ffi_ref
            //         (0..#ffi_ref.#count)
            //             .into_iter()
            //             .map(#mapper)
            //             .collect::<Vec<_>>()
            //             .try_into()
            //             .expect("Array Length mismatch")
            //     }
            // }
            Self::Mapper(context, expr) =>
                quote!(|#context| #expr),
            // Self::MapKeysCloned(field_name) =>
            //     quote!(#field_name.keys().cloned()),
            // Self::MapValuesCloned(field_name) =>
            //     quote!(#field_name.values().cloned()),
            Self::SelfProp(prop) =>
                quote!(self.#prop),
            Self::AsMut_(field_path) =>
                quote!(#field_path as *mut _),
            Self::IfNotNull(condition, expr) =>
                quote!(if (!(#condition).is_null()) { #expr }),
            // Self::IfNotNullThen(condition, expr) =>
            //     Self::IfThen(quote!((!(#condition).is_null())), expr.clone())
            //         .to_token_stream(),
            Self::IfThen(condition, expr) =>
                quote!(#condition.then(|| #expr)),
            // Self::MapOr(condition, def, mapper) =>
            //     quote!(#condition.map_or(#def, #mapper)),
            Self::NullMut =>
                quote!(std::ptr::null_mut()),
            Self::CChar =>
                quote!(std::os::raw::c_char),
            Self::AsSlice(expr) =>
                quote!(#expr.as_slice()),
            Self::FromRawParts(data, len) =>
                quote!(std::slice::from_raw_parts(#data, #len)),
            Self::MapCollect(iter, mapper) =>
                quote!(#iter.map(#mapper).collect()),
            Self::ToVec(expr) =>
                quote!(#expr.to_vec()),
            Self::Match(expr) =>
                quote!(match #expr),
            Self::FromRoot(conversion) => {
                let let_ffi_ref = Self::LetFfiRef;
                quote!(#let_ffi_ref #conversion)
            },
            Self::UnwrapOr(field_path, def) =>
                quote!(#field_path.unwrap_or(#def)),
            Self::CountRange =>
                Self::Range(DictionaryName::Count.to_token_stream())
                    .to_token_stream(),
            Self::Range(expr) =>
                quote!((0..#expr)),
            Self::NewBox(conversion) =>
                quote!(Box::new(#conversion)),
            Self::Add(field_path, index) =>
                quote!(#field_path.add(#index)),
            Self::CastAs(ty, as_ty) =>
                quote!(<#ty as #as_ty>),
            Self::CallMethod(ns, args) =>
                quote!(#ns(#args))

        }.to_tokens(dst)
    }
}