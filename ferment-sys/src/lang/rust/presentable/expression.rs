use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::CommaPunctuated;
use crate::composer::{FFIAspect, SourceComposable};
use crate::context::ScopeContext;
use crate::ext::Terminated;
use crate::lang::RustSpecification;
use crate::presentable::{ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionFromMethod, FFIConversionToMethod, InterfacesMethod, InterfacesMethodExpr};

impl ScopeContextPresentable for Expression<RustSpecification> {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Simple(expr) |
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Primitive, expr) |
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::Primitive, expr) =>
                expr.to_token_stream(),
            Self::Empty |
            Self::ConversionExprTokens(_, ConversionExpressionKind::Primitive, _) =>
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
                Self::DictionaryExpr(DictionaryExpr::ffi_ref_prop(name))
                    .present(source),
            Self::AsRef(expr) =>
                Self::DictionaryExpr(DictionaryExpr::AsRef(expr.present(source)))
                    .present(source),
            Self::DerefRef(expr) =>
                Self::DictionaryExpr(DictionaryExpr::DerefRef(expr.present(source)))
                    .present(source),
            Self::DerefMutRef(expr) =>
                Self::DictionaryExpr(DictionaryExpr::DerefMutRef(expr.present(source)))
                    .present(source),
            Self::LeakBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::LeakBox(expr.present(source)))
                    .present(source),
            Self::AsMutRef(expr) =>
                Self::DictionaryExpr(DictionaryExpr::AsMutRef(expr.present(source)))
                    .present(source),
            Self::Clone(expr) =>
                Self::DictionaryExpr(DictionaryExpr::Clone(expr.present(source)))
                    .present(source),
            Self::FromPtrRead(expr) =>
                Self::DictionaryExpr(DictionaryExpr::FromPtrRead(expr.present(source)))
                    .present(source),
            Self::DerefExpr(expr) =>
                Self::DictionaryExpr(DictionaryExpr::Deref(expr.present(source)))
                    .present(source),
            Self::MapExpression(expr, mapper) =>
                Self::DictionaryExpr(DictionaryExpr::Mapper(expr.present(source), mapper.present(source)))
                    .present(source),
            Self::MapIntoBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::MapIntoBox(expr.present(source)))
                    .present(source),
            Self::FromRawBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::FromRawBox(expr.present(source)))
                    .present(source),

            Self::Boxed(expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::Boxed(expr.present(source)))
                    .present(source),
            Self::DestroyString(expr, _ty) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxString(expr.present(source)))
                    .present(source),
            Self::DestroyBigInt(expr, _target_ty, _ffi_ty) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAnyOpt(expr.present(source)))
                    .present(source),

            Self::Named((l_value, expr)) => {
                let ty = expr.present(source);
                quote!(#l_value: #ty)
            }
            Self::NamedComposer((l_value, composer)) =>
                Self::Named((l_value.clone(), composer.compose(source).into()))
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
                Self::FromLambdaTokens(expr.present(source), lambda_args.clone())
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
            Self::DestroyStringGroup(expr) => {
                let package = DictionaryName::Package;
                let method = InterfacesMethod::UnboxString;
                let args = CommaPunctuated::from_iter([expr.present(source), quote!(#package::#method)]);
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxGroup(args.to_token_stream()))
                    .present(source)
            },

            Self::CastConversionExpr(aspect, kind, expr, target_type, ffi_type) =>
                Self::CastConversionExprTokens(aspect.clone(), kind.clone(), expr.present(source), target_type.clone(), ffi_type.clone())
                    .present(source),

            Self::ConversionExpr(aspect, kind, expr) =>
                Self::ConversionExprTokens(aspect.clone(), kind.clone(), expr.present(source))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitive(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::OpaqueOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptOpaque(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromPrimitiveGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitiveGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::OpaqueOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptOpaqueGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::OpaqueGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOpaqueGroup(expr.to_token_stream()))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Complex, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFrom, expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFromOpt, expr.to_token_stream()))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromComplexGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptComplexGroup(expr.to_token_stream()))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitive(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::OpaqueOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptOpaque(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToPrimitiveGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitiveGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::OpaqueOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptOpaqueGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::OpaqueGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOpaqueGroup(expr.to_token_stream()))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::Complex, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiTo, expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiToOpt, expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToComplexGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptComplexGroup(expr.to_token_stream()))
                    .present(source),

            Self::ConversionExprTokens(.., ConversionExpressionKind::PrimitiveOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::DestroyOptPrimitive(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(.., ConversionExpressionKind::Complex, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAny(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(.., ConversionExpressionKind::PrimitiveGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxVecPtr(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(.., ConversionExpressionKind::ComplexGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAnyVecPtr(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(.., expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAnyOpt(expr.to_token_stream()))
                    .present(source),


            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::Primitive, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::Primitive, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::PrimitiveOpt, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::PrimitiveOpt, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::OpaqueOpt, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::OpaqueOpt, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::PrimitiveGroup, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::PrimitiveGroup, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::PrimitiveOptGroup, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::PrimitiveOptGroup, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::OpaqueOptGroup, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::OpaqueOptGroup, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::OpaqueGroup, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::OpaqueGroup, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::ComplexGroup, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::ComplexGroup, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::ComplexOptGroup, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::ComplexOptGroup, expr.clone())
                    .present(source),

            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Complex, expr, ffi_ty, ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceFrom;
                let method = FFIConversionFromMethod::FfiFrom;
                quote!(<#ffi_ty as #package::#interface<#ty>>::#method(#expr))
            }
            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr, ffi_ty, ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceFrom;
                let method = FFIConversionFromMethod::FfiFromOpt;
                quote!(<#ffi_ty as #package::#interface<#ty>>::#method(#expr))
            }
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::Complex, expr, ffi_ty, ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceTo;
                let method = FFIConversionToMethod::FfiTo;
                quote!(<#ffi_ty as #package::#interface<#ty>>::#method(#expr))
            }
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOpt, expr, ffi_ty, ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceTo;
                let method = FFIConversionToMethod::FfiToOpt;
                quote!(<#ffi_ty as #package::#interface<#ty>>::#method(#expr))
            }
            Self::CastConversionExprTokens(FFIAspect::Drop, ConversionExpressionKind::Complex, expr, ..) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAny(expr.to_token_stream()))
                    .present(source),
            Self::CastConversionExprTokens(FFIAspect::Drop, ConversionExpressionKind::ComplexOpt, expr, ..) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAnyOpt(expr.to_token_stream()))
                    .present(source),
        }
    }
}

