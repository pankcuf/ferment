use std::cell::Ref;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{AngleBracketedGenericArguments, Attribute, BareFnArg, Field, GenericArgument, Generics, ParenthesizedGenericArguments, parse_quote, Pat, PathArguments, PathSegment, PatType, ReturnType, Type, TypeBareFn, TypeParamBound, TypePath, TypeSlice, Visibility, VisPublic};
use syn::__private::TokenStream2;
use syn::token::Brace;
use crate::ast::{AddPunctuated, CommaPunctuated, Depunctuated, ParenWrapped, Wrapped};
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{ComposerPresenter, struct_composer_ctor_root, ParentComposer, STRUCT_COMPOSER_CTOR_NAMED_ITEM, VarComposer, Composer, VariableComposer, CommaPunctuatedOwnedItems};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, expand_attributes, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{Accessory, FFISpecialTypeResolve, FFIVarResolve, GenericNestedArg, Mangle, Resolve, SpecialType, Terminated, ToPath, ToType, CrateExtension};
use crate::presentable::{BindingPresentableContext, ConstructorBindingPresentableContext, ConstructorPresentableContext, Expression, OwnedItemPresentableContext, ScopeContextPresentable};
use crate::presentation::{ArgPresentation, create_callback, create_struct, DictionaryExpr, DictionaryName, DropInterfacePresentation, FFIConversionFromMethod, FFIConversionToMethod, FFIConversionToMethodExpr, FFIObjectPresentation, FFIVecConversionMethodExpr, InterfacePresentation, InterfacesMethodExpr, Name};

// #[allow(unused)]
// pub type TyComposer<'a> = ComposerPresenter<(&'a Type, &'a ScopeContext), Type>;

// #[allow(unused)]
// pub type VarComposer<T: FFIVarResolve> = ComposerPresenter<(T, &'static ScopeContext), Type>;


pub type ExpressionComposer = ComposerPresenter<TokenStream2, Expression>;
#[allow(unused)]
pub type ExprComposer = dyn Fn(TokenStream2) -> Expression;
#[allow(unused)]
pub type InterfacesMethodComposer = ComposerPresenter<TokenStream2, InterfacesMethodExpr>;

pub const FROM_PRIMITIVE: ExpressionComposer = |expr| Expression::Simple(expr);
pub const TO_PRIMITIVE: ExpressionComposer = |expr| Expression::Simple(expr);
// pub const FROM_LAMBDA: ExpressionComposer = |expr| Expression::Simple(expr);
pub const DESTROY_PRIMITIVE: ExpressionComposer = |_| Expression::Empty;
pub const FROM_OPAQUE: ExpressionComposer = |expr| Expression::Deref(expr);
pub const TO_OPAQUE: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::Boxed(expr));
pub const FROM_OPT_PRIMITIVE: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitive(expr));
pub const TO_OPT_PRIMITIVE: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitive(expr));
pub const FROM_COMPLEX: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFrom, expr));
pub const FROM_OPT_COMPLEX: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFromOpt, expr));
pub const TO_COMPLEX: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiTo, expr));
pub const TO_OPT_COMPLEX: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionTo(FFIConversionToMethod::FfiToOpt, expr));

pub const FROM_PRIMITIVE_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromPrimitiveGroup(expr));
pub const FROM_COMPLEX_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromComplexGroup(expr));
pub const FROM_OPT_PRIMITIVE_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromOptPrimitiveGroup(expr));
pub const FROM_OPT_COMPLEX_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::FromOptComplexGroup(expr));

pub const TO_PRIMITIVE_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToPrimitiveGroup(expr));
pub const TO_COMPLEX_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToComplexGroup(expr));
pub const TO_OPT_PRIMITIVE_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToOptPrimitiveGroup(expr));
pub const TO_OPT_COMPLEX_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::ToOptComplexGroup(expr));

pub const DESTROY_PRIMITIVE_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxVecPtr(expr));
pub const DESTROY_COMPLEX_GROUP: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxAnyVecPtr(expr));
pub const DESTROY_COMPLEX: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxAny(expr));
pub const DESTROY_OPT_COMPLEX: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::UnboxAnyOpt(expr));
// Expression::DestroyOpt(expr.into())
pub const DESTROY_OPT_PRIMITIVE: ExpressionComposer = |expr| Expression::InterfacesExpr(InterfacesMethodExpr::DestroyOptPrimitive(expr));

#[allow(unused)]
pub const PRIMITIVE_ARG_COMPOSER: GenericArgComposer = GenericArgComposer::new(FROM_PRIMITIVE, TO_PRIMITIVE, DESTROY_PRIMITIVE);
#[allow(unused)]
pub const PRIMITIVE_OPT_ARG_COMPOSER: GenericArgComposer = GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE);

#[allow(unused)]
pub const COMPLEX_ARG_COMPOSER: GenericArgComposer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX);
pub const COMPLEX_OPT_ARG_COMPOSER: GenericArgComposer = GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX);

// pub const LAMBDA_COMPOSER: GenericArgComposer = GenericArgComposer::new()

