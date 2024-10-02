use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, parse_quote, Type, TypeSlice};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, constants, GenericComposerInfo, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{ExpressionComposer, GenericArgPresentation, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, Mangle, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIVariable, FFIVecConversionMethodExpr, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct SliceComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub ty: Type,
    base: BasicComposerLink<Self, LANG, SPEC>,
}

impl<LANG, SPEC> SliceComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(AttrsModel::from(&attrs), ty_context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            ty: ty.clone(),
        }
    }
}

impl<SPEC> SourceComposable for SliceComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { ty, .. } = self;
        let ffi_name = ty.mangle_tokens_default();
        let type_slice: TypeSlice = parse_quote!(#ty);
        let arg_0_name = Name::Dictionary(DictionaryName::Values);
        let count_name = Name::Dictionary(DictionaryName::Count);
        let self_props = CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())]);

        let arg_0_destroy = |composer: ExpressionComposer<RustFermentate, SPEC>|
            composer(self_props.to_token_stream());
        let arg_0_from = |composer: ExpressionComposer<RustFermentate, SPEC>|
            composer(self_props.to_token_stream());
        let arg_0_to = |composer: ExpressionComposer<RustFermentate, SPEC>|
            Expression::boxed_tokens(DictionaryExpr::SelfDestructuring(
                CommaPunctuated::from_iter([
                    FieldComposer::<RustFermentate, SPEC>::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                    FieldComposer::<RustFermentate, SPEC>::named(arg_0_name.clone(), FieldTypeKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).present(source).to_token_stream()))
                ]).to_token_stream()));
        let arg_0_presentation = match TypeKind::from(&type_slice.elem) {
            TypeKind::Primitive(arg_0_target_path) =>
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::direct(arg_0_target_path.clone()),
                    arg_0_destroy(Expression::destroy_primitive_group_tokens),
                    arg_0_from(Expression::from_primitive_group_tokens),
                    arg_0_to(Expression::ffi_to_primitive_group_tokens)),
            TypeKind::Complex(arg_0_target_ty) =>
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::mut_ptr(FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&arg_0_target_ty, source)),
                    arg_0_destroy(Expression::destroy_complex_group_tokens),
                    arg_0_from(Expression::from_complex_group_tokens),
                    arg_0_to(Expression::ffi_to_complex_group_tokens)),
            TypeKind::Generic(arg_0_generic_path_conversion) =>
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    FFIVariable::mut_ptr(FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&arg_0_generic_path_conversion, source)),
                    arg_0_destroy(Expression::destroy_complex_group_tokens),
                    arg_0_from(Expression::from_complex_group_tokens),
                    arg_0_to(Expression::ffi_to_complex_group_tokens))

        };
        let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let field_composers = Depunctuated::from_iter([
            FieldComposer::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
            FieldComposer::named(arg_0_name, FieldTypeKind::Type(arg_0_presentation.ty.to_type().joined_mut()))
        ]);
        let expr_destroy_iterator = [
            <SPEC::Expr as ScopeContextPresentable>::present(&arg_0_presentation.destructor, source).to_token_stream()
        ];
        let attrs = self.compose_attributes();
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from(&attrs, &types, FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()), &None),
            InterfacePresentation::conversion_to(&attrs, &types, FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()), &None),
            InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
            InterfacePresentation::vec(&attrs, &types, <SPEC::Expr as ScopeContextPresentable>::present(&arg_0_presentation.from_conversion, source).to_token_stream(), <SPEC::Expr as ScopeContextPresentable>::present(&arg_0_presentation.to_conversion, source).to_token_stream()),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), SemiPunctuated::from_iter(expr_destroy_iterator))

        ]);
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(ffi_name, &attrs, field_composers, interfaces))
    }
}
