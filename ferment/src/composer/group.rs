use std::marker::PhantomData;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics, parse_quote, Type};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, RustFieldComposer};
use crate::composer::{Composer, GenericComposerInfo, NameComposable, ComposerLink, BasicComposer, constants, BasicComposerOwner, AttrComposable, NameContext, SourceAccessible};
use crate::context::ScopeContext;
use crate::conversion::{DESTROY_COMPLEX_GROUP, DESTROY_PRIMITIVE_GROUP, FROM_COMPLEX_GROUP, FROM_OPT_COMPLEX_GROUP, FROM_OPT_PRIMITIVE_GROUP, FROM_PRIMITIVE_GROUP, GenericArgComposer, GenericArgPresentation, GenericTypeKind, RustExpressionComposer, TO_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, TO_OPT_PRIMITIVE_GROUP, TO_PRIMITIVE_GROUP, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg, Mangle};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIVecConversionMethodExpr, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};



pub struct GroupComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub ty: Type,
    pub group_conversion_ty: Type,
    pub nested_type_kind: TypeKind,
    pub from_conversion_presentation: TokenStream2,
    pub to_conversion_presentation: TokenStream2,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    _phantom_data: PhantomData<LANG>,
}

impl<LANG, SPEC, Gen> GroupComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn new<F: ToTokens, T: ToTokens>(
        ty: &Type,
        context: Context,
        attrs: Vec<Attribute>,
        group_conversion_ty: Type,
        nested_type_kind: TypeKind,
        from_conversion_presentation: F,
        to_conversion_presentation: T,
        scope_context: &ComposerLink<ScopeContext>
    ) -> Self {
        Self {
            ty: ty.clone(),
            // attrs: SPEC::from_attrs(),
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(AttrsModel::from(&attrs), context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            _phantom_data: Default::default(),
            group_conversion_ty,
            nested_type_kind,
            from_conversion_presentation: from_conversion_presentation.to_token_stream(),
            to_conversion_presentation: to_conversion_presentation.to_token_stream()
        }
    }
    pub fn default(ty: &Type, context: Context, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        let nested_ty = ty.first_nested_type().unwrap();
        Self::new(
            ty,
            context,
            attrs,
            ty.clone(),
            TypeKind::from(nested_ty),
            FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()),
            FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()),
            scope_context
        )
    }
    pub fn array(ty: &Type, context: Context, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        let nested_ty = ty.first_nested_type().unwrap();
        Self::new(
            ty,
            context,
            attrs,
            parse_quote!(Vec<#nested_ty>),
            TypeKind::from(nested_ty),
            DictionaryExpr::TryIntoUnwrap(FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream()),
            FFIVecConversionMethodExpr::Encode(DictionaryExpr::ObjToVec.to_token_stream()),
            scope_context
        )
    }
}

impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for GroupComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}

impl<LANG, SPEC, Gen> NameContext<Context> for GroupComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for GroupComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}


impl<'a> Composer<'a> for GroupComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let ffi_name = self.ty.mangle_ident_default();
        let arg_0_name = Name::Dictionary(DictionaryName::Values);
        let count_name = Name::Dictionary(DictionaryName::Count);
        let from_args = CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())
        ]);
        let arg_0_from = |composer: RustExpressionComposer|
            composer(from_args.to_token_stream());

        let arg_0_to = |composer: RustExpressionComposer|
            Expression::InterfacesExpr(
                InterfacesMethodExpr::Boxed(
                    DictionaryExpr::SelfDestructuring(
                        CommaPunctuated::from_iter([
                            RustFieldComposer::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                            RustFieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).present(source)))])
                            .to_token_stream())
                        .to_token_stream()));

        let arg_0_destroy = |composer: RustExpressionComposer|
            composer(from_args.to_token_stream());

        let arg_presentation = match &self.nested_type_kind {
            TypeKind::Primitive(arg_0_target_path) => {
                GenericArgPresentation::new(
                    arg_0_target_path.clone(),
                    arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                    arg_0_from(FROM_PRIMITIVE_GROUP),
                    arg_0_to(TO_PRIMITIVE_GROUP)
                )
            }
            TypeKind::Complex(arg_0_target_ty) => {
                GenericArgPresentation::new(
                    arg_0_target_ty.special_or_to_ffi_full_path_variable_type(source),
                    arg_0_destroy(DESTROY_COMPLEX_GROUP),
                    arg_0_from(FROM_COMPLEX_GROUP),
                    arg_0_to(TO_COMPLEX_GROUP)
                )
            }
            TypeKind::Generic(arg_0_generic_path_conversion) => {
                let (arg_0_composer, arg_ty) = {
                    if let GenericTypeKind::Optional(..) = arg_0_generic_path_conversion {
                        match arg_0_generic_path_conversion.ty() {
                            None => unimplemented!("Mixin inside generic: {}", arg_0_generic_path_conversion),
                            Some(ty) => match TypeKind::from(ty) {
                                TypeKind::Primitive(_) =>
                                    (GenericArgComposer::new(FROM_OPT_PRIMITIVE_GROUP, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP), ty.special_or_to_ffi_full_path_variable_type(source)),
                                TypeKind::Generic(nested_nested) => {
                                    (GenericArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), nested_nested.special_or_to_ffi_full_path_variable_type(source))
                                },
                                _ => (GenericArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), ty.special_or_to_ffi_full_path_variable_type(source) ),
                            }
                        }
                    } else {
                        (GenericArgComposer::new(FROM_COMPLEX_GROUP, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), arg_0_generic_path_conversion.special_or_to_ffi_full_path_variable_type(source))
                    }
                };
                GenericArgPresentation::new(
                    arg_ty,
                    arg_0_destroy(arg_0_composer.destroy_composer),
                    arg_0_from(arg_0_composer.from_composer),
                    arg_0_to(arg_0_composer.to_composer)
                )
            }
        };
        let attrs = self.base.compose_attributes();
        let ffi_type = self.compose_ffi_name();
        let types = (ffi_type.clone(), self.compose_target_name());
        let expr_destroy_iterator = [arg_presentation.destructor.present(source)];
        Some(GenericComposerInfo::<RustFermentate, Vec<Attribute>, Option<Generics>>::default(
            ffi_name,
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
                FieldComposer::named(arg_0_name, FieldTypeKind::Type(arg_presentation.ty.joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from(&attrs, &types, self.from_conversion_presentation.clone(), &None),
                InterfacePresentation::conversion_to(&attrs, &types, self.to_conversion_presentation.clone(), &None),
                InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
                InterfacePresentation::vec(&attrs, &(ffi_type.clone(), self.group_conversion_ty.clone()), arg_presentation.from_conversion.present(source), arg_presentation.to_conversion.present(source)),
                InterfacePresentation::drop(&attrs, ffi_type, SemiPunctuated::from_iter(expr_destroy_iterator))
            ])
        ))
    }
}
