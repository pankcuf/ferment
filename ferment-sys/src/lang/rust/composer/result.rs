use quote::{quote, ToTokens};
use syn::Lifetime;
use crate::ast::Depunctuated;
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, SourceComposable, GenericComposerInfo, ConversionFromComposer, VarComposer, ConversionToComposer, ConversionDropComposer, ResultComposer};
use crate::context::ScopeContext;
use crate::ext::{Accessory, GenericNestedArg, LifetimeProcessor, Mangle, Primitive};
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, Name};

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

        let name_self_ok = Name::DictionaryExpr(DictionaryExpr::self_prop(DictionaryName::Ok));
        let name_self_error = Name::DictionaryExpr(DictionaryExpr::self_prop(DictionaryName::Error));

        let ok_is_primitive = type_ok.is_primitive();
        let error_is_primitive = type_error.is_primitive();

        let map_var_name = Name::dictionary_name(DictionaryName::O);
        let var_ok = VarComposer::<RustSpecification>::value(type_ok).compose(source);
        let var_error = VarComposer::<RustSpecification>::value(type_error).compose(source);
        let from_conversion_expr_ok = ConversionFromComposer::<RustSpecification>::value_maybe_expr(map_var_name.clone(), type_ok, ok_is_primitive.then(|| Expression::DictionaryExpr(DictionaryExpr::Deref(map_var_name.to_token_stream())))).compose(source);
        let from_conversion_expr_error = ConversionFromComposer::<RustSpecification>::value_maybe_expr(map_var_name.clone(), type_error, error_is_primitive.then(|| Expression::DictionaryExpr(DictionaryExpr::Deref(map_var_name.to_token_stream())))).compose(source);
        let to_conversion_expr_ok = ConversionToComposer::<RustSpecification>::value_maybe_expr(map_var_name.clone(), type_ok, ok_is_primitive.then(|| Expression::boxed_tokens(&map_var_name))).compose(source);
        let to_conversion_expr_error = ConversionToComposer::<RustSpecification>::value_maybe_expr(map_var_name.clone(), type_error, error_is_primitive.then(|| Expression::boxed_tokens(&map_var_name))).compose(source);
        let destroy_conversion_expr_ok = ConversionDropComposer::<RustSpecification>::value(name_self_ok.clone(), type_ok).compose(source).unwrap_or_else(|| Expression::black_hole(name_self_ok.clone()));
        let destroy_conversion_expr_error = ConversionDropComposer::<RustSpecification>::value(name_self_error.clone(), type_error).compose(source).unwrap_or_else(|| Expression::black_hole(name_self_error.clone()));
        let from_conversion_ok = Expression::map_o_expr(from_conversion_expr_ok).present(source);
        let to_conversion_ok = Expression::map_o_expr(to_conversion_expr_ok).present(source);
        let destroy_conversion_ok = destroy_conversion_expr_ok.present(source);
        let from_conversion_error = Expression::map_o_expr(from_conversion_expr_error).present(source);
        let to_conversion_error = Expression::map_o_expr(to_conversion_expr_error).present(source);
        let destroy_conversion_error = destroy_conversion_expr_error.present(source);

        let from_body = quote! {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, #from_conversion_ok, ffi_ref.error, #from_conversion_error)
        };
        let to_body = quote! {
            let (ok, error) = ferment::to_result(obj, #to_conversion_ok, #to_conversion_error);
            ferment::boxed(Self { ok, error })

        };
        let drop_body = quote! {
            #destroy_conversion_ok;
            #destroy_conversion_error;
        };

        let var_ok = ok_is_primitive.then(|| var_ok.joined_mut()).unwrap_or(var_ok);
        let var_error = error_is_primitive.then(|| var_error.joined_mut()).unwrap_or(var_error);

        let field_composers = Depunctuated::from_iter([
            FieldComposer::named_no_attrs(<RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Ok), FieldTypeKind::Var(var_ok)),
            FieldComposer::named_no_attrs(<RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Error), FieldTypeKind::Var(var_error))
        ]);

        Some(GenericComposerInfo::<RustSpecification>::default(
            Aspect::raw_struct_ident(self.ty.mangle_ident_default()),
            &attrs,
            field_composers,
            Depunctuated::from_iter([
                InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &lifetimes),
                InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &lifetimes),
                InterfacePresentation::drop(&attrs, ffi_type, drop_body)
            ])
        ))
    }
}
