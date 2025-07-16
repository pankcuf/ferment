use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Type, Lifetime};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericArgComposer, GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFISpecialTypeResolve, FFIVarResolve, FermentableDictionaryType, GenericNestedArg, LifetimeProcessor, Mangle, SpecialType};
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, FFIVariable, InterfacePresentation};

#[derive(ComposerBase)]
pub struct GroupComposer<SPEC>
    where SPEC: Specification + 'static {
    pub ty: Type,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> GroupComposer<SPEC>
    where SPEC: Specification {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            ty: ty.clone(),
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
        }

    }
}

impl SourceComposable for GroupComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut lifetimes = Vec::<Lifetime>::new();
        let nested_type_kind = TypeKind::from(self.ty.maybe_first_nested_type_ref()?);

        let arg_0_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Values);
        let count_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Count);
        let from_args = CommaPunctuated::from_iter([
            DictionaryExpr::ffi_ref_prop(&arg_0_name),
            DictionaryExpr::ffi_ref_prop(&count_name),
        ]);
        let drop_args = CommaPunctuated::from_iter([
            DictionaryExpr::self_prop(&arg_0_name),
            DictionaryExpr::self_prop(&count_name)
        ]);
        let arg_0_to = |expr: Expression<RustSpecification>|
            Expression::boxed_tokens(DictionaryExpr::self_destruct(
                CommaPunctuated::from_iter([
                    FieldComposer::<RustSpecification>::named(count_name.clone(), FieldTypeKind::conversion(DictionaryExpr::ObjLen)),
                    FieldComposer::<RustSpecification>::named(arg_0_name.clone(), FieldTypeKind::Conversion(expr.present(source)))
                ])));
        lifetimes.extend(nested_type_kind.unique_lifetimes());
        let arg_presentation = match &nested_type_kind {
            TypeKind::Primitive(arg_0_target_path) => {
                GenericArgPresentation::<RustSpecification>::new(
                    FFIVariable::direct(arg_0_target_path.clone()),
                    Expression::destroy_primitive_group_tokens(drop_args),
                    Expression::from_primitive_group_tokens(from_args),
                    arg_0_to(Expression::ffi_to_primitive_group_tokens(DictionaryExpr::ObjIntoIter))
                )
            }
            TypeKind::Complex(arg_0_target_ty) => {
                match FFISpecialTypeResolve::<RustSpecification>::maybe_special_type(arg_0_target_ty, source) {
                    Some(SpecialType::Opaque(..)) => {
                        GenericArgPresentation::<RustSpecification>::new(
                            FFIVariable::mut_ptr(FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(arg_0_target_ty, source)),
                            Expression::destroy_complex_group_tokens(drop_args),
                            Expression::from_opaque_group_tokens(from_args),
                            arg_0_to(Expression::ffi_to_opaque_group_tokens(DictionaryExpr::ObjIntoIter))
                        )
                    }
                    _ => {
                        GenericArgPresentation::<RustSpecification>::new(
                            FFIVariable::mut_ptr(FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(arg_0_target_ty, source)),
                            if arg_0_target_ty.is_fermentable_string() {
                                Expression::destroy_string_group_tokens(drop_args)
                            } else {
                                Expression::destroy_complex_group_tokens(drop_args)
                            },
                            Expression::from_complex_group_tokens(from_args),
                            arg_0_to(Expression::ffi_to_complex_group_tokens(DictionaryExpr::ObjIntoIter))
                        )

                    }
                }
            }
            TypeKind::Generic(arg_0_generic_path_conversion) => {
                let (arg_0_composer, arg_ty) = {
                    if let GenericTypeKind::Optional(..) = arg_0_generic_path_conversion {
                        match arg_0_generic_path_conversion.ty() {
                            None => unimplemented!("Mixin inside generic: {}", arg_0_generic_path_conversion),
                            Some(ty) => match TypeKind::from(ty) {
                                TypeKind::Primitive(_) =>
                                    (GenericArgComposer::<RustSpecification>::new(
                                        Some(Expression::from_primitive_opt_group_tokens),
                                        Some(Expression::ffi_to_primitive_opt_group_tokens),
                                        Some(Expression::destroy_complex_group_tokens)),
                                     FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_variable_type(ty, source)),
                                TypeKind::Generic(nested_nested) => {
                                    (GenericArgComposer::<RustSpecification>::new(
                                        Some(Expression::from_complex_opt_group_tokens),
                                        Some(Expression::ffi_to_complex_opt_group_tokens),
                                        Some(Expression::destroy_complex_group_tokens)),
                                     FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_variable_type(&nested_nested, source))
                                },
                                _ => (GenericArgComposer::<RustSpecification>::new(
                                    Some(Expression::from_complex_opt_group_tokens),
                                    Some(Expression::ffi_to_complex_opt_group_tokens),
                                    Some(Expression::destroy_complex_group_tokens)),
                                      FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_variable_type(ty, source)),
                            }
                        }
                    } else {
                        (GenericArgComposer::<RustSpecification>::new(
                            Some(Expression::from_complex_group_tokens),
                            Some(Expression::ffi_to_complex_group_tokens),
                            Some(Expression::destroy_complex_group_tokens)),
                         FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_variable_type(arg_0_generic_path_conversion, source))
                    }
                };
                GenericArgPresentation::<RustSpecification>::new(
                    FFIVariable::direct(arg_ty),
                    arg_0_composer.destroy(drop_args),
                    arg_0_composer.from(from_args),
                    arg_0_to(arg_0_composer.to_composer.map(|c| c(DictionaryExpr::ObjIntoIter.to_token_stream())).unwrap_or_default())
                )
            }
        };
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let expr_destroy_iterator = [
            arg_presentation.destructor.present(source)
        ];
        let to_body = arg_presentation.to_conversion.present(source);
        let from_body = DictionaryExpr::FromRoot(arg_presentation.from_conversion.present(source));

        Some(GenericComposerInfo::default(
            Aspect::raw_struct_ident(self.ty.mangle_ident_default()),
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::<RustSpecification>::named(count_name, FieldTypeKind::type_count()),
                FieldComposer::<RustSpecification>::named(arg_0_name, FieldTypeKind::Var(arg_presentation.ty.joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &lifetimes),
                InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &lifetimes),
                InterfacePresentation::drop(&attrs, ffi_type, SemiPunctuated::from_iter(expr_destroy_iterator))
            ])
        ))
    }
}




