use quote::quote;
use syn::{parse_quote, TypeSlice};
use crate::ast::Depunctuated;
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, SourceComposable, GenericComposerInfo, VarComposer, ConversionFromComposer, ConversionToComposer, ConversionDropComposer, SliceComposer, NameKind};
use crate::context::ScopeContext;
use crate::ext::{Accessory, Mangle, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, InterfacePresentation, InterfacesMethodExpr, Name};

impl SourceComposable for SliceComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { ty, .. } = self;
        let ffi_name = ty.mangle_tokens_default();
        let type_slice: TypeSlice = parse_quote!(#ty);
        let arg_0_name = <RustSpecification as Specification>::Name::values();
        let count_name = <RustSpecification as Specification>::Name::count();
        let nested_ty = &type_slice.elem;
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let attrs = self.compose_attributes();
        let map_var_name = Name::o();
        let var_value = VarComposer::<RustSpecification>::value(nested_ty).compose(source);
        let from_conversion_expr_value = ConversionFromComposer::<RustSpecification>::value_ref_expr(&map_var_name, nested_ty, Expression::dict_expr(DictionaryExpr::deref(&map_var_name))).compose(source);
        let to_conversion_expr_value = ConversionToComposer::<RustSpecification>::value_ref(&map_var_name, nested_ty).compose(source);
        let destroy_conversion_expr_value = ConversionDropComposer::<RustSpecification>::value_ref(&map_var_name, nested_ty).compose(source).unwrap_or_else(|| Expression::black_hole(&map_var_name));
        let from_conversion_value = Expression::map_o_expr(from_conversion_expr_value).present(source);
        let to_conversion_value = Expression::map_o_expr(to_conversion_expr_value).present(source);
        let destroy_conversion_value = Expression::map_o_expr(destroy_conversion_expr_value).present(source);
        let from_body = quote! {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.#count_name, ffi_ref.#arg_0_name, #from_conversion_value)
        };
        let to_body = InterfacesMethodExpr::Boxed(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment::to_group(obj.into_iter(), #to_conversion_value) }));
        let drop_body = quote!(ferment::unbox_group(self.#arg_0_name, self.#count_name, #destroy_conversion_value););

        let field_composers = Depunctuated::from_iter([
            FieldComposer::named_no_attrs(count_name, FieldTypeKind::type_count()),
            FieldComposer::named_no_attrs(arg_0_name, FieldTypeKind::Var(var_value.joined_mut()))
        ]);

        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &[]),
            InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &[]),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), drop_body)

        ]);
        let aspect = Aspect::raw_struct_ident(ty.mangle_ident_default());
        let signature_context = (attrs.clone(), <RustSpecification as Specification>::Lt::default(), <RustSpecification as Specification>::Gen::default());
        let dtor_context = (aspect.clone(), signature_context.clone(), NameKind::Named);
        let ctor_context = (dtor_context.clone(), Vec::from_iter(field_composers.iter().map(ArgKind::named_ready_struct_ctor_pair)));
        let get_at_index_context = (aspect.clone(), signature_context, ffi_type.clone(), var_value.to_type());
        Some(GenericComposerInfo::<RustSpecification>::default_with_bindings(
            aspect,
            &attrs,
            field_composers,
            interfaces,
            Depunctuated::from_iter([
                BindingPresentableContext::<RustSpecification>::ctor::<Vec<_>>(ctor_context),
                BindingPresentableContext::<RustSpecification>::dtor((dtor_context, Default::default())),
                BindingPresentableContext::<RustSpecification>::get_at_index(get_at_index_context.clone()),
                BindingPresentableContext::<RustSpecification>::set_at_index(get_at_index_context)
            ])
        ))
    }
}
