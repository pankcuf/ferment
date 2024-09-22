use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Expr, Type};
use crate::ast::CommaPunctuated;
use crate::composer::{Composer, FFIAspect, FieldTypeLocalContext};
use crate::context::ScopeContext;
use crate::ext::{ConversionType, Terminated};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ScopeContextPresentable, Aspect};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionDestroyMethod, FFIConversionFromMethod, FFIConversionToMethod, InterfacesMethodExpr, Name, RustFermentate};


#[derive(Clone, Copy, Debug)]
pub enum ConversionExpressionKind {
    Primitive,
    PrimitiveOpt,
    Complex,
    ComplexOpt,
    PrimitiveGroup,
    PrimitiveOptGroup,
    ComplexGroup,
    ComplexOptGroup,
}

pub trait ExpressionComposable<LANG, SPEC>: Clone
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
}

impl<LANG, SPEC> ExpressionComposable<LANG, SPEC> for Expression<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {}

#[derive(Clone)]
pub enum Expression<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    ConversionExpr(FFIAspect, ConversionExpressionKind, Box<Expression<LANG, SPEC>>),
    ConversionExprTokens(FFIAspect, ConversionExpressionKind, TokenStream2),


    Empty,
    Simple(TokenStream2),
    DictionaryName(DictionaryName),
    Name(Name),
    DictionaryExpr(DictionaryExpr),
    InterfacesExpr(InterfacesMethodExpr),

    SelfAsTrait(TokenStream2),
    ObjName(Name),
    LineTermination,
    ConversionType(Box<ConversionType<LANG, SPEC>>),
    Terminated(Box<ConversionType<LANG, SPEC>>),
    UnboxAny(Box<Expression<LANG, SPEC>>),
    UnboxAnyTerminated(Box<Expression<LANG, SPEC>>),
    DestroyString(Box<Expression<LANG, SPEC>>, TokenStream2),
    IntoBox(Box<Expression<LANG, SPEC>>),
    MapIntoBox(Box<Expression<LANG, SPEC>>),
    FromRawBox(Box<Expression<LANG, SPEC>>),
    CastDestroy(Box<Expression<LANG, SPEC>>, TokenStream2, TokenStream2),
    AsRef(Box<Expression<LANG, SPEC>>),
    Clone(Box<Expression<LANG, SPEC>>),
    Named((TokenStream2, Box<Expression<LANG, SPEC>>)),
    NamedComposer((TokenStream2, Box<ConversionType<LANG, SPEC>>)),
    DerefTokens(TokenStream2),
    DerefExpr(Box<Expression<LANG, SPEC>>),
    FfiRefWithName(Name),
    MapExpression(Box<Expression<LANG, SPEC>>, Box<Expression<LANG, SPEC>>),
    MapTokens(TokenStream2, TokenStream2),
    Expr(Expr),
    FromLambda(Box<Expression<LANG, SPEC>>, CommaPunctuated<Name>),
    FromLambdaTokens(TokenStream2, CommaPunctuated<Name>),
    FromPtrClone(Box<Expression<LANG, SPEC>>),
    Boxed(Box<Expression<LANG, SPEC>>),
}

