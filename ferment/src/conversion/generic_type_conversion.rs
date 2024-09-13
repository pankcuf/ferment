use crate::lang::LangAttrSpecification;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, Attribute, Field, GenericArgument, PathArguments, PathSegment, Type, TypeParamBound, TypePath, Visibility, VisPublic};
use syn::__private::TokenStream2;
use crate::ast::{AddPunctuated, Depunctuated};
use crate::composable::{FieldComposer, GenericBoundsModel};
use crate::composer::{ComposerPresenter, Composer, VariableComposer};
use crate::context::ScopeContext;
use crate::conversion::TypeKind;
use crate::ext::{Mangle, Resolve, ToType, AsType};
use crate::presentable::{Expression, RustExpression};
use crate::presentation::{DictionaryExpr, RustFermentate, FFIConversionFromMethod, FFIConversionToMethod, InterfacesMethodExpr, Name};

// #[allow(unused)]
// pub type TyComposer<'a> = ComposerPresenter<(&'a Type, &'a ScopeContext), Type>;

// #[allow(unused)]
// pub type VarComposer<T: FFIVarResolve> = ComposerPresenter<(T, &'static ScopeContext), Type>;


pub type ExpressionComposer<LANG, SPEC> = ComposerPresenter<TokenStream2, Expression<LANG, SPEC>>;
pub type RustExpressionComposer = ComposerPresenter<TokenStream2, RustExpression>;
#[allow(unused)]
pub type ExprComposer<LANG, SPEC> = dyn Fn(TokenStream2) -> Expression<LANG, SPEC>;
#[allow(unused)]
pub type InterfacesMethodComposer = ComposerPresenter<TokenStream2, InterfacesMethodExpr>;

// pub const fn from_primitive<LANG: Clone, SPEC: LangAttrSpecification<LANG>>() -> ExpressionComposer<LANG, SPEC> { |expr| Expression::Simple(expr) }
// pub const fn to_primitive<LANG: Clone, SPEC: LangAttrSpecification<LANG>>() -> ExpressionComposer<LANG, SPEC> { |expr| Expression::Simple(expr) }
// pub const fn destroy_primitive<LANG: Clone, SPEC: LangAttrSpecification<LANG>>() -> ExpressionComposer<LANG, SPEC> { |_| Expression::Empty }
// pub const fn from_opaque<LANG: Clone, SPEC: LangAttrSpecification<LANG>>() -> ExpressionComposer<LANG, SPEC> { |_| Expression::Empty }
pub const FROM_PRIMITIVE: RustExpressionComposer = |expr| Expression::Simple(expr);
pub const TO_PRIMITIVE: RustExpressionComposer = |expr| Expression::Simple(expr);
// pub const FROM_LAMBDA: ExpressionComposer = |expr| Expression::Simple(expr);
pub const DESTROY_PRIMITIVE: RustExpressionComposer = |_| Expression::Empty;
pub const FROM_OPAQUE: RustExpressionComposer = |expr| Expression::Deref(expr);
pub const TO_OPAQUE: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::Boxed(expr));
pub const FROM_OPT_PRIMITIVE: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitive(expr));
pub const TO_OPT_PRIMITIVE: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitive(expr));
pub const FROM_COMPLEX: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFrom, expr));
pub const FROM_OPT_COMPLEX: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFromOpt, expr));
pub const TO_COMPLEX: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiTo, expr));
pub const TO_OPT_COMPLEX: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiToOpt, expr));

pub const FROM_PRIMITIVE_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromPrimitiveGroup(expr));
pub const FROM_COMPLEX_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromComplexGroup(expr));
pub const FROM_OPT_PRIMITIVE_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitiveGroup(expr));
pub const FROM_OPT_COMPLEX_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromOptComplexGroup(expr));

pub const TO_PRIMITIVE_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToPrimitiveGroup(expr));
pub const TO_COMPLEX_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToComplexGroup(expr));
pub const TO_OPT_PRIMITIVE_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitiveGroup(expr));
pub const TO_OPT_COMPLEX_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToOptComplexGroup(expr));

pub const DESTROY_PRIMITIVE_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxVecPtr(expr));
pub const DESTROY_COMPLEX_GROUP: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxAnyVecPtr(expr));
pub const DESTROY_COMPLEX: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxAny(expr));
pub const DESTROY_OPT_COMPLEX: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxAnyOpt(expr));
// Expression::DestroyOpt(expr.into())
pub const DESTROY_OPT_PRIMITIVE: RustExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::DestroyOptPrimitive(expr));

