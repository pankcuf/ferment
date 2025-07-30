use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, Type};
use ferment_macro::ComposerBase;
use crate::ast::Depunctuated;
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink, VarComposer, ConversionFromComposer, ConversionToComposer, ConversionDropComposer};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{Accessory, GenericNestedArg, LifetimeProcessor, Mangle};
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, InterfacePresentation, Name};

#[derive(ComposerBase)]
pub struct ArrayComposer<SPEC>
where SPEC: Specification + 'static {
    pub ty: Type,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> ArrayComposer<SPEC>
where SPEC: Specification {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            ty: ty.clone(),
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
        }

    }
}

impl SourceComposable for ArrayComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let nested_ty = self.ty.maybe_first_nested_type_ref()?;
        let lifetimes = nested_ty.unique_lifetimes();
        let arg_0_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Values);
        let count_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Count);
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let map_var_name = Name::dictionary_name(DictionaryName::O);
        let var_value = VarComposer::<RustSpecification>::value(nested_ty).compose(source);
        let from_conversion_expr_value = ConversionFromComposer::<RustSpecification>::value_expr(map_var_name.clone(), nested_ty, Expression::dict_expr(DictionaryExpr::Deref(map_var_name.to_token_stream()))).compose(source);
        let to_conversion_expr_value = ConversionToComposer::<RustSpecification>::value(map_var_name.clone(), nested_ty).compose(source);
        let destroy_conversion_expr_value = ConversionDropComposer::<RustSpecification>::value(map_var_name.clone(), nested_ty).compose(source).unwrap_or_else(|| Expression::black_hole(map_var_name.clone()));
        let from_conversion_value = Expression::map_o_expr(from_conversion_expr_value).present(source);
        let to_conversion_value = Expression::map_o_expr(to_conversion_expr_value).present(source);
        let destroy_conversion_value = Expression::map_o_expr(destroy_conversion_expr_value).present(source);
        let from_body = quote! {
            let ffi_ref = &*ffi;
            let vec: Vec<#nested_ty> = ferment::from_group(ffi_ref.#count_name, ffi_ref.#arg_0_name, #from_conversion_value);
            vec.try_into().unwrap()
        };
        let to_body = quote! {
            let #count_name = obj.len();
            let #arg_0_name = ferment::to_group(obj.into_iter(), #to_conversion_value);
            ferment::boxed(Self { #count_name, #arg_0_name })
        };
        let drop_body = quote! {
            unsafe {
                ferment::unbox_group(self.#arg_0_name, self.#count_name, #destroy_conversion_value);
            }
        };
        Some(GenericComposerInfo::<RustSpecification>::default(
            Aspect::raw_struct_ident(self.ty.mangle_ident_default()),
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::<RustSpecification>::named(count_name, FieldTypeKind::type_count()),
                FieldComposer::<RustSpecification>::named(arg_0_name, FieldTypeKind::Var(var_value.joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &lifetimes),
                InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &lifetimes),
                InterfacePresentation::drop(&attrs, ffi_type, drop_body)
            ])
        ))
    }
}




