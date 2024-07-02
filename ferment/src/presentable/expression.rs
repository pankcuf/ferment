use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use ferment_macro::Display;
use crate::ast::CommaPunctuated;
use crate::composable::FieldComposer;
use crate::context::ScopeContext;
use crate::ext::Terminated;
use crate::presentable::{ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryExpr, DictionaryName, FFICallbackMethodExpr, FFIConversionMethod, FFIConversionMethodExpr, InterfacesMethodExpr};

#[derive(Clone, Debug, Display)]
#[allow(unused)]
pub enum Expression {
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
    Add(Box<Expression>, TokenStream2),
    To(Box<Expression>),
    ToOpt(Box<Expression>),
    UnwrapOr(Box<Expression>, TokenStream2),
    MapOr(Box<Expression>, Box<Expression>, Box<Expression>),
    OwnerIteratorPresentation(SequenceOutput),
    FromOptPrimitive(Box<Expression>),
    ToVec(Box<Expression>),
    ToPrimitiveGroup(Box<Expression>),
    ToOptPrimitive(Box<Expression>),
    ToOptPrimitiveGroup(Box<Expression>),
    ToComplexGroup(Box<Expression>),
    ToOptComplexGroup(Box<Expression>),
    ToVecPtr,
    SelfAsTrait(TokenStream2),
    ObjFieldName(TokenStream2),
    FieldTypeConversionName(FieldComposer),
    LineTermination,
    UnboxAny(Box<Expression>),
    UnboxAnyTerminated(Box<Expression>),
    DestroyOpt(Box<Expression>),
    DestroyString(Box<Expression>, TokenStream2),
    FromRawParts(TokenStream2),
    From(Box<Expression>),
    IntoBox(Box<Expression>),
    IntoBoxRaw(Box<Expression>),
    CastFrom(Box<Expression>, TokenStream2, TokenStream2),
    CastDestroy(Box<Expression>, TokenStream2, TokenStream2),
    FromOffsetMap,
    FromOpt(Box<Expression>),
    AsRef(Box<Expression>),
    AsMutRef(Box<Expression>),
    AsSlice(Box<Expression>),
    IfThen(Box<Expression>, TokenStream2),
    Named((TokenStream2, Box<Expression>)),
    Deref(TokenStream2),
    DerefContext(Box<Expression>),
    FfiRefWithFieldName(Box<Expression>),
    FfiRefWithConversion(FieldComposer),
    Match(Box<Expression>),
    FromTuple(Box<Expression>, CommaPunctuated<Expression>),
    MapExpression(Box<Expression>, Box<Expression>),
    AsMut_(Box<Expression>)
}

impl ScopeContextPresentable for Expression {
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
            Self::IntoBoxRaw(expr) =>
                Self::DictionaryExpr(DictionaryExpr::FromRawBox(expr.present(source)))
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
                field_type.name.to_token_stream(),
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

