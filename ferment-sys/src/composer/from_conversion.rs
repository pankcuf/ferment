use std::marker::PhantomData;
use quote::ToTokens;
use syn::{BareFnArg, PatType, Type};
use crate::ast::CommaPunctuated;
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictTypeModelKind, GenericTypeKind, ObjectKind, TypeModelKind, TypeKind, DictFermentableModelKind, SmartPointerModelKind};
use crate::ext::{FFIObjectResolve, FFISpecialTypeResolve, FFITypeResolve, GenericNestedArg, Primitive, Resolve, SpecialType, AsType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, Name};

// #[allow(unused)]
// pub struct FromConversionModel<LANG, SPEC>
//     where
//         LANG: LangFermentable,
//         SPEC: LangAttrSpecification<LANG> {
//     pub name: Name,
//     pub expr: Option<Expression<LANG, SPEC>>,
// }

#[allow(unused)]
#[derive(Clone)]
pub struct FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: Name,
    pub search: ScopeSearch<'a>,
    pub expr: Option<SPEC::Expr>,
    _marker: PhantomData<(LANG, SPEC)>,
}

impl<'a, LANG, SPEC> FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn new(name: Name, search: ScopeSearch<'a>, expr: Option<SPEC::Expr>) -> Self {
        Self { name, search , expr, _marker: PhantomData }
    }
    pub fn expr_less(name: Name, search: ScopeSearch<'a>) -> Self {
        Self::new(name, search, None)
    }
    pub fn key_in_scope_with_expr(name: Name, ty: &'a Type, scope: &'a ScopeChain, expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope), expr)
    }
    pub fn key_in_scope(name: Name, ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::expr_less(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }
}
// impl<'a, LANG, SPEC> Display for FromConversionFullComposer<'a, LANG, SPEC>
//     where LANG: LangFermentable + Debug,
//           SPEC: Specification<LANG> + Debug,
//           <SPEC as Specification<LANG>>::Attr: Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(format!("{} --- {} --- {}", self.name, self.search, self.expr.as_ref().map_or("None".to_string(), |e| format!("{}", e))).as_str())
//     }
// }

