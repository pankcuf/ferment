use std::marker::PhantomData;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, Generics, Type};
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated, SemiPunctuated, Void};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{Composer, GenericComposerInfo, ComposerLink, BasicComposer, constants, NameComposable, BasicComposerOwner, AttrComposable, SourceAccessible, NameContext};
use crate::context::ScopeContext;
use crate::conversion::{COMPLEX_OPT_ARG_COMPOSER, DESTROY_OPT_COMPLEX, DESTROY_OPT_PRIMITIVE, FROM_COMPLEX, FROM_OPAQUE, GenericArgComposer, GenericArgPresentation, GenericTypeKind, PRIMITIVE_OPT_ARG_COMPOSER, TO_COMPLEX, TO_OPAQUE, TypeKind};
use crate::ext::{Accessory, FFISpecialTypeResolve, FFIVarResolve, GenericNestedArg, Mangle, SpecialType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

pub struct ResultComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub ty: Type,
    // pub attrs: SPEC,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    phantom_data: PhantomData<LANG>,
}

impl<LANG, SPEC, Gen> ResultComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn new(ty: &Type, context: Context, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(AttrsModel::from(&attrs), context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            ty: ty.clone(),
            // attrs: attrs.clone(),
            phantom_data: PhantomData
        }
    }
}
impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for ResultComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}

impl<LANG, SPEC, Gen> NameContext<Context> for ResultComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for ResultComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}



impl<'a> Composer<'a> for ResultComposer<RustFermentate, Vec<Attribute>, Option<Generics>>  {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let compose = |arg_name: &Name, ty: &Type| match TypeKind::from(ty) {
            TypeKind::Primitive(arg_ty) => {
                GenericArgPresentation::new(
                    arg_ty,
                    DESTROY_OPT_PRIMITIVE(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()),
                    Expression::MapExpression(Expression::O.into(), FROM_OPAQUE(DictionaryName::O.to_token_stream()).into()),
                    TO_OPAQUE(DictionaryName::O.to_token_stream()))
            }
            TypeKind::Complex(arg_ty) => {
                let arg_composer = match arg_ty.maybe_special_type(source) {
                    Some(SpecialType::Opaque(..)) =>
                        GenericArgComposer::new(FROM_OPAQUE, TO_OPAQUE, DESTROY_OPT_COMPLEX),
                    _ =>
                        GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX),
                };
                GenericArgPresentation::new(
                    arg_ty.special_or_to_ffi_full_path_type(source),
                    arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()),
                    Expression::MapExpression(Expression::O.into(), arg_composer.from(DictionaryName::O.to_token_stream()).into()),
                    arg_composer.to(DictionaryName::O.to_token_stream()))
            }
            TypeKind::Generic(generic_arg_ty) => {
                let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                    match generic_arg_ty.ty() {
                        None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                        Some(ty) => match TypeKind::from(ty) {
                            TypeKind::Primitive(_) => (PRIMITIVE_OPT_ARG_COMPOSER, ty.special_or_to_ffi_full_path_type(source)),
                            TypeKind::Generic(nested_nested) => (COMPLEX_OPT_ARG_COMPOSER, nested_nested.special_or_to_ffi_full_path_type(source)),
                            _ => (COMPLEX_OPT_ARG_COMPOSER, ty.special_or_to_ffi_full_path_type(source)),
                        }
                    }
                } else { (GenericArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_OPT_COMPLEX), generic_arg_ty.special_or_to_ffi_full_path_type(source)) };
                GenericArgPresentation::new(
                    arg_ty,
                    arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()),
                    Expression::MapExpression(Expression::O.into(), arg_composer.from(DictionaryName::O.to_token_stream()).into()),
                    arg_composer.to(DictionaryName::O.to_token_stream()))
            }
        };

        let nested_types = self.ty.nested_types();
        let ffi_type = self.compose_ffi_name();
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
                let GenericArgPresentation { from_conversion,to_conversion, destructor, ty } = compose(name, nested_types[index]);
                from_conversions.push(quote!(ffi_ref.#name));
                from_conversions.push(from_conversion.present(source));
                to_conversions.push(DictionaryExpr::Mapper(DictionaryName::O.to_token_stream(), to_conversion.present(source)));
                destroy_conversions.push(destructor.present(source));
                field_composers.push(FieldComposer::named(name.clone(), FieldTypeKind::Type(ty.joined_mut())));
            });
        let attrs = self.base.compose_attributes();
        let types = (ffi_type.clone(), self.compose_target_name());
        Some(GenericComposerInfo::<RustFermentate, Vec<Attribute>, Option<Generics>>::default(
            self.ty.mangle_ident_default(),
            &attrs,
            field_composers,
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from_root(&attrs, &types, InterfacesMethodExpr::FoldToResult(from_conversions.to_token_stream()), &None),
                InterfacePresentation::conversion_to_boxed(&attrs, &types, BraceWrapped::<_, Void>::new(quote!(let (#field_names) = ferment_interfaces::to_result(obj, #to_conversions); Self { #field_names })), &None),
                InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
                InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
            ])
        ))
    }
}