pub struct GenericArgComposer {
    // pub ty: Type,
    pub from_composer: ExpressionComposer,
    pub to_composer: ExpressionComposer,
    pub destroy_composer: ExpressionComposer,
    // pub ty_composer: TyComposer<'a>,
}
impl GenericArgComposer {
    pub const fn new(
        // ty_composer: TyComposer,
        from_composer: ExpressionComposer,
        to_composer: ExpressionComposer,
        destroy_composer: ExpressionComposer
    ) -> Self {
        Self { from_composer, to_composer, destroy_composer }
    }
    // pub fn ty(&self, ty: &Type, source: &ScopeContext) -> Type {
    //     (self.ty_composer)((ty, source))
    // }
    pub fn from(&self, expr: TokenStream2) -> Expression {
        (self.from_composer)(expr)
    }
    pub fn to(&self, expr: TokenStream2) -> Expression {
        (self.to_composer)(expr)
    }
    pub fn destroy(&self, expr: TokenStream2) -> Expression {
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
    pub destructor: Expression,
    pub from_conversion: Expression,
    pub to_conversion: Expression,
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
    pub fn new(ty: Type, destructor: Expression, from_conversion: Expression, to_conversion: Expression) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum GenericTypeKind {
    Map(Type),
    IndexMap(Type),
    SerdeJsonMap(Type),
    Vec(Type),
    BTreeSet(Type),
    HashSet(Type),
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
            GenericTypeKind::IndexMap(_) => "IndexMap",
            GenericTypeKind::SerdeJsonMap(_) => "SerdeJsonMap",
            GenericTypeKind::Vec(_) => "Vec",
            GenericTypeKind::BTreeSet(_) => "BTreeSet",
            GenericTypeKind::HashSet(_) => "HashSet",
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
            GenericTypeKind::IndexMap(ty) |
            GenericTypeKind::SerdeJsonMap(ty) |
            GenericTypeKind::Vec(ty) |
            GenericTypeKind::BTreeSet(ty) |
            GenericTypeKind::HashSet(ty) |
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
            GenericTypeKind::IndexMap(ty) |
            GenericTypeKind::SerdeJsonMap(ty) |
            GenericTypeKind::Vec(ty) |
            GenericTypeKind::BTreeSet(ty) |
            GenericTypeKind::HashSet(ty) |
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

impl GenericTypeKind {

    pub fn expand(&self, attrs: &HashSet<Option<Attribute>>, scope_context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        let source = scope_context.borrow();
        let attrs = expand_attributes(attrs);
        println!("Generic::Expand: {} ---- {}", self, quote!(#(#attrs)*));

        match self {
            GenericTypeKind::Result(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let arg_0_name = Name::Dictionary(DictionaryName::Ok);
                let arg_1_name = Name::Dictionary(DictionaryName::Error);
                let compose_arg = |arg_ty: Type, from_expr: Expression, to_expr: Expression, destroy_expr: Expression|
                    GenericArgPresentation::new(
                        arg_ty,
                        destroy_expr,
                        Expression::MapExpression(Expression::O.into(), from_expr.into()),
                        to_expr);


                let compose = |arg_name: &Name, ty: &Type| {
                    println!("RESULT ARG: {} -- {}", arg_name, ty.to_token_stream());
                    // let from_composer = FromConversionComposer::new(arg_name.clone(), ty.clone(), Some(Expression::DictionaryName(DictionaryName::O)));
                    // let from = from_composer.compose(&source);
                    match TypeKind::from(ty) {
                        TypeKind::Primitive(arg_ty) => {
                            compose_arg(
                                arg_ty.clone(),
                                // from,
                                Expression::Deref(DictionaryName::O.to_token_stream()),
                                Expression::InterfacesExpr(InterfacesMethodExpr::Boxed(DictionaryName::O.to_token_stream())),
                                DESTROY_OPT_PRIMITIVE(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()))
                        }
                        TypeKind::Complex(arg_ty) => {

                            let arg_composer = match arg_ty.maybe_special_type(&source) {
                                Some(SpecialType::Opaque(..)) =>
                                    GenericArgComposer::new(FROM_OPAQUE, TO_OPAQUE, DESTROY_OPT_COMPLEX),
                                _ =>
                                    GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX),
                            };

                            // let arg_composer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX);
                            compose_arg(
                                arg_ty.special_or_to_ffi_full_path_type(&source),
                                // from,
                                arg_composer.from(DictionaryName::O.to_token_stream()),
                                arg_composer.to(DictionaryName::O.to_token_stream()),
                                arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()))
                        }
                        TypeKind::Generic(generic_arg_ty) => {
                            // let (arg_composer, arg_ty) = match generic_arg_ty {
                            //     GenericTypeKind::Optional(_) => match generic_arg_ty.ty() {
                            //         None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                            //         Some(ty) => match TypeKind::from(ty) {
                            //             TypeKind::Primitive(_) => (GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE), ty.special_or_to_ffi_full_path_type(&source)),
                            //             TypeKind::Generic(nested_nested) => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), nested_nested.special_or_to_ffi_full_path_type(&source)),
                            //             _ => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), ty.special_or_to_ffi_full_path_type(&source)),
                            //         }
                            //     },
                            //     GenericTypeKind::Box(_) => match generic_arg_ty.ty().and_then(|ty| ty.first_nested_type()) {
                            //         None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                            //         Some(ty) => match TypeKind::from(ty) {
                            //             TypeKind::Primitive(_) => (GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE), ty.special_or_to_ffi_full_path_type(&source)),
                            //             TypeKind::Generic(nested_nested) => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), nested_nested.special_or_to_ffi_full_path_type(&source)),
                            //             _ => (GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_OPT_COMPLEX), ty.special_or_to_ffi_full_path_type(&source)),
                            //         }
                            //     },
                            //     _ => (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX), generic_arg_ty.special_or_to_ffi_full_path_type(&source)),
                            // };

                            let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                                match generic_arg_ty.ty() {
                                    None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                    Some(ty) => match TypeKind::from(ty) {
                                        TypeKind::Primitive(_) => (PRIMITIVE_OPT_ARG_COMPOSER, ty.special_or_to_ffi_full_path_type(&source)),
                                        TypeKind::Generic(nested_nested) => (COMPLEX_OPT_ARG_COMPOSER, nested_nested.special_or_to_ffi_full_path_type(&source)),
                                        _ => (COMPLEX_OPT_ARG_COMPOSER, ty.special_or_to_ffi_full_path_type(&source)),
                                    }
                                }
                            } else { (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX), generic_arg_ty.special_or_to_ffi_full_path_type(&source)) };
                            compose_arg(
                                arg_ty,
                                arg_composer.from(DictionaryName::O.to_token_stream()),
                                arg_composer.to(DictionaryName::O.to_token_stream()),
                                arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()))
                        }
                    }
                };

                let nested_types = ty.nested_types();
                let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
                let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
                let field_names = CommaPunctuated::from_iter([arg_0_name.clone(), arg_1_name.clone()]);
                let types = (ffi_as_type, ty.clone());
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldComposer::named(arg_0_name.clone(), FieldTypeKind::Type(arg_0_presentation.ty.joined_mut())),
                        FieldComposer::named(arg_1_name.clone(), FieldTypeKind::Type(arg_1_presentation.ty.joined_mut())),
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::ConversionFrom {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                DictionaryExpr::FromRoot(
                                    InterfacesMethodExpr::FoldToResult(CommaPunctuated::from_iter([
                                        Expression::FfiRefWithName(arg_0_name),
                                        Expression::FfiRefWithName(arg_1_name),
                                        arg_0_presentation.from_conversion,
                                        arg_1_presentation.from_conversion,
                                    ]).present(&source).to_token_stream())
                                        .to_token_stream())
                                    .to_token_stream(),
                                None
                            )
                        },
                        InterfacePresentation::ConversionTo {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                DictionaryExpr::Depunctuated(
                                    Depunctuated::from_iter([
                                        DictionaryExpr::LetExpr(
                                            quote!((#field_names)),
                                            InterfacesMethodExpr::ToResult(
                                                CommaPunctuated::from_iter([
                                                    DictionaryName::Obj.to_token_stream(),
                                                    DictionaryExpr::Mapper(
                                                        DictionaryName::O.to_token_stream(),
                                                        arg_0_presentation.to_conversion.present(&source)).to_token_stream(),
                                                    DictionaryExpr::Mapper(
                                                        DictionaryName::O.to_token_stream(),
                                                        arg_1_presentation.to_conversion.present(&source)).to_token_stream(),
                                                ]).to_token_stream())
                                                .to_token_stream()
                                                .terminated())
                                            .to_token_stream(),
                                        InterfacesMethodExpr::Boxed(DictionaryExpr::SelfDestructuring(field_names.to_token_stream()).to_token_stream()).to_token_stream(),


                                    ])).to_token_stream(),
                                None
                            )
                        },
                        InterfacePresentation::ConversionDestroy {
                            attrs,
                            types,
                            conversions: (
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([arg_0_presentation.destructor.present(&source).terminated(), arg_1_presentation.destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeKind::Map(ty) |
            GenericTypeKind::IndexMap(ty) |
            GenericTypeKind::SerdeJsonMap(ty)=> {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let arg_0_name = Name::Dictionary(DictionaryName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryName::Values);
                let count_name = Name::Dictionary(DictionaryName::Count);

                let arg_context = |arg_name: &Name| quote!(obj.#arg_name().cloned());
                let arg_args = |arg_name: &Name| CommaPunctuated::from_iter([
                    DictionaryExpr::SelfProp(arg_name.to_token_stream()),
                    DictionaryExpr::SelfProp(count_name.to_token_stream())]);

                let compose_arg = |arg_ty: Type, from_expr: Expression, to_expr: Expression, destroy_expr: Expression|
                    GenericArgPresentation::new(
                        arg_ty,
                        destroy_expr,
                        Expression::MapExpression(Expression::O.into(), from_expr.into()),
                        to_expr);
                let compose = |arg_name: &Name, ty: &Type| match TypeKind::from(ty) {
                    TypeKind::Primitive(arg_ty) => {
                        compose_arg(
                            arg_ty.clone(),
                            Expression::O,
                            TO_PRIMITIVE_GROUP(arg_context(arg_name)),
                            DESTROY_PRIMITIVE_GROUP(arg_args(arg_name).to_token_stream()))
                    },
                    TypeKind::Complex(arg_ty) => {
                        let arg_composer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP);
                        compose_arg(
                            arg_ty.special_or_to_ffi_full_path_variable_type(&source),
                            arg_composer.from(DictionaryName::O.to_token_stream()).into(),
                            arg_composer.to(arg_context(arg_name)),
                            arg_composer.destroy(arg_args(arg_name).to_token_stream())
                        )
                    },
                    TypeKind::Generic(generic_arg_ty) => {
                        let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                            match generic_arg_ty.ty() {
                                None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                Some(ty) => (match TypeKind::from(ty) {
                                    TypeKind::Primitive(_) => GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP),
                                    _ => GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP),
                                }, ty.special_or_to_ffi_full_path_variable_type(&source))
                            }
                        } else { (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), generic_arg_ty.special_or_to_ffi_full_path_variable_type(&source)) };
                        compose_arg(
                            arg_ty,
                            arg_composer.from(DictionaryName::O.to_token_stream()),
                            arg_composer.to(arg_context(arg_name)),
                            arg_composer.destroy(arg_args(arg_name).to_token_stream())
                        )
                    },
                };

                let nested_types = ty.nested_types();
                let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
                let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                let types = (ffi_as_type.clone(), ty.clone());
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldComposer::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
                        FieldComposer::named(arg_0_name, FieldTypeKind::Type(key.joined_mut())),
                        FieldComposer::named(arg_1_name, FieldTypeKind::Type(value.joined_mut()))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::ConversionFrom {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                {
                                    let ffi_ref = DictionaryName::FfiRef;
                                    let count = DictionaryName::Count;
                                    let keys = DictionaryName::Keys;
                                    let values = DictionaryName::Values;
                                    let args = CommaPunctuated::from_iter([
                                        quote!(#ffi_ref.#count),
                                        quote!(#ffi_ref.#keys),
                                        quote!(#ffi_ref.#values),
                                        from_key_conversion.present(&source),
                                        from_value_conversion.present(&source),
                                    ]);
                                    DictionaryExpr::FromRoot(InterfacesMethodExpr::FoldToMap(args.to_token_stream()).to_token_stream())
                                        .to_token_stream()
                                },
                                None
                            )
                        },
                        InterfacePresentation::ConversionTo {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::SelfDestructuring(
                                        CommaPunctuated::from_iter([
                                            FieldComposer::named(Name::Dictionary(DictionaryName::Count), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                                            FieldComposer::named(Name::Dictionary(DictionaryName::Keys), FieldTypeKind::Conversion(to_key_conversion.present(&source))),
                                            FieldComposer::named(Name::Dictionary(DictionaryName::Values), FieldTypeKind::Conversion(to_value_conversion.present(&source))),
                                        ]).to_token_stream())
                                        .to_token_stream())
                                    .to_token_stream(),
                                None
                            )
                        },
                        InterfacePresentation::ConversionDestroy {
                            attrs,
                            types,
                            conversions: (
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        },
                    ]),
                    Depunctuated::from_iter([key_destructor.present(&source).terminated(), value_destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeKind::BTreeSet(ty) |
            GenericTypeKind::HashSet(ty) |
            GenericTypeKind::Vec(ty) => {
                let nested_ty = ty.first_nested_type().unwrap();
                compose_generic_group(
                    ty,
                    ty.clone(),
                    TypeKind::from(nested_ty),
                    FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream(),
                    FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()).to_token_stream(),
                    attrs,
                    &source)
            },
            GenericTypeKind::Array(ty) => {
                let nested_ty = ty.first_nested_type().unwrap();
                compose_generic_group(
                    ty,
                    parse_quote!(Vec<#nested_ty>),
                    TypeKind::from(nested_ty),
                    DictionaryExpr::TryIntoUnwrap(FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream()).to_token_stream(),
                    FFIVecConversionMethodExpr::Encode(DictionaryExpr::ObjToVec.to_token_stream()).to_token_stream(),
                    attrs,
                    &source)
            },
            GenericTypeKind::Slice(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let type_slice: TypeSlice = parse_quote!(#ty);
                let elem_type = &type_slice.elem;
                let target_type: Type = parse_quote!(Vec<#elem_type>);
                let arg_0_name = Name::Dictionary(DictionaryName::Values);
                let count_name = Name::Dictionary(DictionaryName::Count);
                let self_props = CommaPunctuated::from_iter([
                    DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
                    DictionaryExpr::SelfProp(count_name.to_token_stream())]);
                let arg_0_destroy = |composer: ExpressionComposer|
                    composer(self_props.to_token_stream());
                let arg_0_from = |composer: ExpressionComposer|
                    composer(self_props.to_token_stream());
                let arg_0_to = |composer: ExpressionComposer|
                    Expression::InterfacesExpr(
                    InterfacesMethodExpr::Boxed(
                        DictionaryExpr::SelfDestructuring(
                            CommaPunctuated::from_iter([
                                FieldComposer::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                                FieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).present(&source)))]).to_token_stream())
                            .to_token_stream()));

                let arg_0_presentation = match TypeKind::from(&type_slice.elem) {
                    TypeKind::Primitive(arg_0_target_path) => {
                        GenericArgPresentation::new(
                            arg_0_target_path.clone(),
                            arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                            arg_0_from(FROM_PRIMITIVE_GROUP),
                            arg_0_to(TO_PRIMITIVE_GROUP))
                    }
                    TypeKind::Complex(arg_0_target_ty) => {
                        GenericArgPresentation::new(
                            arg_0_target_ty.special_or_to_ffi_full_path_variable_type(&source),
                            arg_0_destroy(DESTROY_COMPLEX_GROUP),
                            arg_0_from(FROM_COMPLEX_GROUP),
                            arg_0_to(TO_COMPLEX_GROUP))
                    }
                    TypeKind::Generic(arg_0_generic_path_conversion) => {
                        GenericArgPresentation::new(
                            arg_0_generic_path_conversion.special_or_to_ffi_full_path_variable_type(&source),
                            arg_0_destroy(DESTROY_COMPLEX_GROUP),
                            arg_0_from(FROM_COMPLEX_GROUP),
                            arg_0_to(TO_COMPLEX_GROUP))
                    }
                };
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;
                let types = (ffi_as_type.clone(), target_type.clone());
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldComposer::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
                        FieldComposer::named(arg_0_name, FieldTypeKind::Type(value.joined_mut()))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::ConversionFrom {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream(),
                                None
                            )
                        },
                        InterfacePresentation::ConversionTo {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()).to_token_stream(),
                                None
                            )
                        },
                        InterfacePresentation::ConversionDestroy {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { attrs, types: (ffi_as_type, target_type), decode: decode.present(&source), encode: encode.present(&source) }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeKind::Tuple(Type::Tuple(type_tuple)) => {
                let ffi_name = type_tuple.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let tuple_items = type_tuple.elems.iter()
                    .enumerate()
                    .map(|(index, ty)|
                        dictionary_generic_arg_pair(
                            Name::UnnamedArg(index),
                            Name::Index(index),
                            ty,
                            &source))
                    .collect::<Depunctuated<(Type, Depunctuated<GenericArgPresentation>)>>();
                let types = (ffi_as_type, Type::Tuple(type_tuple.clone()));
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter(
                        tuple_items.iter()
                            .enumerate()
                            .map(|(index, (root_path, _))|
                                FieldComposer::unnamed(Name::UnnamedArg(index), FieldTypeKind::Type(root_path.clone())))),
                    Depunctuated::from_iter([
                        InterfacePresentation::ConversionFrom {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                DictionaryExpr::FromRoot(
                                    ParenWrapped::new(
                                        CommaPunctuated::from_iter(
                                            tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.clone()))))
                                        .present(&source))
                                    .to_token_stream(),
                                None
                            )
                        },
                        InterfacePresentation::ConversionTo {
                            attrs: attrs.clone(),
                            types: types.clone(),
                            conversions: (
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::SelfDestructuring(
                                        CommaPunctuated::from_iter(
                                            tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.to_conversion.clone())))
                                            .present(&source)
                                            .to_token_stream())
                                        .to_token_stream())
                                    .to_token_stream(),
                                None
                            )
                        },
                        InterfacePresentation::ConversionDestroy {
                            attrs,
                            types,
                            conversions: (
                                InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                                None
                            )
                        },
                    ]),
                    Depunctuated::from_iter(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.present(&source).terminated()))),
                    &source
                )
            },
            GenericTypeKind::AnyOther(ty) => {
                compose_any_other(ty, attrs, source)
            },
            GenericTypeKind::Callback(ty) =>
                compose_callback(ty, attrs, source),
            GenericTypeKind::Optional(_) |
            GenericTypeKind::Box(_) |
            GenericTypeKind::TraitBounds(_) |
            _ => FFIObjectPresentation::Empty,
        }.to_token_stream()
    }
}

