use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, Type};
use ferment_macro::ComposerBase;
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated, SemiPunctuated, Void};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, constants, GenericComposerInfo, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{complex_opt_arg_composer, GenericArgComposer, GenericArgPresentation, GenericTypeKind, primitive_opt_arg_composer, result_complex_arg_composer, TypeKind};
use crate::ext::{Accessory, FFISpecialTypeResolve, FFIVarResolve, GenericNestedArg, Mangle, SpecialType, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIVariable, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct ResultComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub ty: Type,
    base: BasicComposerLink<Self, LANG, SPEC>,
}

impl<LANG, SPEC> ResultComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(AttrsModel::from(&attrs), ty_context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            ty: ty.clone()
        }
    }
}

impl<SPEC> SourceComposable for ResultComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let compose = |arg_name: &Name, ty: &Type| match TypeKind::from(ty) {
            TypeKind::Primitive(arg_ty) => {
                GenericArgPresentation::new(
                    FFIVariable::direct(arg_ty),
                    Expression::destroy_primitive_opt_tokens(DictionaryExpr::SelfProp(arg_name.to_token_stream())),
                    Expression::map_o_expr(Expression::deref_tokens(DictionaryName::O.to_token_stream())),
                    Expression::boxed_tokens(DictionaryName::O))
            }
            TypeKind::Complex(arg_ty) => {
                let arg_composer = match <Type as FFISpecialTypeResolve<RustFermentate, SPEC>>::maybe_special_type(&arg_ty, source) {
                    Some(SpecialType::Opaque(..)) =>
                        GenericArgComposer::<RustFermentate, SPEC>::new(
                            Some(Expression::deref_tokens),
                            Some(Expression::boxed_tokens),
                            Some(Expression::destroy_complex_opt_tokens)),
                    _ =>
                        result_complex_arg_composer(),
                };
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::direct(FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&arg_ty, source)),
                    arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()),
                    Expression::map_o_expr(arg_composer.from(DictionaryName::O.to_token_stream())),
                    arg_composer.to(DictionaryName::O.to_token_stream()))
            }
            TypeKind::Generic(generic_arg_ty) => {
                let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                    match generic_arg_ty.ty() {
                        None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                        Some(ty) => match TypeKind::from(ty) {
                            TypeKind::Primitive(_) => (
                                primitive_opt_arg_composer::<RustFermentate, SPEC>(),
                                FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(ty, source)
                            ),
                            TypeKind::Generic(nested_nested) => (
                                complex_opt_arg_composer::<RustFermentate, SPEC>(),
                                FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&nested_nested, source)
                            ),
                            _ => (
                                complex_opt_arg_composer::<RustFermentate, SPEC>(),
                                FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(ty, source)
                            ),
                        }
                    }
                } else {
                    (result_complex_arg_composer(), FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&generic_arg_ty, source))
                };
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::direct(arg_ty),
                    arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()),
                    Expression::map_o_expr(arg_composer.from(DictionaryName::O.to_token_stream())),
                    arg_composer.to(DictionaryName::O.to_token_stream()))
            }
        };

        let nested_types = self.ty.nested_types();
        let ffi_type = self.present_ffi_aspect();
        let field_names = CommaPunctuated::from_iter([
            Name::Dictionary(DictionaryName::Ok),
            Name::Dictionary(DictionaryName::Error)
        ]);
        let mut from_conversions = CommaPunctuated::new();
        let mut to_conversions = CommaPunctuated::new();
        let mut destroy_conversions = SemiPunctuated::new();
        let mut field_composers = Depunctuated::new();
        field_names.iter()
            .enumerate()
            .for_each(|(index, name)| {
                let GenericArgPresentation { from_conversion, to_conversion, destructor, ty } = compose(name, nested_types[index]);
                from_conversions.push(Expression::<RustFermentate, SPEC>::ffi_ref_with_name(name).present(source));
                from_conversions.push(from_conversion.present(source));
                to_conversions.push(DictionaryExpr::Mapper(DictionaryName::O.to_token_stream(), to_conversion.present(source)));
                destroy_conversions.push(<SPEC::Expr as ScopeContextPresentable>::present(&destructor, source));
                field_composers.push(FieldComposer::named(name.clone(), FieldTypeKind::Type(
                    ty.to_type().joined_mut()
                )));
            });
        let attrs = self.compose_attributes();
        let types = (ffi_type.clone(), self.present_target_aspect());
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(
            self.ty.mangle_tokens_default(),
            &attrs,
            field_composers,
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from_root(&attrs, &types, InterfacesMethodExpr::FoldToResult(from_conversions.to_token_stream()), &None),
                InterfacePresentation::conversion_to_boxed(&attrs, &types, BraceWrapped::<_, Void>::new(quote!(let (#field_names) = ferment::to_result(obj, #to_conversions); Self { #field_names })), &None),
                InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
                InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
            ])
        ))
    }
}
