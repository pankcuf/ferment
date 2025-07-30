use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Type;
use crate::ast::CommaPunctuated;
use crate::composer::{SourceComposable, FFIAspect, FieldTypeLocalContext};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, TypeKind};
use crate::ext::{Conversion, GenericNestedArg, Terminated, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionFromMethod, FFIConversionToMethod, InterfacesMethodExpr};


#[derive(Clone, Copy, Debug)]
pub enum ConversionExpressionKind {
    Primitive,
    PrimitiveOpt,
    Complex,
    ComplexOpt,
    OpaqueOpt,
    PrimitiveGroup,
    PrimitiveOptGroup,
    ComplexGroup,
    ComplexOptGroup,
    OpaqueGroup,
    OpaqueOptGroup,
}


impl From<&Type> for ConversionExpressionKind {
    fn from(value: &Type) -> Self {
        Self::from(value.clone())
    }
}
impl From<Type> for ConversionExpressionKind {
    fn from(value: Type) -> Self {
        match TypeKind::from(value) {
            TypeKind::Primitive(_) =>
                ConversionExpressionKind::Primitive,
            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                Some(TypeKind::Primitive(_)) =>
                    ConversionExpressionKind::PrimitiveOpt,
                _ =>
                    ConversionExpressionKind::ComplexOpt,
            }
            _ =>
                ConversionExpressionKind::Complex,
        }
    }
}

pub trait ExpressionComposable<SPEC>: Clone + Debug + ScopeContextPresentable
    where SPEC: Specification {
    fn simple<T: ToTokens>(tokens: T) -> SPEC::Expr;
}

impl<SPEC> ExpressionComposable<SPEC> for Expression<SPEC>
    where SPEC: Specification<Expr=Self>,
          SPEC::Expr: ScopeContextPresentable {
    fn simple<T: ToTokens>(tokens: T) -> SPEC::Expr {
        Expression::Simple(tokens.to_token_stream())
    }
}

impl<SPEC> Default for Expression<SPEC>
where SPEC: Specification<Expr=Self>,
      Self: ScopeContextPresentable {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Debug)]
pub enum Expression<SPEC>
    where SPEC: Specification<Expr=Self>,
          Self: ScopeContextPresentable {
    ConversionExpr(FFIAspect, ConversionExpressionKind, Box<Expression<SPEC>>),
    ConversionExprTokens(FFIAspect, ConversionExpressionKind, TokenStream2),
    CastConversionExpr(FFIAspect, ConversionExpressionKind, Box<Expression<SPEC>>, /*ffi*/Type, /*target*/Type),
    CastConversionExprTokens(FFIAspect, ConversionExpressionKind, TokenStream2, /*ffi*/Type, /*target*/Type),

    Empty,
    Simple(TokenStream2),
    DictionaryName(DictionaryName),
    Name(SPEC::Name),
    FfiRefWithName(SPEC::Name),
    ObjName(SPEC::Name),
    DictionaryExpr(DictionaryExpr),
    InterfacesExpr(InterfacesMethodExpr),
    AsRef(Box<Expression<SPEC>>),
    AsMutRef(Box<Expression<SPEC>>),
    DerefRef(Box<Expression<SPEC>>),
    DerefMutRef(Box<Expression<SPEC>>),
    Clone(Box<Expression<SPEC>>),
    FromPtrClone(Box<Expression<SPEC>>),
    FromPtrRead(Box<Expression<SPEC>>),
    DerefExpr(Box<Expression<SPEC>>),
    MapExpression(Box<Expression<SPEC>>, Box<Expression<SPEC>>),
    MapIntoBox(Box<Expression<SPEC>>),
    FromRawBox(Box<Expression<SPEC>>),

    CastDestroy(Box<Expression<SPEC>>, /*ffi*/TokenStream2, /*target*/TokenStream2),
    DestroyString(Box<Expression<SPEC>>, TokenStream2),
    DestroyStringGroup(TokenStream2),
    DestroyBigInt(Box<Expression<SPEC>>, /*ffi*/TokenStream2, /*target*/TokenStream2),

    ConversionType(Box<Conversion<SPEC>>),
    Terminated(Box<Conversion<SPEC>>),

    Named((TokenStream2, Box<Expression<SPEC>>)),
    NamedComposer((TokenStream2, Box<Conversion<SPEC>>)),

    FromLambda(Box<Expression<SPEC>>, CommaPunctuated<SPEC::Name>),
    FromLambdaTokens(TokenStream2, CommaPunctuated<SPEC::Name>),
    Boxed(Box<Expression<SPEC>>),
    LeakBox(Box<Expression<SPEC>>),
    NewSmth(Box<Expression<SPEC>>, TokenStream2),
    NewCow(Box<Expression<SPEC>>),
    CowIntoOwned(Box<Expression<SPEC>>),
}

