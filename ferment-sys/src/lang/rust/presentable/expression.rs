use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::CommaPunctuated;
use crate::composer::{FFIAspect, SourceComposable};
use crate::context::ScopeContext;
use crate::ext::Terminated;
use crate::lang::RustSpecification;
use crate::presentable::{ConversionAspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionFromMethod, FFIConversionToMethod, InterfacesMethod, InterfacesMethodExpr};

impl ScopeContextPresentable for Expression<RustSpecification> {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Simple(expr) |
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::Primitive }, expr) |
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::Primitive }, expr) =>
                expr.to_token_stream(),
            Self::Empty |
            Self::ConversionExprTokens(ConversionAspect { kind: ConversionExpressionKind::Primitive, .. }, _) =>
                quote!(),
            Self::SimpleExpr(expr) =>
                expr.present(source),
            Self::DictionaryName(expr) =>
                expr.to_token_stream(),
            Self::DictionaryExpr(expr) =>
                expr.to_token_stream(),
            Self::InterfacesExpr(expr) =>
                expr.to_token_stream(),
            Self::Name(name) =>
                name.to_token_stream(),

            Self::ObjName(name) =>
                quote!(obj.#name),

            Self::FfiRefWithName(name) =>
                DictionaryExpr::ffi_ref_prop(name)
                    .to_token_stream(),
            Self::AsRef(expr) =>
                DictionaryExpr::AsRef(expr.present(source))
                    .to_token_stream(),
            Self::DerefRef(expr) =>
                DictionaryExpr::DerefRef(expr.present(source))
                    .to_token_stream(),
            Self::DerefMutRef(expr) =>
                DictionaryExpr::DerefMutRef(expr.present(source))
                    .to_token_stream(),
            Self::LeakBox(expr) =>
                DictionaryExpr::LeakBox(expr.present(source))
                    .to_token_stream(),
            Self::AsMutRef(expr) =>
                DictionaryExpr::AsMutRef(expr.present(source))
                    .to_token_stream(),
            Self::Clone(expr) =>
                DictionaryExpr::Clone(expr.present(source))
                    .to_token_stream(),
            Self::FromPtrRead(expr) =>
                DictionaryExpr::FromPtrRead(expr.present(source))
                    .to_token_stream(),
            Self::DerefExpr(expr) =>
                DictionaryExpr::Deref(expr.present(source))
                    .to_token_stream(),
            Self::MapExpr(expr, mapper) =>
                DictionaryExpr::Mapper(expr.present(source), mapper.present(source))
                    .to_token_stream(),
            Self::MapIntoBox(expr) =>
                DictionaryExpr::MapIntoBox(expr.present(source))
                    .to_token_stream(),
            Self::FromRawBox(expr) =>
                DictionaryExpr::FromRawBox(expr.present(source))
                    .to_token_stream(),

            Self::Boxed(expr) =>
                InterfacesMethodExpr::Boxed(expr.present(source))
                    .to_token_stream(),
            Self::DestroyString(expr, _ty) =>
                InterfacesMethodExpr::UnboxString(expr.present(source))
                    .to_token_stream(),
            Self::DestroyBigInt(expr, _target_ty, _ffi_ty) =>
                InterfacesMethodExpr::UnboxAnyOpt(expr.present(source))
                    .to_token_stream(),

            Self::Named((l_value, expr)) => {
                let ty = expr.present(source);
                quote!(#l_value: #ty)
            }
            Self::NamedComposer((l_value, composer)) =>
                Self::Named((l_value.to_owned(), composer.compose(source).into()))
                    .present(source),

            Self::ConversionType(expr) =>
                expr.compose(source)
                    .present(source),
            Self::Terminated(expr) =>
                expr.compose(source)
                    .present(source)
                    .to_token_stream()
                    .terminated(),
            Self::FromLambda(expr, lambda_args) =>
                Self::FromLambdaTokens(expr.present(source), lambda_args.to_owned())
                    .present(source),
            Self::FromLambdaTokens(field_path, lambda_args) =>
                quote!(move |#lambda_args| unsafe { #field_path.call(#lambda_args) }),

            Self::NewSmth(expr, smth) => {
                let expr = expr.present(source);
                quote!(#smth::new(#expr))
            },
            Self::NewCow(expr) => {
                let expr = expr.present(source);
                quote!(std::borrow::Cow::Owned(#expr))
            },
            Self::CowIntoOwned(expr) => {
                let expr = expr.present(source);
                quote!(#expr.into_owned())
            },
            Self::DestroyStringGroup(expr) =>
                InterfacesMethodExpr::UnboxGroup(CommaPunctuated::from_iter([expr.present(source), InterfacesMethod::UnboxString.full_path()]))
                    .to_token_stream(),

            Self::CastConversionExpr(aspect, expr, target_type, ffi_type) =>
                Self::CastConversionExprTokens(*aspect, expr.present(source), target_type.to_owned(), ffi_type.to_owned())
                    .present(source),

            Self::ConversionExpr(aspect, expr) =>
                Self::ConversionExprTokens(*aspect, expr.present(source))
                    .present(source),

            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveOpt }, expr) =>
                InterfacesMethodExpr::FromOptPrimitive(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::OpaqueOpt }, expr) =>
                InterfacesMethodExpr::FromOptOpaque(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveGroup }, expr) =>
                InterfacesMethodExpr::FromPrimitiveGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveOptGroup }, expr) =>
                InterfacesMethodExpr::FromOptPrimitiveGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::OpaqueOptGroup }, expr) =>
                InterfacesMethodExpr::FromOptOpaqueGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::OpaqueGroup }, expr) =>
                InterfacesMethodExpr::FromOpaqueGroup(expr.to_token_stream())
                    .to_token_stream(),

            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::Complex }, expr) =>
                InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::Mut, expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexOpt }, expr) =>
                InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::Opt, expr.to_token_stream())
                    .to_token_stream(),

            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexGroup }, expr) =>
                InterfacesMethodExpr::FromComplexGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexOptGroup }, expr) =>
                InterfacesMethodExpr::FromOptComplexGroup(expr.to_token_stream())
                    .to_token_stream(),

            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveOpt }, expr) =>
                InterfacesMethodExpr::ToOptPrimitive(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::OpaqueOpt }, expr) =>
                InterfacesMethodExpr::ToOptOpaque(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveGroup }, expr) =>
                InterfacesMethodExpr::ToPrimitiveGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveOptGroup }, expr) =>
                InterfacesMethodExpr::ToOptPrimitiveGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::OpaqueOptGroup }, expr) =>
                InterfacesMethodExpr::ToOptOpaqueGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::OpaqueGroup }, expr) =>
                InterfacesMethodExpr::ToOpaqueGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::Complex }, expr) =>
                InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::Mut, expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexOpt }, expr) =>
                InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::Opt, expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexGroup }, expr) =>
                InterfacesMethodExpr::ToComplexGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexOptGroup }, expr) =>
                InterfacesMethodExpr::ToOptComplexGroup(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::PrimitiveOpt }, expr) =>
                InterfacesMethodExpr::DestroyOptPrimitive(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::Complex }, expr) =>
                InterfacesMethodExpr::UnboxAny(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::PrimitiveGroup }, expr) =>
                InterfacesMethodExpr::UnboxVecPtr(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::ComplexGroup }, expr) =>
                InterfacesMethodExpr::UnboxAnyVecPtr(expr.to_token_stream())
                    .to_token_stream(),
            Self::ConversionExprTokens(.., expr) =>
                InterfacesMethodExpr::UnboxAnyOpt(expr.to_token_stream())
                    .to_token_stream(),

            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::Complex }, expr, ffi_ty, ty) =>
                DictionaryExpr::casted_ffi_conversion(DictionaryName::InterfaceFrom, FFIConversionFromMethod::Mut, ffi_ty, ty, expr).to_token_stream(),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexOpt }, expr, ffi_ty, ty) =>
                DictionaryExpr::casted_ffi_conversion(DictionaryName::InterfaceFrom, FFIConversionFromMethod::Opt, ffi_ty, ty, expr).to_token_stream(),

            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::Complex }, expr, ffi_ty, ty) =>
                DictionaryExpr::casted_ffi_conversion(DictionaryName::InterfaceTo, FFIConversionToMethod::Mut, ffi_ty, ty, expr).to_token_stream(),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexOpt }, expr, ffi_ty, ty) =>
                DictionaryExpr::casted_ffi_conversion(DictionaryName::InterfaceTo, FFIConversionToMethod::Opt, ffi_ty, ty, expr).to_token_stream(),

            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::Complex }, expr, ..) =>
                InterfacesMethodExpr::UnboxAny(expr.to_token_stream()).to_token_stream(),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::ComplexOpt }, expr, ..) =>
                InterfacesMethodExpr::UnboxAnyOpt(expr.to_token_stream()).to_token_stream(),

            Self::CastConversionExprTokens(aspect, expr, ..) =>
                Self::ConversionExprTokens(*aspect, expr.to_owned())
                    .present(source),

        }
    }
}

