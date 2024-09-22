use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, Type};
use ferment_macro::ComposerBase;
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated, SemiPunctuated, Void};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, Composer, ComposerLink, constants, GenericComposerInfo};
use crate::context::ScopeContext;
use crate::conversion::{complex_opt_arg_composer, GenericArgComposer, GenericArgPresentation, GenericTypeKind, primitive_opt_arg_composer, TypeKind};
use crate::ext::{Accessory, FFISpecialTypeResolve, FFIVarResolve, GenericNestedArg, Mangle, SpecialType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct ResultComposer<LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG> + 'static,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub ty: Type,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC>,
}

impl<LANG, SPEC> ResultComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        Self {
            base: BasicComposer::from(AttrsModel::from(&attrs), ty_context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            ty: ty.clone()
        }
    }
}

impl<'a, SPEC> Composer<'a> for ResultComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let compose = |arg_name: &Name, ty: &Type| match TypeKind::from(ty) {
            TypeKind::Primitive(arg_ty) => {
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    arg_ty,
                    Expression::destroy_primitive_opt_tokens(DictionaryExpr::SelfProp(arg_name.to_token_stream())),
                    Expression::map_expr(Expression::DictionaryName(DictionaryName::O), Expression::deref_tokens(DictionaryName::O.to_token_stream())),
                    Expression::boxed(DictionaryName::O))
            }
            TypeKind::Complex(arg_ty) => {
                let arg_composer = match arg_ty.maybe_special_type(source) {
                    Some(SpecialType::Opaque(..)) =>
                        GenericArgComposer::<RustFermentate, SPEC>::new(Some(Expression::deref_tokens), Some(Expression::boxed), Some(Expression::destroy_complex_opt_tokens)),
                    _ =>
                        GenericArgComposer::<RustFermentate, SPEC>::new(Some(Expression::from_complex_tokens), Some(Expression::ffi_to_complex_tokens), Some(Expression::destroy_complex_opt_tokens)),
                };
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    arg_ty.special_or_to_ffi_full_path_type(source),
                    arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()),
                    Expression::map_expr(Expression::DictionaryName(DictionaryName::O), arg_composer.from(DictionaryName::O.to_token_stream())),
                    arg_composer.to(DictionaryName::O.to_token_stream()))
            }
            TypeKind::Generic(generic_arg_ty) => {
                let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                    match generic_arg_ty.ty() {
                        None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                        Some(ty) => match TypeKind::from(ty) {
                            TypeKind::Primitive(_) => (primitive_opt_arg_composer::<RustFermentate, SPEC>(), ty.special_or_to_ffi_full_path_type(source)),
                            TypeKind::Generic(nested_nested) => (complex_opt_arg_composer::<RustFermentate, SPEC>(), nested_nested.special_or_to_ffi_full_path_type(source)),
                            _ => (complex_opt_arg_composer::<RustFermentate, SPEC>(), ty.special_or_to_ffi_full_path_type(source)),
                        }
                    }
                } else { (GenericArgComposer::new(Some(Expression::from_complex_tokens), Some(Expression::ffi_to_complex_tokens), Some(Expression::destroy_complex_opt_tokens)), generic_arg_ty.special_or_to_ffi_full_path_type(source)) };
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    arg_ty,
                    arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()),
                    Expression::map_expr(Expression::DictionaryName(DictionaryName::O), arg_composer.from(DictionaryName::O.to_token_stream())),
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
                field_composers.push(FieldComposer::named(name.clone(), FieldTypeKind::Type(ty.joined_mut())));
            });
        let attrs = self.compose_attributes();
        let types = (ffi_type.clone(), self.present_target_aspect());
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(
            self.ty.mangle_ident_default(),
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
