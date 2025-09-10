use quote::quote;
use crate::ast::Depunctuated;
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, SourceComposable, GenericComposerInfo, VarComposer, ConversionFromComposer, ConversionToComposer, ConversionDropComposer, ArrayComposer, NameKind};
use crate::context::ScopeContext;
use crate::ext::{Accessory, GenericNestedArg, LifetimeProcessor, Mangle, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, InterfacePresentation, InterfacesMethodExpr, Name};
use crate::presentation::DictionaryName::Package;

impl SourceComposable for ArrayComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let nested_ty = self.ty.maybe_first_nested_type_ref()?;
        let lifetimes = nested_ty.unique_lifetimes();
        let arg_0_name = <RustSpecification as Specification>::Name::values();
        let count_name = <RustSpecification as Specification>::Name::count();
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let map_var_name = Name::o();
        let var_value_composer = VarComposer::<RustSpecification>::value(nested_ty);
        let var_value = var_value_composer.compose(source);
        let from_conversion_expr_value = ConversionFromComposer::<RustSpecification>::value_ref_expr(&map_var_name, nested_ty, Expression::dict_expr(DictionaryExpr::deref(&map_var_name))).compose(source);
        let to_conversion_expr_value = ConversionToComposer::<RustSpecification>::value_ref(&map_var_name, nested_ty).compose(source);
        let destroy_conversion_expr_value = ConversionDropComposer::<RustSpecification>::value_ref(&map_var_name, nested_ty).compose(source).unwrap_or_else(|| Expression::black_hole(&map_var_name));
        let from_conversion_value = Expression::map_o_expr(from_conversion_expr_value).present(source);
        let to_conversion_value = Expression::map_o_expr(to_conversion_expr_value).present(source);
        let destroy_conversion_value = Expression::map_o_expr(destroy_conversion_expr_value).present(source);
        let from_body = quote! {
            let ffi_ref = &*ffi;
            TryFrom::<Vec<#nested_ty>>::try_from(#Package::from_group(ffi_ref.#count_name, ffi_ref.#arg_0_name, #from_conversion_value)).unwrap()
        };
        let arg_0_conversion = InterfacesMethodExpr::ToGroup(quote!(obj.into_iter(), #to_conversion_value));
        let to_body = InterfacesMethodExpr::Boxed(quote!(Self { #count_name: obj.len(), #arg_0_name: #arg_0_conversion }));
        let drop_body = InterfacesMethodExpr::UnboxGroup(quote!(self.#arg_0_name, self.#count_name, #destroy_conversion_value));
        let arr_var = var_value.joined_mut();
        let field_composers = Depunctuated::from_iter([
            FieldComposer::<RustSpecification>::named_no_attrs(count_name, FieldTypeKind::type_count()),
            FieldComposer::<RustSpecification>::named_no_attrs(arg_0_name, FieldTypeKind::Var(arr_var))
        ]);
        let aspect = Aspect::raw_struct_ident(self.ty.mangle_ident_default());
        let signature_context = (attrs.clone(), Default::default(), Default::default());
        let dtor_context = (aspect.clone(), signature_context.clone(), NameKind::Named);
        let ctor_context = (dtor_context.clone(), Vec::from_iter(field_composers.iter().map(ArgKind::named_ready_struct_ctor_pair)));
        let get_at_index_context = (aspect.clone(), signature_context, ffi_type.clone(), var_value.to_type());
        Some(GenericComposerInfo::<RustSpecification>::default_with_bindings(
            aspect,
            &attrs,
            field_composers,
            Depunctuated::from_iter([
                InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &lifetimes),
                InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &lifetimes),
                InterfacePresentation::drop(&attrs, ffi_type, drop_body)
            ]),
            Depunctuated::from_iter([
                BindingPresentableContext::<RustSpecification>::ctor::<Vec<_>>(ctor_context),
                BindingPresentableContext::<RustSpecification>::dtor((dtor_context, Default::default())),
                BindingPresentableContext::<RustSpecification>::get_at_index(get_at_index_context.clone()),
                BindingPresentableContext::<RustSpecification>::set_at_index(get_at_index_context)
            ])
        ))
    }
}




