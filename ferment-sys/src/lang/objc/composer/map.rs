use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Type};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens, Depunctuated};
use crate::composable::FieldTypeKind;
use crate::composer::{AspectPresentable, AttrComposable, ConversionDropComposer, ConversionFromComposer, GenericComposerInfo, MapComposer, SourceComposable, TargetVarComposer, ConversionToComposer, VarComposer};
use crate::context::ScopeContext;
use crate::kind::{GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg};
use crate::lang::{FromDictionary, Specification};
use crate::lang::objc::ObjCSpecification;
use crate::lang::objc::composer::var::objc_primitive;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::lang::objc::presentable::TypeContext;
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryName, FFIVariable, Name};

fn compose_arg(
    arg_name: &Name<ObjCSpecification>,
    arg_item_name: TokenStream2,
    ty: &Type,
    source: &ScopeContext
) -> ((FFIVariable<ObjCSpecification, TokenStream2>, Expression<ObjCSpecification>, Expression<ObjCSpecification>, Expression<ObjCSpecification>), TokenStream2) {
    println!("MapComposer::compose_arg: {} --- {}", arg_name.to_token_stream(), ty.to_token_stream());
    let from_composer = ConversionFromComposer::<ObjCSpecification>::value_expr(arg_name.clone(), ty, Expression::Simple(quote!(ffi_ref->#arg_name[i])));
    let to_composer = ConversionToComposer::<ObjCSpecification>::value_expr(arg_name.clone(), ty, Expression::Simple(arg_item_name));
    let destroy_composer = ConversionDropComposer::<ObjCSpecification>::value_expr(arg_name.clone(), ty, Expression::Simple(quote!(ffi_ref->#arg_name[i])));
    let var_composer = VarComposer::<ObjCSpecification>::value(ty);
    let target_composer = TargetVarComposer::<ObjCSpecification>::value(ty);
    let from_expr = from_composer.compose(source);
    let to_expr = to_composer.compose(source);
    let destroy_expr = destroy_composer.compose(source);
    let var_expr = var_composer.compose(source);
    let target_expr = target_composer.compose(source);
    println!("MapComposer:: OBJC FROM EXPR: {}", from_expr.present(source));
    println!("MapComposer:: OBJC TO EXPR: {}", to_expr.present(source));
    println!("MapComposer:: OBJC DESTROY EXPR: {}", destroy_expr.as_ref().map(|e| e.present(source)).unwrap_or_default());
    println!("MapComposer:: OBJC VAR EXPR: {}", var_expr.to_token_stream());
    println!("MapComposer:: OBJC TARGET EXPR: {}", target_expr.to_token_stream());
    // let arg_args = |arg_name: &Name<ObjCFermentate, SPEC>| CommaPunctuated::from_iter([
    //     DictionaryExpr::SelfProp(arg_name.to_token_stream()),
    //     DictionaryExpr::SelfProp(DictionaryName::Count.to_token_stream())]);

    match TypeKind::from(ty) {
        TypeKind::Primitive(arg_ty) =>
            ((
                <ObjCSpecification as Specification>::Var::direct(objc_primitive(&arg_ty)),
                destroy_expr.unwrap_or_default(),
                from_expr,
                to_expr,
            ), objc_primitive(ty)),
        TypeKind::Complex(arg_ty) => {
            ((
                FFIVariable::direct(FFIVarResolve::<ObjCSpecification>::special_or_to_ffi_full_path_variable_type(&arg_ty, source).to_token_stream()),
                destroy_expr.unwrap_or_default(),
                from_expr,
                to_expr,
            ), objc_primitive(ty))
        },
        TypeKind::Generic(generic_arg_ty) => {
            let arg_ty = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                match generic_arg_ty.ty() {
                    None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                    Some(ty) => FFIVarResolve::<ObjCSpecification>::special_or_to_ffi_full_path_variable_type(ty, source).to_token_stream(),
                }
            } else {
                // GenericArgComposer::<ObjCFermentate, SPEC>::new(
                //     Some(Expression::from_complex_tokens),
                //     Some(Expression::ffi_to_complex_group_tokens),
                //     Some(Expression::destroy_complex_group_tokens)),
                FFIVarResolve::<ObjCSpecification>::special_or_to_ffi_full_path_variable_type(&generic_arg_ty, source).to_token_stream()
            };
            ((
                FFIVariable::direct(arg_ty),
                destroy_expr.unwrap_or_default(),
                from_expr,
                to_expr,
            ), objc_primitive(ty))
        },
    }
}

impl SourceComposable for MapComposer<ObjCSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let count = DictionaryName::Count;
        let keys = DictionaryName::Keys;
        let values = DictionaryName::Values;
        let count_name = <ObjCSpecification as Specification>::Name::dictionary_name(count.clone());
        let arg_0_name = <ObjCSpecification as Specification>::Name::dictionary_name(keys.clone());
        let arg_1_name = <ObjCSpecification as Specification>::Name::dictionary_name(values.clone());
        let aspect = Aspect::RawTarget(TypeContext::Struct { ident: format_ident!("Dictionary"), prefix: "NS".to_string(), attrs: vec![] });

        let objc_name = aspect.present(source);
        let ffi_type = self.present_ffi_aspect();
        let attrs = self.compose_attributes();
        let c_name = ffi_type.to_token_stream();

        let nested_types = self.ty.nested_types();
        let arg_0_target_ty = nested_types[0];
        let arg_1_target_ty = nested_types[1];
        let ((arg_0_var, arg_0_destructor, arg_0_from_conversion, arg_0_to_conversion), c0_type) = compose_arg(&arg_0_name, quote!(key), arg_0_target_ty, source);
        let ((arg_1_var, arg_1_destructor, arg_1_from_conversion, arg_1_to_conversion), c1_type) = compose_arg(&arg_1_name, quote!(obj[key]), arg_1_target_ty, source);
        let arg_0_ty = &arg_0_var;
        let arg_1_ty = &arg_1_var;
        let arg_0_var: <ObjCSpecification as Specification>::Var = Accessory::joined_mut(arg_0_ty);
        let arg_1_var: <ObjCSpecification as Specification>::Var = Accessory::joined_mut(arg_1_ty);

        let field_composers = Depunctuated::from_iter([
            count_name.field_composer(FieldTypeKind::Type(parse_quote!(uintptr_t))),
            arg_0_name.field_composer(FieldTypeKind::Var(arg_0_var.clone())),
            arg_1_name.field_composer(FieldTypeKind::Var(arg_1_var.clone())),
        ]);
        // FFIMapConversion(TYPE, VarKey, FromKey, ToKey, DestroyKey, VarValue, FromValue, ToValue, DestroyValue)


        // @implementation NSDictionary (Conversions_std_collections_Map_keys_u32_values_u32)
        //     - (struct std_collections_Map_keys_u32_values_u32 *)ffi_to:(instancetype)obj {
        //     std_collections_Map_keys_u32_values_u32 *ffi_ref = malloc(sizeof(std_collections_Map_keys_u32_values_u32));
        //     ffi_ref->count = [obj count];
        //     NSUInteger i = 0;
        //     for (NSNumber *key in obj) {
        //     ffi_ref->keys[i] = key.unsignedIntValue;
        //     ffi_ref->values[i] = obj[key].unsignedIntValue;
        //     i++;
        //     }
        //     return values;
        // }
        //     + (struct std_collections_Map_keys_u32_values_u32 *)ffi_to_opt:(instancetype _Nullable)obj {
        //     return obj ? [self ffi_to:obj] : nil;
        // }
        //     - (instancetype)ffi_from:(struct std_collections_Map_keys_u32_values_u32 *)ffi_ref {
        //     NSMutableDictionary<NSNumber *, NSNumber *> *obj = [NSMutableDictionary dictionaryWithCapacity:ffi_ref->count];
        //     for (NSUInteger i = 0; i < ffi_ref->count; i++) {
        //     [obj setObject:@(ffi_ref->keys[i]) forKey:@(ffi_ref->values[i])];
        //     }
        //     return obj;
        // }
        //     + (instancetype _Nullable)ffi_from_opt:(struct std_collections_Map_keys_u32_values_u32 *)ffi_ref {
        //     return ffi_ref ? [self ffi_from:ffi_ref] : nil;
        // }
        //     + (void)ffi_destroy:(struct std_collections_Map_keys_u32_values_u32 *)ffi_ref {
        //     if (!ffi_ref) return;
        //     if (ffi_ref->keys) free(ffi_ref->keys);
        //     if (ffi_ref->values) free(ffi_ref->values);
        //     free(ffi_ref);
        // }
        // @end
        //
        // @implementation NSDictionary (Bindings_std_collections_Map_keys_u32_values_u32)
        //
        //     + (struct std_collections_Map_keys_u32_values_u32 *)ffi_ctor:(instancetype)obj {
        //     NSUInteger i = 0, count = [obj count];
        //     uint32_t *keys = malloc(count * sizeof(uint32_t));
        //     uint32_t *values = malloc(count * sizeof(uint32_t));
        //     for (NSNumber *key in obj) {
        //     keys[i] = key.unsignedIntValue;
        //     values[i] = obj[key].unsignedIntValue;
        //     i++;
        //     }
        //     return std_collections_Map_keys_u32_values_u32_ctor(count, keys, values);
        // }
        //
        //     + (void)ffi_dtor:(struct std_collections_Map_keys_u32_values_u32 *)ffi_ref {
        //     dash_spv_masternode_processor_crypto_byte_util_UInt768_destroy(ffi_ref);
        // }
        //
        // @end
        let to_0_values = arg_0_to_conversion.present(source);
        let to_1_values = arg_1_to_conversion.present(source);



        let interfaces = Depunctuated::from_iter([
            InterfaceImplementation::ConversionsImplementation {
                objc_name: objc_name.to_token_stream(),
                c_name: c_name.clone(),
                from_conversions_statements: {
                    let from_key = arg_0_from_conversion.present(source);
                    let from_value = arg_1_from_conversion.present(source);
                    quote! {
                        uintptr_t count = ffi_ref->count;
                        NSMutableDictionary *obj = [NSMutableDictionary dictionaryWithCapacity:count];
                        for (int i = 0; i < count; i++) {
                            [obj setObject:#from_key forKey:#from_value];
                        }
                        return obj;
                    }
                },
                to_conversions_statements: quote! {
                    NSUInteger count = [obj count];
                    struct #c_name *ffi_ref = malloc(sizeof(struct #c_name));

                    // ffi_ref->#count_name = [obj #count_name];
                    // NSUInteger i = 0;
                    // for (#arg_0_var *key in obj) {
                    //     ffi_ref->#arg_0_name[i] = key.unsignedIntValue;
                    //     ffi_ref->#arg_1_name[i] = obj[key].unsignedIntValue;
                    //     i++;
                    // }

                    // NSUInteger count = [obj count];
                    // struct std_collections_Map_keys_dash_spv_masternode_processor_common_block_Block_values_dash_spv_masternode_processor_models_operator_public_key_OperatorPublicKey *ffi_ref = malloc(sizeof(struct std_collections_Map_keys_dash_spv_masternode_processor_common_block_Block_values_dash_spv_masternode_processor_models_operator_public_key_OperatorPublicKey));
                    #c0_type *keys = malloc(count * sizeof(#c0_type));
                    #c1_type *values = malloc(count * sizeof(#c1_type));
                    for (id key in obj) {
                        keys[i] = #to_0_values;
                        values[i] = #to_1_values;
                    }
                    ffi_ref->count = count;
                    ffi_ref->keys = keys;
                    ffi_ref->values = values;
                    return ffi_ref;

                },
                destroy_body: {
                    let destroy_key = arg_0_destructor.present(source);
                    let destroy_value = arg_1_destructor.present(source);
                    quote! {
                        if (!ffi_ref) return;
                        if (ffi_ref->count > 0) {
                            for (int i = 0; i < ffi_ref->count; i++) {
                                #destroy_key
                                #destroy_value
                            }
                            if (ffi_ref->keys) free(ffi_ref->keys);
                            if (ffi_ref->values) free(ffi_ref->values);
                        }
                        free(ffi_ref);
                    }
                },
            },
            InterfaceImplementation::BindingsImplementation {
                objc_name: objc_name.to_token_stream(),
                c_name,
                to_conversions:
                CommaPunctuated::from_iter([
                    quote!(obj.#count_name),
                    quote!(#to_0_values),
                    quote!(#to_1_values),
                ]),
                property_names: CommaPunctuatedTokens::from_iter([
                    count_name.to_token_stream(),
                    arg_0_name.to_token_stream(),
                    arg_1_name.to_token_stream()
                ]),
            }
        ]);
        println!("OBJC MAP => \n{}", format_interface_implementations(&interfaces));
        Some(GenericComposerInfo::<ObjCSpecification>::default(
            aspect,
            &attrs,
            field_composers,
            interfaces
        ))

    }
}