#[allow(unused)]
pub const PRIMITIVE_ARG_COMPOSER: RustGenericArgComposer = GenericArgComposer::new(FROM_PRIMITIVE, TO_PRIMITIVE, DESTROY_PRIMITIVE);
#[allow(unused)]
pub const PRIMITIVE_OPT_ARG_COMPOSER: RustGenericArgComposer = GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE);

#[allow(unused)]
pub const COMPLEX_ARG_COMPOSER: RustGenericArgComposer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX);
pub const COMPLEX_OPT_ARG_COMPOSER: RustGenericArgComposer = GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX);

// pub const LAMBDA_COMPOSER: GenericArgComposer = GenericArgComposer::new()
pub type RustGenericArgComposer = GenericArgComposer<RustFermentate, Vec<Attribute>>;
pub struct GenericArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    // pub ty: Type,
    pub from_composer: ExpressionComposer<LANG, SPEC>,
    pub to_composer: ExpressionComposer<LANG, SPEC>,
    pub destroy_composer: ExpressionComposer<LANG, SPEC>,
    // pub ty_composer: TyComposer<'a>,
}

impl<LANG, SPEC> GenericArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub const fn new(
        // ty_composer: TyComposer,
        from_composer: ExpressionComposer<LANG, SPEC>,
        to_composer: ExpressionComposer<LANG, SPEC>,
        destroy_composer: ExpressionComposer<LANG, SPEC>
    ) -> Self {
        Self { from_composer, to_composer, destroy_composer }
    }
    // pub fn ty(&self, ty: &Type, source: &ScopeContext) -> Type {
    //     (self.ty_composer)((ty, source))
    // }
    pub fn from(&self, expr: TokenStream2) -> Expression<LANG, SPEC> {
        (self.from_composer)(expr)
    }
    pub fn to(&self, expr: TokenStream2) -> Expression<LANG, SPEC> {
        (self.to_composer)(expr)
    }
    pub fn destroy(&self, expr: TokenStream2) -> Expression<LANG, SPEC> {
        (self.destroy_composer)(expr)
    }
}

pub type GenericNestedArgComposer = fn(arg_name: &Name, arg_ty: &Type) -> GenericArgPresentation;

#[allow(unused)]
pub trait GenericNamedArgComposer {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer) -> GenericArgPresentation;
}

impl GenericNamedArgComposer for Type {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer) -> GenericArgPresentation {
        composer(name, self)
    }
}

pub struct GenericArgPresentation {
    pub ty: Type,
    pub destructor: RustExpression,
    pub from_conversion: RustExpression,
    pub to_conversion: RustExpression,
}

impl Debug for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericArgPresentation({})", self.ty.to_token_stream()))
    }
}
impl Display for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl GenericArgPresentation {
    pub fn new(ty: Type, destructor: Expression<RustFermentate, Vec<Attribute>>, from_conversion: Expression<RustFermentate, Vec<Attribute>>, to_conversion: Expression<RustFermentate, Vec<Attribute>>) -> Self {
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

impl Resolve<Field> for FieldComposer<RustFermentate, Vec<Attribute>>  {
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

pub(crate) fn dictionary_generic_arg_pair(name: Name, field_name: Name, ty: &Type, source: &ScopeContext) -> (Type, Depunctuated<GenericArgPresentation>) {
    let ty: Type = ty.resolve(source);
    let (destroy_expr,
        from_expr,
        to_expr) = match TypeKind::from(&ty) {
        TypeKind::Primitive(..) => (
            Expression::Empty,
            Expression::FfiRefWithName(name.clone()),
            Expression::ObjName(field_name).into()
        ),
        _ => (
            DESTROY_COMPLEX(DictionaryExpr::SelfProp(name.to_token_stream()).to_token_stream()),
            Expression::From(Expression::FfiRefWithName(name.clone()).into()),
            Expression::To(Expression::ObjName(field_name).into())
        ),
    };
    (ty.clone(), Depunctuated::from_iter([GenericArgPresentation::new(ty, destroy_expr, from_expr, Expression::Named((name.to_token_stream(), to_expr.into())))]))
}

