use quote::quote;
use crate::ast::Depunctuated;
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, SourceComposable, GenericComposerInfo, ConversionFromComposer, ConversionToComposer, ConversionDropComposer, VarComposer, MapComposer};
use crate::context::ScopeContext;
use crate::ext::{Accessory, GenericNestedArg, LifetimeProcessor, Mangle};
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, RustSpecification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryName, InterfacePresentation, Name};

impl SourceComposable for MapComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let count_name = Name::dictionary_name(DictionaryName::Count);
        let arg_0_name = Name::dictionary_name(DictionaryName::Keys);
        let arg_1_name = Name::dictionary_name(DictionaryName::Values);
        let map_var_name = Name::dictionary_name(DictionaryName::O);
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let nested_types = self.ty.nested_types();
        let lifetimes = Vec::from_iter(nested_types.iter().flat_map(|ty| ty.unique_lifetimes()));
        let key_type = nested_types[0];
        let value_type = nested_types[1];
        let var_key = VarComposer::<RustSpecification>::value(key_type).compose(source);
        let var_value = VarComposer::<RustSpecification>::value(value_type).compose(source);
        let from_conversion_expr_key = ConversionFromComposer::<RustSpecification>::value(map_var_name.clone(), key_type).compose(source);
        let from_conversion_expr_value = ConversionFromComposer::<RustSpecification>::value(map_var_name.clone(), value_type).compose(source);
        let to_conversion_expr_key = ConversionToComposer::<RustSpecification>::value(map_var_name.clone(), key_type).compose(source);
        let to_conversion_expr_value = ConversionToComposer::<RustSpecification>::value(map_var_name.clone(), value_type).compose(source);
        let destroy_conversion_expr_key = ConversionDropComposer::<RustSpecification>::value(map_var_name.clone(), key_type).compose(source).unwrap_or_else(|| Expression::black_hole(map_var_name.clone()));
        let destroy_conversion_expr_value = ConversionDropComposer::<RustSpecification>::value(map_var_name.clone(), value_type).compose(source).unwrap_or_else(|| Expression::black_hole(map_var_name.clone()));
        let from_conversion_key = Expression::map_o_expr(from_conversion_expr_key).present(source);
        let from_conversion_value = Expression::map_o_expr(from_conversion_expr_value).present(source);
        let to_conversion_key = Expression::map_o_expr(to_conversion_expr_key).present(source);
        let to_conversion_value = Expression::map_o_expr(to_conversion_expr_value).present(source);
        let destroy_conversion_key = Expression::map_o_expr(destroy_conversion_expr_key).present(source);
        let destroy_conversion_value = Expression::map_o_expr(destroy_conversion_expr_value).present(source);
        let from_body = quote! {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(ffi_ref.#count_name, ffi_ref.#arg_0_name, ffi_ref.#arg_1_name, #from_conversion_key, #from_conversion_value)
        };
        let to_body = quote! {
            let (#count_name, #arg_0_name, #arg_1_name) = ferment::to_map(obj, #to_conversion_key, #to_conversion_value);
            ferment::boxed(Self { #count_name, #arg_0_name, #arg_1_name })
        };
        let drop_body = quote! {
            unsafe {
                ferment::unbox_group(self.#arg_0_name, self.#count_name, #destroy_conversion_key);
                ferment::unbox_group(self.#arg_1_name, self.#count_name, #destroy_conversion_value);
            }
        };
        let attrs = self.compose_attributes();
        Some(GenericComposerInfo::<RustSpecification>::default(
            Aspect::raw_struct_ident(self.ty.mangle_ident_default()),
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::<RustSpecification>::named(count_name, FieldTypeKind::type_count()),
                FieldComposer::<RustSpecification>::named(arg_0_name, FieldTypeKind::Var(var_key.joined_mut())),
                FieldComposer::<RustSpecification>::named(arg_1_name, FieldTypeKind::Var(var_value.joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &lifetimes),
                InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &lifetimes),
                InterfacePresentation::drop(&attrs, ffi_type, drop_body)
            ])
        ))
    }
}


