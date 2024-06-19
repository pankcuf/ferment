use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::CommaPunctuated;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::Terminated;
use crate::naming::{DictionaryExpr, DictionaryName, FFICallbackMethodExpr, FFIConversionMethod, FFIConversionMethodExpr, InterfacesMethodExpr};
use crate::presentation::context::OwnerIteratorPresentationContext;
use crate::presentation::ScopeContextPresentable;

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum FieldContext {
    Empty,
    O,
    Obj,
    Self_,
    Simple(TokenStream2),
    DictionaryName(DictionaryName),
    DictionaryExpr(DictionaryExpr),
    FFIConversionExpr(FFIConversionMethodExpr),
    FFICallbackExpr(FFICallbackMethodExpr),
    InterfacesExpr(InterfacesMethodExpr),
    Add(Box<FieldContext>, TokenStream2),
    To(Box<FieldContext>),
    ToOpt(Box<FieldContext>),
    UnwrapOr(Box<FieldContext>, TokenStream2),
    MapOr(Box<FieldContext>, Box<FieldContext>, Box<FieldContext>),
    OwnerIteratorPresentation(OwnerIteratorPresentationContext),
    FromOptPrimitive(Box<FieldContext>),
    ToVec(Box<FieldContext>),
    ToPrimitiveGroup(Box<FieldContext>),
    ToOptPrimitive(Box<FieldContext>),
    ToOptPrimitiveGroup(Box<FieldContext>),
    ToComplexGroup(Box<FieldContext>),
    ToOptComplexGroup(Box<FieldContext>),
    ToVecPtr,
    SelfAsTrait(TokenStream2),
    ObjFieldName(TokenStream2),
    FieldTypeConversionName(FieldTypeConversion),
    LineTermination,
    UnboxAny(Box<FieldContext>),
    UnboxAnyTerminated(Box<FieldContext>),
    DestroyOpt(Box<FieldContext>),
    // DestroyPrimitiveGroup(TokenStream2),
    // DestroyComplexGroup(Box<FieldContext>),
    DestroyString(Box<FieldContext>, TokenStream2),
    FromRawParts(TokenStream2),
    From(Box<FieldContext>),
    IntoBox(Box<FieldContext>),
    CastFrom(Box<FieldContext>, TokenStream2, TokenStream2),
    CastDestroy(Box<FieldContext>, TokenStream2, TokenStream2),
    FromOffsetMap,
    FromOpt(Box<FieldContext>),
    AsRef(Box<FieldContext>),
    AsMutRef(Box<FieldContext>),
    AsSlice(Box<FieldContext>),
    IfThen(Box<FieldContext>, TokenStream2),
    Named((TokenStream2, Box<FieldContext>)),
    Deref(TokenStream2),
    DerefContext(Box<FieldContext>),
    FfiRefWithFieldName(Box<FieldContext>),
    FfiRefWithConversion(FieldTypeConversion),
    Match(Box<FieldContext>),
    FromTuple(Box<FieldContext>, CommaPunctuated<FieldContext>),
    MapExpression(Box<FieldContext>, Box<FieldContext>),
    AsMut_(Box<FieldContext>)
}

