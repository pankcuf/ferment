use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, parse_quote, Type, TypeSlice};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{ExpressionComposer, GenericArgPresentation, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, FermentableDictionaryType, Mangle, ToType};
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, FFIVariable, InterfacePresentation, ToFFIVariable};

#[derive(ComposerBase)]
pub struct SliceComposer<SPEC>
    where SPEC: Specification + 'static {
    pub ty: Type,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> SliceComposer<SPEC>
    where SPEC: Specification {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            ty: ty.clone(),
        }
    }
}

impl SourceComposable for SliceComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { ty, .. } = self;
        let ffi_name = ty.mangle_tokens_default();
        let type_slice: TypeSlice = parse_quote!(#ty);
        let arg_0_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Values);
        let count_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Count);
        let self_props = CommaPunctuated::from_iter([
            DictionaryExpr::self_prop(&arg_0_name),
            DictionaryExpr::self_prop(&count_name)]);
        let from_args = CommaPunctuated::from_iter([
            DictionaryExpr::ffi_ref_prop(&arg_0_name),
            DictionaryExpr::ffi_ref_prop(&count_name),
        ]);

        let arg_0_destroy = |composer: ExpressionComposer<RustSpecification>|
            composer(self_props.to_token_stream());
        let arg_0_from = |composer: ExpressionComposer<RustSpecification>|
            composer(from_args.to_token_stream());
        let arg_0_to = |composer: ExpressionComposer<RustSpecification>|
            Expression::boxed_tokens(DictionaryExpr::self_destruct(
                CommaPunctuated::from_iter([
                    FieldComposer::<RustSpecification>::named(count_name.clone(), FieldTypeKind::conversion(DictionaryExpr::ObjLen)),
                    FieldComposer::<RustSpecification>::named(arg_0_name.clone(), FieldTypeKind::conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).present(source)))
                ])));
        let arg_0_presentation = match TypeKind::from(&type_slice.elem) {
            TypeKind::Primitive(arg_0_target_path) =>
                GenericArgPresentation::<RustSpecification>::new(
                    arg_0_target_path.to_direct_var(),
                    arg_0_destroy(Expression::destroy_primitive_group_tokens),
                    arg_0_from(Expression::from_primitive_group_tokens),
                    arg_0_to(Expression::ffi_to_primitive_group_tokens)),
            TypeKind::Complex(arg_0_target_ty) =>
                GenericArgPresentation::<RustSpecification>::new(
                    FFIVariable::mut_ptr(FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&arg_0_target_ty, source)),
                    arg_0_destroy(if arg_0_target_ty.is_fermentable_string() {
                        Expression::destroy_string_group_tokens
                    } else {
                        Expression::destroy_complex_group_tokens
                    }),
                    arg_0_from(Expression::from_complex_group_tokens),
                    arg_0_to(Expression::ffi_to_complex_group_tokens)),
            TypeKind::Generic(arg_0_generic_path_conversion) =>
                GenericArgPresentation::<RustSpecification>::new(
                    FFIVariable::mut_ptr(FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&arg_0_generic_path_conversion, source)),
                    arg_0_destroy(Expression::destroy_complex_group_tokens),
                    arg_0_from(Expression::from_complex_group_tokens),
                    arg_0_to(Expression::ffi_to_complex_group_tokens))

        };
        let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let field_composers = Depunctuated::from_iter([
            FieldComposer::named(count_name, FieldTypeKind::type_count()),
            FieldComposer::named(arg_0_name, FieldTypeKind::Var(arg_0_presentation.ty.joined_mut()))
        ]);
        let expr_destroy_iterator = [
            arg_0_presentation.destructor.present(source)
        ];
        let attrs = self.compose_attributes();
        let from_body = DictionaryExpr::FromRoot(arg_0_presentation.from_conversion.present(source));
        let to_body = arg_0_presentation.to_conversion.present(source);

        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &vec![]),
            InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &vec![]),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), SemiPunctuated::from_iter(expr_destroy_iterator))

        ]);
        let aspect = Aspect::raw_struct_ident(ty.mangle_ident_default());
        Some(GenericComposerInfo::<RustSpecification>::default(aspect, &attrs, field_composers, interfaces))
    }
}
