use quote::quote;
use syn::Lifetime;
use crate::ast::Depunctuated;
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, SourceComposable, GenericComposerInfo, ConversionFromComposer, VarComposer, ConversionToComposer, ConversionDropComposer, ResultComposer, NameKind};
use crate::context::ScopeContext;
use crate::ext::{Accessory, GenericNestedArg, LifetimeProcessor, Mangle, Optional, Primitive};
use crate::kind::FieldTypeKind;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, InterfacesMethod, Name};

impl SourceComposable for ResultComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut lifetimes = Vec::<Lifetime>::new();
        let nested_types = self.ty.nested_types();
        let ffi_type = self.present_ffi_aspect();
        let attrs = self.compose_attributes();
        let types = (ffi_type.clone(), self.present_target_aspect());

        let type_ok = nested_types[0];
        let type_error = nested_types[1];

        lifetimes.extend(type_ok.unique_lifetimes());
        lifetimes.extend(type_error.unique_lifetimes());

        let name_self_ok = Name::self_prop(DictionaryName::Ok);
        let name_self_error = Name::self_prop(DictionaryName::Error);

        let ok_is_primitive = type_ok.is_primitive();
        let error_is_primitive = type_error.is_primitive();
        let error_is_optional = type_error.is_optional();

        let map_var_name = Name::o();
        let var_ok = VarComposer::<RustSpecification>::value(type_ok).compose(source);
        let var_error = VarComposer::<RustSpecification>::value(type_error).compose(source);
        let from_conversion_expr_ok = ConversionFromComposer::<RustSpecification>::value_ref_maybe_expr(&map_var_name, type_ok, ok_is_primitive.then(|| Expression::DictionaryExpr(DictionaryExpr::deref(&map_var_name)))).compose(source);
        let from_conversion_expr_error = ConversionFromComposer::<RustSpecification>::value_ref_maybe_expr(&map_var_name, type_error, error_is_primitive.then(|| Expression::DictionaryExpr(DictionaryExpr::deref(&map_var_name)))).compose(source);
        let to_conversion_expr_ok = ConversionToComposer::<RustSpecification>::value_ref_maybe_expr(&map_var_name, type_ok, ok_is_primitive.then(|| Expression::boxed_tokens(&map_var_name))).compose(source);
        let to_conversion_expr_error = ConversionToComposer::<RustSpecification>::value_ref_maybe_expr(&map_var_name, type_error, error_is_primitive.then(|| Expression::boxed_tokens(&map_var_name))).compose(source);
        let destroy_conversion_expr_ok = ConversionDropComposer::<RustSpecification>::value_ref(&name_self_ok, type_ok).compose(source).unwrap_or_else(|| Expression::black_hole(name_self_ok.clone()));
        let destroy_conversion_expr_error = ConversionDropComposer::<RustSpecification>::value_ref(&name_self_error, type_error).compose(source).unwrap_or_else(|| Expression::black_hole(name_self_error.clone()));
        let from_conversion_ok = Expression::map_o_expr(from_conversion_expr_ok).present(source);
        let to_conversion_ok = Expression::map_o_expr(to_conversion_expr_ok).present(source);
        let destroy_conversion_ok = destroy_conversion_expr_ok.present(source);
        let from_conversion_error = Expression::map_o_expr(from_conversion_expr_error).present(source);
        let to_conversion_error = Expression::map_o_expr(to_conversion_expr_error).present(source);
        let destroy_conversion_error = destroy_conversion_expr_error.present(source);
        let fold_method = if error_is_optional { InterfacesMethod::FoldToResultPreferOk } else { InterfacesMethod::FoldToResult };
        let from_body = quote! {
            let ffi_ref = &*ffi;
            ferment::#fold_method(ffi_ref.ok, #from_conversion_ok, ffi_ref.error, #from_conversion_error)
        };
        let to_body = quote! {
            let (ok, error) = ferment::to_result(obj, #to_conversion_ok, #to_conversion_error);
            ferment::boxed(Self { ok, error })
        };
        let drop_body = quote! {
            #destroy_conversion_ok;
            #destroy_conversion_error;
        };

        let var_ok = if ok_is_primitive { var_ok.joined_mut() } else { var_ok };
        let var_error = if error_is_primitive { var_error.joined_mut() } else { var_error };

        let field_composers = Depunctuated::from_iter([
            FieldComposer::named_no_attrs(<RustSpecification as Specification>::Name::ok(), FieldTypeKind::Var(var_ok.clone())),
            FieldComposer::named_no_attrs(<RustSpecification as Specification>::Name::error(), FieldTypeKind::Var(var_error.clone()))
        ]);
        let aspect = Aspect::raw_struct_ident(self.ty.mangle_ident_default());
        let signature_context = (attrs.clone(), <RustSpecification as Specification>::Lt::default(), <RustSpecification as Specification>::Gen::default());
        let dtor_context = (aspect.clone(), signature_context.clone(), NameKind::Named);
        let ctor_context = (dtor_context.clone(), Vec::from_iter(field_composers.iter().map(ArgKind::named_ready_struct_ctor_pair)));
        let ok_context = (signature_context.clone(), ffi_type.clone(), var_ok);
        let error_context = (signature_context, ffi_type.clone(), var_error);

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
                BindingPresentableContext::<RustSpecification>::ctor(ctor_context),
                BindingPresentableContext::<RustSpecification>::dtor((dtor_context, Default::default())),
                BindingPresentableContext::<RustSpecification>::ctor_result_ok(ok_context),
                BindingPresentableContext::<RustSpecification>::ctor_result_error(error_context),
            ])
        ))
    }
}
