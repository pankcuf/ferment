use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, parse_quote, Type, Lifetime, Generics};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericArgComposer, GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, FermentableDictionaryType, GenericNestedArg, LifetimeProcessor, Mangle, ToType};
use crate::lang::{FromDictionary, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable, TypeContext};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, FFIVariable, InterfacePresentation, RustFermentate};

#[derive(ComposerBase)]
pub struct ArrayComposer<LANG, SPEC>
where LANG: LangFermentable + 'static,
      SPEC: Specification<LANG> + 'static {
    pub ty: Type,
    pub nested_type_kind: TypeKind,
    base: BasicComposerLink<LANG, SPEC, Self>,
}

impl<LANG, SPEC> ArrayComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        let nested_ty = ty.maybe_first_nested_type_ref().unwrap();
        Self {
            ty: ty.clone(),
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            nested_type_kind: TypeKind::from(nested_ty),
        }

    }
}

impl<SPEC> SourceComposable for ArrayComposer<RustFermentate, SPEC>
where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut lifetimes = Vec::<Lifetime>::new();
        let arg_0_name = SPEC::Name::dictionary_name(DictionaryName::Values);
        let count_name = SPEC::Name::dictionary_name(DictionaryName::Count);
        let from_args = CommaPunctuated::from_iter([
            quote!(ffi_ref.#arg_0_name),
            quote!(ffi_ref.#count_name),
        ]);
        let drop_args = CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())
        ]);
        let arg_0_to = |expr: Expression<RustFermentate, SPEC>|
            Expression::boxed_tokens(DictionaryExpr::SelfDestructuring(
                CommaPunctuated::from_iter([
                    FieldComposer::<RustFermentate, SPEC>::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                    FieldComposer::<RustFermentate, SPEC>::named(arg_0_name.clone(), FieldTypeKind::Conversion(expr.present(source)))
                ])
                    .to_token_stream()));
        let nested_ty_kind = &self.nested_type_kind;
        lifetimes.extend(nested_ty_kind.to_type().unique_lifetimes());
        let arg_presentation = match nested_ty_kind {
            TypeKind::Primitive(arg_0_target_path) => {
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::direct(arg_0_target_path.clone()),
                    Expression::destroy_primitive_group_tokens(drop_args),
                    Expression::from_primitive_group_tokens(from_args),
                    arg_0_to(Expression::ffi_to_primitive_group_tokens(DictionaryExpr::ObjIntoIter))
                )
            }
            TypeKind::Complex(arg_0_target_ty) => {
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::mut_ptr(FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(arg_0_target_ty, source)),
                    if arg_0_target_ty.is_fermentable_string() {
                        Expression::destroy_string_group_tokens(drop_args)
                    } else {
                        Expression::destroy_complex_group_tokens(drop_args)
                    },
                    Expression::from_complex_group_tokens(from_args),
                    arg_0_to(Expression::ffi_to_complex_group_tokens(DictionaryExpr::ObjIntoIter))
                )
            }
            TypeKind::Generic(arg_0_generic_path_conversion) => {
                let (arg_0_composer, arg_ty) = {
                    if let GenericTypeKind::Optional(..) = arg_0_generic_path_conversion {
                        match arg_0_generic_path_conversion.ty() {
                            None => unimplemented!("Mixin inside generic: {}", arg_0_generic_path_conversion),
                            Some(ty) => match TypeKind::from(ty) {
                                TypeKind::Primitive(_) =>
                                    (GenericArgComposer::<RustFermentate, SPEC>::new(
                                        Some(Expression::from_primitive_opt_group_tokens),
                                        Some(Expression::ffi_to_primitive_opt_group_tokens),
                                        Some(Expression::destroy_complex_group_tokens)),
                                     FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(ty, source)),
                                TypeKind::Generic(nested_nested) => {
                                    (GenericArgComposer::<RustFermentate, SPEC>::new(
                                        Some(Expression::from_complex_opt_group_tokens),
                                        Some(Expression::ffi_to_complex_opt_group_tokens),
                                        Some(Expression::destroy_complex_group_tokens)),
                                     FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(&nested_nested, source))
                                },
                                _ => (GenericArgComposer::<RustFermentate, SPEC>::new(
                                    Some(Expression::from_complex_opt_group_tokens),
                                    Some(Expression::ffi_to_complex_opt_group_tokens),
                                    Some(Expression::destroy_complex_group_tokens)),
                                      FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(ty, source)),
                            }
                        }
                    } else {
                        (GenericArgComposer::<RustFermentate, SPEC>::new(
                            Some(Expression::from_complex_group_tokens),
                            Some(Expression::ffi_to_complex_group_tokens),
                            Some(Expression::destroy_complex_group_tokens)),
                         FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(arg_0_generic_path_conversion, source))
                    }
                };
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::direct(arg_ty),
                    arg_0_composer.destroy(drop_args.to_token_stream()),
                    arg_0_composer.from(from_args.to_token_stream()),
                    arg_0_to(arg_0_composer.to_composer.map(|c| c(DictionaryExpr::ObjIntoIter.to_token_stream())).unwrap_or(Expression::empty()))
                )
            }
        };
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let expr_destroy_iterator = [
            arg_presentation.destructor.present(source)
        ];
        let from_group_conversion = SPEC::Expr::present(&arg_presentation.from_conversion, source);
        let result_conversion = DictionaryExpr::TryIntoUnwrap(quote!(vec));
        let root_body = quote! {
            let vec: Vec<#nested_ty_kind> = #from_group_conversion;
            #result_conversion
        };
        let from_body = DictionaryExpr::FromRoot(root_body);
        let to_body = SPEC::Expr::present(&arg_presentation.to_conversion, source);

        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(
            Aspect::RawTarget(TypeContext::Struct { ident: self.ty.mangle_ident_default(), attrs: vec![], generics: Generics::default() }),
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::<RustFermentate, SPEC>::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
                FieldComposer::<RustFermentate, SPEC>::named(arg_0_name, FieldTypeKind::Type(arg_presentation.ty.to_type().joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from(&attrs, &types, from_body, &None, &lifetimes),
                InterfacePresentation::conversion_to(&attrs, &types, to_body, &None, &lifetimes),
                InterfacePresentation::drop(&attrs, ffi_type, SemiPunctuated::from_iter(expr_destroy_iterator))
            ])
        ))
    }
}




