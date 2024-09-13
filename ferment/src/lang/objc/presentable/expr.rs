use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::FROM_OPT_COMPLEX;
use crate::ext::Terminated;
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionDestroyMethod, FFIConversionFromMethod, FFIConversionFromMethodExpr, FFIConversionToMethod, InterfacesMethodExpr};


impl ScopeContextPresentable for Expression<ObjCFermentate, AttrWrapper> {
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
            Self::FFIConversionFromExpr(expr) =>
                expr.to_token_stream(),
            Self::FFIConversionToExpr(expr) =>
                expr.to_token_stream(),
            Self::FFIConversionDestroyExpr(expr) =>
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
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFrom, presentable.present(source)))
                    .present(source),
            Self::FromOpt(presentable) =>
                FROM_OPT_COMPLEX(presentable.present(source))
                    .present(source),
            Self::FromOptPrimitive(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitive(presentable.present(source)))
                    .present(source),
            Self::To(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiTo, presentable.present(source)))
                    .present(source),
            Self::ToOpt(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiToOpt, presentable.present(source)))
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
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAnyOpt(presentable.present(source)))
                    .present(source),
            Self::DestroyOptPrimitive(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::DestroyOptPrimitive(presentable.present(source)))
                    .present(source),
            Self::DestroyString(presentable, path) => {
                Self::CastDestroy(
                    presentable.clone(),
                    path.to_token_stream(),
                    DictionaryExpr::CChar.to_token_stream())
                    .present(source)
            },
            Self::CastFrom(presentable, ty, ffi_ty) => {
                let field_path = presentable.present(source);
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceFrom;
                quote!(<#ffi_ty as #package::#interface<#ty>>::ffi_from(#field_path))
            }
            Self::CastDestroy(args, ty, ffi_ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceDestroy;
                let method = FFIConversionDestroyMethod::Destroy;
                DictionaryExpr::CallMethod(
                    quote!(<#ffi_ty as #package::#interface<#ty>>::#method),
                    args.present(source))
                    .to_token_stream()
            }
            Self::FromOffsetMap =>
                DictionaryExpr::MapCollect(
                    DictionaryExpr::CountRange.to_token_stream(),
                    DictionaryExpr::Mapper(
                        DictionaryName::I.to_token_stream(),
                        FFIConversionFromMethodExpr::FfiFrom(DictionaryExpr::Add(quote!(*values), DictionaryName::I.to_token_stream()).to_token_stream()).to_token_stream())
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
            Self::NamedComposer((l_value, composer)) => {
                let expression = composer.compose(source);
                Self::Named((l_value.clone(), expression.into())).present(source)
            },
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
            Self::MapIntoBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::MapIntoBox(expr.present(source)))
                    .present(source),
            Self::FromRawBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::FromRawBox(expr.present(source)))
                    .present(source),

            Self::AsMut_(expr) =>
                Self::DictionaryExpr(DictionaryExpr::AsMut_(expr.present(source)))
                    .present(source),
            Self::DerefName(name) =>
                Self::Deref(name.to_token_stream())
                    .present(source),
            Self::Deref(field_name) =>
                Self::DictionaryExpr(DictionaryExpr::Deref(field_name.clone()))
                    .present(source),
            Self::DerefContext(presentable) =>
                Self::Deref(presentable.present(source)).present(source),
            Self::ObjFieldName(field_name) =>
                quote!(obj.#field_name),
            Self::ObjName(name) =>
                quote!(obj.#name),
            Self::FfiRefWithName(name) => {
                quote!(ffi_ref.#name)
            }
            Self::FfiRefWithTokenizedName(field_name) => {
                quote!(ffi_ref.#field_name)
            }
            Self::FieldPath(chunks) => chunks.to_token_stream(),
            Self::FromTuple(presentable, items) => {
                let root_path = presentable.present(source);
                let items = items.present(source);
                quote!({ let ffi_ref = &*#root_path; (#items) })
            }
            Self::Name(name) => name
                .to_token_stream(),
            Self::ConversionType(expr) => {
                expr.compose(source)
                    .present(source)
            }
            Self::Terminated(expr) => {
                expr.compose(source)
                    .present(source)
                    .terminated()
            }
            Self::FromLambda(field_path, lambda_args) => {
                let field_path = field_path.present(source);
                quote!(move |#lambda_args| unsafe { #field_path.call(#lambda_args) })
                // quote!(move |#lambda_args| unsafe { (&*#field_path).call(#lambda_args) })
            }
            Self::FromPtr(field_path) => {
                let field_path = field_path.present(source);
                quote!(*#field_path)
            }
            Self::FromPtrClone(field_path) => {
                let field_path = field_path.present(source);
                quote!((&*#field_path).clone())
            }
            Self::Expr(expr) =>
                expr.to_token_stream(),
            Self::Boxed(expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::Boxed(expr.present(source)))
                    .present(source),
            Self::Clone(expr) => {
                let expr = expr.present(source);
                quote! { #expr.clone() }
            }
        }
    }
}
