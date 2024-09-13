use std::marker::PhantomData;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, Generics, parse_quote, Type};
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, RustFieldComposer};
use crate::composer::{Composer, GenericComposerInfo, NameComposable, ComposerLink, BasicComposer, constants, BasicComposerOwner, AttrComposable, NameContext, SourceAccessible};
use crate::context::ScopeContext;
use crate::conversion::{DESTROY_COMPLEX_GROUP, DESTROY_PRIMITIVE_GROUP, FROM_COMPLEX, FROM_OPT_COMPLEX, FROM_OPT_PRIMITIVE, GenericArgComposer, GenericArgPresentation, GenericTypeKind, TO_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, TO_OPT_PRIMITIVE_GROUP, TO_PRIMITIVE_GROUP, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg, Mangle};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, Expression, OwnedItemPresentableContext, RustExpression, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

pub struct MapComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub ty: Type,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    phantom_data: PhantomData<LANG>,
}

impl<LANG, SPEC, Gen> MapComposer<LANG, SPEC, Gen>
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
impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for MapComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}
impl<LANG, SPEC, Gen> NameContext<Context> for MapComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for MapComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}

// impl<'a, LANG, SPEC> NameComposable<Context> for MapComposer<'a, LANG, SPEC>
//     where LANG: Clone,
//           SPEC: LangAttrSpecification<LANG> {
//     fn compose_ffi_name(&self) -> <Aspect<Context> as ScopeContextPresentable>::Presentation {
//         self.ty.mangle_ident_default().to_type()
//     }
//
//     fn compose_target_name(&self) -> <Aspect<Context> as ScopeContextPresentable>::Presentation {
//         self.ty.clone()
//     }
// }


impl<'a> Composer<'a> for MapComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let ffi_name = self.ty.mangle_ident_default();
        let count = DictionaryName::Count;
        let keys = DictionaryName::Keys;
        let values = DictionaryName::Values;
        let count_name = Name::Dictionary(count.clone());
        let arg_0_name = Name::Dictionary(keys.clone());
        let arg_1_name = Name::Dictionary(values.clone());

        let arg_context = |arg_name: &Name| quote!(obj.#arg_name().cloned());
        let arg_args = |arg_name: &Name| CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())]);

        let compose_arg = |arg_ty: Type, from_expr: RustExpression, to_expr: RustExpression, destroy_expr: RustExpression|
            GenericArgPresentation::new(
                arg_ty,
                destroy_expr,
                Expression::MapExpression(Expression::O.into(), from_expr.into()),
                to_expr);
        let compose = |arg_name: &Name, ty: &Type| match TypeKind::from(ty) {
            TypeKind::Primitive(arg_ty) =>
                GenericArgPresentation::new(
                    arg_ty,
                    DESTROY_PRIMITIVE_GROUP(arg_args(arg_name).to_token_stream()),
                    Expression::MapExpression(Expression::O.into(), Expression::O.into()),
                    TO_PRIMITIVE_GROUP(arg_context(arg_name))),
            TypeKind::Complex(arg_ty) => {
                let arg_composer = GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP);
                compose_arg(
                    arg_ty.special_or_to_ffi_full_path_variable_type(source),
                    arg_composer.from(DictionaryName::O.to_token_stream()).into(),
                    arg_composer.to(arg_context(arg_name)),
                    arg_composer.destroy(arg_args(arg_name).to_token_stream())
                )
            },
            TypeKind::Generic(generic_arg_ty) => {
                let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                    match generic_arg_ty.ty() {
                        None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                        Some(ty) => (match TypeKind::from(ty) {
                            TypeKind::Primitive(_) => GenericArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP),
                            _ => GenericArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP),
                        }, ty.special_or_to_ffi_full_path_variable_type(source))
                    }
                } else { (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), generic_arg_ty.special_or_to_ffi_full_path_variable_type(source)) };
                compose_arg(
                    arg_ty,
                    arg_composer.from(DictionaryName::O.to_token_stream()),
                    arg_composer.to(arg_context(arg_name)),
                    arg_composer.destroy(arg_args(arg_name).to_token_stream())
                )
            },
        };
        let ffi_type = self.compose_ffi_name();
        let types = (ffi_type.clone(), self.compose_target_name());

        let nested_types = self.ty.nested_types();
        let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
        let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
        let expr_from_iterator = [
            quote!(ffi_ref.#count),
            quote!(ffi_ref.#keys),
            quote!(ffi_ref.#values),
            arg_0_presentation.from_conversion.present(source),
            arg_1_presentation.from_conversion.present(source),
        ];
        let expr_to_iterator = [
            RustFieldComposer::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
            RustFieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(arg_0_presentation.to_conversion.present(source))),
            RustFieldComposer::named(arg_1_name.clone(), FieldTypeKind::Conversion(arg_1_presentation.to_conversion.present(source))),
        ];

        let expr_destroy_iterator = [
            arg_0_presentation.destructor.present(source),
            arg_1_presentation.destructor.present(source),
        ];
        let attrs = self.base.compose_attributes();
        Some(GenericComposerInfo::<RustFermentate, Vec<Attribute>, Option<Generics>>::default(
            ffi_name,
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
                FieldComposer::named(arg_0_name, FieldTypeKind::Type(arg_0_presentation.ty.joined_mut())),
                FieldComposer::named(arg_1_name, FieldTypeKind::Type(arg_1_presentation.ty.joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from_root(&attrs, &types, InterfacesMethodExpr::FoldToMap(CommaPunctuated::from_iter(expr_from_iterator).to_token_stream()), &None),
                InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, CommaPunctuated::from_iter(expr_to_iterator), &None),
                InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
                InterfacePresentation::drop(&attrs, ffi_type, SemiPunctuated::from_iter(expr_destroy_iterator))
            ])
        ))
    }
}