fn compose_any_other(ty: &Type, attrs: Vec<Attribute>, source: Ref<ScopeContext>) -> FFIObjectPresentation {

    let ffi_name = ty.mangle_ident_default();
    let ffi_type = ffi_name.to_type();
    let arg_0_name = Name::Dictionary(DictionaryName::Obj);

    let path = ty.to_path();
    let ctor_path = path.arg_less();

    // Arc/Rc: primitive arg: to: "*obj"
    // Arc/Rc: complex arg: to: "(*obj).clone()"
    // Mutex/RwLock: primitive/complex arg: to: "obj.into_inner().expect("Err")"
    // Arc<RwLock>>: to: obj.borrow().clone()
    // RefCell: primitive/complex arg: to: "obj.into_inner()"
    let obj_by_value = source.maybe_object_by_value(ty);
    let nested_ty = ty.first_nested_type().unwrap();
    let maybe_opaque = source.maybe_opaque_object(nested_ty);
    let nested_obj_by_value = source.maybe_object_by_value(nested_ty);
    println!("AnyOther.ty: {}", nested_ty.to_token_stream());
    println!("AnyOther.nested.ty: {}", nested_ty.to_token_stream());
    println!("AnyOther by_value: {}", obj_by_value.as_ref().map_or("None".to_string(), |o| format!("{o}")));
    println!("AnyOther nested: by_value: {}", nested_obj_by_value.as_ref().map_or("None".to_string(), |o| format!("{o}")));
    println!("AnyOther opaque: {}", maybe_opaque.to_token_stream());

    // let compose = |arg_name: &Name, ty: &Type| {
    // };
    let arg_name = &arg_0_name;
    // let ty = nested_ty;
    // compose(&arg_0_name, nested_ty)

    // let search = ScopeSearch::Value(ScopeSearchKey::TypeRef(ty));
    let search = ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(nested_ty).unwrap());
    let ffi_var = VarComposer::new(search.clone()).compose(&source).to_type();
    let maybe_obj = source.maybe_object_by_value(nested_ty);
    let maybe_opaque = source.maybe_opaque_object(nested_ty);
    // println!("compose ffi_type: {}", ffi_var.to_token_stream());
    let default_composer_set = maybe_opaque.as_ref().map_or((FROM_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_COMPLEX), DESTROY_COMPLEX), |_| (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_COMPLEX));

    let (from_conversion, to_composer, destroy_composer) = match maybe_obj.as_ref().and_then(|o| o.maybe_type_model_kind_ref()) {
        Some(ty_model_kind) => match ty_model_kind {
            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) =>
                (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE),
            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) => {
                if let Some(lambda_args) = ty_model_kind.maybe_lambda_args() {
                    (Expression::FromLambda(Expression::Simple(quote!((&*ffi_ref.#arg_0_name))).into(), lambda_args), None, DESTROY_PRIMITIVE)
                } else {
                    (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE)
                }

            },
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) =>
                (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_COMPLEX),
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(kind)) => match kind {
                DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(..)) => (FROM_OPT_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX),
                _ => default_composer_set
            },
            TypeModelKind::Optional(model) => match model.first_nested_argument() {
                Some(nested_arg) => match nested_arg.maybe_type_model_kind_ref() {
                    Some(nested_ty_model_kind) => match nested_ty_model_kind {
                        TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) =>
                            (FROM_OPT_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_PRIMITIVE), DESTROY_OPT_PRIMITIVE),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(..)))) => {
                            let boxed_comp: ExpressionComposer = |expr| Expression::MapIntoBox(Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFromOpt, expr)).into());
                            (boxed_comp(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX)
                        },
                        _ => (FROM_OPT_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX),
                    },
                    _ => (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE),
                },
                _ => (FROM_OPT_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX),
            },
            _ => default_composer_set,
        },
        None => (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE)
    };
    let to_expr = {
        match &path.segments.last() {
            Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                "Arc" | "Rc" => {
                    match TypeKind::from(nested_ty) {
                        TypeKind::Primitive(_) => DictionaryExpr::Deref(arg_0_name.to_token_stream()).to_token_stream(),
                        TypeKind::Complex(_) => {
                            if maybe_opaque.is_some() {
                                quote!(#ctor_path::into_raw(#arg_0_name).cast_mut())
                            } else {
                                quote!((*#arg_0_name).clone())
                            }
                        },
                        TypeKind::Generic(nested_generic_ty) => {
                            println!("GENERIC inside Arc/Rc: {}", nested_generic_ty);
                            match nested_generic_ty {
                                GenericTypeKind::AnyOther(ty) => {
                                    println!("GENERIC (AnyOther) inside Arc/Rc: {}", ty.to_token_stream());
                                    let path = ty.to_path();
                                    match &path.segments.last() {
                                        Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                                            "RwLock" | "Mutex" => quote!(std::sync::#ident::new(obj.read().expect("Poisoned").clone())),
                                            _ => quote!((*#arg_0_name).clone())
                                        },
                                        None => quote!((*#arg_0_name).clone())
                                    }
                                },
                                _ => quote!((*#arg_0_name).clone())
                            }
                        },
                    }
                },
                "Mutex" | "RwLock" => quote!(#arg_0_name.into_inner().expect("Err")),
                "RefCell" => quote!(#arg_0_name.into_inner()),
                "Pin" => quote!(&**#arg_0_name),
                _ => quote!((*#arg_0_name).clone())
            }
            None => quote!((*#arg_0_name).clone())
        }
    };
    let types = (ffi_type.clone(), ty.clone());

    let mut presentations = Depunctuated::new();
    presentations.push(InterfacePresentation::ConversionFrom {
        attrs: attrs.clone(),
        types: types.clone(),
        conversions: (
            {
                let conversion = from_conversion.present(&source);
                let from = maybe_opaque.as_ref().map_or(quote!(new), |_| quote!(from_raw));
                quote! {
                    let ffi_ref = &*ffi;
                    #ctor_path::#from(#conversion)
                }
            },
            None
        )
    });
    match to_composer {
        None => {}
        Some(to_composer) => {
            presentations.push(InterfacePresentation::ConversionTo {
                attrs: attrs.clone(),
                types: types.clone(),
                conversions: (
                    Expression::InterfacesExpr(
                        InterfacesMethodExpr::Boxed(
                            DictionaryExpr::SelfDestructuring(
                                CommaPunctuated::from_iter([
                                    FieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(to_composer(to_expr).present(&source)))
                                ])
                                    .to_token_stream())
                                .to_token_stream()))
                        .present(&source),
                    None
                )
            });
        }
    };
    presentations.push(InterfacePresentation::ConversionDestroy {
        attrs: attrs.clone(),
        types,
        conversions: (
            InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
            None
        )
    });



    compose_generic_presentation(
        ffi_name,
        attrs.clone(),
        Depunctuated::from_iter([
            FieldComposer::named(arg_0_name.clone(), FieldTypeKind::Type(ffi_var))
        ]),
        presentations,
        Depunctuated::from_iter([destroy_composer(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()).present(&source).terminated()]),
        &source
    )
}

pub fn compose_callback(ty: &Type, attrs: Vec<Attribute>, source: Ref<ScopeContext>) -> FFIObjectPresentation {
    let type_path: TypePath = parse_quote!(#ty);
    let PathSegment { arguments, ..} = type_path.path.segments.last().unwrap();
    let ParenthesizedGenericArguments { inputs, output, .. } = parse_quote!(#arguments);
    let ffi_result = DictionaryName::FFiResult;
    let opt_conversion = |conversion: TokenStream2| quote! {
                    if ffi_result.is_null() {
                        None
                    } else {
                        #conversion
                    }
                };
    let from_complex_result = |ty: &Type, ffi_ty: &Type| {
        DictionaryExpr::CallbackDestructor(
            DictionaryExpr::CastedFFIConversionFrom(
                quote!(#ffi_ty),
                quote!(#ty),
                quote!(#ffi_result))
                .to_token_stream(),
            quote!(#ffi_result))
            .to_token_stream()
    };
    let from_opt_complex_result = |ty: &Type, ffi_ty: &Type| {
        DictionaryExpr::CallbackDestructor(
            DictionaryExpr::CastedFFIConversionFromOpt(
                quote!(#ffi_ty),
                quote!(#ty),
                quote!(#ffi_result))
                .to_token_stream(),
            quote!(#ffi_result))
            .to_token_stream()
    };
    let from_primitive_result = || quote!(ffi_result);
    let from_opt_primitive_result = || DictionaryExpr::Deref(ffi_result.to_token_stream()).to_token_stream();
    let (return_type, ffi_return_type, post_processing) = match output {
        ReturnType::Type(token, field_type) => {
            // FromConversionComposer::
            let full_ty: Type = field_type.resolve(&source);
            let (ffi_ty, conversion) = match TypeKind::from(&full_ty) {
                TypeKind::Primitive(_) => (full_ty.clone(), from_primitive_result()),
                TypeKind::Complex(ty) => {
                    let ffi_ty = ty.special_or_to_ffi_full_path_type(&source);
                    let conversion = from_complex_result(&ty, &ffi_ty);
                    (ffi_ty.joined_mut(), conversion)
                },
                TypeKind::Generic(generic_ty) => match generic_ty {
                    GenericTypeKind::Optional(ty) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                        TypeKind::Primitive(ty) => (ty.joined_mut(), opt_conversion(from_opt_primitive_result())),
                        TypeKind::Complex(ty) => {
                            let ffi_ty = ty.special_or_to_ffi_full_path_type(&source);
                            let conversion = opt_conversion(from_opt_complex_result(&ty, &ffi_ty));
                            println!("Callback:: Return:: Generic:: Optional:: Complex:: {} -- {} -- {}", ty.to_token_stream(), ffi_ty.to_token_stream(), conversion);
                            (ffi_ty.joined_mut(), conversion)
                        },
                        TypeKind::Generic(ty) => {
                            let ffi_ty = ty.special_or_to_ffi_full_path_type(&source);
                            let conversion = from_opt_complex_result(ty.ty().unwrap(), &ffi_ty);
                            println!("Callback:: Return:: Generic:: Optional:: Generic:: {} -- {} -- {}", ty.to_token_stream(), ffi_ty.to_token_stream(), conversion);
                            (ffi_ty.joined_mut(), conversion)
                        },
                    },
                    GenericTypeKind::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                    _ => {

                        let ffi_ty = full_ty.special_or_to_ffi_full_path_type(&source);

                        (ffi_ty.joined_mut(), DictionaryExpr::CallbackDestructor(DictionaryExpr::CastedFFIConversionFrom(quote!(#ffi_ty), quote!(#generic_ty), quote!(#ffi_result)).to_token_stream(), quote!(#ffi_result)).to_token_stream())
                    }
                }
            };
            (
                ReturnType::Type(token.clone(), Box::new(full_ty)),
                ReturnType::Type(token.clone(), Box::new(ffi_ty)),
                conversion
            )
        },
        ReturnType::Default => (ReturnType::Default, ReturnType::Default, from_primitive_result()),
    };
    let mut args = CommaPunctuated::new();
    let mut arg_names = CommaPunctuated::new();
    let mut ffi_args = CommaPunctuated::new();
    let mut ffi_named_args = CommaPunctuated::new();
    let mut arg_target_types = CommaPunctuated::new();
    let mut arg_to_conversions = CommaPunctuated::new();
    inputs
        .iter()
        .enumerate()
        .for_each(|(index, ty)| {
            let conversion = TypeKind::from(ty);
            let name = Name::UnnamedArg(index);
            arg_names.push(name.to_token_stream());
            arg_target_types.push(ty.to_token_stream());
            //println!("CALLBACK: {}", ty.to_token_stream());
            // args.push(ArgPresentation::NamedType { attrs: quote!(), name: name.to_token_stream(), var: ty.to_token_stream() });
            args.push(ArgPresentation::Pat(Pat::Type(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Verbatim(name.to_token_stream())),
                colon_token: Default::default(),
                ty: Box::new(ty.clone()),
            })));

            let var = VarComposer::new(ScopeSearch::Value(ScopeSearchKey::TypeRef(ty, None))).compose(&source);
            ffi_named_args.push(quote! { #name: #var });
            ffi_args.push({
                BareFnArg { attrs: vec![], name: None, ty: var.to_type() }
                // var.to_token_stream()
                // match &conversion {
                //     TypeKind::Primitive(ty) => ty.clone(),
                //     TypeKind::Complex(ty) => ty.special_or_to_ffi_full_path_variable_type(&source),
                //     TypeKind::Generic(generic_arg_ty) => if let GenericTypeKind::Optional(..) = generic_arg_ty {
                //         match generic_arg_ty.ty() {
                //             None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                //             Some(ty) => match TypeKind::from(ty) {
                //                 TypeKind::Primitive(_) => ty.special_or_to_ffi_full_path_type(&source),
                //                 TypeKind::Generic(nested_nested) => nested_nested.special_or_to_ffi_full_path_type(&source),
                //                 _ => ty.special_or_to_ffi_full_path_type(&source),
                //             }
                //         }
                //     } else {
                //         generic_arg_ty.special_or_to_ffi_full_path_variable_type(&source)
                //     },
                // }.to_token_stream()
            });
            arg_to_conversions.push(match &conversion {
                TypeKind::Primitive(..) => name.to_token_stream(),
                TypeKind::Generic(generic_ty) => match generic_ty {
                    GenericTypeKind::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                    GenericTypeKind::Optional(ty) => match TypeKind::from(ty) {
                        TypeKind::Primitive(_) => InterfacesMethodExpr::ToOptPrimitive(name.to_token_stream()).to_token_stream(),
                        TypeKind::Complex(_) |
                        TypeKind::Generic(_) => FFIConversionToMethodExpr::FfiToOpt(name.to_token_stream()).to_token_stream(),
                    }
                    _ => FFIConversionToMethodExpr::FfiTo(name.to_token_stream()).to_token_stream()
                },
                TypeKind::Complex(..) => FFIConversionToMethodExpr::FfiTo(name.to_token_stream()).to_token_stream(),
            });
        });
    let ffi_name = ty.mangle_ident_default();
    let ffi_type = ffi_name.to_type();
    let dtor_inputs = match &ffi_return_type {
        ReturnType::Default => {
            CommaPunctuated::new()
        },
        ReturnType::Type(_, ty) => {
            CommaPunctuated::from_iter([BareFnArg {
                attrs: vec![],
                name: None,
                ty: *ty.clone(),
            }])
        }
    };

    FFIObjectPresentation::Generic {
        object_presentation: create_callback(&ffi_name, &attrs, ffi_args.to_token_stream(), ffi_return_type.clone()),
        interface_presentations: Depunctuated::from_iter([
            InterfacePresentation::Callback {
                attrs: attrs.clone(),
                ffi_type: ffi_type.clone(),
                inputs: args,
                output: return_type,
                body: DictionaryExpr::CallbackCaller(arg_to_conversions.to_token_stream(), post_processing).to_token_stream()
            },
            InterfacePresentation::SendAndSync {
                attrs: attrs.clone(),
                ffi_type: ffi_type.clone(),
            }
        ]),
        drop_presentation: DropInterfacePresentation::Empty,
        bindings:
        {
            let caller_composer = FieldComposer::named(Name::Dictionary(DictionaryName::Caller), FieldTypeKind::Type(Type::BareFn(TypeBareFn {
                lifetimes: None,
                unsafety: Some(Default::default()),
                abi: Some(parse_quote!(extern "C")),
                fn_token: Default::default(),
                paren_token: Default::default(),
                inputs: ffi_args.clone(),
                variadic: None,
                output: ffi_return_type.clone()
            })));

            let mut args = CommaPunctuatedOwnedItems::from_iter([
                OwnedItemPresentableContext::BareArg(caller_composer.name.clone(), caller_composer.ty().clone(), attrs.clone())
            ]);
            let mut names = CommaPunctuatedOwnedItems::from_iter([
                OwnedItemPresentableContext::BindingFieldName(caller_composer)
            ]);
            if !dtor_inputs.is_empty() {
                let destructor_composer = FieldComposer::named(Name::Dictionary(DictionaryName::Destructor), FieldTypeKind::Type(Type::BareFn(TypeBareFn {
                    lifetimes: None,
                    unsafety: Some(Default::default()),
                    abi: Some(parse_quote!(extern "C")),
                    fn_token: Default::default(),
                    paren_token: Default::default(),
                    inputs: dtor_inputs,
                    variadic: None,
                    output: ReturnType::Default,
                })));
                args.push(OwnedItemPresentableContext::BareArg(destructor_composer.name.clone(), destructor_composer.ty().clone(), attrs.clone()));
                names.push(OwnedItemPresentableContext::BindingFieldName(destructor_composer));
            }
            Depunctuated::from_iter([
                BindingPresentableContext::Constructor(ConstructorPresentableContext::Default((ffi_type.clone(), attrs.clone(), None)), args, Wrapped::<_, _, Brace>::new(names)),
                BindingPresentableContext::Destructor(ffi_type.clone(), attrs, None)
            ]).present(&source)
        }
    }
}
fn compose_generic_group(ty: &Type, vec_conversion_type: Type, arg_conversion: TypeKind, from_conversion_presentation: TokenStream2, to_conversion_presentation: TokenStream2, attrs: Vec<Attribute>, source: &ScopeContext) -> FFIObjectPresentation {
    let ffi_name = ty.mangle_ident_default();
    let ffi_type = ffi_name.to_type();
    let arg_0_name = Name::Dictionary(DictionaryName::Values);
    let count_name = Name::Dictionary(DictionaryName::Count);
    let from_args = CommaPunctuated::from_iter([
        DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
        DictionaryExpr::SelfProp(count_name.to_token_stream())]);
    let arg_0_from = |composer: ExpressionComposer|
        composer(from_args.to_token_stream());

    let arg_0_to = |composer: ExpressionComposer|
        Expression::InterfacesExpr(
            InterfacesMethodExpr::Boxed(
                DictionaryExpr::SelfDestructuring(
                    CommaPunctuated::from_iter([
                        FieldComposer::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                        FieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).present(source)))])
                        .to_token_stream())
                    .to_token_stream()));

    let arg_0_destroy = |composer: ExpressionComposer|
        composer(from_args.to_token_stream());

    let arg_0_presentation = match arg_conversion {
        TypeKind::Primitive(arg_0_target_path) => {
            GenericArgPresentation::new(
                arg_0_target_path.clone(),
                arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                arg_0_from(FROM_PRIMITIVE_GROUP),
                arg_0_to(TO_PRIMITIVE_GROUP)
            )
        }
        TypeKind::Complex(arg_0_target_ty) => {
            GenericArgPresentation::new(
                arg_0_target_ty.special_or_to_ffi_full_path_variable_type(source),
                arg_0_destroy(DESTROY_COMPLEX_GROUP),
                arg_0_from(FROM_COMPLEX_GROUP),
                arg_0_to(TO_COMPLEX_GROUP)
            )
        }
        TypeKind::Generic(arg_0_generic_path_conversion) => {
            let (arg_0_composer, arg_ty) = {
                if let GenericTypeKind::Optional(..) = arg_0_generic_path_conversion {
                    match arg_0_generic_path_conversion.ty() {
                        None => unimplemented!("Mixin inside generic: {}", arg_0_generic_path_conversion),
                        Some(ty) => match TypeKind::from(ty) {
                            TypeKind::Primitive(_) =>
                                (GenericArgComposer::new(FROM_OPT_PRIMITIVE_GROUP, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP), ty.special_or_to_ffi_full_path_variable_type(source)),
                            TypeKind::Generic(nested_nested) => {
                                (GenericArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), nested_nested.special_or_to_ffi_full_path_variable_type(source))
                            },
                            _ => (GenericArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), ty.special_or_to_ffi_full_path_variable_type(source) ),
                        }
                    }
                } else {
                    (GenericArgComposer::new(FROM_COMPLEX_GROUP, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), arg_0_generic_path_conversion.special_or_to_ffi_full_path_variable_type(source))
                }
            };
            GenericArgPresentation::new(
                arg_ty,
                arg_0_destroy(arg_0_composer.destroy_composer),
                arg_0_from(arg_0_composer.from_composer),
                arg_0_to(arg_0_composer.to_composer)
            )
        }
    };
    let types = (ffi_type.clone(), ty.clone());
    compose_generic_presentation(
        ffi_name,
        attrs.clone(),
        Depunctuated::from_iter([
            FieldComposer::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
            FieldComposer::named(arg_0_name, FieldTypeKind::Type(arg_0_presentation.ty.joined_mut()))
        ]),
        Depunctuated::from_iter([
            InterfacePresentation::ConversionFrom {
                attrs: attrs.clone(),
                types: types.clone(),
                conversions: (
                    from_conversion_presentation,
                    None
                )
            },
            InterfacePresentation::ConversionTo {
                attrs: attrs.clone(),
                types: types.clone(),
                conversions: (
                    to_conversion_presentation,
                    None
                )
            },
            InterfacePresentation::ConversionDestroy {
                attrs: attrs.clone(),
                types: types.clone(),
                conversions: (
                    InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
                    None
                )
            },
            InterfacePresentation::VecConversion { attrs, types: (ffi_type, vec_conversion_type), decode: arg_0_presentation.from_conversion.present(source), encode: arg_0_presentation.to_conversion.present(source) }
        ]),
        Depunctuated::from_iter([arg_0_presentation.destructor.present(source).terminated()]),
        source
    )
}
pub fn compose_generic_presentation(
    ffi_name: Ident,
    attrs: Vec<Attribute>,
    field_composers: Depunctuated<FieldComposer>,
    interface_presentations: Depunctuated<InterfacePresentation>,
    drop_body: Depunctuated<TokenStream2>,
    source: &ScopeContext) -> FFIObjectPresentation {
    let fields = CommaPunctuated::from_iter(field_composers.iter().map(|composer| ArgPresentation::Field(composer.resolve(source))));
    FFIObjectPresentation::Generic {
        object_presentation: create_struct(
            &ffi_name,
            &attrs,
            quote!({#fields})
        ),
        interface_presentations,
        drop_presentation: DropInterfacePresentation::Full {
            attrs: attrs.clone(),
            ty: ffi_name.to_type(),
            body: drop_body.to_token_stream()
        },
        bindings: compose_bindings(
            ffi_name.to_type(),
            attrs,
            None,
            field_composers)
            .present(source)
    }
}

impl Resolve<Field> for FieldComposer  {
    fn resolve(&self, source: &ScopeContext) -> Field {
        let FieldComposer { attrs, name, kind, .. } = self;
        println!("<FieldComposer as Resolve<Field>>::resolve({})", self);
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
        }    }
}

fn compose_bindings(
    ffi_type: Type,
    attrs: Vec<Attribute>,
    generics: Option<Generics>,
    field_composers: Depunctuated<FieldComposer>
) -> Depunctuated<ConstructorBindingPresentableContext<Brace>> {
    Depunctuated::from_iter([
        struct_composer_ctor_root()((
            ConstructorPresentableContext::Default((ffi_type.clone(), attrs.clone(), generics.clone())),
            Vec::from_iter(field_composers.iter().map(STRUCT_COMPOSER_CTOR_NAMED_ITEM))
        )),
        BindingPresentableContext::Destructor(ffi_type.clone(), attrs, generics)
    ])
}

pub(crate) fn dictionary_generic_arg_pair(name: Name, field_name: Name, ty: &Type, source: &ScopeContext) -> (Type, Depunctuated<GenericArgPresentation>) {
    let ty: Type = ty.resolve(source);
    let (destroy_expr, from_expr, to_expr) = match TypeKind::from(&ty) {
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



// #[repr(C)]
// #[derive(Clone)]
// pub struct std_sync_Arc_dyn_trait_Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//     pub obj : * mut crate :: fermented :: generics :: Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String
// }
//
//
// impl ferment_interfaces :: FFIConversionFrom < std :: sync :: Arc < dyn Fn (* const ferment_example_thread_safe :: entry :: FFIContext , u32 , [u8 ; 32] , u32) -> Result < [u8 ; 48] , String > > > for std_sync_Arc_dyn_trait_Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//     unsafe fn ffi_from_const (ffi : * const std_sync_Arc_dyn_trait_Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String) -> std :: sync :: Arc < dyn Fn (* const ferment_example_thread_safe :: entry :: FFIContext , u32 , [u8 ; 32] , u32) -> Result < [u8 ; 48] , String > > {
//         let ffi_ref = & * ffi ;
//         std::sync::Arc::new(|context , quorum_type, quorum_hash, core_chain_locked_height |
//             (&*ffi_ref.obj).call(context, quorum_type, quorum_hash, core_chain_locked_height))
//     }
// }
// impl ferment_interfaces :: FFIConversionTo < std :: sync :: Arc < dyn Fn (* const ferment_example_thread_safe :: entry :: FFIContext , u32 , [u8 ; 32] , u32) -> Result < [u8 ; 48] , String > > > for std_sync_Arc_dyn_trait_Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//     unsafe fn ffi_to_const (obj : std :: sync :: Arc < dyn Fn (* const ferment_example_thread_safe :: entry :: FFIContext , u32 , [u8 ; 32] , u32) -> Result < [u8 ; 48] , String > >) -> * const std_sync_Arc_dyn_trait_Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//         ferment_interfaces :: boxed (Self {
//             obj: ferment_interfaces::boxed(Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//                 caller: move |context, quorum_type, quorum_hash, core_chain_locked_height| {
//                     let result = (obj)(context, quorum_type, ferment_interfaces::FFIConversionFrom::ffi_from(quorum_hash), core_chain_locked_height);
//                     ferment_interfaces::FFIConversionTo::ffi_to(result)
//                 },
//                 destructor: move |result| {
//                     ferment_interfaces::unbox_any(result);
//                 }
//             })
//         })
//     }
// }
// impl ferment_interfaces :: FFIConversionDestroy < std :: sync :: Arc < dyn Fn (* const ferment_example_thread_safe :: entry :: FFIContext , u32 , [u8 ; 32] , u32) -> Result < [u8 ; 48] , String > > > for std_sync_Arc_dyn_trait_Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//     unsafe fn destroy (ffi : * mut std_sync_Arc_dyn_trait_Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String) {
//         ferment_interfaces :: unbox_any (ffi) ; ;
//     }
// }

// ferment_interfaces :: boxed (Self {
//     obj: ferment_interfaces::boxed(Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//         caller: |context: *const ferment_example_thread_safe::entry::FFIContext, quorum_type: u32, quorum_hash: *mut crate::fermented::generics::Arr_u8_32, core_chain_locked_height: u32| -> *mut crate::fermented::generics::Result_ok_u8_arr_48_err_String {
//             let result = (obj)(context, quorum_type, ferment_interfaces::FFIConversionFrom::ffi_from(quorum_hash), core_chain_locked_height);
//             ferment_interfaces::FFIConversionTo::ffi_to(result)
//         },
//         destructor: |result: *mut crate::fermented::generics::Result_ok_u8_arr_48_err_String| {
//             ferment_interfaces::unbox_any(result);
//         },
//     })
// })

// ferment_interfaces::boxed(Fn_ARGS_ferment_example_thread_safe_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String {
//     caller: |context: *const ferment_example_thread_safe::entry::FFIContext, quorum_type: u32, quorum_hash: *mut crate::fermented::generics::Arr_u8_32, core_chain_locked_height: u32| -> *mut crate::fermented::generics::Result_ok_u8_arr_48_err_String {
//         let result = (obj)(context, quorum_type, ferment_interfaces::FFIConversionFrom::ffi_from(quorum_hash), core_chain_locked_height);
//         ferment_interfaces::FFIConversionTo::ffi_to(result)
//     },
//     destructor: |result: *mut crate::fermented::generics::Result_ok_u8_arr_48_err_String| {
//         ferment_interfaces::unbox_any(result);
//     },
// })