impl<LANG, SPEC> Expression<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub(crate) fn empty() -> Self {
        Self::Empty
    }

    pub(crate) fn line_termination() -> Self {
        Self::LineTermination
    }

    pub(crate) fn expr_as_ref(expr: Self) -> Self {
        Self::AsRef(expr.into())
    }

    fn conversion_type(conversion_type: &ConversionType<LANG, SPEC>) -> Self {
        Self::ConversionType(Box::new(conversion_type.clone()))
    }
    pub(crate) fn named(name: TokenStream2, expr: Self) -> Self {
        Self::Named((name, Box::new(expr)))
    }

    pub(crate) fn empty_conversion_type(_context: &FieldTypeLocalContext<LANG, SPEC>) -> Self {
        Self::Empty
    }

    pub(crate) fn bypass_conversion_type(context: &FieldTypeLocalContext<LANG, SPEC>) -> Self {
        let (_, conversion_type) = context;
        Self::conversion_type(conversion_type)
    }

    pub(crate) fn named_conversion_type(context: &FieldTypeLocalContext<LANG, SPEC>) -> Self {
        let (name, conversion_type) = context;
        Self::NamedComposer((name.to_token_stream(), Box::new(conversion_type.clone())))
    }

    pub(crate) fn terminated(context: &FieldTypeLocalContext<LANG, SPEC>) -> Self {
        let (_, conversion_type) = context;
        Self::Terminated(Box::new(conversion_type.clone()))
    }

    pub(crate) fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }

    pub(crate) fn name(name: &Name) -> Self {
        Self::Name(name.clone())
    }

    pub(crate) fn obj_name(name: &Name) -> Self {
        Self::ObjName(name.clone())
    }

    pub(crate) fn ffi_ref_with_name(name: &Name) -> Self {
        Self::FfiRefWithName(name.clone())
    }

    // pub(crate) fn deref_name(name: &Name) -> Self {
    //     Self::DerefName(name.clone())
    // }

    pub(crate) fn deref_tokens<T: ToTokens>(token_stream: T) -> Self {
        Self::DerefTokens(token_stream.to_token_stream())
    }

    pub(crate) fn deref_expr(expr: Self) -> Self {
        Self::DerefExpr(expr.into())
    }

    pub(crate) fn self_as_trait_type(ty: Type) -> Self {
        Self::SelfAsTrait(ty.to_token_stream())
    }

    pub(crate) fn map_expr(presentable: Self, mapper: Self) -> Self {
        Self::MapExpression(presentable.into(), mapper.into())
    }

    pub(crate) fn map_tokens<T: ToTokens, U: ToTokens>(presentable: T, mapper: U) -> Self {
        Self::MapTokens(presentable.to_token_stream(), mapper.to_token_stream())
    }

    pub(crate) fn from_complex(expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::From, ConversionExpressionKind::Complex, expr.into())
    }

    pub(crate) fn from_complex_opt(expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr.into())
    }

    pub(crate) fn from_primitive_opt(expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt, expr.into())
    }

    pub(crate) fn from_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Complex, expr.to_token_stream())
    }

    pub(crate) fn from_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr.to_token_stream())
    }

    pub(crate) fn from_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Primitive, expr.to_token_stream())
    }
    pub(crate) fn from_primitive(expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::From, ConversionExpressionKind::Primitive, expr.into())
    }
    pub(crate) fn from_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt, expr.to_token_stream())
    }

    pub(crate) fn from_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveGroup, expr.to_token_stream())
    }

    pub(crate) fn from_primitive_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOptGroup, expr.to_token_stream())
    }

    pub(crate) fn from_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexGroup, expr.to_token_stream())
    }

    pub(crate) fn from_complex_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::ComplexOptGroup, expr.to_token_stream())
    }

    pub(crate) fn from_lambda(expr: Self, args: CommaPunctuated<Name>) -> Self {
        Self::FromLambda(expr.into(), args)
    }

    pub(crate) fn from_ptr_clone(expr: Self) -> Self {
        Self::FromPtrClone(expr.into())
    }

    pub(crate) fn into_box(expr: Self) -> Self {
        Self::IntoBox(expr.into())
    }

    pub(crate) fn map_into_box(expr: Self) -> Self {
        Self::MapIntoBox(expr.into())
    }

    pub(crate) fn from_raw_box(expr: Self) -> Self {
        Self::FromRawBox(expr.into())
    }
    pub(crate) fn ffi_to_complex(expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Complex, expr.into())
    }

    pub(crate) fn ffi_to_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::Simple(expr.to_token_stream())
    }

    pub(crate) fn ffi_to_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::Complex, expr.to_token_stream())
    }

    pub(crate) fn ffi_to_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOpt, expr.to_token_stream())
    }

    pub(crate) fn ffi_to_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOpt, expr.to_token_stream())
    }

    pub(crate) fn ffi_to_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveGroup, expr.to_token_stream())
    }

    pub(crate) fn ffi_to_primitive_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOptGroup, expr.to_token_stream())
    }

    pub(crate) fn ffi_to_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexGroup, expr.to_token_stream())
    }

    pub(crate) fn ffi_to_complex_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::To, ConversionExpressionKind::ComplexOptGroup, expr.to_token_stream())
    }

    pub(crate) fn boxed<T: ToTokens>(expr: T) -> Self {
        Self::InterfacesExpr(InterfacesMethodExpr::Boxed(expr.to_token_stream()))
    }

    pub(crate) fn boxed_expr(expr: Self) -> Self {
        Self::Boxed(expr.into())
    }

    pub(crate) fn destroy_string<T: ToTokens>(expr: Self, token_stream: T) -> Self {
        Self::DestroyString(expr.into(), token_stream.to_token_stream())
    }

    pub(crate) fn destroy_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::Destroy, ConversionExpressionKind::PrimitiveOpt, expr.to_token_stream())
    }

    pub(crate) fn destroy_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::Destroy, ConversionExpressionKind::Complex, expr.to_token_stream())
    }
    pub(crate) fn destroy_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::Destroy, ConversionExpressionKind::Primitive, expr.to_token_stream())
    }

    pub(crate) fn destroy_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::Destroy, ConversionExpressionKind::ComplexOpt, expr.to_token_stream())
    }

    pub(crate) fn destroy_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::Destroy, ConversionExpressionKind::PrimitiveGroup, expr.to_token_stream())
    }

    pub(crate) fn destroy_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::ConversionExprTokens(FFIAspect::Destroy, ConversionExpressionKind::ComplexGroup, expr.to_token_stream())
    }

    pub(crate) fn unbox_any<T: ToTokens>(expr: T) -> Self {
        Self::InterfacesExpr(InterfacesMethodExpr::UnboxAny(expr.to_token_stream()))
    }

    pub(crate) fn unbox_any_expr(expr: Self) -> Self {
        Self::UnboxAny(expr.into())
    }
    pub(crate) fn unbox_any_expr_terminated(expr: Self) -> Self {
        Self::UnboxAnyTerminated(expr.into())
    }
}

