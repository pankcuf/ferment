use crate::lang::{RustSpecification, Specification};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, Field, FieldMutability, GenericArgument, PathArguments, PathSegment, Type, TypeParamBound, TypePath, Visibility};
use syn::__private::TokenStream2;
use crate::ast::AddPunctuated;
use crate::composable::{FieldComposer, GenericBoundsModel};
use crate::composer::{SourceComposable, VariableComposer};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType, AsType};
use crate::presentable::{Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SmartPointerPresentableContext};
use crate::presentation::DictionaryExpr;

#[allow(unused)]
pub struct GenericArgPresentation<SPEC>
    where SPEC: Specification {
    pub ty: SPEC::Var,
    // pub alloc: SPEC::Expr,
    pub destructor: SPEC::Expr,
    pub from_conversion: SPEC::Expr,
    pub to_conversion: SPEC::Expr,
}

impl<SPEC> Debug for GenericArgPresentation<SPEC>
    where SPEC: Specification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericArgPresentation({})", self.ty.to_token_stream()))
    }
}
impl<SPEC> Display for GenericArgPresentation<SPEC>
    where SPEC: Specification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<SPEC> GenericArgPresentation<SPEC>
    where SPEC: Specification {
    #[allow(unused)]
    pub fn new(ty: SPEC::Var, destructor: SPEC::Expr, from_conversion: SPEC::Expr, to_conversion: SPEC::Expr) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}
#[derive(Clone)]
pub enum MixinKind {
    Generic(GenericTypeKind),
    Bounds(GenericBoundsModel)
}

impl Debug for MixinKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MixinKind::Generic(kind) => format!("MixinKind::Generic({})", kind.to_token_stream()),
            MixinKind::Bounds(model) => format!("MixinKind::Bounds({})", model),
        }.as_str())
    }
}
impl Display for MixinKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for MixinKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MixinKind::Generic(kind), MixinKind::Generic(other_kind)) =>
                kind.eq(other_kind),
            (MixinKind::Bounds(bounds), MixinKind::Bounds(other_bounds)) =>
                bounds.eq(other_bounds),
            _ => false
        }
    }
}

impl Eq for MixinKind {}

impl Hash for MixinKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MixinKind::Generic(kind) => {
                kind.to_token_stream().to_string().hash(state);
            }
            MixinKind::Bounds(model) => {
                model.as_type().to_token_stream().to_string().hash(state);
                model.bounds.iter().for_each(|bound| bound.to_token_stream().to_string().hash(state));
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum CallbackKind {
    FnOnce(Type),
    Fn(Type),
    FnMut(Type),
    FnPointer(Type),
}
impl Debug for CallbackKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("CallbackKind::{}({})", match self {
            CallbackKind::FnOnce(_) => "FnOnce",
            CallbackKind::Fn(_) => "Fn",
            CallbackKind::FnMut(_) => "FnMut",
            CallbackKind::FnPointer(_) => "FnPointer",
        }, self.to_token_stream()))
    }
}

impl Display for CallbackKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for CallbackKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty.to_tokens(tokens),
        }
    }
}

impl ToType for CallbackKind {
    fn to_type(&self) -> Type {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty.clone(),
        }
    }
}

impl CallbackKind {
    pub fn ty(&self) -> &Type {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty,
        }
    }
    pub fn ty_mut(&mut self) -> &mut Type {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum SmartPointerKind {
    Box(Type),
    Cell(Type),
    Rc(Type),
    Arc(Type),
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

#[derive(Clone, PartialEq, Eq)]
pub enum GenericTypeKind {
    Map(Type),
    Group(Type),
    Result(Type),
    Box(Type),
    SmartPointer(SmartPointerKind),
    Cow(Type),
    AnyOther(Type),
    Array(Type),
    Slice(Type),
    Tuple(Type),
    Optional(Type),
    Callback(CallbackKind),
    TraitBounds(AddPunctuated<TypeParamBound>),
}
impl Debug for GenericTypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericTypeKind::{}({})", match self {
            GenericTypeKind::Map(_) => "Map",
            GenericTypeKind::Group(_) => "Group",
            GenericTypeKind::Result(_) => "Result",
            GenericTypeKind::Box(_) => "Box",
            GenericTypeKind::Cow(_) => "Cow",
            GenericTypeKind::SmartPointer(_) => "SmartPointer",
            GenericTypeKind::AnyOther(_) => "AnyOther",
            GenericTypeKind::Array(_) => "Array",
            GenericTypeKind::Slice(_) => "Slice",
            GenericTypeKind::Tuple(_) => "Tuple",
            GenericTypeKind::Callback(_) => "Callback",
            GenericTypeKind::TraitBounds(_) => "TraitBounds",
            GenericTypeKind::Optional(_) => "Optional"
        }, self.to_token_stream()))
    }
}
impl Display for GenericTypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for GenericTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GenericTypeKind::Map(ty) |
            GenericTypeKind::Group(ty) |
            GenericTypeKind::Result(ty) |
            GenericTypeKind::Box(ty) |
            GenericTypeKind::Array(ty) |
            GenericTypeKind::Slice(ty) |
            GenericTypeKind::AnyOther(ty) |
            GenericTypeKind::Optional(ty) |
            GenericTypeKind::Cow(ty) |
            GenericTypeKind::Tuple(ty) => ty.to_tokens(tokens),
            GenericTypeKind::SmartPointer(kind) => kind.to_tokens(tokens),
            GenericTypeKind::Callback(kind) => kind.to_tokens(tokens),
            GenericTypeKind::TraitBounds(bounds) => bounds.to_tokens(tokens),
        }
    }
}

impl GenericTypeKind {
    pub fn ty(&self) -> Option<&Type> {
        match self {
            GenericTypeKind::Map(ty) |
            GenericTypeKind::Group(ty) |
            GenericTypeKind::Result(ty) |
            GenericTypeKind::Box(ty) |
            GenericTypeKind::Array(ty) |
            GenericTypeKind::Slice(ty) |
            GenericTypeKind::AnyOther(ty) |
            GenericTypeKind::Cow(ty) |
            GenericTypeKind::Tuple(ty) => Some(ty),
            GenericTypeKind::Optional(Type::Path(TypePath { qself: _, path })) => match path.segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => Some(ty),
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", self.to_token_stream()),
                },
                _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", self.to_token_stream()),
            }
            GenericTypeKind::Callback(kind) => Some(kind.ty()),
            GenericTypeKind::TraitBounds(_) => {
                // TODO: Make mixin here
                None
            },
            GenericTypeKind::SmartPointer(ptr) => Some(ptr.as_type()),
            conversion => panic!("TODO: Non-supported generic conversion: {}", conversion),
        }
    }
}

impl Resolve<Field> for FieldComposer<RustSpecification> {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<Field> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> Field {
        let FieldComposer { attrs, name, kind, .. } = self;
        Field {
            attrs: attrs.clone(),
            vis: Visibility::Public(Default::default()),
            mutability: FieldMutability::None,
            ident: Some(name.mangle_ident_default()),
            colon_token: Some(Default::default()),
            ty: VariableComposer::<RustSpecification>::new(kind.to_type())
                .compose(source)
                .to_type()
        }
    }
}
