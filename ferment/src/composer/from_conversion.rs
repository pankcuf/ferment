use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use quote::ToTokens;
use syn::{BareFnArg, PatType, Type};
use syn::punctuated::Punctuated;
use crate::ast::CommaPunctuated;
use crate::composable::TypeModel;
use crate::composer::Composer;
use crate::context::{ScopeContext, ScopeSearch};
use crate::conversion::{DictTypeModelKind, GenericTypeKind, ObjectKind, ScopeItemKind, TypeModelKind, TypeKind, DictFermentableModelKind, SmartPointerModelKind};
use crate::ext::{FFITypeModelKindResolve, FFIObjectResolve, FFISpecialTypeResolve, FFITypeResolve, GenericNestedArg, Primitive, Resolve, SpecialType, AsType, ToType};
use crate::lang::LangAttrSpecification;
use crate::presentable::Expression;
use crate::presentation::Name;

// #[allow(unused)]
// pub struct FromConversionModel<LANG, SPEC>
//     where
//         LANG: Clone,
//         SPEC: LangAttrSpecification<LANG> {
//     pub name: Name,
//     pub expr: Option<Expression<LANG, SPEC>>,
// }

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub name: Name,
    pub search: ScopeSearch<'a>,
    pub expr: Option<Expression<LANG, SPEC>>,
    phantom_lang: PhantomData<LANG>,
    phantom_spec: PhantomData<SPEC>
}

impl<'a, LANG, SPEC> FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub fn new(name: Name, search: ScopeSearch<'a>, expr: Option<Expression<LANG, SPEC>>) -> Self {
        Self { name, search , expr, phantom_lang: PhantomData::default(), phantom_spec: PhantomData::default() }
    }
    pub fn expr_less(name: Name, search: ScopeSearch<'a>) -> Self {
        Self::new(name, search, None)
    }
}
impl<'a, LANG, SPEC> Display for FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} --- {} --- {}", self.name, self.search, self.expr.as_ref().map_or("None".to_string(), |e| format!("{}", e))).as_str())
    }
}

