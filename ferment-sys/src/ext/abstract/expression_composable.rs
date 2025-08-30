use std::fmt::Debug;
use quote::ToTokens;
use crate::lang::Specification;
use crate::presentable::ScopeContextPresentable;

pub trait ExpressionComposable<SPEC>: Clone + Debug + ScopeContextPresentable
where SPEC: Specification {
    fn simple<T: ToTokens>(tokens: T) -> SPEC::Expr;
    fn simple_expr(expr: SPEC::Expr) -> SPEC::Expr;
    fn leak_box(expr: SPEC::Expr) -> SPEC::Expr;
}