use crate::lang::{RustSpecification, Specification};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, Field, FieldMutability, GenericArgument, PathArguments, PathSegment, Type, TypeParamBound, TypePath, Visibility};
use syn::__private::TokenStream2;
use crate::ast::AddPunctuated;
use crate::composable::{FieldComposer, GenericBoundsModel};
use crate::composer::{Composer, SourceComposable, VariableComposer};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType, AsType};
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::Name;

pub type ExpressionComposer<SPEC> = Composer<TokenStream2, <SPEC as Specification>::Expr>;
#[allow(unused)]
pub type ExprComposer<SPEC> = dyn Fn(TokenStream2) -> <SPEC as Specification>::Expr;

pub const fn primitive_opt_arg_composer<SPEC>() -> GenericArgComposer<SPEC>
    where SPEC: Specification<Expr = Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    GenericArgComposer::new(Some(Expression::from_primitive_opt_tokens), Some(Expression::ffi_to_primitive_opt_tokens), Some(Expression::destroy_primitive_opt_tokens))
}
#[allow(unused)]
pub const fn complex_arg_composer<SPEC>() -> GenericArgComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    GenericArgComposer::new(Some(Expression::from_complex_tokens), Some(Expression::ffi_to_complex_tokens), Some(Expression::destroy_complex_tokens))
}
pub const fn complex_opt_arg_composer<SPEC>() -> GenericArgComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    GenericArgComposer::new(Some(Expression::from_complex_opt_tokens), Some(Expression::ffi_to_complex_opt_tokens), Some(Expression::destroy_complex_opt_tokens))
}
pub const fn result_complex_arg_composer<SPEC>() -> GenericArgComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    GenericArgComposer::new(Some(Expression::from_complex_tokens), Some(Expression::ffi_to_complex_tokens), Some(Expression::destroy_complex_opt_tokens))
}

pub struct GenericArgComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    pub from_composer: Option<ExpressionComposer<SPEC>>,
    pub to_composer: Option<ExpressionComposer<SPEC>>,
    pub destroy_composer: Option<ExpressionComposer<SPEC>>,
}

impl<SPEC> GenericArgComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    pub const fn new(
        from_composer: Option<ExpressionComposer<SPEC>>,
        to_composer: Option<ExpressionComposer<SPEC>>,
        destroy_composer: Option<ExpressionComposer<SPEC>>
    ) -> Self {
        Self { from_composer, to_composer, destroy_composer }
    }

    pub fn from<T: ToTokens>(&self, expr: T) -> SPEC::Expr {
        self.from_composer.map(|c| c(expr.to_token_stream()))
            .unwrap_or_default()
    }
    pub fn to<T: ToTokens>(&self, expr: T) -> SPEC::Expr {
        self.to_composer.map(|c| c(expr.to_token_stream()))
            .unwrap_or_default()
    }
    pub fn destroy<T: ToTokens>(&self, expr: T) -> SPEC::Expr {
        self.destroy_composer.map(|c| c(expr.to_token_stream()))
            .unwrap_or_default()
    }
}

pub type GenericNestedArgComposer<SPEC> = fn(arg_name: &Name<SPEC>, arg_ty: &Type) -> GenericArgPresentation<SPEC>;

#[allow(unused)]
pub trait GenericNamedArgComposer<SPEC>
    where SPEC: Specification {
    fn compose_with(&self, name: &Name<SPEC> , composer: GenericNestedArgComposer<SPEC>) -> GenericArgPresentation<SPEC>;
}

impl<SPEC> GenericNamedArgComposer<SPEC> for Type
    where SPEC: Specification {
    fn compose_with(&self, name: &Name<SPEC> , composer: GenericNestedArgComposer<SPEC>) -> GenericArgPresentation<SPEC> {
        composer(name, self)
    }
}

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
pub enum GenericTypeKind {
    Map(Type),
    Group(Type),
    Result(Type),
    Box(Type),
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
            GenericTypeKind::Tuple(ty) => ty.to_tokens(tokens),
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
