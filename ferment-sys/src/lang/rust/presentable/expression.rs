use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::{FFIAspect, SourceComposable};
use crate::context::ScopeContext;
use crate::ext::Terminated;
use crate::lang::RustSpecification;
use crate::presentable::{ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionFromMethod, FFIConversionToMethod, InterfacesMethodExpr};

impl ScopeContextPresentable for Expression<RustSpecification> {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Empty => quote!(),
            Self::Simple(expr) =>
                expr.to_token_stream(),
            Self::DictionaryName(expr) =>
                expr.to_token_stream(),
            Self::DictionaryExpr(expr) =>
                expr.to_token_stream(),
            Self::InterfacesExpr(expr) =>
                expr.to_token_stream(),

            Self::ObjName(name) =>
                quote!(obj.#name),
            Self::FfiRefWithName(name) =>
                DictionaryExpr::ffi_ref_prop(name).to_token_stream(),
            Self::Name(name) => name
                .to_token_stream(),

            Self::AsRef(field_path) => {
                Self::DictionaryExpr(DictionaryExpr::AsRef(field_path.present(source)))
                    .present(source)
            },
            Self::DerefRef(field_path) => {
                Self::DictionaryExpr(DictionaryExpr::DerefRef(field_path.present(source)))
                    .present(source)
            },
            Self::DerefMutRef(field_path) => {
                Self::DictionaryExpr(DictionaryExpr::DerefMutRef(field_path.present(source)))
                    .present(source)
            },
            Self::LeakBox(field_path) => {
                Self::DictionaryExpr(DictionaryExpr::LeakBox(field_path.present(source)))
                    .present(source)
            },
            Self::AsMutRef(field_path) =>
                Self::DictionaryExpr(DictionaryExpr::AsMutRef(field_path.present(source)))
                    .present(source),
            Self::Clone(expr) =>
                Self::DictionaryExpr(DictionaryExpr::Clone(expr.present(source)))
                    .present(source),
            Self::FromPtrClone(field_path) =>
                Self::DictionaryExpr(DictionaryExpr::FromPtrClone(field_path.present(source)))
                    .present(source),
            Self::FromPtrRead(field_path) =>
                Self::DictionaryExpr(DictionaryExpr::FromPtrRead(field_path.present(source)))
                    .present(source),
            Self::DerefExpr(presentable) =>
                Self::DictionaryExpr(DictionaryExpr::Deref(presentable.present(source)))
                    .present(source),

            Self::MapExpression(presentable, mapper) =>
                Self::DictionaryExpr(DictionaryExpr::Mapper(presentable.present(source), mapper.present(source)))
                    .present(source),

            Self::MapIntoBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::MapIntoBox(expr.present(source)))
                    .present(source),
            Self::FromRawBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::FromRawBox(expr.present(source)))
                    .present(source),

            Self::DestroyString(presentable, _ty) => {
                InterfacesMethodExpr::UnboxString(presentable.present(source))
                    .to_token_stream()
            },
            Self::DestroyBigInt(presentable, _target_ty, _ffi_ty) => {
                InterfacesMethodExpr::UnboxAnyOpt(presentable.present(source))
                    .to_token_stream()
            },
            Self::Named((l_value, presentable)) => {
                let ty = presentable.present(source).to_token_stream();
                quote!(#l_value: #ty)
            }
            Self::NamedComposer((l_value, composer)) => {
                let expression = composer.compose(source);
                Self::Named((l_value.clone(), expression.into()))
                    .present(source)
            },

            Self::ConversionType(expr) => {
                expr.compose(source)
                    .present(source)
            },
            Self::Terminated(expr) => {
                expr.compose(source)
                    .present(source)
                    .to_token_stream()
                    .terminated()
            },
            Self::FromLambda(field_path, lambda_args) =>
                Self::FromLambdaTokens(field_path.present(source), lambda_args.clone())
                    .present(source),
            Self::FromLambdaTokens(field_path, lambda_args) =>
                quote!(move |#lambda_args| unsafe { #field_path.call(#lambda_args) }),

            Self::Boxed(expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::Boxed(expr.present(source).to_token_stream()))
                    .present(source),
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

            Self::CastConversionExpr(aspect, kind, expr, target_type, ffi_type) =>
                Self::CastConversionExprTokens(aspect.clone(), kind.clone(), expr.present(source), target_type.clone(), ffi_type.clone())
                    .present(source),

            Self::ConversionExpr(aspect, kind, expr) =>
                Self::ConversionExprTokens(aspect.clone(), kind.clone(), expr.present(source))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Primitive, expr) =>
                expr.to_token_stream(),
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
            // Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::OpaqueOpt, expr) =>
            //     Self::InterfacesExpr(InterfacesMethodExpr::FromOptOpaque(expr.to_token_stream()))
            //         .present(source),

            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromComplexGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptComplexGroup(expr.to_token_stream()))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::Primitive, expr) =>
                expr.present(source),
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

            Self::ConversionExprTokens(.., ConversionExpressionKind::Primitive, _expr) =>
                quote!(),
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
            Self::ConversionExprTokens(.., _, expr) =>
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
                Self::DictionaryExpr(DictionaryExpr::CallMethod(quote!(<#ffi_ty as #package::#interface<#ty>>::#method), expr.present(source)))
                    .present(source)
            }
            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr, ffi_ty, ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceFrom;
                let method = FFIConversionFromMethod::FfiFromOpt;
                Self::DictionaryExpr(DictionaryExpr::CallMethod(quote!(<#ffi_ty as #package::#interface<#ty>>::#method), expr.present(source)))
                    .present(source)
            }
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::Complex, expr, ffi_ty, ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceTo;
                let method = FFIConversionToMethod::FfiTo;
                Self::DictionaryExpr(DictionaryExpr::CallMethod(quote!(<#ffi_ty as #package::#interface<#ty>>::#method), expr.present(source)))
                    .present(source)
            }
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOpt, expr, ffi_ty, ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceTo;
                let method = FFIConversionToMethod::FfiToOpt;
                Self::DictionaryExpr(DictionaryExpr::CallMethod(quote!(<#ffi_ty as #package::#interface<#ty>>::#method), expr.present(source)))
                    .present(source)
            }
            Self::CastDestroy(expr, ..) => {
                InterfacesMethodExpr::UnboxAny(expr.present(source))
                    .to_token_stream()
            }
            Self::CastConversionExprTokens(FFIAspect::Drop, ConversionExpressionKind::Complex, expr, ..) => {
                InterfacesMethodExpr::UnboxAny(expr.to_token_stream())
                    .to_token_stream()
            }
            Self::CastConversionExprTokens(FFIAspect::Drop, ConversionExpressionKind::ComplexOpt, expr, ..) => {
                InterfacesMethodExpr::UnboxAnyOpt(expr.to_token_stream())
                    .to_token_stream()
            }
            Self::DestroyStringGroup(expr) => {
                let pres = expr.present(source);
                InterfacesMethodExpr::UnboxGroup(quote!(#pres, ferment::unbox_string)).to_token_stream()
                // InterfacesMethodExpr::UnboxAnyVecPtrComposer(quote!(#pres, ferment::unbox_string)).to_token_stream()
            },
        }
    }
}

