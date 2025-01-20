use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, parse_quote, Type, Generics};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink, FromConversionFullComposer};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericArgComposer, GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, FermentableDictionaryType, GenericNestedArg, Mangle, ToType};
use crate::lang::{FromDictionary, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable, TypeContext};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, FFIVariable, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct MapComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub ty: Type,
    base: BasicComposerLink<LANG, SPEC, Self>,
}

impl<LANG, SPEC> MapComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            ty: ty.clone(),
        }
    }
}

impl<SPEC> SourceComposable for MapComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let count = DictionaryName::Count;
        let keys = DictionaryName::Keys;
        let values = DictionaryName::Values;
        let count_name = SPEC::Name::dictionary_name(count.clone());
        let arg_0_name = SPEC::Name::dictionary_name(keys.clone());
        let arg_1_name = SPEC::Name::dictionary_name(values.clone());

        let arg_context = |arg_name: &Name<RustFermentate, SPEC>| quote!(obj.#arg_name().cloned());
        let arg_args = |arg_name: &Name<RustFermentate, SPEC>| CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())]);

        let compose = |arg_name: &Name<RustFermentate, SPEC>, ty: &Type| {
            let from_conversion =
                Expression::map_o_expr(
                    FromConversionFullComposer::<RustFermentate, SPEC>::value(SPEC::Name::dictionary_name(DictionaryName::O), ty)
                        .compose(source));
            let result = match TypeKind::from(ty) {
                TypeKind::Primitive(arg_ty) =>
                    GenericArgPresentation::<RustFermentate, SPEC>::new(
                        FFIVariable::direct(arg_ty),
                        Expression::destroy_primitive_group_tokens(arg_args(arg_name)),
                        from_conversion,
                        Expression::ffi_to_primitive_group_tokens(arg_context(arg_name))),
                TypeKind::Complex(arg_ty) => {
                    let arg_composer = GenericArgComposer::<RustFermentate, SPEC>::new(
                        Some(Expression::from_complex_tokens),
                        Some(Expression::ffi_to_complex_group_tokens),
                        Some(if arg_ty.is_fermentable_string() {
                            Expression::destroy_string_group_tokens
                        } else {
                            Expression::destroy_complex_group_tokens
                        }));

                    GenericArgPresentation::<RustFermentate, SPEC>::new(
                        FFIVariable::direct(FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(&arg_ty, source)),
                        arg_composer.destroy(arg_args(arg_name).to_token_stream()),
                        from_conversion,
                        arg_composer.to(arg_context(arg_name)))
                },
                TypeKind::Generic(generic_arg_ty) => {
                    let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                        match generic_arg_ty.ty() {
                            None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                            Some(ty) => (match TypeKind::from(ty) {
                                TypeKind::Primitive(_) =>
                                    GenericArgComposer::<RustFermentate, SPEC>::new(
                                        Some(Expression::from_primitive_opt_tokens),
                                        Some(Expression::ffi_to_primitive_opt_group_tokens),
                                        Some(Expression::destroy_complex_group_tokens)),
                                _ =>
                                    GenericArgComposer::<RustFermentate, SPEC>::new(
                                        Some(Expression::from_complex_opt_tokens),
                                        Some(Expression::ffi_to_complex_opt_group_tokens),
                                        Some(Expression::destroy_complex_group_tokens)),
                            }, FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(ty, source))
                        }
                    } else { (
                        GenericArgComposer::<RustFermentate, SPEC>::new(
                            Some(Expression::from_complex_tokens),
                            Some(Expression::ffi_to_complex_group_tokens),
                            Some(Expression::destroy_complex_group_tokens)),
                        FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(&generic_arg_ty, source))
                    };
                    GenericArgPresentation::<RustFermentate, SPEC>::new(
                        FFIVariable::direct(arg_ty),
                        arg_composer.destroy(arg_args(arg_name).to_token_stream()),
                        from_conversion,
                        arg_composer.to(arg_context(arg_name)))
                },
            };
            result
        };
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());

        let nested_types = self.ty.nested_types();
        let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
        let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
        let expr_from_iterator = [
            quote!(ffi_ref.#count),
            quote!(ffi_ref.#keys),
            quote!(ffi_ref.#values),
            SPEC::Expr::present(&arg_0_presentation.from_conversion, source),
            SPEC::Expr::present(&arg_1_presentation.from_conversion, source),
        ];
        let expr_to_iterator = [
            FieldComposer::<RustFermentate, SPEC>::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
            FieldComposer::<RustFermentate, SPEC>::named(arg_0_name.clone(), FieldTypeKind::Conversion(<SPEC::Expr as ScopeContextPresentable>::present(&arg_0_presentation.to_conversion, source).to_token_stream())),
            FieldComposer::<RustFermentate, SPEC>::named(arg_1_name.clone(), FieldTypeKind::Conversion(<SPEC::Expr as ScopeContextPresentable>::present(&arg_1_presentation.to_conversion, source).to_token_stream())),
        ];

        let expr_destroy_iterator = [
            SPEC::Expr::present(&arg_0_presentation.destructor, source),
            SPEC::Expr::present(&arg_1_presentation.destructor, source),
        ];
        let attrs = self.compose_attributes();
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(
            Aspect::RawTarget(TypeContext::Struct { ident: self.ty.mangle_ident_default(), attrs: vec![], generics: Generics::default()}),
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::<RustFermentate, SPEC>::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
                FieldComposer::<RustFermentate, SPEC>::named(arg_0_name, FieldTypeKind::Type(arg_0_presentation.ty.to_type().joined_mut())),
                FieldComposer::<RustFermentate, SPEC>::named(arg_1_name, FieldTypeKind::Type(arg_1_presentation.ty.to_type().joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from_root(&attrs, &types, InterfacesMethodExpr::FoldToMap(CommaPunctuated::from_iter(expr_from_iterator).to_token_stream()), &None, &vec![]),
                InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, CommaPunctuated::from_iter(expr_to_iterator), &None, &vec![]),
                InterfacePresentation::drop(&attrs, ffi_type, SemiPunctuated::from_iter(expr_destroy_iterator))
            ])
        ))
    }
}


