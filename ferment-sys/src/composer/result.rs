use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, Lifetime, Type};
use ferment_macro::ComposerBase;
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated, SemiPunctuated, Void};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink, FromConversionFullComposer};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{complex_opt_arg_composer, GenericArgComposer, GenericArgPresentation, GenericTypeKind, primitive_opt_arg_composer, result_complex_arg_composer, TypeKind};
use crate::ext::{Accessory, FFISpecialTypeResolve, FFIVarResolve, GenericNestedArg, LifetimeProcessor, Mangle, SpecialType};
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, FFIVariable, InterfacePresentation, InterfacesMethodExpr, Name};

#[derive(ComposerBase)]
pub struct ResultComposer<SPEC>
    where SPEC: Specification + 'static {
    pub ty: Type,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> ResultComposer<SPEC>
    where SPEC: Specification {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            ty: ty.clone()
        }
    }
}

impl SourceComposable for ResultComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut lifetimes = Vec::<Lifetime>::new();
        let compose = |arg_name: &Name<RustSpecification>, ty: &Type| {
            let from_conversion = FromConversionFullComposer::<RustSpecification>::value(Name::dictionary_name(DictionaryName::O), ty)
                .compose(source);
            match TypeKind::from(ty) {
                TypeKind::Primitive(arg_ty) => {
                    GenericArgPresentation::<RustSpecification>::new(
                        FFIVariable::direct(arg_ty),
                        Expression::destroy_primitive_opt_tokens(DictionaryExpr::self_prop(&arg_name)),
                        Expression::map_o_expr(Expression::deref_expr(from_conversion)),
                        Expression::boxed_tokens(DictionaryName::O))
                }
                TypeKind::Complex(arg_ty) => {
                    let arg_composer = match <Type as FFISpecialTypeResolve<RustSpecification>>::maybe_special_type(&arg_ty, source) {
                        Some(SpecialType::Opaque(..)) =>
                            GenericArgComposer::<RustSpecification>::new(
                                Some(Expression::deref_tokens),
                                Some(Expression::boxed_tokens),
                                Some(Expression::destroy_complex_opt_tokens)),
                        _ =>
                            result_complex_arg_composer(),
                    };
                    GenericArgPresentation::<RustSpecification>::new(
                        FFIVariable::direct(FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&arg_ty, source)),
                        arg_composer.destroy(DictionaryExpr::self_prop(&arg_name)),
                        Expression::map_o_expr(from_conversion),
                        arg_composer.to(DictionaryName::O))
                }
                TypeKind::Generic(generic_arg_ty) => {
                    let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                        match generic_arg_ty.ty() {
                            None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                            Some(ty) => match TypeKind::from(ty) {
                                TypeKind::Primitive(_) => (
                                    primitive_opt_arg_composer::<RustSpecification>(),
                                    FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(ty, source)
                                ),
                                TypeKind::Generic(nested_nested) => (
                                    complex_opt_arg_composer::<RustSpecification>(),
                                    FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&nested_nested, source)
                                ),
                                _ => (
                                    complex_opt_arg_composer::<RustSpecification>(),
                                    FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(ty, source)
                                ),
                            }
                        }
                    } else {
                        (
                            result_complex_arg_composer::<RustSpecification>(),
                            FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&generic_arg_ty, source)
                        )
                    };
                    GenericArgPresentation::<RustSpecification>::new(
                        FFIVariable::direct(arg_ty),
                        arg_composer.destroy(DictionaryExpr::self_prop(&arg_name)),
                        Expression::map_o_expr(from_conversion),
                        arg_composer.to(DictionaryName::O))
                }
            }
        };

        let nested_types = self.ty.nested_types();
        let ffi_type = self.present_ffi_aspect();
        let field_names = CommaPunctuated::from_iter([
            <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Ok),
            <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Error)
        ]);
        let mut from_conversions = CommaPunctuated::new();
        let mut to_conversions = CommaPunctuated::new();
        let mut destroy_conversions = SemiPunctuated::new();
        let mut field_composers = Depunctuated::new();
        field_names.iter()
            .enumerate()
            .for_each(|(index, name)| {
                let nested = nested_types[index];
                lifetimes.extend(nested.unique_lifetimes());
                let GenericArgPresentation { from_conversion, to_conversion, destructor, ty } = compose(name, nested);
                from_conversions.push(Expression::<RustSpecification>::ffi_ref_with_name(name).present(source));
                from_conversions.push(from_conversion.present(source));
                to_conversions.push(DictionaryExpr::Mapper(DictionaryName::O.to_token_stream(), to_conversion.present(source)));
                destroy_conversions.push(destructor.present(source));
                field_composers.push(FieldComposer::named(name.clone(), FieldTypeKind::Var(ty.joined_mut())));
            });
        let attrs = self.compose_attributes();
        let types = (ffi_type.clone(), self.present_target_aspect());
        Some(GenericComposerInfo::<RustSpecification>::default(
            Aspect::raw_struct_ident(self.ty.mangle_ident_default()),
            &attrs,
            field_composers,
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from_root(&attrs, &types, InterfacesMethodExpr::FoldToResult(from_conversions.to_token_stream()), &None, &lifetimes),
                InterfacePresentation::conversion_to_boxed(&attrs, &types, BraceWrapped::<_, Void>::new(quote!(let (#field_names) = ferment::to_result(obj, #to_conversions); Self { #field_names })), &None, &lifetimes),
                InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
            ])
        ))
    }
}
