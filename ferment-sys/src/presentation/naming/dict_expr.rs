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
    ObjIntoIter,
    FfiDeref,
    FfiDerefAsRef,
    LetFfiRef,
    LetExpr(TokenStream2, TokenStream2),
    Deref(TokenStream2),
    DerefRef(TokenStream2),
    DerefMutRef(TokenStream2),
    AsRef(TokenStream2),
    AsMutRef(TokenStream2),
    Mapper(TokenStream2, TokenStream2),
    SelfProp(TokenStream2),
    FfiRefProp(TokenStream2),
    AsMut_(TokenStream2),
    IfNotNull(TokenStream2, TokenStream2),
    IfThen(TokenStream2, TokenStream2),
    MapOr(TokenStream2, TokenStream2, TokenStream2),
    NullMut,
    CChar,
    Arc,
    Rc,
    Box,
    Mutex,
    OnceLock,
    RwLock,
    Cell,
    RefCell,
    UnsafeCell,
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
    NewSmth(TokenStream2, TokenStream2),
    LeakBox(TokenStream2),
    MapIntoBox(TokenStream2),
    FromRawBox(TokenStream2),
    TryIntoUnwrap(TokenStream2),
    CallbackCaller(TokenStream2, TokenStream2),
    CallbackDestructor(TokenStream2, TokenStream2),
    CastedFFIConversionFrom(TokenStream2, TokenStream2, TokenStream2),
    CastedFFIConversionFromOpt(TokenStream2, TokenStream2, TokenStream2),
    Clone(TokenStream2),
    FromPtrRead(TokenStream2),
    FromArc(TokenStream2),
    FromRc(TokenStream2),
    SelfAsTrait(TokenStream2, TokenStream2),
}

impl DictionaryExpr {
    pub fn self_prop<T: ToTokens>(name: T) -> Self {
        Self::SelfProp(name.to_token_stream())
    }
    pub fn self_as_trait<T: ToTokens, U: ToTokens>(ty: T, acc: U) -> Self {
        Self::SelfAsTrait(ty.to_token_stream(), acc.to_token_stream())
    }
    pub fn ffi_ref_prop<T: ToTokens>(name: T) -> Self {
        Self::FfiRefProp(name.to_token_stream())
    }
    pub fn deref_ref<T: ToTokens>(name: T) -> Self {
        Self::DerefRef(name.to_token_stream())
    }
    pub fn self_destruct<T: ToTokens>(name: T) -> Self {
        Self::SelfDestructuring(name.to_token_stream())
    }

    pub fn from_root<T: ToTokens>(body: T) -> Self {
        Self::FromRoot(body.to_token_stream())
    }
    pub fn from_ptr_read<T: ToTokens>(body: T) -> Self {
        Self::FromPtrRead(body.to_token_stream())
    }
    pub fn from_arc<T: ToTokens>(body: T) -> Self {
        Self::FromArc(body.to_token_stream())
    }
    pub fn from_rc<T: ToTokens>(body: T) -> Self {
        Self::FromRc(body.to_token_stream())
    }

    pub fn mapper<T: ToTokens, U: ToTokens>(item: T, result: U) -> Self {
        Self::Mapper(item.to_token_stream(), result.to_token_stream())
    }
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
            Self::DerefMutRef(expr) => {
                quote!(&mut *).to_tokens(tokens);
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
            Self::FfiRefProp(prop) => {
                quote!(ffi_ref.).to_tokens(tokens);
                prop.to_tokens(tokens);
            },
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
            Self::Arc =>
                quote!(std::sync::Arc).to_tokens(tokens),
            Self::Rc =>
                quote!(std::rc::Rc).to_tokens(tokens),
            Self::Box =>
                quote!(Box).to_tokens(tokens),
            Self::Mutex =>
                quote!(std::sync::Mutex).to_tokens(tokens),
            Self::OnceLock =>
                quote!(std::sync::OnceLock).to_tokens(tokens),
            Self::RwLock =>
                quote!(std::sync::RwLock).to_tokens(tokens),
            Self::Cell =>
                quote!(std::cell::Cell).to_tokens(tokens),
            Self::RefCell =>
                quote!(std::cell::RefCell).to_tokens(tokens),
            Self::UnsafeCell =>
                quote!(std::cell::UnsafeCell).to_tokens(tokens),
            Self::AsSlice(expr) => {
                expr.to_tokens(tokens);
                quote!(.as_slice()).to_tokens(tokens);
            },
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
                    ArgPresentation::attr_less_arm(Pat::Verbatim(quote!(Ok(#arg_path))), quote!((#to_ok_conversion, #null_mut))),
                    ArgPresentation::attr_less_arm(Pat::Verbatim(quote!(Err(#arg_path))), quote!((#null_mut, #to_error_conversion))),
                ])).to_tokens(tokens)
            },
            Self::FromRoot(conversion) => {
                Self::LetFfiRef.to_tokens(tokens);
                conversion.to_tokens(tokens);
            },
            Self::CountRange =>
                quote!((0..count)).to_tokens(tokens),
            Self::NewSmth(conversion, smth) =>
                quote!(#smth::new(#conversion)).to_tokens(tokens),
            Self::LeakBox(conversion) =>
                quote!(Box::leak(Box::new(#conversion))).to_tokens(tokens),
            Self::MapIntoBox(conversion) => {
                conversion.to_tokens(tokens);
                quote!(.map(Box::new)).to_tokens(tokens);
            },
            Self::FromRawBox(conversion) =>
                quote!(Box::from_raw(#conversion)).to_tokens(tokens),
            Self::Clone(expr) =>
                quote!(#expr.clone()).to_tokens(tokens),

            Self::FromPtrRead(expr) =>
                quote!(std::ptr::read(#expr)).to_tokens(tokens),
            Self::FromArc(expr) =>
                quote!(std::sync::Arc::clone(#expr)).to_tokens(tokens),
            Self::FromRc(expr) =>
                quote!(std::rc::Rc::clone(#expr)).to_tokens(tokens),
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
            Self::CastedFFIConversionFrom(ffi_type, target_type, expr) =>
                quote!(<#ffi_type as ferment::FFIConversionFrom<#target_type>>::ffi_from(#expr)).to_tokens(tokens),
            Self::CastedFFIConversionFromOpt(ffi_type, target_type, expr) =>
                quote!(<#ffi_type as ferment::FFIConversionFrom<#target_type>>::ffi_from_opt(#expr)).to_tokens(tokens),
            Self::BoxedSelfDestructuring(expr) =>
                InterfacesMethodExpr::Boxed(DictionaryExpr::self_destruct(expr).to_token_stream()).to_tokens(tokens),

        }
    }
}