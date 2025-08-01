use quote::{quote, ToTokens};
use crate::ast::Depunctuated;
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, ConversionDropComposer, ConversionFromComposer, ConversionToComposer, GenericComposerInfo, GroupComposer, SourceComposable, VarComposer};
use crate::context::ScopeContext;
use crate::ext::{Accessory, GenericNestedArg, LifetimeProcessor, Mangle};
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, RustSpecification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, Name};

impl SourceComposable for GroupComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let nested_ty = self.ty.maybe_first_nested_type_ref()?;
        let arg_0_name = Name::dictionary_name(DictionaryName::Values);
        let count_name = Name::dictionary_name(DictionaryName::Count);
        let lifetimes = nested_ty.unique_lifetimes();
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
            ferment::from_group(ffi_ref.#count_name, ffi_ref.#arg_0_name, #from_conversion_value)
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
        Some(GenericComposerInfo::default(
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
