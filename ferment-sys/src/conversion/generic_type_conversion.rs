use crate::lang::{RustSpecification, Specification};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, Field, GenericArgument, PathArguments, PathSegment, Type, TypeParamBound, TypePath, Visibility, VisPublic};
use syn::__private::TokenStream2;
use crate::ast::{AddPunctuated, Depunctuated};
use crate::composable::{FieldComposer, GenericBoundsModel};
use crate::composer::{ComposerPresenter, Composer, VariableComposer, FFIAspect};
use crate::context::ScopeContext;
use crate::conversion::TypeKind;
use crate::ext::{Mangle, Resolve, ToType, AsType};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, RustFermentate, Name};

pub type ExpressionComposer<LANG, SPEC> = ComposerPresenter<TokenStream2, <SPEC as Specification<LANG>>::Expr>;
#[allow(unused)]
pub type ExprComposer<LANG, SPEC> = dyn Fn(TokenStream2) -> <SPEC as Specification<LANG>>::Expr;

// pub const fn from_primitive<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG>,
//           Aspect<<SPEC as Specification<LANG>>::TYC>: ScopeContextPresentable {
//     |expr| Expression::from_primitive_tokens(expr)
// }
// pub const fn to_primitive<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| SPEC::Expr::ffi_to_primitive_tokens(expr)
// }
// pub const fn destroy_primitive<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |_| SPEC::Expr::empty()
// }
// pub const fn from_opaque<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::deref_token_stream(expr)
// }
// pub const fn to_opaque<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::boxed(expr)
// }
// pub const fn from_opt_primitive<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::from_opt_primitive_tokens(expr)
// }
// pub const fn to_opt_primitive<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::ffi_to_opt_primitive_tokens(expr)
// }
// pub const fn from_complex<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::ffi_from_tokens(expr)
// }
// pub const fn from_opt_complex<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::from_opt_complex_tokens(expr)
// }
// pub const fn to_complex<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::ffi_to_complex_tokens(expr)
// }
// pub const fn to_opt_complex<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//    |expr| Expression::ffi_to_opt_complex_tokens(expr)
// }
// pub const fn from_primitive_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::from_primitive_group_tokens(expr)
// }
// pub const fn from_complex_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::from_complex_group_tokens(expr)
// }
//
// pub const fn from_opt_primitive_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::from_opt_primitive_group_tokens(expr)
// }
// pub const fn from_opt_complex_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::from_opt_complex_group_tokens(expr)
// }
// pub const fn to_primitive_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::ffi_to_primitive_group_tokens(expr)
// }
// pub const fn to_complex_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::ffi_to_complex_group_tokens(expr)
// }
// pub const fn to_opt_primitive_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::ffi_to_opt_primitive_group_tokens(expr)
// }
// pub const fn to_opt_complex_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::ffi_to_opt_complex_group_tokens(expr)
// }
// pub const fn destroy_primitive_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::destroy_primitive_group_tokens(expr)
// }
// pub const fn destroy_complex_group<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::destroy_complex_group_tokens(expr)
// }
// pub const fn destroy_complex<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::destroy_complex_tokens(expr)
// }
// pub const fn destroy_opt_complex<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::destroy_opt_complex_tokens(expr)
// }
// pub const fn destroy_opt_primitive<LANG, SPEC>() -> ExpressionComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     |expr| Expression::destroy_opt_primitive_tokens(expr)
// }

// #[allow(unused)]
// pub const fn primitive_arg_composer<LANG, SPEC>() -> GenericArgComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     GenericArgComposer::new(Some(Expression::from_primitive_tokens), Some(Expression::ffi_to_primitive_tokens), Some(Expression::destroy_primitive_tokens))
// }
pub const fn primitive_opt_arg_composer<LANG, SPEC>() -> GenericArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    GenericArgComposer::new(Some(Expression::from_primitive_opt_tokens), Some(Expression::ffi_to_primitive_opt_tokens), Some(Expression::destroy_primitive_opt_tokens))
}
#[allow(unused)]
pub const fn complex_arg_composer<LANG, SPEC>() -> GenericArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    GenericArgComposer::new(Some(Expression::from_complex_tokens), Some(Expression::ffi_to_complex_tokens), Some(Expression::destroy_complex_tokens))
}
pub const fn complex_opt_arg_composer<LANG, SPEC>() -> GenericArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    GenericArgComposer::new(Some(Expression::from_complex_opt_tokens), Some(Expression::ffi_to_complex_opt_tokens), Some(Expression::destroy_complex_opt_tokens))
}

pub struct GenericArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    // pub ty: Type,
    pub from_composer: Option<ExpressionComposer<LANG, SPEC>>,
    pub to_composer: Option<ExpressionComposer<LANG, SPEC>>,
    pub destroy_composer: Option<ExpressionComposer<LANG, SPEC>>,
    // pub ty_composer: TyComposer<'a>,
}

