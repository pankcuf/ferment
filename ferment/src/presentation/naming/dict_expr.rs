use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::Depunctuated;
use crate::ext::Terminated;
use crate::presentation::DictionaryName;

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum DictionaryExpr {
    Simple(TokenStream2),
    Depunctuated(Depunctuated<TokenStream2>),
    SelfDestructuring(TokenStream2),
    ObjLen,
    ObjIntoIter,
    ObjToVec,
    FfiDeref,
    FfiDerefAsRef,
    LetFfiRef,
    LetExpr(TokenStream2, TokenStream2),
    Deref(TokenStream2),
    AsRef(TokenStream2),
    AsMutRef(TokenStream2),
    Mapper(TokenStream2, TokenStream2),
    SelfProp(TokenStream2),
    AsMut_(TokenStream2),
    IfNotNull(TokenStream2, TokenStream2),
    IfThen(TokenStream2, TokenStream2),
    MapOr(TokenStream2, TokenStream2, TokenStream2),
    NullMut,
    CChar,
    AsSlice(TokenStream2),
    FromRawParts(TokenStream2, TokenStream2),
    ToVec(TokenStream2),
    MapCollect(TokenStream2, TokenStream2),
    Match(TokenStream2),
    MatchResult(TokenStream2, TokenStream2),
    FromRoot(TokenStream2),
    UnwrapOr(TokenStream2, TokenStream2),
    CountRange,
    Range(TokenStream2),
    NewBox(TokenStream2),
    MapIntoBox(TokenStream2),
    FromRawBox(TokenStream2),
    Add(TokenStream2, TokenStream2),
    CastAs(TokenStream2, TokenStream2),
    CallMethod(TokenStream2, TokenStream2),
    TryIntoUnwrap(TokenStream2),
    CallbackCaller(TokenStream2, TokenStream2),
    CallbackDestructor(TokenStream2, TokenStream2),
    CastedFFIConversionFrom(TokenStream2, TokenStream2, TokenStream2),
    CastedFFIConversionFromOpt(TokenStream2, TokenStream2, TokenStream2),
    CastedFFIConversionDestroy(TokenStream2, TokenStream2, TokenStream2),
}


impl std::fmt::Display for DictionaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl ToTokens for DictionaryExpr {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            Self::Simple(tokens) =>
                tokens.to_token_stream(),
            Self::Depunctuated(tokens) =>
                tokens.to_token_stream(),
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
            Self::LetExpr(left, right) =>
                quote!(let #left = #right),
            Self::LetFfiRef =>
                Self::LetExpr(
                    DictionaryName::FfiRef.to_token_stream(),
                    Self::FfiDerefAsRef.to_token_stream().terminated())
                    .to_token_stream(),
            Self::Deref(expr) =>
                quote!(*#expr),
            Self::AsRef(expr) =>
                quote!(&#expr),
            Self::AsMutRef(expr) =>
                quote!(&mut #expr),
            Self::Mapper(context, expr) =>
                quote!(|#context| #expr),
            Self::SelfProp(prop) =>
                quote!(self.#prop),
            Self::AsMut_(field_path) =>
                quote!(#field_path as *mut _),
            Self::IfNotNull(condition, expr) =>
                quote!(if (!(#condition).is_null()) { #expr }),
            Self::IfThen(condition, expr) =>
                quote!(#condition.then(|| #expr)),
            Self::MapOr(condition, def, mapper) =>
                quote!(#condition.map_or(#def, #mapper)),
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
            Self::MapIntoBox(conversion) =>
                quote!(#conversion.map(Box::new)),
            Self::FromRawBox(conversion) =>
                quote!(Box::from_raw(#conversion)),
            Self::Add(field_path, index) =>
                quote!(#field_path.add(#index)),
            Self::CastAs(ty, as_ty) =>
                // Expr::Cast(ExprCast {
                //     attrs: vec![],
                //     expr: Box::new(Expr::__NonExhaustive),
                //     as_token: Default::default(),
                //     ty: Box::new(Type::__NonExhaustive),
                // }).to_token_stream()
                quote!(<#ty as #as_ty>),
            Self::CallMethod(ns, args) =>
                quote!(#ns(#args)),
            Self::SelfDestructuring(tokens) =>
                quote!(Self { #tokens }),
            Self::TryIntoUnwrap(expr) =>
                quote!(#expr.try_into().unwrap()),
            Self::MatchResult(to_ok_conversion, to_error_conversion) => {
                let null_mut = DictionaryExpr::NullMut;
                let field_path = DictionaryName::Obj;
                let arg_path = DictionaryName::O;
                Self::Match(quote!(#field_path {
                    Ok(#arg_path) => (#to_ok_conversion, #null_mut),
                    Err(#arg_path) => (#null_mut, #to_error_conversion)
                })).to_token_stream()

                // Expr::Match(ExprMatch {
                //     attrs: vec![],
                //     match_token: Default::default(),
                //     expr: Box::new(Expr::Path(ExprPath {
                //         attrs: vec![],
                //         qself: None,
                //         path: DictionaryName::Obj.to_path(),
                //     })),
                //     brace_token: Default::default(),
                //     arms: vec![
                //         Arm {
                //             attrs: vec![],
                //             pat: Pat::TupleStruct(PatTupleStruct {
                //                 attrs: vec![],
                //                 path: parse_quote!(Ok),
                //                 pat: PatTuple {
                //                     attrs: vec![],
                //                     paren_token: Default::default(),
                //                     elems: CommaPunctuated::from_iter([Pat::Path(DictionaryName::O.)]),
                //                 },
                //             }),
                //             guard: None,
                //             fat_arrow_token: Default::default(),
                //             body: Box::new(Expr::__NonExhaustive),
                //             comma: None,
                //         },
                //         Arm {
                //             attrs: vec![],
                //             pat: Pat::__NonExhaustive,
                //             guard: None,
                //             fat_arrow_token: Default::default(),
                //             body: Box::new(Expr::__NonExhaustive),
                //             comma: None,
                //         },
                //     ],
                // }).to_token_stream()
            },
            Self::CallbackCaller(args_to_conversion, post_processing) => quote! {
                let ffi_result = (self.caller)(#args_to_conversion);
                #post_processing
            },
            Self::CallbackDestructor(result_conversion, ffi_result) => quote! {
                let result = #result_conversion;
                // (self.destructor)(o_0, #ffi_result);
                (self.destructor)(#ffi_result);
                result
            },
            Self::CastedFFIConversionFrom(ffi_type, target_type, expr) =>
                quote!(<#ffi_type as ferment_interfaces::FFIConversionFrom<#target_type>>::ffi_from(#expr)),
            Self::CastedFFIConversionFromOpt(ffi_type, target_type, expr) =>
                quote!(<#ffi_type as ferment_interfaces::FFIConversionFrom<#target_type>>::ffi_from_opt(#expr)),
            Self::CastedFFIConversionDestroy(ffi_type, target_type, expr) => {
                quote!(<#ffi_type as ferment_interfaces::FFIConversionDestroy<#target_type>>::destroy(#expr))
            }
        }.to_tokens(dst)
    }
}