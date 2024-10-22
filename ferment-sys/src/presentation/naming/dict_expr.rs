use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Pat;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::presentation::{ArgPresentation, DictionaryName, InterfacesMethodExpr};

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum DictionaryExpr {
    Simple(TokenStream2),
    DictionaryName(DictionaryName),
    Depunctuated(Depunctuated<TokenStream2>),
    SelfDestructuring(TokenStream2),
    BoxedSelfDestructuring(TokenStream2),
    ObjLen,
    ObjIntoIter,
    ObjToVec,
    FfiDeref,
    FfiDerefAsRef,
    LetFfiRef,
    LetExpr(TokenStream2, TokenStream2),
    Deref(TokenStream2),
    DerefRef(TokenStream2),
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
    ToVec(TokenStream2),
    MapCollect(TokenStream2, TokenStream2),
    Match(TokenStream2),
    MatchFields(TokenStream2, CommaPunctuated<ArgPresentation>),
    #[cfg(feature = "objc")]
    SwitchFields(TokenStream2, Depunctuated<crate::lang::objc::presentable::ArgPresentation>),
    #[cfg(feature = "objc")]
    Case(TokenStream2, TokenStream2),
    MatchResult(TokenStream2, TokenStream2),
    FromRoot(TokenStream2),
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
    CastedFFIConversionMethod(TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2),
    CastedFFIConversionFrom(TokenStream2, TokenStream2, TokenStream2),
    CastedFFIConversionFromOpt(TokenStream2, TokenStream2, TokenStream2),
    CastedFFIConversionDestroy(TokenStream2, TokenStream2, TokenStream2),
    Clone(TokenStream2),
    FromPtrClone(TokenStream2),
    SelfAsTrait(TokenStream2, TokenStream2),

}


