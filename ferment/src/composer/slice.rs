use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics, parse_quote, Type, TypeSlice};
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, RustFieldComposer};
use crate::composer::{Composer, GenericComposerInfo, NameComposable, ComposerLink, BasicComposer, constants, BasicComposerOwner, AttrComposable, SourceAccessible, NameContext};
use crate::context::ScopeContext;
use crate::conversion::{DESTROY_COMPLEX_GROUP, DESTROY_PRIMITIVE_GROUP, FROM_COMPLEX_GROUP, FROM_PRIMITIVE_GROUP, GenericArgPresentation, RustExpressionComposer, TO_COMPLEX_GROUP, TO_PRIMITIVE_GROUP, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, Mangle, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIVecConversionMethodExpr, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

pub struct SliceComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub ty: Type,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    phantom_data: PhantomData<LANG>,
}

impl<LANG, SPEC, Gen> SliceComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn new(ty: &Type, context: Context, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(AttrsModel::from(&attrs), context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            ty: ty.clone(),
            phantom_data: PhantomData }
    }
}
impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for SliceComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}
impl<LANG, SPEC, Gen> NameContext<Context> for SliceComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for SliceComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}

impl<'a> Composer<'a> for SliceComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let Self { ty, .. } = self;
        let ffi_name = ty.mangle_ident_default();
        let type_slice: TypeSlice = parse_quote!(#ty);
        let arg_0_name = Name::Dictionary(DictionaryName::Values);
        let count_name = Name::Dictionary(DictionaryName::Count);
        let self_props = CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())]);

        let arg_0_destroy = |composer: RustExpressionComposer|
            composer(self_props.to_token_stream());
        let arg_0_from = |composer: RustExpressionComposer|
            composer(self_props.to_token_stream());
        let arg_0_to = |composer: RustExpressionComposer|
            Expression::InterfacesExpr(
                InterfacesMethodExpr::Boxed(
                    DictionaryExpr::SelfDestructuring(
                        CommaPunctuated::from_iter([
                            RustFieldComposer::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                            RustFieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).present(source)))]).to_token_stream())
                        .to_token_stream()));


        let arg_0_presentation = match TypeKind::from(&type_slice.elem) {
            TypeKind::Primitive(arg_0_target_path) =>
                GenericArgPresentation::new(
                    arg_0_target_path.clone(),
                    arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                    arg_0_from(FROM_PRIMITIVE_GROUP),
                    arg_0_to(TO_PRIMITIVE_GROUP)),
            TypeKind::Complex(arg_0_target_ty) =>
                GenericArgPresentation::new(
                    arg_0_target_ty.special_or_to_ffi_full_path_variable_type(source),
                    arg_0_destroy(DESTROY_COMPLEX_GROUP),
                    arg_0_from(FROM_COMPLEX_GROUP),
                    arg_0_to(TO_COMPLEX_GROUP)),
            TypeKind::Generic(arg_0_generic_path_conversion) =>
                GenericArgPresentation::new(
                    arg_0_generic_path_conversion.special_or_to_ffi_full_path_variable_type(source),
                    arg_0_destroy(DESTROY_COMPLEX_GROUP),
                    arg_0_from(FROM_COMPLEX_GROUP),
                    arg_0_to(TO_COMPLEX_GROUP))

        };
        let types = (self.compose_ffi_name(), self.compose_target_name());
        let field_composers = Depunctuated::from_iter([
            FieldComposer::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
            FieldComposer::named(arg_0_name, FieldTypeKind::Type(arg_0_presentation.ty.joined_mut()))
        ]);
        let expr_destroy_iterator = [
            arg_0_presentation.destructor.present(source)
        ];
        let attrs = self.base.compose_attributes();
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from(&attrs, &types, FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()), &None),
            InterfacePresentation::conversion_to(&attrs, &types, FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()), &None),
            InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
            InterfacePresentation::vec(&attrs, &types, arg_0_presentation.from_conversion.present(source), arg_0_presentation.to_conversion.present(source)),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), SemiPunctuated::from_iter(expr_destroy_iterator))

        ]);
        Some(GenericComposerInfo::<RustFermentate, Vec<Attribute>, Option<Generics>>::default(ffi_name, &attrs, field_composers, interfaces))
    }
}