impl<'a, LANG, SPEC> Composer<'a> for FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    type Source = ScopeContext;
    type Output = Expression<LANG, SPEC>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let Self { name, search, expr, .. } = self;
        let search_key = self.search.search_key();
        println!("FromConversionFullComposer:: {}({}) -- {} -- {:?}", name,  name.to_token_stream(), search, expr);
        let field_path = expr.clone().unwrap_or(Expression::Simple(name.to_token_stream()));
        let maybe_object = source.maybe_object_by_predicate(search.clone());
        let full_type = maybe_object.as_ref().and_then(ObjectKind::maybe_type).unwrap_or(search_key.to_type());
        println!("FromConversionFullComposer::maybe_object {} ", maybe_object.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        let composition = maybe_object.as_ref()
            .and_then(|external_type| {
                match external_type {
                    ObjectKind::Item(.., ScopeItemKind::Fn(..)) => {
                        source.scope
                            .parent_object()
                            .and_then(|parent_obj| parent_obj.maybe_trait_or_regular_model_kind(source))
                    },
                    ObjectKind::Type(ref ty_conversion) |
                    ObjectKind::Item(ref ty_conversion, ..) =>
                        ty_conversion.maybe_trait_model_kind_or_same(source),
                    ObjectKind::Empty => {
                        // println!("Type::<TypeModelKind> Has no object --> {}", external_type.type_conversion().to_token_stream());
                        None
                    }
                }
            })
            .unwrap_or_else(|| {
                // println!("Type::<TypeModelKind> Default Unknown --> {}", self.to_token_stream());
                TypeModelKind::Unknown(TypeModel::new(search_key.to_type(), None, Punctuated::new()))
            });



        let expression = match full_type.maybe_special_type(source) {
            Some(SpecialType::Opaque(..)) => {
                println!("FromConversionFullComposer:: Opaque: {}({})", search_key, full_type.to_token_stream());
                match composition {
                    TypeModelKind::FnPointer(..) |
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) => field_path,
                    TypeModelKind::Bounds(bounds) => {
                        if bounds.bounds.is_empty() {
                            field_path
                        } else  if bounds.is_lambda() {
                            if let Some(lambda_args) = bounds.bounds.first().unwrap().maybe_lambda_args() {
                                Expression::FromLambda(field_path.into(), lambda_args)
                            } else {
                                Expression::From(field_path.into())
                            }
                        } else {
                            Expression::From(field_path.into())
                        }
                    },
                    _ => {
                        println!("FromConversionFullComposer: FROMPTRCLONE ({}): {}", search_key.maybe_originally_is_const_ptr(), search_key);
                        if search_key.maybe_originally_is_const_ptr() {
                            field_path
                        } else if search_key.maybe_originally_is_mut_ptr() {
                            field_path
                        } else {
                            Expression::FromPtrClone(field_path.into())
                        }
                    }
                }
            },
            Some(SpecialType::Custom(..)) =>
                Expression::From(field_path.into()),
            None => {
                println!("FromConversionFullComposer (Non Special): {} ({})", search_key, full_type.to_token_stream());
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        field_path,
                    TypeModelKind::FnPointer(..) => {
                        println!("FromConversionFullComposer (Non Special FnPointer): {} --- {}", search_key, maybe_object.to_token_stream());
                        if let Some(bare) = source.maybe_fn_sig(&full_type) {
                            let lambda_args = CommaPunctuated::from_iter(bare.inputs.iter().enumerate().map(|(index, BareFnArg { name, ..})| match name {
                                Some((ident, ..)) => Name::Ident(ident.clone()),
                                None => Name::UnnamedArg(index)
                            }));
                            Expression::FromLambda(field_path.into(), lambda_args)
                        } else {
                            field_path
                        }
                    },
                    TypeModelKind::Optional(ty) => if ty.as_type().first_nested_type().unwrap().is_primitive() {
                        Expression::FromOptPrimitive(field_path.into())
                    } else {
                        Expression::FromOpt(field_path.into())
                    }
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        let full_nested_ty = full_ty.first_nested_type().unwrap();
                        // println!("FromConversionComposer (Non Special Boxed): {} ({}) --- {} ---- {}", nested_ty.to_token_stream(), full_nested_ty.to_token_stream(), <Type as Resolve<Option<SpecialType>>>::resolve(full_nested_ty, source).to_token_stream(), nested_ty.maybe_object(source).to_token_stream());
                        match (<Type as Resolve<Option<SpecialType>>>::resolve(full_nested_ty, source),
                               source.maybe_object_by_value(full_nested_ty)) {
                            (Some(SpecialType::Opaque(..)),
                                Some(ObjectKind::Item(TypeModelKind::FnPointer(..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..) |
                                     ObjectKind::Type(TypeModelKind::FnPointer(..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..))))
                            ) =>
                                Expression::IntoBox(field_path.into()),
                            (Some(SpecialType::Opaque(..)), _any_other) =>
                                Expression::FromRawBox(field_path.into()),
                            (Some(SpecialType::Custom(..)), _any) =>
                                Expression::IntoBox(Expression::From(field_path.into()).into()),
                            (_, Some(obj)) => {
                                println!("FromConversionFullComposer (Non Special Boxed Lambda): {}", obj);
                                Expression::IntoBox(match obj.maybe_lambda_args() {
                                    Some(lambda_args) =>
                                        Expression::FromLambda(field_path.into(), lambda_args),
                                    None =>
                                        Expression::From(field_path.into())
                                }.into())
                            }
                            _ =>
                                Expression::IntoBox(Expression::From(field_path.into()).into())
                        }
                    },
                    TypeModelKind::Bounds(bounds) => {
                        println!("FromConversionFullComposer (Bounds): {}", bounds);
                        if bounds.bounds.is_empty() {
                            field_path
                        } else  if bounds.is_lambda() {
                            if let Some(lambda_args) = bounds.bounds.first().unwrap().maybe_lambda_args() {
                                Expression::FromLambda(field_path.into(), lambda_args)
                            } else {
                                Expression::From(field_path.into())
                            }
                        } else {
                            Expression::From(field_path.into())
                        }
                    },
                    TypeModelKind::Unknown(..) => {
                        println!("FromConversionFullComposer (Unknown): {}", search_key);

                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                field_path,
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) => Expression::FromOptPrimitive(field_path.into()),
                                _ => Expression::FromOpt(field_path.into()),
                            }
                            TypeKind::Generic(..) =>
                                Expression::From(field_path.into()),
                            _ =>
                                field_path,
                        }
                    },
                    _ => {
                        println!("FromConversionFullComposer (Regular): {}", composition);
                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                field_path,
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) => Expression::FromOptPrimitive(field_path.into()),
                                _ => Expression::FromOpt(field_path.into()),
                            }
                            _ =>
                                Expression::From(field_path.into())
                        }
                    }
                }
            }
        };
        expression
    }
}

