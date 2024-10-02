use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::{SourceComposable, FFIAspect};
use crate::context::ScopeContext;
use crate::ext::Terminated;
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::presentable::{ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionDestroyMethod, FFIConversionFromMethod, FFIConversionToMethod, InterfacesMethodExpr};


impl<SPEC> ScopeContextPresentable for Expression<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Empty => quote!().to_token_stream(),
            Self::Simple(expr) =>
                expr.to_token_stream(),
            Self::DictionaryName(expr) =>
                expr.to_token_stream(),
            Self::DictionaryExpr(expr) =>
                expr.to_token_stream(),


            Self::InterfacesExpr(expr) => expr.to_token_stream(),
            Self::MapExpression(presentable, mapper) =>
                DictionaryExpr::Mapper(presentable.present(source).to_token_stream(), mapper.present(source).to_token_stream()).to_token_stream(),
            Self::DestroyString(presentable, path) => {
                Self::CastDestroy(
                    presentable.clone(),
                    path.to_token_stream(),
                    quote!(NSString))
                    .present(source)
            },
            Self::CastDestroy(args, ty, ffi_ty) => {
                let package = DictionaryName::Package;
                let interface = DictionaryName::InterfaceDestroy;
                let method = FFIConversionDestroyMethod::Destroy;
                DictionaryExpr::CallMethod(
                    quote!(<#ffi_ty as #package::#interface<#ty>>::#method),
                    args.present(source).to_token_stream())
                    .to_token_stream()
            }
            Self::AsRef(field_path) =>
                DictionaryExpr::AsRef(field_path.present(source).to_token_stream())
                    .to_token_stream(),
            Self::Named((l_value, presentable)) => {
                let ty = presentable.present(source).to_token_stream();
                quote!(#l_value: #ty)
            }
            Self::NamedComposer((l_value, composer)) => {
                let expression = composer.compose(source);
                Self::Named((l_value.clone(), expression.into())).present(source)
            },
            Self::NewBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::NewBox(expr.present(source)))
                    .present(source),
            Self::MapIntoBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::MapIntoBox(expr.present(source)))
                    .present(source),
            Self::FromRawBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::FromRawBox(expr.present(source)))
                    .present(source),
            Self::DerefExpr(presentable) =>
                Self::DictionaryExpr(DictionaryExpr::Deref(presentable.present(source)))
                    .present(source),
            Self::ObjName(name) =>
                quote!(obj.#name),
            Self::FfiRefWithName(name) =>
                quote!(ffi_ref->#name),
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
            Self::FromLambda(field_path, lambda_args) =>
                Self::FromLambdaTokens(field_path.present(source), lambda_args.clone())
                    .present(source),
            Self::FromLambdaTokens(field_path, lambda_args) =>
                quote!(move |#lambda_args| unsafe { #field_path.call(#lambda_args) }),
            Self::FromPtrClone(field_path) => {
                let field_path = field_path.present(source);
                quote!((&*#field_path).clone())
            }
            Self::Boxed(expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::Boxed(expr.present(source)))
                    .present(source),
            Self::Clone(expr) => {
                let expr = expr.present(source);
                quote! { #expr.clone() }
            }
            Self::ConversionExpr(aspect, kind, expr) =>
                Self::ConversionExprTokens(aspect.clone(), kind.clone(), expr.present(source))
                    .present(source),

            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Primitive, expr) =>
                expr.to_token_stream(),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt, expr) => {
                println!("OBJC: ConversionExprTokens: From: PrimitiveOpt: {}", expr);
                quote!(#expr ? *#expr : 0)
            },
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveGroup, expr) => {
                println!("OBJC: ConversionExprTokens: From: PrimitiveGroup: {}", expr);
                quote!([DSFerment from_primitive_group:#expr])
            },
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOptGroup, expr) => {
                println!("OBJC: ConversionExprTokens: From: PrimitiveOptGroup: {}", expr);
                quote!([DSFerment from_primitive_opt_group:#expr])

                // Self::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitiveGroup(expr.to_token_stream()))
                //     .present(source)
            },
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Complex, expr) => {
                println!("OBJC: ConversionExprTokens: From: Complex: {}", expr);

                //[DSArr_u8_96 ffi_from:ffi_ref->o_0];

                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFrom, expr.to_token_stream()))
                    .present(source)
            },
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFromOpt, expr.to_token_stream()))
                    .present(source),
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
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToPrimitiveGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitiveGroup(expr.to_token_stream()))
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
            Self::ConversionExprTokens(.., _, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FFIConversionDestroy(FFIConversionDestroyMethod::Destroy, expr.to_token_stream()))
                    .present(source),


            Self::CastConversionExpr(aspect, kind, expr, ffi_ty, ty) => {
                let expr = expr.present(source);

                Self::CastConversionExprTokens(aspect.clone(), kind.clone(), expr, ffi_ty.clone(), ty.clone())
                    .present(source)
            },

            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::Primitive, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::Primitive, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(aspect, ConversionExpressionKind::PrimitiveOpt, expr, ..) =>
                Self::ConversionExprTokens(aspect.clone(), ConversionExpressionKind::PrimitiveOpt, expr.clone())
                    .present(source),
            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveGroup, expr, ..) => {
                quote!([DSFerment from_primitive_group:#expr])
            }
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveGroup, expr, ..) => {
                quote!([DSFerment to_primitive_group:#expr])
            }
            Self::CastConversionExprTokens(FFIAspect::Destroy | FFIAspect::Drop, ConversionExpressionKind::PrimitiveGroup, expr, ..) => {
                quote!([DSFerment destroy_primitive_group:#expr])
            }

            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOptGroup, expr, ..) => {
                quote!([DSFerment from_opt_primitive_group:#expr])
            }
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOptGroup, expr, ..) => {
                quote!([DSFerment to_opt_primitive_group:#expr])
            }
            Self::CastConversionExprTokens(FFIAspect::Destroy | FFIAspect::Drop, ConversionExpressionKind::PrimitiveOptGroup, expr, ..) => {
                quote!([DSFerment destroy_opt_primitive_group:#expr])
            }

            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexGroup, expr, ..) => {
                quote!([DSFerment from_complex_group:#expr])
            }
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexGroup, expr, ..) => {
                quote!([DSFerment to_complex_group:#expr])
            }
            Self::CastConversionExprTokens(FFIAspect::Destroy | FFIAspect::Drop, ConversionExpressionKind::ComplexGroup, expr, ..) => {
                quote!([DSFerment destroy_complex_group:#expr])
            }
            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOptGroup, expr, ..) => {
                quote!([DSFerment from_opt_complex_group:#expr])
            },
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOptGroup, expr, ..) => {
                quote!([DSFerment to_opt_complex_group:#expr])
            },
            Self::CastConversionExprTokens(FFIAspect::Destroy | FFIAspect::Drop, ConversionExpressionKind::ComplexOptGroup, expr, ..) => {
                quote!([DSFerment destroy_opt_complex_group:#expr])
            },

            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Complex, expr, ffi_ty, ty) =>
                quote!([#ffi_ty ffi_from:#expr]),
            Self::CastConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr, ffi_ty, ty) =>
                quote!([#ffi_ty ffi_from_opt:#expr]),
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::Complex, expr, ffi_ty, ty) =>
                quote!([#ffi_ty ffi_to:#expr]),
            Self::CastConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOpt, expr, ffi_ty, ty) =>
                quote!([#ffi_ty ffi_to_opt:#expr]),
            Self::CastConversionExprTokens(FFIAspect::Destroy | FFIAspect::Drop, ConversionExpressionKind::Complex | ConversionExpressionKind::ComplexOpt, expr, ffi_ty, ty) =>
                quote!([#ffi_ty ffi_destroy:#expr]),

        }
    }
}