impl Display for FieldContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FieldContext::Empty =>
                "FieldTypePresentableContext::Empty".to_string(),
            FieldContext::DictionaryName(name) =>
                format!("FieldTypePresentableContext::DictionaryName({})", name),
            FieldContext::DictionaryExpr(expr) =>
                format!("FieldTypePresentableContext::DictionaryExpr({})", expr),
            FieldContext::InterfacesExpr(expr) =>
                format!("FieldTypePresentableContext::InterfacesExpr({})", expr.to_token_stream()),
            FieldContext::FFIConversionExpr(expr) =>
                format!("FieldTypePresentableContext::FFIConversionExpr({})", expr.to_token_stream()),
            FieldContext::FFICallbackExpr(expr) =>
                format!("FieldTypePresentableContext::FFICallbackExpr({})", expr.to_token_stream()),
            FieldContext::Simple(simple) =>
                format!("FieldTypePresentableContext::Simple({})", quote!(#simple)),
            FieldContext::Add(context, index) =>
                format!("FieldTypePresentableContext::Add({}, {})", context, index),
            FieldContext::FromOptPrimitive(context) =>
                format!("FieldTypePresentableContext::FromOptPrimitive({})", context),
            FieldContext::To(context) =>
                format!("FieldTypePresentableContext::To({})", context),
            FieldContext::ToVec(context) =>
                format!("FieldTypePresentableContext::ToVec({})", context),
            FieldContext::ToPrimitiveGroup(context) =>
                format!("FieldTypePresentableContext::ToPrimitiveGroup({})", context),
            FieldContext::ToOptPrimitive(context) =>
                format!("FieldTypePresentableContext::ToOptPrimitive({})", context),
            FieldContext::ToOptPrimitiveGroup(context) =>
                format!("FieldTypePresentableContext::ToOptPrimitiveGroup({})", context),
            FieldContext::ToComplexGroup(context) =>
                format!("FieldTypePresentableContext::ToComplexGroup({})", context),
            FieldContext::ToOptComplexGroup(context) =>
                format!("FieldTypePresentableContext::ToOptComplexGroup({})", context),
            FieldContext::ToOpt(context) =>
                format!("FieldTypePresentableContext::ToOpt({})", context),
            FieldContext::UnwrapOr(context, or) =>
                format!("FieldTypePresentableContext::UnwrapOr({}, {})", context, or),
            FieldContext::MapOr(condition, def, mapper) =>
                format!("FieldTypePresentableContext::MapOr({}, {}, {})", condition, def, mapper),
            FieldContext::OwnerIteratorPresentation(context) =>
                format!("FieldTypePresentableContext::OwnerIteratorPresentation({})", context),
            FieldContext::ToVecPtr =>
                "FieldTypePresentableContext::ToVecPtr".to_string(),
            FieldContext::Obj =>
                "FieldTypePresentableContext::Obj".to_string(),
            FieldContext::Self_ =>
                "FieldTypePresentableContext::Self_".to_string(),
            FieldContext::SelfAsTrait(trait_ty) =>
                format!("FieldTypePresentableContext::SelfAsTrait({})", trait_ty),
            FieldContext::ObjFieldName(field_name) =>
                format!("FieldTypePresentableContext::ObjFieldName({})", field_name),
            FieldContext::FieldTypeConversionName(conversion) =>
                format!("FieldTypePresentableContext::ObjFieldName({})", conversion),
            FieldContext::LineTermination =>
                "FieldTypePresentableContext::LineTermination".to_string(),
            FieldContext::UnboxAny(context) =>
                format!("FieldTypePresentableContext::UnboxAny({})", context),
            FieldContext::UnboxAnyTerminated(context) =>
                format!("FieldTypePresentableContext::UnboxAnyTerminated({})", context),
            FieldContext::DestroyOpt(context) =>
                format!("FieldTypePresentableContext::DestroyOpt({})", context),
            FieldContext::DestroyString(context, smth) =>
                format!("FieldTypePresentableContext::DestroyString({}, {})", context, smth),
            FieldContext::FromRawParts(context) =>
                format!("FieldTypePresentableContext::FromRawParts({})", context),
            FieldContext::From(context) =>
                format!("FieldTypePresentableContext::From({})", context),
            FieldContext::IntoBox(context) =>
                format!("FieldTypePresentableContext::IntoBox({})", context),
            FieldContext::CastFrom(context, ty, ffi_ty) =>
                format!("FieldTypePresentableContext::CastFrom({}, {}, {})", context, ty, ffi_ty),
            FieldContext::CastDestroy(context, ty, ffi_ty) =>
                format!("FieldTypePresentableContext::CastDestroy({}, {}, {})", context, ty, ffi_ty),
            FieldContext::FromOffsetMap =>
                "FieldTypePresentableContext::FromOffsetMap".to_string(),
            FieldContext::FromOpt(context) =>
                format!("FieldTypePresentableContext::FromOpt({})", context),
            FieldContext::AsRef(context) =>
                format!("FieldTypePresentableContext::AsRef({})", context),
            FieldContext::AsMutRef(context) =>
                format!("FieldTypePresentableContext::AsMutRef({})", context),
            FieldContext::AsSlice(context) =>
                format!("FieldTypePresentableContext::AsSlice({})", context),
            FieldContext::IfThen(context, statement) =>
                format!("FieldTypePresentableContext::IfThen({}, {})", context, statement),
            FieldContext::Named((ff, context)) =>
                format!("FieldTypePresentableContext::Named(({}, {}))", ff, context),
            FieldContext::Deref(context) =>
                format!("FieldTypePresentableContext::Deref({})", context),
            FieldContext::DerefContext(context) =>
                format!("FieldTypePresentableContext::DerefContext({})", context),
            FieldContext::FfiRefWithFieldName(context) =>
                format!("FieldTypePresentableContext::FfiRefWithFieldName({})", context),
            FieldContext::FfiRefWithConversion(context) =>
                format!("FieldTypePresentableContext::FfiRefWithConversion({})", context),
            FieldContext::Match(context) =>
                format!("FieldTypePresentableContext::Match({})", context),
            FieldContext::FromTuple(context, items) =>
                format!("FieldTypePresentableContext::FromTuple({}, [{}])", context, items.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")),
            FieldContext::MapExpression(context, mapper) =>
                format!("FieldTypePresentableContext::MapExpression({}, {})", context, mapper),
            FieldContext::O =>
                "FieldTypePresentableContext::O".to_string(),
            FieldContext::AsMut_(context) =>
                format!("FieldTypePresentableContext::AsMut_({})", context),
        }.as_str())
    }
}