#[derive(Clone, Debug)]
pub struct FromConversionComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub name: Name,
    pub ty: Type,
    pub expr: Option<Expression<LANG, SPEC>>,
    phantom_lang: PhantomData<LANG>,
    phantom_spec: PhantomData<SPEC>
}

impl<LANG, SPEC> Display for FromConversionComposer<LANG, SPEC>
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} --- {} --- {}", self.name, self.ty.to_token_stream(), self.expr.as_ref().map_or("None".to_string(), |e| format!("{}", e))).as_str())
    }
}

#[allow(unused)]
impl<LANG, SPEC> From<&PatType> for FromConversionComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn from(value: &PatType) -> Self {
        let PatType { ty, pat, .. } = value;
        Self { name: Name::Pat(*pat.clone()), ty: *ty.clone(), expr: None, phantom_lang: PhantomData::default(), phantom_spec: PhantomData::default() }
    }
}

#[allow(unused)]
impl<LANG, SPEC> FromConversionComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub fn new(name: Name, ty: Type, expr: Option<Expression<LANG, SPEC>>) -> Self {
        Self { name, ty, expr, phantom_lang: Default::default(), phantom_spec: Default::default() }
    }
}
impl<'a, LANG, SPEC> Composer<'a> for FromConversionComposer<LANG, SPEC>
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    type Source = ScopeContext;
    type Output = Expression<LANG, SPEC>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let Self { name, ty, expr, .. } = self;
        println!("FromConversionComposer:: {}({}) -- {} -- {:?}", name,  name.to_token_stream(), ty.to_token_stream(), expr);
        let field_path = expr.clone().unwrap_or(Expression::Simple(name.to_token_stream()));
        let full_type = ty.full_type(source);
        let maybe_object = <Type as Resolve<Option<ObjectKind>>>::resolve(ty, source);
        println!("FromConversionComposer::maybe_object {} ", maybe_object.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        let composition = maybe_object.as_ref()
            .and_then(|external_type| {
                match external_type {
                    ObjectKind::Item(.., ScopeItemKind::Fn(..)) => {
                        source.scope
                            .parent_object()
                            .and_then(|parent_obj| parent_obj.maybe_trait_or_regular_model_kind(source))
                    },
                    ObjectKind::Type(ref ty_conversion) |
                    ObjectKind::Item(ref ty_conversion, ..) =>
                        ty_conversion.maybe_trait_model_kind_or_same(source),
                    ObjectKind::Empty => {
                        // println!("Type::<TypeModelKind> Has no object --> {}", external_type.type_conversion().to_token_stream());
                        None
                    }
                }
            })
            .unwrap_or_else(|| {
                // println!("Type::<TypeModelKind> Default Unknown --> {}", self.to_token_stream());
                TypeModelKind::Unknown(TypeModel::new(ty.clone(), None, Punctuated::new()))
            });



        let expression = match full_type.maybe_special_type(source) {
            Some(SpecialType::Opaque(..)) => {
                println!("FromConversionComposer:: Opaque: {}({})", ty.to_token_stream(), full_type.to_token_stream());
                match composition {
                    TypeModelKind::FnPointer(..) |
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) => field_path,
                    TypeModelKind::Bounds(bounds) => {
                        if bounds.bounds.is_empty() {
                            field_path
                        } else  if bounds.is_lambda() {
                            if let Some(lambda_args) = bounds.bounds.first().unwrap().maybe_lambda_args() {
                                Expression::FromLambda(field_path.into(), lambda_args)
                            } else {
                                Expression::From(field_path.into())
                            }
                        } else {
                            Expression::From(field_path.into())
                        }

                    },
                    _ => {
                        Expression::DerefContext(field_path.into())
                    }
                }
            },
            Some(SpecialType::Custom(..)) =>
                Expression::From(field_path.into()),
            None => {
                println!("FromConversionComposer (Non Special): {} ({}) --- {}", ty.to_token_stream(), full_type.to_token_stream(), ty.type_model_kind(source));
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        field_path,
                    TypeModelKind::FnPointer(..) => {
                        println!("FromConversionComposer (Non Special FnPointer): {} --- {}", ty.to_token_stream(), maybe_object.to_token_stream());
                        if let Some(bare) = source.maybe_fn_sig(&full_type) {
                            let lambda_args = CommaPunctuated::from_iter(bare.inputs.iter().enumerate().map(|(index, BareFnArg { name, ..})| match name {
                                Some((ident, ..)) => Name::Ident(ident.clone()),
                                None => Name::UnnamedArg(index)
                            }));
                            Expression::FromLambda(field_path.into(), lambda_args)
                        } else {
                            field_path
                        }
                    },
                    TypeModelKind::Optional(ty) => if ty.as_type().first_nested_type().unwrap().is_primitive() {
                        Expression::FromOptPrimitive(field_path.into())
                    } else {
                        Expression::FromOpt(field_path.into())
                    }
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        let nested_ty = ty.first_nested_type().unwrap();
                        let full_nested_ty = full_ty.first_nested_type().unwrap();
                        println!("FromConversionComposer (Non Special Boxed): {} ({}) --- {} ---- {}", nested_ty.to_token_stream(), full_nested_ty.to_token_stream(), <Type as Resolve<Option<SpecialType>>>::resolve(full_nested_ty, source).to_token_stream(), nested_ty.maybe_object(source).to_token_stream());
                        match (<Type as Resolve<Option<SpecialType>>>::resolve(full_nested_ty, source),
                               nested_ty.maybe_object(source)) {
                            (Some(SpecialType::Opaque(..)),
                                Some(ObjectKind::Item(TypeModelKind::FnPointer(..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..) |
                                     ObjectKind::Type(TypeModelKind::FnPointer(..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..))))
                            ) =>
                                Expression::IntoBox(field_path.into()),
                            (Some(SpecialType::Opaque(..)), _any_other) =>
                                Expression::FromRawBox(field_path.into()),
                            (Some(SpecialType::Custom(..)), _any) =>
                                Expression::IntoBox(Expression::From(field_path.into()).into()),
                            (_, Some(obj)) => {
                                println!("FromConversionComposer (Non Special Boxed Lambda): {}", obj);
                                Expression::IntoBox(match obj.maybe_lambda_args() {
                                    Some(lambda_args) =>
                                        Expression::FromLambda(field_path.into(), lambda_args),
                                    None =>
                                        Expression::From(field_path.into())
                                }.into())
                            }
                            _ =>
                                Expression::IntoBox(Expression::From(field_path.into()).into())
                        }
                    },
                    TypeModelKind::Bounds(bounds) => {
                        println!("FromConversionComposer (Bounds): {}", bounds);
                        if bounds.bounds.is_empty() {
                            field_path
                        } else  if bounds.is_lambda() {
                            if let Some(lambda_args) = bounds.bounds.first().unwrap().maybe_lambda_args() {
                                Expression::FromLambda(field_path.into(), lambda_args)
                            } else {
                                Expression::From(field_path.into())
                            }
                        } else {
                            Expression::From(field_path.into())
                        }
                    },
                    TypeModelKind::Unknown(..) => {
                        println!("FromConversionComposer (Unknown): {}", ty.to_token_stream());
                        match TypeKind::from(ty) {
                            TypeKind::Primitive(_) =>
                                field_path,
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) => Expression::FromOptPrimitive(field_path.into()),
                                _ => Expression::FromOpt(field_path.into()),
                            }
                            TypeKind::Generic(..) =>
                                Expression::From(field_path.into()),
                            _ =>
                                field_path,
                                // Expression::From(field_path.into())
                        }
                    },
                    _ => {
                        println!("FromConversionComposer (Regular): {}", composition);
                        match TypeKind::from(ty) {
                            TypeKind::Primitive(_) =>
                                field_path,
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) => Expression::FromOptPrimitive(field_path.into()),
                                _ => Expression::FromOpt(field_path.into()),
                            }
                            _ =>
                                Expression::From(field_path.into())
                        }
                    }
                }
            }
        };
        expression
    }
}