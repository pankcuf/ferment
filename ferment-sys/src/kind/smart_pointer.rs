use std::fmt::{Debug, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Type;
use crate::ext::{AsType, GenericNestedArg, ToType};
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, Expression, ExpressionComposable, ScopeContextPresentable, SmartPointerPresentableContext};
use crate::presentation::DictionaryExpr;


#[derive(Clone, PartialEq, Eq)]
pub enum SmartPointerKind {
    Box(Type),
    Rc(Type),
    Arc(Type),

    Cell(Type),
    RefCell(Type),
    UnsafeCell(Type),

    Mutex(Type),
    OnceLock(Type),
    RwLock(Type),

    Pin(Type),
}
impl SmartPointerKind {
    pub fn is_once_lock(&self) -> bool {
        match self {
            SmartPointerKind::OnceLock(_) => true,
            _ => false
        }
    }
    pub fn dictionary_type(&self) -> DictionaryExpr {
        match self {
            SmartPointerKind::Box(_) => DictionaryExpr::Box,
            SmartPointerKind::Arc(_) => DictionaryExpr::Arc,
            SmartPointerKind::Rc(_) => DictionaryExpr::Rc,
            SmartPointerKind::Mutex(_) => DictionaryExpr::Mutex,
            SmartPointerKind::OnceLock(_) => DictionaryExpr::OnceLock,
            SmartPointerKind::RwLock(_) => DictionaryExpr::RwLock,
            SmartPointerKind::Cell(_) => DictionaryExpr::Cell,
            SmartPointerKind::RefCell(_) => DictionaryExpr::RefCell,
            SmartPointerKind::UnsafeCell(_) => DictionaryExpr::UnsafeCell,
            _ => panic!("SmartPointerKind::dictionary_type")
        }
    }

    pub fn wrapped_arg_type(&self) -> Option<&Type> {
        match self {
            SmartPointerKind::Rc(_) |
            SmartPointerKind::Arc(_) =>
                self.as_type().maybe_first_nested_type_ref()?.maybe_first_nested_type_ref(),
            _ => self.as_type().maybe_first_nested_type_ref()
        }
    }
    pub fn wrap_alloc<SPEC, T>(&self, expr: Expression<SPEC>) -> Expression<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Expression<SPEC>: ScopeContextPresentable,
          T: ToTokens {
        match self {
            SmartPointerKind::Rc(_) |
            SmartPointerKind::Arc(_) => Expression::<SPEC>::new_smth(expr, self.dictionary_type()),
            _ => expr,
        }
    }
    pub fn wrap_from<SPEC, T>(&self, expr: T) -> Expression<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Expression<SPEC>: ScopeContextPresentable,
          T: ToTokens {
        match self {
            SmartPointerKind::Rc(_) => Expression::dict_expr(DictionaryExpr::from_rc(DictionaryExpr::deref_ref(expr))),
            SmartPointerKind::Arc(_) => Expression::dict_expr(DictionaryExpr::from_arc(DictionaryExpr::deref_ref(expr))),
            SmartPointerKind::Mutex(_) |
            SmartPointerKind::OnceLock(_) |
            SmartPointerKind::RwLock(_) |
            SmartPointerKind::Cell(_) |
            SmartPointerKind::RefCell(_) |
            SmartPointerKind::UnsafeCell(_) => Expression::dict_expr(DictionaryExpr::from_ptr_read(expr)),
            _ => Expression::simple(expr),
        }
    }
    pub fn wrap_arg_to<SPEC>(&self, expr: Expression<SPEC>) -> Expression<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Expression<SPEC>: ScopeContextPresentable {
        match self {
            SmartPointerKind::Cell(_) => expr,
            _ => Expression::Clone(expr.into()),
        }
    }
    pub fn binding_presentable<SPEC: Specification>(&self, aspect: &Aspect<SPEC::TYC>, attrs: &SPEC::Attr, lifetimes: &SPEC::Lt, context: SmartPointerPresentableContext<SPEC>) -> BindingPresentableContext<SPEC> {
        BindingPresentableContext::smart_pointer(self, aspect, attrs, lifetimes, context)

    }
}
impl Debug for SmartPointerKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("SmartPointerKind::{}({})", match self {
            SmartPointerKind::Box(_) => "Box",
            SmartPointerKind::Rc(_) => "Rc",
            SmartPointerKind::Arc(_) => "Arc",
            SmartPointerKind::Cell(_) => "Cell",
            SmartPointerKind::RefCell(_) => "RefCell",
            SmartPointerKind::UnsafeCell(_) => "UnsafeCell",
            SmartPointerKind::Mutex(_) => "Mutex",
            SmartPointerKind::RwLock(_) => "RwLock",
            SmartPointerKind::OnceLock(_) => "OnceLock",
            SmartPointerKind::Pin(_) => "Pin",
        }, self.to_token_stream()))
    }
}

impl ToTokens for SmartPointerKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SmartPointerKind::Box(ty) |
            SmartPointerKind::Rc(ty) |
            SmartPointerKind::Arc(ty) |
            SmartPointerKind::Cell(ty) |
            SmartPointerKind::RefCell(ty) |
            SmartPointerKind::UnsafeCell(ty) |
            SmartPointerKind::Mutex(ty) |
            SmartPointerKind::RwLock(ty) |
            SmartPointerKind::OnceLock(ty) |
            SmartPointerKind::Pin(ty) => ty.to_tokens(tokens),
        }
    }
}

impl<'a> AsType<'a> for SmartPointerKind{
    fn as_type(&'a self) -> &'a Type {
        match self {
            SmartPointerKind::Box(ty) |
            SmartPointerKind::Rc(ty) |
            SmartPointerKind::Arc(ty) |
            SmartPointerKind::Cell(ty) |
            SmartPointerKind::RefCell(ty) |
            SmartPointerKind::UnsafeCell(ty) |
            SmartPointerKind::Mutex(ty) |
            SmartPointerKind::RwLock(ty) |
            SmartPointerKind::OnceLock(ty) |
            SmartPointerKind::Pin(ty) => ty,
        }
    }
}
impl ToType for SmartPointerKind {
    fn to_type(&self) -> Type {
        match self {
            SmartPointerKind::Box(ty) |
            SmartPointerKind::Rc(ty) |
            SmartPointerKind::Arc(ty) |
            SmartPointerKind::Cell(ty) |
            SmartPointerKind::RefCell(ty) |
            SmartPointerKind::UnsafeCell(ty) |
            SmartPointerKind::Mutex(ty) |
            SmartPointerKind::OnceLock(ty) |
            SmartPointerKind::RwLock(ty) |
            SmartPointerKind::Pin(ty) => ty.clone(),
        }
    }
}