// impl<LANG, SPEC> Display for Expression<LANG, SPEC>
//     where LANG: Clone + Debug,
//           SPEC: Specification<LANG> + Debug,
//           <SPEC as Specification<LANG>>::Attr: Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         std::fmt::Debug::fmt(self, f)
//     }
// }

impl<SPEC> ScopeContextPresentable for Expression<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Empty => quote!(),
            Self::LineTermination => quote!(;),
            Self::Simple(expr) =>
                expr.to_token_stream(),
            Self::DictionaryName(expr) =>
                expr.to_token_stream(),
            Self::DictionaryExpr(expr) =>
                expr.to_token_stream(),
            Self::InterfacesExpr(expr) =>
                expr.to_token_stream(),
            Self::MapExpression(presentable, mapper) =>
                DictionaryExpr::Mapper(
                    presentable.present(source).to_token_stream(),
                    mapper.present(source).to_token_stream())
                    .to_token_stream(),
            Self::MapTokens(presentable, mapper) =>
                DictionaryExpr::Mapper(
                    presentable.to_token_stream(),
                    mapper.to_token_stream())
                    .to_token_stream(),
            Self::UnboxAny(presentable) =>
                Self::InterfacesExpr(InterfacesMethodExpr::UnboxAny(presentable.present(source).to_token_stream()))
                    .present(source),
            Self::UnboxAnyTerminated(presentable) =>
                Self::UnboxAny(presentable.clone())
                    .present(source)
                    .to_token_stream()
                    .terminated(),
            Self::DestroyString(presentable, path) => {
                Self::CastDestroy(
                    presentable.clone(),
                    path.to_token_stream(),
                    DictionaryExpr::CChar.to_token_stream())
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
            Self::SelfAsTrait(self_ty) =>
                quote!(*((*self_).object as *const #self_ty)),
            Self::IntoBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::NewBox(expr.present(source).to_token_stream()))
                    .present(source),
            Self::MapIntoBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::MapIntoBox(expr.present(source).to_token_stream()))
                    .present(source),
            Self::FromRawBox(expr) =>
                Self::DictionaryExpr(DictionaryExpr::FromRawBox(expr.present(source).to_token_stream()))
                    .present(source),
            Self::DerefTokens(field_name) =>
                Self::DictionaryExpr(DictionaryExpr::Deref(field_name.clone()))
                    .present(source),
            Self::DerefExpr(presentable) =>
                Self::DerefTokens(presentable.present(source))
                    .present(source),
            Self::ObjName(name) =>
                quote!(obj.#name),
            Self::FfiRefWithName(name) =>
                quote!(ffi_ref.#name),
            Self::Name(name) => name
                .to_token_stream(),
            Self::ConversionType(expr) =>
                expr.compose(source)
                    .present(source),
            Self::Terminated(expr) =>
                expr.compose(source)
                    .present(source)
                    .to_token_stream()
                    .terminated(),
            Self::FromLambda(field_path, lambda_args) =>
                Self::FromLambdaTokens(field_path.present(source), lambda_args.clone()).present(source),
            Self::FromLambdaTokens(field_path, lambda_args) =>
                quote!(move |#lambda_args| unsafe { #field_path.call(#lambda_args) }),

            Self::FromPtrClone(field_path) => {
                let field_path = field_path.present(source).to_token_stream();
                quote!((&*#field_path).clone())
            }
            Self::Expr(expr) =>
                expr.to_token_stream(),
            Self::Clone(expr) => {
                let expr = expr.present(source).to_token_stream();
                quote! { #expr.clone() }
            }
            Self::Boxed(expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::Boxed(expr.present(source).to_token_stream()))
                    .present(source),

            Self::ConversionExpr(aspect, kind, expr) =>
                Self::ConversionExprTokens(aspect.clone(), kind.clone(), expr.present(source))
                    .present(source),


            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::Primitive, expr) =>
                expr.to_token_stream(),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitive(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromPrimitiveGroup(expr.to_token_stream()))
                    .present(source),
            Self::ConversionExprTokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOptGroup, expr) =>
                Self::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitiveGroup(expr.to_token_stream()))
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


        }

    }
}

