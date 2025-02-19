use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, parse_quote, Type, TypeSlice, Generics};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{ExpressionComposer, GenericArgPresentation, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, Mangle, ToType};
use crate::lang::{FromDictionary, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable, TypeContext};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, FFIVariable, InterfacePresentation, RustFermentate};

#[derive(ComposerBase)]
pub struct SliceComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub ty: Type,
    base: BasicComposerLink<LANG, SPEC, Self>,
}

impl<LANG, SPEC> SliceComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
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
        let arg_0_name = SPEC::Name::dictionary_name(DictionaryName::Values);
        let count_name = SPEC::Name::dictionary_name(DictionaryName::Count);
        let self_props = CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())]);
        let from_args = CommaPunctuated::from_iter([
            quote!(ffi_ref.#arg_0_name),
            quote!(ffi_ref.#count_name),
        ]);

        let arg_0_destroy = |composer: ExpressionComposer<RustFermentate, SPEC>|
            composer(self_props.to_token_stream());
        let arg_0_from = |composer: ExpressionComposer<RustFermentate, SPEC>|
            composer(from_args.to_token_stream());
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
        let from_body = DictionaryExpr::FromRoot(SPEC::Expr::present(&arg_0_presentation.from_conversion, source));
        let to_body = SPEC::Expr::present(&arg_0_presentation.to_conversion, source);

        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from(&attrs, &types, from_body, &None, &vec![]),
            InterfacePresentation::conversion_to(&attrs, &types, to_body, &None, &vec![]),
            // InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), SemiPunctuated::from_iter(expr_destroy_iterator))

        ]);
        let aspect = Aspect::RawTarget(TypeContext::Struct { ident: ty.mangle_ident_default(), attrs: vec![], generics: Generics::default() });
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(aspect, &attrs, field_composers, interfaces))
    }
}