impl ScopeContextPresentable for FieldContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> TokenStream2 {
        match self {
            Self::Empty => quote!(),
            Self::O =>
                DictionaryName::O.to_token_stream(),
            Self::Obj =>
                DictionaryName::Obj.to_token_stream(),
            Self::Self_ =>
                DictionaryName::Self_.to_token_stream(),
            Self::LineTermination => quote!(;),
            Self::Simple(expr) =>
                expr.to_token_stream(),
            Self::DictionaryName(expr) =>
                expr.to_token_stream(),
            Self::DictionaryExpr(expr) =>
                expr.to_token_stream(),
            Self::InterfacesExpr(expr) =>
                expr.to_token_stream(),
            Self::FFIConversionExpr(expr) =>
                expr.to_token_stream(),
            Self::FFICallbackExpr(expr) =>
                expr.to_token_stream(),
            Self::OwnerIteratorPresentation(presentable) =>
                presentable.present(source),
            Self::MapExpression(presentable, mapper) =>
                DictionaryExpr::Mapper(
                    presentable.present(source),
                    mapper.present(source))
                    .to_token_stream(),
            Self::From(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiFrom, presentable.present(source)))
                    .present(source),
            Self::FromOpt(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiFromOpt, presentable.present(source)))
                    .present(source),
            Self::FromOptPrimitive(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitive(presentable.present(source)))
                    .present(source),
            Self::To(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiTo, presentable.present(source)))
                    .present(source),
            Self::ToOpt(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiToOpt, presentable.present(source)))
                    .present(source),
            Self::ToPrimitiveGroup(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToPrimitiveGroup(presentable.present(source)))
                    .present(source),
            Self::ToOptPrimitive(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitive(presentable.present(source)))
                    .present(source),
            Self::ToOptPrimitiveGroup(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitiveGroup(presentable.present(source)))
                    .present(source),
            Self::ToComplexGroup(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToComplexGroup(presentable.present(source)))
                    .present(source),
            Self::ToOptComplexGroup(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptComplexGroup(presentable.present(source)))
                    .present(source),
            Self::ToVecPtr =>
                Self::InterfacesExpr(InterfacesMethodExpr::BoxedVec(
                    DictionaryExpr::MapCollect(
                        DictionaryName::Obj.to_token_stream(),
                        Self::MapExpression(
                            Self::DictionaryName(DictionaryName::O).into(),
                            Self::InterfacesExpr(InterfacesMethodExpr::Boxed(DictionaryName::O.to_token_stream())).into())
                            .present(source))
                        .to_token_stream()))
                    .present(source),
            Self::UnboxAny(presentable) =>

                // Self::FFIConversionExpr(FFIConversionMethodExpr::Destroy(presentable.present(source)))
                //     .present(source),
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAny(presentable.present(source)))
                    .present(source),
            Self::UnboxAnyTerminated(presentable) =>
                Self::UnboxAny(presentable.clone())
                    .present(source).terminated(),
            Self::Add(presentable, index) =>
                DictionaryExpr::Add(presentable.present(source), index.clone())
                    .to_token_stream(),
            Self::UnwrapOr(presentable, default) =>
                DictionaryExpr::UnwrapOr(presentable.present(source), default.clone())
                    .to_token_stream(),
            Self::MapOr(condition, def, mapper) =>
                DictionaryExpr::MapOr(
                    condition.present(source).to_token_stream(),
                    def.present(source).to_token_stream(),
                    mapper.present(source).to_token_stream()).to_token_stream(),
            Self::ToVec(presentable) =>
                DictionaryExpr::ToVec(presentable.present(source))
                    .to_token_stream(),
            Self::FromRawParts(field_type) =>
                DictionaryExpr::ToVec(
                    DictionaryExpr::FromRawParts(
                        quote!(values as *const #field_type),
                        DictionaryName::Count.to_token_stream())
                        .to_token_stream())
                    .to_token_stream(),
            Self::DestroyOpt(presentable) =>
                DictionaryExpr::IfNotNull(
                    presentable.present(source),
                    Self::UnboxAnyTerminated(presentable.clone())
                        .present(source))
                    .to_token_stream(),
            Self::DestroyString(presentable, path) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::Interface;
                let field_path = presentable.present(source);
                let cchar = DictionaryExpr::CChar;
                quote!(<#cchar as #package::#interface<#path>>::destroy(#field_path))
            },
            Self::CastFrom(presentable, ty, ffi_ty) => {
                let field_path = presentable.present(source);
                let package = DictionaryName::Package;
                let interface = DictionaryName::Interface;
                quote!(<#ffi_ty as #package::#interface<#ty>>::ffi_from(#field_path))
            }
            Self::CastDestroy(presentable, ty, ffi_ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::Interface;
                let method = FFIConversionMethod::Destroy;
                DictionaryExpr::CallMethod(
                    quote!(<#ffi_ty as #package::#interface<#ty>>::#method),
                    presentable.present(source))
                    .to_token_stream()
            }
            Self::FromOffsetMap =>
                DictionaryExpr::MapCollect(
                    DictionaryExpr::CountRange.to_token_stream(),
                    DictionaryExpr::Mapper(
                        DictionaryName::I.to_token_stream(),
                        FFIConversionMethodExpr::FfiFrom(DictionaryExpr::Add(quote!(*values), DictionaryName::I.to_token_stream()).to_token_stream()).to_token_stream())
                        .to_token_stream())
                    .to_token_stream(),
            Self::AsRef(field_path) =>
                DictionaryExpr::AsRef(field_path.present(source))
                    .to_token_stream(),
            Self::AsMutRef(field_path) =>
                Self::DictionaryExpr(DictionaryExpr::AsMutRef(field_path.present(source)))
                    .present(source),
            Self::IfThen(presentable, condition) => {
                let field_path = presentable.present(source);
                Self::DictionaryExpr(DictionaryExpr::IfThen(quote!((#field_path #condition)), field_path))
                    .present(source)
            }
            Self::Named((l_value, presentable)) => {
                let ty = presentable.present(source);
                quote!(#l_value: #ty)
            }
            Self::FfiRefWithFieldName(presentable) => {
                let field_name = presentable.present(source);
                quote!(ffi_ref.#field_name)
            }
            Self::Match(presentable) =>
                Self::DictionaryExpr(DictionaryExpr::Match(presentable.present(source)))
                    .present(source),
            Self::SelfAsTrait(self_ty) =>
                quote!(*((*self_).object as *const #self_ty)),
            Self::AsSlice(expr) =>
                Self::DictionaryExpr(DictionaryExpr::AsSlice(expr.present(source)))
                    .present(source),
            Self::IntoBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::NewBox(expr.present(source)))
                    .present(source),
            Self::AsMut_(expr) =>
                Self::DictionaryExpr(DictionaryExpr::AsMut_(expr.present(source)))
                    .present(source),
            Self::Deref(field_name) =>
                Self::DictionaryExpr(DictionaryExpr::Deref(field_name.clone()))
                    .present(source),
            Self::DerefContext(presentable) =>
                Self::Deref(presentable.present(source)).present(source),
            Self::ObjFieldName(field_name) =>
                quote!(obj.#field_name),
            Self::FieldTypeConversionName(field_type) =>
                field_type.name(),
            Self::FfiRefWithConversion(field_type) =>
                Self::FfiRefWithFieldName(Self::FieldTypeConversionName(field_type.clone()).into())
                    .present(source),
            Self::FromTuple(presentable, items) => {
                let root_path = presentable.present(source);
                let items = items.present(source);
                quote!({ let ffi_ref = &*#root_path; (#items) })
            }
        }
    }
}