impl<'a, LANG, SPEC> SourceComposable for FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, expr, .. } = self;
        let search_key = self.search.search_key();
        println!("FromConversionFullComposer:: {}({}) -- {}", name,  name.to_token_stream(), search);

        let field_path = expr.clone().unwrap_or(Expression::Simple(name.to_token_stream()));
        let maybe_object = source.maybe_object_by_predicate(search.clone());
        let full_type = maybe_object.as_ref().and_then(ObjectKind::maybe_type).unwrap_or(search_key.to_type());
        // println!("FromConversionFullComposer::maybe_object {} ", maybe_object.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        let composition = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or(TypeModelKind::unknown_type(search_key.to_type()));

        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_type.maybe_special_type(source);
        let expression = match maybe_special {
            Some(SpecialType::Opaque(..)) => {
                match composition {
                    TypeModelKind::FnPointer(..) |
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        field_path,
                    TypeModelKind::Bounds(bounds) =>
                        bounds.expr_from(field_path),
                    _ if search_key.maybe_originally_is_ptr() =>
                        Expression::from_primitive(field_path),
                    _ => Expression::from_ptr_clone(field_path),

                }
            },
            Some(SpecialType::Custom(..)) =>
                Expression::from_complex(field_path),
            _ => {
                // println!("FromConversionFullComposer (Non Special): {} ({})", search_key, full_type.to_token_stream());
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        Expression::from_primitive(field_path),
                    TypeModelKind::FnPointer(..) => {
                        // println!("FromConversionFullComposer (Non Special FnPointer): {} --- {}", search_key, maybe_object.to_token_stream());
                        if let Some(bare) = source.maybe_fn_sig(&full_type) {
                            let lambda_args = CommaPunctuated::from_iter(bare.inputs.iter().enumerate().map(|(index, BareFnArg { name, ..})| match name {
                                Some((ident, ..)) => Name::Ident(ident.clone()),
                                None => Name::UnnamedArg(index)
                            }));
                            Expression::from_lambda(field_path, lambda_args)
                        } else {
                            Expression::from_primitive(field_path)
                        }
                    },
                    TypeModelKind::Optional(ty) => if ty.as_type().first_nested_type().unwrap().is_primitive() {
                        Expression::from_primitive_opt(field_path)
                    } else {
                        Expression::from_complex_opt(field_path)
                    }
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        let full_nested_ty = full_ty.first_nested_type().unwrap();
                        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_nested_ty.maybe_resolve(source);
                        let maybe_object = source.maybe_object_by_value(full_nested_ty);
                        match (maybe_special, maybe_object) {
                            (Some(SpecialType::Opaque(..)),
                                Some(ObjectKind::Item(TypeModelKind::FnPointer(..) |
                                                      TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..) |
                                     ObjectKind::Type(TypeModelKind::FnPointer(..) |
                                                      TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..))))
                            ) =>
                                Expression::new_box(field_path),
                            (Some(SpecialType::Opaque(..)), _any_other) =>
                                Expression::from_raw_box(field_path),
                            (Some(SpecialType::Custom(..)), _any) =>
                                Expression::new_box(Expression::from_complex(field_path)),
                            (_, Some(obj)) =>
                                Expression::new_box(match obj.maybe_lambda_args() {
                                    Some(lambda_args) =>
                                        Expression::from_lambda(field_path, lambda_args),
                                    None =>
                                        Expression::from_complex(field_path)
                                }),
                            _ =>
                                Expression::new_box(Expression::from_complex(field_path)),
                        }
                    },
                    TypeModelKind::Bounds(bounds) => {
                        // println!("FromConversionFullComposer (Bounds): {}", bounds);
                        bounds.expr_from(field_path)
                    },
                    TypeModelKind::Unknown(..) => {
                        // println!("FromConversionFullComposer (Unknown): {}", search_key);

                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                Expression::from_primitive(field_path),
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) => Expression::from_primitive_opt(field_path),
                                _ => Expression::from_complex_opt(field_path),
                            }
                            TypeKind::Generic(..) =>
                                Expression::from_complex(field_path),
                            _ =>
                                Expression::from_primitive(field_path),
                        }
                    },
                    _ => {
                        // println!("FromConversionFullComposer (Regular): {}", composition);
                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                Expression::from_primitive(field_path),
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) => Expression::from_primitive_opt(field_path),
                                _ => Expression::from_complex_opt(field_path),
                            }
                            _ =>
                                Expression::from_complex(field_path)
                        }
                    }
                }
            }
        };
        expression
    }
}

#[derive(Clone)]
pub struct FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: Name,
    pub ty: Type,
    pub expr: Option<SPEC::Expr>,
    _marker: PhantomData<(LANG, SPEC)>,
}

// impl<LANG, SPEC> Display for FromConversionComposer<LANG, SPEC>
//     where LANG: LangFermentable + Debug,
//           SPEC: Specification<LANG> + Debug,
//           <SPEC as Specification<LANG>>::Attr: Debug  {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(format!("{} --- {} --- {}", self.name, self.ty.to_token_stream(), self.expr.as_ref().map_or("None".to_string(), |e| format!("{}", e))).as_str())
//     }
// }

#[allow(unused)]
impl<LANG, SPEC> From<&PatType> for FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn from(value: &PatType) -> Self {
        let PatType { ty, pat, .. } = value;
        Self { name: Name::Pat(*pat.clone()), ty: *ty.clone(), expr: None, _marker: PhantomData }
    }
}

