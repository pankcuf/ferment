use quote::{quote, ToTokens};
use syn::{parse_quote, TypeSlice};
use crate::ast::Depunctuated;
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, SourceComposable, GenericComposerInfo, VarComposer, ConversionFromComposer, ConversionToComposer, ConversionDropComposer, SliceComposer};
use crate::context::ScopeContext;
use crate::ext::{Accessory, Mangle, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, Name};

impl SourceComposable for SliceComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { ty, .. } = self;
        let ffi_name = ty.mangle_tokens_default();
        let type_slice: TypeSlice = parse_quote!(#ty);
        let arg_0_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Values);
        let count_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Count);
        let nested_ty = &type_slice.elem;
        let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let attrs = self.compose_attributes();
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

        let field_composers = Depunctuated::from_iter([
            FieldComposer::named_no_attrs(count_name, FieldTypeKind::type_count()),
            FieldComposer::named_no_attrs(arg_0_name, FieldTypeKind::Var(var_value.joined_mut()))
        ]);

        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &vec![]),
            InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &vec![]),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), drop_body)

        ]);
        let aspect = Aspect::raw_struct_ident(ty.mangle_ident_default());
        Some(GenericComposerInfo::<RustSpecification>::default(aspect, &attrs, field_composers, interfaces))
    }
}
