use std::fmt::Debug;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Type;
use crate::ast::CommaPunctuated;
use crate::composer::FieldTypeLocalContext;
use crate::ext::{Conversion, ExpressionComposable, ToType};
use crate::lang::Specification;
use crate::presentable::{ConversionAspect, ConversionExpressionKind, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacesMethodExpr};
#[derive(Clone, Debug)]
pub enum Expression<SPEC>
    where SPEC: Specification<Expr=Self>,
          Self: ScopeContextPresentable {
    ConversionExpr(ConversionAspect, Box<Expression<SPEC>>),
    ConversionExprTokens(ConversionAspect, TokenStream2),
    CastConversionExpr(ConversionAspect, Box<Expression<SPEC>>, /*ffi*/Type, /*target*/Type),
    CastConversionExprTokens(ConversionAspect, TokenStream2, /*ffi*/Type, /*target*/Type),

    Empty,
    Simple(TokenStream2),
    SimpleExpr(Box<Expression<SPEC>>),
    DictionaryName(DictionaryName),
    Name(SPEC::Name),
    FfiRefWithName(SPEC::Name),
    ObjName(SPEC::Name),
    DictionaryExpr(DictionaryExpr),
    InterfacesExpr(InterfacesMethodExpr<TokenStream2>),
    AsRef(Box<Expression<SPEC>>),
    AsMutRef(Box<Expression<SPEC>>),
    DerefRef(Box<Expression<SPEC>>),
    DerefMutRef(Box<Expression<SPEC>>),
    Clone(Box<Expression<SPEC>>),
    FromPtrRead(Box<Expression<SPEC>>),
    DerefExpr(Box<Expression<SPEC>>),
    MapExpr(Box<Expression<SPEC>>, Box<Expression<SPEC>>),
    MapIntoBox(Box<Expression<SPEC>>),
    FromRawBox(Box<Expression<SPEC>>),

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
    pub(crate) fn cloned(self) -> Self {
        Self::Clone(self.into())
    }

    pub(crate) fn conversion_expr(aspect: ConversionAspect, expr: Self) -> Self {
        Self::ConversionExpr(aspect, expr.into())
    }
    pub(crate) fn expression_from(kind: ConversionExpressionKind, expr: Self) -> Self {
        Self::ConversionExpr(ConversionAspect::kind_from(kind), expr.into())
    }
    pub(crate) fn expression_to(kind: ConversionExpressionKind, expr: Self) -> Self {
        Self::ConversionExpr(ConversionAspect::kind_to(kind), expr.into())
    }
    pub(crate) fn expression_drop(kind: ConversionExpressionKind, expr: Self) -> Self {
        Self::ConversionExpr(ConversionAspect::kind_drop(kind), expr.into())
    }

    pub(crate) fn mut_ref(expr: Self) -> Self {
        Self::AsMutRef(expr.into())
    }
    pub(crate) fn r#ref(expr: Self) -> Self {
        Self::AsRef(expr.into())
    }

    fn tokens<T: ToTokens>(aspect: ConversionAspect, expr: T) -> Self {
        Self::ConversionExprTokens(aspect, expr.to_token_stream())
    }

    pub(crate) fn dict_expr(expr: DictionaryExpr) -> Self {
        Self::DictionaryExpr(expr)
    }
    pub(crate) fn interface_expr(expr: InterfacesMethodExpr<TokenStream2>) -> Self {
        Self::InterfacesExpr(expr)
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
        Self::DictionaryExpr(DictionaryExpr::deref(token_stream))
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
        Self::MapExpr(Expression::DictionaryName(DictionaryName::O).into(), mapper.into())
    }

    pub(crate) fn from_lambda(expr: Self, args: CommaPunctuated<SPEC::Name>) -> Self {
        Self::FromLambda(expr.into(), args)
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
        Self::conversion_expr(ConversionAspect::complex_from(), expr)
    }

    pub(crate) fn from_complex_opt(expr: Self) -> Self {
        Self::conversion_expr(ConversionAspect::complex_from_opt(), expr)
    }

    #[allow(unused)]
    pub(crate) fn from_primitive_opt(expr: Self) -> Self {
        Self::conversion_expr(ConversionAspect::primitive_from_opt(), expr)
    }

    pub(crate) fn from_primitive(expr: Self) -> Self {
        Self::conversion_expr(ConversionAspect::primitive_from(), expr)
    }

    pub(crate) fn destroy_complex(expr: Self) -> Self {
        Self::conversion_expr(ConversionAspect::complex_drop(), expr)
    }

    pub(crate) fn from_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_from(), expr)
    }

    pub(crate) fn from_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_from_opt(), expr)
    }

    pub(crate) fn from_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_from(), expr)
    }
    pub(crate) fn from_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_from_opt(), expr)
    }

    #[allow(unused)]
    pub(crate) fn from_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_group_from(), expr)
    }

    #[allow(unused)]
    pub(crate) fn from_primitive_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_group_opt_from(), expr)
    }
    #[allow(unused)]
    pub(crate) fn from_opaque_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::opaque_group_opt_from(), expr)
    }
    #[allow(unused)]
    pub(crate) fn from_opaque_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::opaque_group_from(), expr)
    }

    #[allow(unused)]
    pub(crate) fn from_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_group_from(), expr)
    }

    #[allow(unused)]
    pub(crate) fn from_complex_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_group_opt_from(), expr)
    }



    pub(crate) fn ffi_to_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_to(), expr)
    }
    pub(crate) fn ffi_to_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_to(), expr)
    }
    pub(crate) fn ffi_to_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_to_opt(), expr)
    }
    pub(crate) fn ffi_to_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_to_opt(), expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_group_to(), expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_primitive_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_group_opt_to(), expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_opaque_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::opaque_group_opt_to(), expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_opaque_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::opaque_group_to(), expr)
    }
    #[allow(unused)]
    pub(crate) fn ffi_to_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_group_to(), expr)
    }


    #[allow(unused)]
    pub(crate) fn ffi_to_complex_opt_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_group_opt_to(), expr)
    }
    pub(crate) fn destroy_primitive_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_opt_drop(), expr)
    }
    pub(crate) fn destroy_complex_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_drop(), expr)
    }
    pub(crate) fn destroy_primitive_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_drop(), expr)
    }
    pub(crate) fn destroy_complex_opt_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_opt_drop(), expr)
    }
    #[allow(unused)]
    pub(crate) fn destroy_primitive_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::primitive_group_drop(), expr)
    }
    #[allow(unused)]
    pub(crate) fn destroy_complex_group_tokens<T: ToTokens>(expr: T) -> Self {
        Self::tokens(ConversionAspect::complex_group_drop(), expr)
    }
    #[allow(unused)]
    pub(crate) fn destroy_string_group_tokens<T: ToTokens>(expr: T) -> Self {
        Expression::DestroyStringGroup(expr.to_token_stream())
    }

    #[allow(unused)]
    fn cast_expression<T: ToType, U: ToType>(aspect: ConversionAspect, expr: Self, ffi_ty: T, target_ty: U) -> Self {
        Self::CastConversionExpr(aspect, expr.into(), ffi_ty.to_type(), target_ty.to_type())
    }
    pub(crate) fn cast_from<T: ToType, U: ToType>(expr: Self, kind: ConversionExpressionKind, ffi_type: T, target_type: U) -> Self {
        Self::cast_expression(ConversionAspect::kind_from(kind), expr, ffi_type, target_type)
    }
    pub(crate) fn cast_to<T: ToType, U: ToType>(expr: Self, kind: ConversionExpressionKind, ffi_type: T, target_type: U) -> Self {
        Self::cast_expression(ConversionAspect::kind_to(kind), expr, ffi_type, target_type)
    }
    pub(crate) fn cast_destroy<T: ToType, U: ToType>(expr: Self, kind: ConversionExpressionKind, ffi_type: T, target_type: U) -> Self {
        Self::cast_expression(ConversionAspect::kind_drop(kind), expr, ffi_type, target_type)
    }

    pub(crate) fn named<T: ToTokens>(name: T, expr: Self) -> Self {
        Self::Named((name.to_token_stream(), expr.into()))
    }
}

impl<SPEC> ExpressionComposable<SPEC> for Expression<SPEC>
where SPEC: Specification<Expr=Self>,
      SPEC::Expr: ScopeContextPresentable {
    fn simple<T: ToTokens>(tokens: T) -> SPEC::Expr {
        Expression::Simple(tokens.to_token_stream())
    }
    fn simple_expr(expr: SPEC::Expr) -> SPEC::Expr {
        Expression::SimpleExpr(expr.into())
    }
    fn leak_box(expr: SPEC::Expr) -> SPEC::Expr {
        Expression::LeakBox(expr.into())
    }
}

impl<SPEC> Default for Expression<SPEC>
where SPEC: Specification<Expr=Self>,
      Self: ScopeContextPresentable {
    fn default() -> Self {
        Self::Empty
    }
}