impl<SPEC> Expression<SPEC>
    where SPEC: Specification<Expr=Self>,
          SPEC::Expr: ScopeContextPresentable {
    pub(crate) fn expression(aspect: FFIAspect, kind: ConversionExpressionKind, expr: Self) -> Self {
        Self::ConversionExpr(aspect, kind, expr.into())
    }
    pub(crate) fn expression_from(kind: ConversionExpressionKind, expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::From, kind, expr.into())
    }
    pub(crate) fn expression_to(kind: ConversionExpressionKind, expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::To, kind, expr.into())
    }
    pub(crate) fn expression_drop(kind: ConversionExpressionKind, expr: Self) -> Self {
        Self::ConversionExpr(FFIAspect::Drop, kind, expr.into())
    }

    #[allow(unused)]
    fn cast_expression(aspect: FFIAspect, kind: ConversionExpressionKind, expr: Self, ffi_ty: Type, target_ty: Type) -> Self {
        Self::CastConversionExpr(aspect, kind, expr.into(), ffi_ty, target_ty)
    }
    fn tokens<T: ToTokens>(aspect: FFIAspect, kind: ConversionExpressionKind, expr: T) -> Self {
        Self::ConversionExprTokens(aspect, kind, expr.to_token_stream())
    }
    #[allow(unused)]
    fn cast_tokens<T: ToTokens>(aspect: FFIAspect, kind: ConversionExpressionKind, expr: T, ffi_ty: Type, target_ty: Type) -> Self {
        Self::CastConversionExprTokens(aspect, kind, expr.to_token_stream(), ffi_ty, target_ty)
    }

    pub(crate) fn dict_expr(expr: DictionaryExpr) -> Self {
        Self::DictionaryExpr(expr)
    }
    pub(crate) fn empty_conversion(_context: &FieldTypeLocalContext<SPEC>) -> Self {
        Self::Empty
    }



    pub(crate) fn black_hole<T: ToTokens>(name: T) -> Self {
        Self::InterfacesExpr(InterfacesMethodExpr::BlackHole(name.to_token_stream()))
    }

    pub(crate) fn bypass(context: &FieldTypeLocalContext<SPEC>) -> Self {
        let (_, conversion_type) = context;
        Self::ConversionType(Box::new(conversion_type.clone()))
    }

    pub(crate) fn named_conversion(context: &FieldTypeLocalContext<SPEC>) -> Self {
        let (name, conversion_type) = context;
        Self::NamedComposer((name.to_token_stream(), Box::new(conversion_type.clone())))
    }

    pub(crate) fn terminated(context: &FieldTypeLocalContext<SPEC>) -> Self {
        let (_, conversion_type) = context;
        Self::Terminated(Box::new(conversion_type.clone()))
    }

    pub(crate) fn name(name: &SPEC::Name) -> Self {
        Self::Name(name.clone())
    }

    pub(crate) fn obj_name(name: &SPEC::Name) -> Self {
        Self::ObjName(name.clone())
    }

    pub(crate) fn ffi_ref_with_name(name: &SPEC::Name) -> Self {
        Self::FfiRefWithName(name.clone())
    }

    pub(crate) fn deref_tokens<T: ToTokens>(token_stream: T) -> Self {
        Self::DictionaryExpr(DictionaryExpr::Deref(token_stream.to_token_stream()))
    }

    pub(crate) fn deref_expr(expr: Self) -> Self {
        Self::DerefExpr(expr.into())
    }
    pub(crate) fn deref_ref(expr: Self) -> Self {
        Self::DerefRef(expr.into())
    }
    pub(crate) fn deref_mut_ref(expr: Self) -> Self {
        Self::DerefMutRef(expr.into())
    }

    pub(crate) fn map_o_expr(mapper: Self) -> Self {
        Self::MapExpression(Expression::DictionaryName(DictionaryName::O).into(), mapper.into())
    }

    pub(crate) fn from_lambda(expr: Self, args: CommaPunctuated<SPEC::Name>) -> Self {
        Self::FromLambda(expr.into(), args)
    }

    #[allow(unused)]
    pub(crate) fn from_ptr_clone(expr: Self) -> Self {
        Self::FromPtrClone(expr.into())
    }
    #[allow(unused)]
    pub(crate) fn from_ptr_read(expr: Self) -> Self {
        Self::FromPtrRead(expr.into())
    }

    pub(crate) fn map_into_box(expr: Self) -> Self {
        Self::MapIntoBox(expr.into())
    }

    pub(crate) fn from_raw_box(expr: Self) -> Self {
        Self::FromRawBox(expr.into())
    }

    pub(crate) fn boxed_tokens<T: ToTokens>(expr: T) -> Self {
        Self::InterfacesExpr(InterfacesMethodExpr::Boxed(expr.to_token_stream()))
    }

    pub(crate) fn boxed(expr: Self) -> Self {
        Self::Boxed(expr.into())
    }
    pub(crate) fn new_smth(expr: Self, smth: impl ToTokens) -> Self {
        Self::NewSmth(expr.into(), smth.to_token_stream())
    }
    pub(crate) fn new_box(expr: Self) -> Self {
        Self::new_smth(expr, DictionaryExpr::Box)
    }
    pub(crate) fn new_cow(expr: Self) -> Self {
        Self::NewCow(expr.into())
    }
    pub(crate) fn cow_into_owned(expr: Self) -> Self {
        Self::CowIntoOwned(expr.into())
    }

    pub(crate) fn destroy_string<T: ToTokens>(expr: Self, token_stream: T) -> Self {
        Self::DestroyString(expr.into(), token_stream.to_token_stream())
    }
    pub(crate) fn destroy_big_int<F: ToTokens, T: ToTokens>(expr: Self, ffi_ty: F, target_ty: T) -> Self {
        Self::DestroyBigInt(expr.into(), ffi_ty.to_token_stream(), target_ty.to_token_stream())
    }




    pub(crate) fn from_complex(expr: Self) -> Self {
        Self::expression(FFIAspect::From, ConversionExpressionKind::Complex, expr)
    }

    pub(crate) fn from_complex_opt(expr: Self) -> Self {
        Self::expression(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr)
    }

    #[allow(unused)]
    pub(crate) fn from_primitive_opt(expr: Self) -> Self {
        Self::expression(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt, expr)
    }

    pub(crate) fn from_primitive(expr: Self) -> Self {
        Self::expression(FFIAspect::From, ConversionExpressionKind::Primitive, expr)
    }

    pub(crate) fn destroy_complex(expr: Self) -> Self {
        Self::expression(FFIAspect::Drop, ConversionExpressionKind::Complex, expr)
    }

    pub(crate) fn from_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::Complex, expr)
    }

    pub(crate) fn from_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::ComplexOpt, expr)
    }

    pub(crate) fn from_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::Primitive, expr)
    }
    pub(crate) fn from_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt, expr)
    }

    #[allow(unused)]
    pub(crate) fn from_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::PrimitiveGroup, expr)
    }

    #[allow(unused)]
    pub(crate) fn from_primitive_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::PrimitiveOptGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn from_opaque_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::OpaqueOptGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn from_opaque_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::OpaqueGroup, expr)
    }

    #[allow(unused)]
    pub(crate) fn from_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::ComplexGroup, expr)
    }

    #[allow(unused)]
    pub(crate) fn from_complex_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::From, ConversionExpressionKind::ComplexOptGroup, expr)
    }



    pub(crate) fn ffi_to_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::Primitive, expr)
    }
    pub(crate) fn ffi_to_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::Complex, expr)
    }
    pub(crate) fn ffi_to_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::ComplexOpt, expr)
    }
    pub(crate) fn ffi_to_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOpt, expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::PrimitiveGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_primitive_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::PrimitiveOptGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_opaque_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::OpaqueOptGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_opaque_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::OpaqueGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::ComplexGroup, expr)
    }


    #[allow(unused)]
    pub(crate) fn ffi_to_complex_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::To, ConversionExpressionKind::ComplexOptGroup, expr)
    }
    pub(crate) fn destroy_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::Drop, ConversionExpressionKind::PrimitiveOpt, expr)
    }
    pub(crate) fn destroy_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::Drop, ConversionExpressionKind::Complex, expr)
    }
    pub(crate) fn destroy_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::Drop, ConversionExpressionKind::Primitive, expr)
    }
    pub(crate) fn destroy_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::Drop, ConversionExpressionKind::ComplexOpt, expr)
    }
    #[allow(unused)]
    pub(crate) fn destroy_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::Drop, ConversionExpressionKind::PrimitiveGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn destroy_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(FFIAspect::Drop, ConversionExpressionKind::ComplexGroup, expr)
    }
    #[allow(unused)]
    pub(crate) fn destroy_string_group_tokens<T: ToTokens>(expr: T) -> Self {
        Expression::DestroyStringGroup(expr.to_token_stream())
    }

    pub(crate) fn cast_from(expr: Self, kind: ConversionExpressionKind, ffi_type: Type, target_type: Type) -> Self {
        Self::CastConversionExpr(FFIAspect::From, kind, expr.into(), ffi_type, target_type)
    }
    pub(crate) fn cast_to(expr: Self, kind: ConversionExpressionKind, ffi_type: Type, target_type: Type) -> Self {
        Self::CastConversionExpr(FFIAspect::To, kind, expr.into(), ffi_type, target_type)
    }
    pub(crate) fn cast_destroy<T: ToType, U: ToType>(expr: Self, kind: ConversionExpressionKind, ffi_type: T, target_type: U) -> Self {
        Self::CastConversionExpr(FFIAspect::Drop, kind, expr.into(), ffi_type.to_type(), target_type.to_type())
    }

    pub(crate) fn named<T: ToTokens>(name: T, expr: Self) -> Self {
        Self::Named((name.to_token_stream(), expr.into()))
    }
}

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