impl<LANG, SPEC> GenericArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub const fn new(
        // ty_composer: TyComposer,
        from_composer: Option<ExpressionComposer<LANG, SPEC>>,
        to_composer: Option<ExpressionComposer<LANG, SPEC>>,
        destroy_composer: Option<ExpressionComposer<LANG, SPEC>>
    ) -> Self {
        Self { from_composer, to_composer, destroy_composer }
    }
    // pub fn ty(&self, ty: &Type, source: &ScopeContext) -> Type {
    //     (self.ty_composer)((ty, source))
    // }
    pub fn from(&self, expr: TokenStream2) -> SPEC::Expr {
        self.from_composer.map(|c| c(expr)).unwrap_or(Expression::empty())
        // (self.from_composer)(expr)
    }
    pub fn to(&self, expr: TokenStream2) -> SPEC::Expr {
        self.to_composer.map(|c| c(expr)).unwrap_or(Expression::empty())
        // (self.to_composer)(expr)
    }
    pub fn destroy(&self, expr: TokenStream2) -> SPEC::Expr {
        self.destroy_composer.map(|c| c(expr)).unwrap_or(Expression::empty())

            // (self.destroy_composer)(expr)
    }
}

pub type GenericNestedArgComposer<LANG, SPEC> = fn(arg_name: &Name, arg_ty: &Type) -> GenericArgPresentation<LANG, SPEC>;

#[allow(unused)]
pub trait GenericNamedArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer<LANG, SPEC>) -> GenericArgPresentation<LANG, SPEC>;
}

impl<LANG, SPEC> GenericNamedArgComposer<LANG, SPEC> for Type
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer<LANG, SPEC>) -> GenericArgPresentation<LANG, SPEC> {
        composer(name, self)
    }
}

pub struct GenericArgPresentation<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub ty: Type,
    pub destructor: <SPEC as Specification<LANG>>::Expr,
    pub from_conversion: <SPEC as Specification<LANG>>::Expr,
    pub to_conversion: <SPEC as Specification<LANG>>::Expr,
}

impl<LANG, SPEC> Debug for GenericArgPresentation<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericArgPresentation({})", self.ty.to_token_stream()))
    }
}
impl<LANG, SPEC> Display for GenericArgPresentation<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<LANG, SPEC> GenericArgPresentation<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(ty: Type, destructor: SPEC::Expr, from_conversion: SPEC::Expr, to_conversion: SPEC::Expr) -> Self {
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
    Callback(Type),
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
            GenericTypeKind::Callback(ty) |
            GenericTypeKind::Tuple(ty) => ty.to_tokens(tokens),
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
            GenericTypeKind::Callback(ty) |
            GenericTypeKind::Tuple(ty) => Some(ty),
            GenericTypeKind::Optional(Type::Path(TypePath { qself: _, path })) => match path.segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => Some(ty),
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", self.to_token_stream()),
                },
                _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", self.to_token_stream()),
            }
            GenericTypeKind::TraitBounds(_) => {
                // TODO: Make mixin here
                None
            },
            conversion => panic!("TODO: Non-supported generic conversion: {}", conversion),
        }
    }
}

impl<SPEC> Resolve<Field> for FieldComposer<RustFermentate, SPEC>
    where
        SPEC: RustSpecification {
    fn resolve(&self, source: &ScopeContext) -> Field {
        let FieldComposer { attrs, name, kind, .. } = self;
        // println!("<FieldComposer as Resolve<Field>>::resolve({:?})", self);
        Field {
            attrs: attrs.clone(),
            vis: Visibility::Public(VisPublic { pub_token: Default::default() }),
            ident: Some(name.mangle_ident_default()),
            colon_token: Some(Default::default()),
            ty:
            VariableComposer::from(kind.ty()).compose(source).to_type()

            // VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(kind.ty()).unwrap(), &source.scope))
            //     .compose(source)
            //     .to_type()
        }
    }
}

pub(crate) fn dictionary_generic_arg_pair<LANG, SPEC>(name: Name, field_name: Name, ty: &Type, source: &ScopeContext) -> (Type, Depunctuated<GenericArgPresentation<LANG, SPEC>>)
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    let ty: Type = ty.resolve(source);
    let (kind, destroy_expr,
        from_expr,
        to_expr) = match TypeKind::from(&ty) {
        TypeKind::Primitive(..) => (
            ConversionExpressionKind::Primitive,
            Expression::empty(),
            Expression::ffi_ref_with_name(&name),
            Expression::obj_name(&field_name),
        ),
        _ => (
            ConversionExpressionKind::Complex,
            Expression::DictionaryExpr(DictionaryExpr::SelfProp(name.to_token_stream())),
            Expression::ffi_ref_with_name(&name),
            Expression::obj_name(&field_name)
        ),
    };
    (ty.clone(),
     Depunctuated::from_iter([
         GenericArgPresentation::new(ty,
             Expression::ConversionExpr(FFIAspect::Destroy, kind, destroy_expr.into()),
             Expression::ConversionExpr(FFIAspect::From, kind, from_expr.into()),
             Expression::named(name.to_token_stream(), Expression::ConversionExpr(FFIAspect::To, kind, to_expr.into())))
     ]))
}

