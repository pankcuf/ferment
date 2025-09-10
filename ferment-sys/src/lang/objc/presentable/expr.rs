use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::{FFIAspect, SourceComposable};
use crate::context::ScopeContext;
use crate::ext::Terminated;
use crate::lang::objc::ObjCSpecification;
use crate::presentable::{ConversionAspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, FFIConversionFromMethod, InterfacesMethodExpr};


impl ScopeContextPresentable for Expression<ObjCSpecification> {
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
            Self::InterfacesExpr(expr) =>
                expr.to_token_stream(),
            Self::MapExpr(presentable, mapper) =>
                DictionaryExpr::mapper(presentable.present(source), mapper.present(source)).to_token_stream(),
            Self::DestroyString(presentable, _path) => {
                let expr = presentable.present(source);
                quote!([NSString ffi_destroy: #expr])
            },
            Self::DestroyBigInt(presentable, _ffi_ty, _target_ty) => {
                let field_path = presentable.present(source);
                quote! { if (#field_path) free(#field_path); }
            },
            Self::AsRef(field_path) =>
                DictionaryExpr::AsRef(field_path.present(source)).to_token_stream(),
            Self::LeakBox(field_path) =>
                DictionaryExpr::LeakBox(field_path.present(source)).to_token_stream(),
            Self::AsMutRef(field_path) =>
                DictionaryExpr::AsMutRef(field_path.present(source)).to_token_stream(),
            Self::DerefRef(field_path) =>
                DictionaryExpr::DerefRef(field_path.present(source)).to_token_stream(),
            Self::DerefMutRef(field_path) =>
                DictionaryExpr::DerefMutRef(field_path.present(source)).to_token_stream(),
            Self::Named((l_value, presentable)) => {
                let ty = presentable.present(source);
                quote!(#l_value: #ty)
            }
            Self::NamedComposer((l_value, composer)) => {
                let presentation = composer.compose(source).present(source);
                quote!(#l_value = #presentation)
            },
            Self::MapIntoBox(expr) =>
                DictionaryExpr::MapIntoBox(expr.present(source)).to_token_stream(),
            Self::FromRawBox(expr) =>
                DictionaryExpr::FromRawBox(expr.present(source)).to_token_stream(),
            Self::DerefExpr(presentable) =>
                DictionaryExpr::Deref(presentable.present(source)).to_token_stream(),
            Self::ObjName(name) =>
                quote!(obj.#name),
            Self::FfiRefWithName(name) =>
                quote!(ffi_ref->#name),
            Self::Name(name) =>
                name.to_token_stream(),
            Self::ConversionType(expr) =>
                expr.compose(source)
                    .present(source),
            Self::Terminated(expr) =>
                expr.compose(source)
                    .present(source)
                    .terminated(),
            Self::FromLambda(field_path, lambda_args) =>
                Self::FromLambdaTokens(field_path.present(source), lambda_args.clone())
                    .present(source),
            Self::FromLambdaTokens(field_path, lambda_args) =>
                quote!(move |#lambda_args| unsafe { #field_path.call(#lambda_args) }),
            Self::Boxed(expr) =>
                InterfacesMethodExpr::Boxed(expr.present(source)).to_token_stream(),
            Self::Clone(expr) => {
                let expr = expr.present(source);
                quote! { #expr.clone() }
            }
            Self::ConversionExpr(aspect, expr) =>
                Self::ConversionExprTokens(*aspect, expr.present(source))
                    .present(source),

            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::Primitive }, expr) =>
                expr.to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveOpt }, expr) =>
                quote!(#expr ? *#expr : 0),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveGroup }, expr) =>
                quote!([NSArray ffi_from:#expr]),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveOptGroup }, expr) =>
                quote!([NSArray ffi_from:#expr]),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::OpaqueOptGroup | ConversionExpressionKind::OpaqueGroup }, expr) =>
                quote!([NSArray ffi_from:#expr]),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::Complex }, expr) =>
                InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::Mut, expr.to_token_stream()).to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexOpt }, expr) =>
                InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::Opt, expr.to_token_stream()).to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexGroup }, expr) =>
                InterfacesMethodExpr::FromComplexGroup(expr.to_token_stream()).to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexOptGroup }, expr) =>
                InterfacesMethodExpr::FromOptComplexGroup(expr.to_token_stream()).to_token_stream(),

            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::Primitive }, expr) =>
                expr.present(source),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveOpt }, expr) =>
                InterfacesMethodExpr::ToOptPrimitive(expr.to_token_stream()).to_token_stream(),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveGroup }, expr) =>
                quote!([NSArray ffi_to:#expr]),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveOptGroup }, expr) =>
                quote!([NSArray ffi_to_opt:#expr]),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::OpaqueOptGroup | ConversionExpressionKind::OpaqueGroup }, expr) =>
                quote!([NSArray ffi_to_opt:#expr]),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::Complex }, expr) =>
                panic!("wrong {}", expr),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexOpt }, expr) =>
                panic!("wrong {}", expr),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexGroup }, expr) =>
                quote!([DSFerment to_complex_group:#expr]),
            Self::ConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexOptGroup }, expr) =>
                quote!([DSFerment to_complex_opt_group:#expr]),
            Self::ConversionExprTokens(ConversionAspect { kind: ConversionExpressionKind::Primitive, .. }, _expr) =>
                quote!(),
            Self::ConversionExprTokens(ConversionAspect { kind: ConversionExpressionKind::PrimitiveOpt, .. }, expr) =>
                quote!(if (#expr) free(#expr);),
            Self::ConversionExprTokens(.., expr) =>
                expr.to_token_stream(),

            Self::CastConversionExpr(aspect, expr, ffi_ty, ty) =>
                Self::CastConversionExprTokens(*aspect, expr.present(source), ffi_ty.clone(), ty.clone())
                    .present(source),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveGroup }, expr, ..) =>
                quote!([NSArray ffi_from:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveGroup }, expr, ..) =>
                quote!([NSArray ffi_to:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::PrimitiveGroup }, expr, ..) =>
                quote!([NSArray ffi_destroy:#expr]),

            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::PrimitiveOptGroup | ConversionExpressionKind::OpaqueOptGroup | ConversionExpressionKind::OpaqueGroup }, expr, ..) =>
                quote!([DSFerment from_opt_primitive_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::PrimitiveOptGroup | ConversionExpressionKind::OpaqueOptGroup | ConversionExpressionKind::OpaqueGroup }, expr, ..) =>
                quote!([DSFerment to_opt_primitive_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::PrimitiveOptGroup | ConversionExpressionKind::OpaqueOptGroup | ConversionExpressionKind::OpaqueGroup }, expr, ..) =>
                quote!([DSFerment destroy_opt_primitive_group:#expr]),

            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexGroup }, expr, ..) =>
                quote!([DSFerment from_complex_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexGroup }, expr, ..) =>
                quote!([DSFerment to_complex_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::ComplexGroup }, expr, ..) |
            Self::DestroyStringGroup(expr) =>
                quote!([DSFerment destroy_complex_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexOptGroup }, expr, ..) =>
                quote!([DSFerment from_opt_complex_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexOptGroup }, expr, ..) =>
                quote!([DSFerment to_opt_complex_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::ComplexOptGroup }, expr, ..) =>
                quote!([DSFerment destroy_opt_complex_group:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::Complex }, expr, ffi_ty, _ty) =>
                quote!([#ffi_ty ffi_from:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::ComplexOpt }, expr, ffi_ty, _ty) =>
                quote!([#ffi_ty ffi_from_opt:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::Complex }, expr, ffi_ty, _ty) =>
                quote!([#ffi_ty ffi_to:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::ComplexOpt }, expr, ffi_ty, _ty) =>
                quote!([#ffi_ty ffi_to_opt:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::Complex | ConversionExpressionKind::ComplexOpt }, expr, ffi_ty, _ty) =>
                quote!([#ffi_ty ffi_destroy:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::From, kind: ConversionExpressionKind::OpaqueOpt }, expr, ffi_ty, _ty) =>
                quote!([#ffi_ty ffi_from_opaque_opt:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::To, kind: ConversionExpressionKind::OpaqueOpt }, expr, ffi_ty, _ty) =>
                quote!([#ffi_ty ffi_to_opaque_opt:#expr]),
            Self::CastConversionExprTokens(ConversionAspect { aspect: FFIAspect::Drop, kind: ConversionExpressionKind::OpaqueOpt }, expr, ..) =>
                quote!([DSFerment destroy_opaque_opt_complex_group:#expr]),

            Self::CastConversionExprTokens(aspect, expr, ..) =>
                Self::ConversionExprTokens(*aspect, expr.clone())
                    .present(source),

            Self::NewSmth(expr, smth) => {
                let expr = expr.present(source);
                quote!(#expr #smth)
            }
            Self::FromPtrRead(expr) |
            Self::NewCow(expr) |
            Self::CowIntoOwned(expr) |
            Self::SimpleExpr(expr) =>
                expr.present(source),
        }
    }
}