impl std::fmt::Display for DictionaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl ToTokens for DictionaryExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Simple(simple) =>
                simple.to_tokens(tokens),
            Self::DictionaryName(name) =>
                name.to_tokens(tokens),
            Self::Depunctuated(sequence) =>
                sequence.to_tokens(tokens),
            Self::ObjLen => {
                DictionaryName::Obj.to_tokens(tokens);
                quote!(.len()).to_tokens(tokens);
            },
            Self::ObjToVec => {
                DictionaryName::Obj.to_tokens(tokens);
                quote!(.to_vec()).to_tokens(tokens);
            }
            Self::ObjIntoIter => {
                DictionaryName::Obj.to_tokens(tokens);
                quote!(.into_iter()).to_tokens(tokens)
            },
            Self::FfiDeref => {
                quote!(*).to_tokens(tokens);
                DictionaryName::Ffi.to_tokens(tokens);
            },
            Self::FfiDerefAsRef => {
                quote!(&*).to_tokens(tokens);
                DictionaryName::Ffi.to_tokens(tokens);
            },
            Self::LetExpr(left, right) =>
                quote!(let #left = #right).to_tokens(tokens),
            Self::LetFfiRef =>
                quote!(let ffi_ref = &*ffi;).to_tokens(tokens),
            Self::Deref(expr) => {
                quote!(*).to_tokens(tokens);
                expr.to_tokens(tokens);
            }
            Self::DerefRef(expr) => {
                quote!(&*).to_tokens(tokens);
                expr.to_tokens(tokens)
            }
            Self::AsRef(expr) => {
                quote!(&).to_tokens(tokens);
                expr.to_tokens(tokens);
            }
            Self::AsMutRef(expr) => {
                quote!(&mut).to_tokens(tokens);
                expr.to_tokens(tokens);
            }
            Self::Mapper(context, expr) => {
                quote!(|#context| ).to_tokens(tokens);
                expr.to_tokens(tokens);
            }
            Self::SelfProp(prop) => {
                quote!(self.).to_tokens(tokens);
                prop.to_tokens(tokens);
            },
            Self::AsMut_(field_path) => {
                field_path.to_tokens(tokens);
                quote!( as *mut _).to_tokens(tokens);
            },
            Self::IfNotNull(condition, expr) => {
                quote!(if (!(#condition).is_null()) { #expr }).to_tokens(tokens);
            },
            Self::IfThen(condition, expr) => {
                condition.to_tokens(tokens);
                quote!(.then(|| #expr)).to_tokens(tokens);
            }
            Self::MapOr(condition, def, mapper) => {
                condition.to_tokens(tokens);
                quote!(.map_or(#def, #mapper)).to_tokens(tokens);
            }
            Self::NullMut =>
                quote!(std::ptr::null_mut()).to_tokens(tokens),
            Self::CChar =>
                quote!(std::os::raw::c_char).to_tokens(tokens),
            Self::AsSlice(expr) => {
                expr.to_tokens(tokens);
                quote!(.as_slice()).to_tokens(tokens);
            },
            // Self::FromRawParts(data, len) =>
            //     quote!(std::slice::from_raw_parts(#data, #len)).to_tokens(tokens),
            Self::MapCollect(iter, mapper) => {
                iter.to_tokens(tokens);
                quote!(.map(#mapper).collect()).to_tokens(tokens);
            },
            Self::ToVec(expr) => {
                expr.to_tokens(tokens);
                quote!(.to_vec()).to_tokens(tokens);
            },
            Self::Match(expr) => {
                quote!(match ).to_tokens(tokens);
                expr.to_tokens(tokens);
            },
            Self::MatchFields(expr, sequences) => {
                Self::Match(quote!(#expr { #sequences }))
                    .to_tokens(tokens)
            },
            #[cfg(feature = "objc")]
            Self::SwitchFields(expr, sequences) => {
                quote!(switch (#expr) { #sequences }).to_tokens(tokens);
            },
            #[cfg(feature = "objc")]
            Self::Case(l_value, r_value) => {
                let case = quote! {
                    case #l_value: {
                        #r_value
                    }
                };
                case.to_tokens(tokens);
            },
            Self::MatchResult(to_ok_conversion, to_error_conversion) => {
                let null_mut = DictionaryExpr::NullMut;
                let field_path = DictionaryName::Obj;
                let arg_path = DictionaryName::O;
                Self::MatchFields(field_path.to_token_stream(), CommaPunctuated::from_iter([
                    ArgPresentation::arm(&vec![], Pat::Verbatim(quote!(Ok(#arg_path))), quote!((#to_ok_conversion, #null_mut))),
                    ArgPresentation::arm(&vec![], Pat::Verbatim(quote!(Err(#arg_path))), quote!((#null_mut, #to_error_conversion))),
                ])).to_tokens(tokens)
            },
            Self::FromRoot(conversion) => {
                Self::LetFfiRef.to_tokens(tokens);
                conversion.to_tokens(tokens);
            },
            Self::CountRange =>
                quote!((0..count)).to_tokens(tokens),
            Self::Range(expr) =>
                quote!((0..#expr)).to_tokens(tokens),
            Self::NewBox(conversion) =>
                quote!(Box::new(#conversion)).to_tokens(tokens),
            Self::MapIntoBox(conversion) => {
                conversion.to_tokens(tokens);
                quote!(.map(Box::new)).to_tokens(tokens);
            },
            Self::FromRawBox(conversion) =>
                quote!(Box::from_raw(#conversion)).to_tokens(tokens),
            Self::Add(field_path, index) => {
                field_path.to_tokens(tokens);
                quote!(.add(#index));
            }
            Self::CastAs(ty, as_ty) =>
                quote!(<#ty as #as_ty>).to_tokens(tokens),
            Self::CallMethod(ns, args) => {
                ns.to_tokens(tokens);
                quote!((#args)).to_tokens(tokens)
            }
            Self::Clone(expr) =>
                quote!(#expr.clone()).to_tokens(tokens),

            Self::FromPtrClone(expr) =>
                quote!((&*#expr).clone()).to_tokens(tokens),
            Self::SelfAsTrait(self_ty, acc) =>
                quote!(*((*self_).object as *#acc #self_ty)).to_tokens(tokens),

            Self::SelfDestructuring(content) =>
                quote!(Self { #content }).to_tokens(tokens),
            Self::TryIntoUnwrap(expr) => {
                expr.to_tokens(tokens);
                quote!(.try_into().unwrap()).to_tokens(tokens)
            }
            Self::CallbackCaller(args_to_conversion, post_processing) => {
                quote!(let ffi_result = (self.caller)(#args_to_conversion);).to_tokens(tokens);
                post_processing.to_tokens(tokens);
            },
            Self::CallbackDestructor(result_conversion, ffi_result) =>
                quote!(
                    let result = #result_conversion;
                    (self.destructor)(#ffi_result);
                    result
                ).to_tokens(tokens),
            Self::CastedFFIConversionMethod(interface, method, ffi_type, target_type, expr) =>
                quote!(<#ffi_type as #interface<#target_type>>::#method(#expr)).to_tokens(tokens),
            Self::CastedFFIConversionFrom(ffi_type, target_type, expr) =>
                quote!(<#ffi_type as ferment::FFIConversionFrom<#target_type>>::ffi_from(#expr)).to_tokens(tokens),
            Self::CastedFFIConversionFromOpt(ffi_type, target_type, expr) =>
                quote!(<#ffi_type as ferment::FFIConversionFrom<#target_type>>::ffi_from_opt(#expr)).to_tokens(tokens),
            Self::CastedFFIConversionDestroy(ffi_type, target_type, expr) => {
                quote!(<#ffi_type as ferment::FFIConversionDestroy<#target_type>>::destroy(#expr)).to_tokens(tokens)
            }
            Self::BoxedSelfDestructuring(expr) =>
                InterfacesMethodExpr::Boxed(DictionaryExpr::SelfDestructuring(expr.to_token_stream()).to_token_stream()).to_tokens(tokens),

        }
    }
}