#[allow(unused)]
impl<LANG, SPEC> FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(name: Name, ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr, _marker: Default::default() }
    }
}
impl<LANG, SPEC> SourceComposable for FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, ty, expr, .. } = self;
        let field_path = expr.clone().unwrap_or(Expression::Simple(name.to_token_stream()));
        let full_type = ty.full_type(source);
        let maybe_object = <Type as Resolve<ObjectKind>>::maybe_resolve(ty, source);
        let composition = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or(TypeModelKind::unknown_type_ref(ty));


        let maybe_special = <Type as FFISpecialTypeResolve<LANG, SPEC>>::maybe_special_type(&full_type, source);
        let expression = match maybe_special {
            Some(SpecialType::Opaque(..)) => {
                match composition {
                    TypeModelKind::FnPointer(..) |
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        field_path,
                    TypeModelKind::Bounds(bounds) =>
                        bounds.expr_from(field_path),
                    _ => Expression::deref_expr(field_path)
                }
            },
            Some(SpecialType::Custom(..)) =>
                Expression::from_complex(field_path),
            _ => {
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        Expression::from_primitive(field_path),
                    TypeModelKind::FnPointer(..) => {
                        if let Some(bare) = source.maybe_fn_sig(&full_type) {
                            let lambda_args = CommaPunctuated::from_iter(bare.inputs.iter().enumerate().map(|(index, BareFnArg { name, ..})| match name {
                                Some((ident, ..)) => Name::Ident(ident.clone()),
                                None => Name::UnnamedArg(index)
                            }));
                            Expression::from_lambda(field_path, lambda_args)
                        } else {
                            Expression::from_primitive(field_path)
                        }
                    },
                    TypeModelKind::Optional(ty) => if ty.as_type().first_nested_type().unwrap().is_primitive() {
                        Expression::from_primitive_opt(field_path)
                    } else {
                        Expression::from_complex_opt(field_path)
                    }
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        let nested_ty = ty.first_nested_type().unwrap();
                        let full_nested_ty = full_ty.first_nested_type().unwrap();
                        match (<Type as Resolve<SpecialType<LANG, SPEC>>>::maybe_resolve(full_nested_ty, source),
                               nested_ty.maybe_object(source)) {
                            (Some(SpecialType::Opaque(..)),
                                Some(ObjectKind::Item(TypeModelKind::FnPointer(..) |
                                                      TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..) |
                                     ObjectKind::Type(TypeModelKind::FnPointer(..) |
                                                      TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..))))
                            ) =>
                                Expression::new_box(field_path),
                            (Some(SpecialType::Opaque(..)), _any_other) =>
                                Expression::from_raw_box(field_path),
                            (Some(SpecialType::Custom(..)), _any) =>
                                Expression::new_box(Expression::from_complex(field_path)),
                            (_, Some(obj)) => {
                                Expression::new_box(match obj.maybe_lambda_args() {
                                    Some(lambda_args) =>
                                        Expression::from_lambda(field_path, lambda_args),
                                    None =>
                                        Expression::from_complex(field_path)
                                })
                            }
                            _ =>
                                Expression::new_box(Expression::from_complex(field_path))
                        }
                    },
                    TypeModelKind::Bounds(bounds) => {
                        bounds.expr_from(field_path)
                    },
                    TypeModelKind::Unknown(..) => {
                        match TypeKind::from(ty) {
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) =>
                                    Expression::from_primitive_opt(field_path),
                                _ =>
                                    Expression::from_complex_opt(field_path),
                            }
                            TypeKind::Generic(..) =>
                                Expression::from_complex(field_path),
                            _ =>
                                Expression::from_primitive(field_path),
                        }
                    },
                    _ => {
                        match TypeKind::from(ty) {
                            TypeKind::Primitive(_) =>
                                Expression::from_primitive(field_path),
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
                                TypeKind::Primitive(_) =>
                                    Expression::from_primitive_opt(field_path),
                                _ =>
                                    Expression::from_complex_opt(field_path),
                            }
                            _ =>
                                Expression::from_complex(field_path)
                        }
                    }
                }
            }
        };
        expression
    }
}