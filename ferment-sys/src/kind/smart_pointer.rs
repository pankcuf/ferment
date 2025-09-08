use std::fmt::{Debug, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Type;
use crate::composer::SignatureAspect;
use crate::ext::{AsType, ExpressionComposable, GenericNestedArg, ToType};
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SmartPointerPresentableContext};
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
            Self::OnceLock(_) => true,
            _ => false
        }
    }
    pub fn dictionary_type(&self) -> DictionaryExpr {
        match self {
            Self::Box(_) => DictionaryExpr::Box,
            Self::Arc(_) => DictionaryExpr::Arc,
            Self::Rc(_) => DictionaryExpr::Rc,
            Self::Mutex(_) => DictionaryExpr::Mutex,
            Self::OnceLock(_) => DictionaryExpr::OnceLock,
            Self::RwLock(_) => DictionaryExpr::RwLock,
            Self::Cell(_) => DictionaryExpr::Cell,
            Self::RefCell(_) => DictionaryExpr::RefCell,
            Self::UnsafeCell(_) => DictionaryExpr::UnsafeCell,
            _ => panic!("SmartPointerKind::dictionary_type")
        }
    }

    pub fn wrapped_arg_type(&self) -> Option<&Type> {
        match self {
            Self::Rc(_) |
            Self::Arc(_) =>
                self.as_type().maybe_first_nested_type_ref()?.maybe_first_nested_type_ref(),
            _ => self.as_type().maybe_first_nested_type_ref()
        }
    }
    pub fn wrap_alloc<SPEC, T>(&self, expr: SPEC::Expr) -> SPEC::Expr
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Expression<SPEC>: ScopeContextPresentable,
          T: ToTokens {
        match self {
            Self::Rc(_) |
            Self::Arc(_) => SPEC::Expr::new_smth(expr, self.dictionary_type()),
            _ => expr,
        }
    }
    pub fn wrap_from<SPEC, T>(&self, expr: T) -> SPEC::Expr
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Expression<SPEC>: ScopeContextPresentable,
          T: ToTokens {
        match self {
            Self::Rc(_) => SPEC::Expr::dict_expr(DictionaryExpr::from_rc(DictionaryExpr::deref_ref(expr))),
            Self::Arc(_) => SPEC::Expr::dict_expr(DictionaryExpr::from_arc(DictionaryExpr::deref_ref(expr))),
            Self::Mutex(_) |
            Self::OnceLock(_) |
            Self::RwLock(_) |
            Self::Cell(_) |
            Self::RefCell(_) |
            Self::UnsafeCell(_) => SPEC::Expr::dict_expr(DictionaryExpr::from_ptr_read(expr)),
            _ => SPEC::Expr::simple(expr),
        }
    }
    pub fn wrap_arg_to<SPEC>(&self, expr: Expression<SPEC>) -> Expression<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          Expression<SPEC>: ScopeContextPresentable {
        match self {
            Self::Cell(_) => expr,
            _ => expr.cloned(),
        }
    }
    pub fn binding_presentable<SPEC: Specification>(
        &self,
        aspect: &Aspect<SPEC::TYC>,
        signature_aspect: &SignatureAspect<SPEC>,
        context: SmartPointerPresentableContext<SPEC>
    ) -> BindingPresentableContext<SPEC> {
        BindingPresentableContext::smart_pointer(self, aspect, signature_aspect, context)
    }
}
impl Debug for SmartPointerKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("SmartPointerKind::{}({})", match self {
            Self::Box(_) => "Box",
            Self::Rc(_) => "Rc",
            Self::Arc(_) => "Arc",
            Self::Cell(_) => "Cell",
            Self::RefCell(_) => "RefCell",
            Self::UnsafeCell(_) => "UnsafeCell",
            Self::Mutex(_) => "Mutex",
            Self::RwLock(_) => "RwLock",
            Self::OnceLock(_) => "OnceLock",
            Self::Pin(_) => "Pin",
        }, self.to_token_stream()))
    }
}

impl ToTokens for SmartPointerKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.as_type().to_tokens(tokens);
    }
}

impl<'a> AsType<'a> for SmartPointerKind{
    fn as_type(&'a self) -> &'a Type {
        match self {
            Self::Box(ty) |
            Self::Rc(ty) |
            Self::Arc(ty) |
            Self::Cell(ty) |
            Self::RefCell(ty) |
            Self::UnsafeCell(ty) |
            Self::Mutex(ty) |
            Self::RwLock(ty) |
            Self::OnceLock(ty) |
            Self::Pin(ty) => ty,
        }
    }
}
impl ToType for SmartPointerKind {
    fn to_type(&self) -> Type {
        self.as_type().clone()
    }
}
