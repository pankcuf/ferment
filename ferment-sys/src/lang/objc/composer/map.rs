use quote::{quote, ToTokens};
use syn::{parse_quote, Type};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens, Depunctuated, SemiPunctuated};
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{SourceComposable, GenericComposerInfo, MapComposer, AspectPresentable, AttrComposable, TypeAspect};
use crate::context::ScopeContext;
use crate::conversion::{GenericArgComposer, GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::composer::var::objc_primitive;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::lang::objc::presentable::ArgPresentation;
use crate::presentable::{Expression, ArgKind, SeqKind, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIVariable, Name};


fn compose_arg<SPEC>(arg_name: &Name<ObjCFermentate, SPEC>, ty: &Type, source: &ScopeContext) -> GenericArgPresentation<ObjCFermentate, SPEC> where SPEC: ObjCSpecification {
    println!("MapComposer::compose_arg: {} --- {}", arg_name.to_token_stream(), ty.to_token_stream());
    let arg_context = |arg_name: &Name<ObjCFermentate, SPEC>| quote!(obj.#arg_name().cloned());
    let arg_args = |arg_name: &Name<ObjCFermentate, SPEC>| CommaPunctuated::from_iter([
        DictionaryExpr::SelfProp(arg_name.to_token_stream()),
        DictionaryExpr::SelfProp(DictionaryName::Count.to_token_stream())]);
    match TypeKind::from(ty) {
        TypeKind::Primitive(arg_ty) =>
            GenericArgPresentation::<ObjCFermentate, SPEC>::new(
                SPEC::Var::direct(objc_primitive(&arg_ty).to_token_stream()),
                // Expression::CastConversionExprTokens(FFIAspect::Destroy, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),

                Expression::destroy_primitive_group_tokens(arg_args(arg_name)),
                Expression::map_o_expr(Expression::DictionaryName(DictionaryName::O)),
                Expression::ffi_to_primitive_group_tokens(arg_context(arg_name))),
        TypeKind::Complex(arg_ty) => {
            let arg_composer = GenericArgComposer::<ObjCFermentate, SPEC>::new(
                Some(Expression::from_complex_tokens),
                Some(Expression::ffi_to_complex_group_tokens),
                Some(Expression::destroy_complex_group_tokens));
            GenericArgPresentation::<ObjCFermentate, SPEC>::new(
                FFIVariable::direct(FFIVarResolve::<ObjCFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(&arg_ty, source).to_token_stream()),
                arg_composer.destroy(arg_args(arg_name).to_token_stream()),
                Expression::map_o_expr(arg_composer.from(DictionaryName::O.to_token_stream())),
                arg_composer.to(arg_context(arg_name)))
        },
        TypeKind::Generic(generic_arg_ty) => {
            let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                match generic_arg_ty.ty() {
                    None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                    Some(ty) => (match TypeKind::from(ty) {
                        TypeKind::Primitive(_) => GenericArgComposer::<ObjCFermentate, SPEC>::new(
                            Some(Expression::from_primitive_opt_tokens),
                            Some(Expression::ffi_to_primitive_opt_group_tokens),
                            Some(Expression::destroy_complex_group_tokens)),
                        _ => GenericArgComposer::<ObjCFermentate, SPEC>::new(Some(Expression::from_complex_opt_tokens), Some(Expression::ffi_to_complex_opt_group_tokens), Some(Expression::destroy_complex_group_tokens)),
                    }, FFIVarResolve::<ObjCFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(ty, source).to_token_stream())
                }
            } else { (
                GenericArgComposer::<ObjCFermentate, SPEC>::new(
                    Some(Expression::from_complex_tokens),
                    Some(Expression::ffi_to_complex_group_tokens),
                    Some(Expression::destroy_complex_group_tokens)),
                FFIVarResolve::<ObjCFermentate, SPEC>::special_or_to_ffi_full_path_variable_type(&generic_arg_ty, source).to_token_stream())
            };
            GenericArgPresentation::<ObjCFermentate, SPEC>::new(
                FFIVariable::direct(arg_ty),
                arg_composer.destroy(arg_args(arg_name).to_token_stream()),
                Expression::map_o_expr(arg_composer.from(DictionaryName::O.to_token_stream())),
                arg_composer.to(arg_context(arg_name)))
        },
    }
}

impl<SPEC> SourceComposable for MapComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let count = DictionaryName::Count;
        let keys = DictionaryName::Keys;
        let values = DictionaryName::Values;
        let count_name = Name::Dictionary(count.clone());
        let arg_0_name = Name::Dictionary(keys.clone());
        let arg_1_name = Name::Dictionary(values.clone());
        let objc_name = quote!(NSDictionary);
        // let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let attrs = self.compose_attributes();
        let c_name = ffi_type.to_token_stream();

        let nested_types = self.ty.nested_types();
        let arg_0_presentation = compose_arg(&arg_0_name, nested_types[0], source);
        let arg_1_presentation = compose_arg(&arg_1_name, nested_types[1], source);

        let arg_0_var: SPEC::Var = <FFIVariable<TokenStream2, ObjCFermentate, SPEC> as Accessory>::joined_mut(&arg_0_presentation.ty);
        let arg_1_var: SPEC::Var = <FFIVariable<TokenStream2, ObjCFermentate, SPEC> as Accessory>::joined_mut(&arg_1_presentation.ty);

        let field_composers = Depunctuated::from_iter([
            FieldComposer::<ObjCFermentate, SPEC>::named(count_name.clone(), FieldTypeKind::Type(parse_quote!(uintptr_t))),
            FieldComposer::<ObjCFermentate, SPEC>::named(arg_0_name.clone(), FieldTypeKind::Var(arg_0_var.clone())),
            FieldComposer::<ObjCFermentate, SPEC>::named(arg_1_name.clone(), FieldTypeKind::Var(arg_1_var.clone())),
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
        let to_0_values = arg_0_presentation.to_conversion.present(source);
        let to_1_values = arg_1_presentation.to_conversion.present(source);

        let interfaces = Depunctuated::from_iter([
            InterfaceImplementation::ConversionsImplementation {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
                from_conversions_statements: SemiPunctuated::from_iter([
                    ArgPresentation::Initializer {
                        field_name: count_name.to_token_stream(),
                        field_initializer: quote!(ffi_ref->#count_name),
                    },
                    ArgPresentation::Initializer {
                        field_name: arg_0_name.to_token_stream(),
                        field_initializer: arg_0_presentation.from_conversion.present(source),
                    },
                    ArgPresentation::Initializer {
                        field_name: arg_1_name.to_token_stream(),
                        field_initializer: arg_1_presentation.from_conversion.present(source),
                    }
                ]).to_token_stream(),
                to_conversions_statements: quote! {
                    ffi_ref->#count_name = [obj #count_name];
                    NSUInteger i = 0;
                    for (#arg_0_var *key in obj) {
                        ffi_ref->#arg_0_name[i] = key.unsignedIntValue;
                        ffi_ref->#arg_1_name[i] = obj[key].unsignedIntValue;
                        i++;
                    }
                },
                destroy_body: SeqKind::StructDropBody(
                    ((self.ffi_type_aspect(), SPEC::Gen::default()), SemiPunctuated::from_iter([
                        ArgKind::<ObjCFermentate, SPEC>::AttrExpression(arg_0_presentation.destructor, attrs.clone()),
                        ArgKind::<ObjCFermentate, SPEC>::AttrExpression(arg_1_presentation.destructor, attrs.clone())
                    ])))
                    .present(source),
            },
            InterfaceImplementation::BindingsImplementation {
                objc_name: objc_name.clone(),
                c_name,
                to_conversions:
                CommaPunctuated::from_iter([
                    quote!(obj.#count_name),
                    quote!(#to_0_values),
                    quote!(#to_1_values),
                ]),
                // SemiPunctuated::from_iter([
                //     quote!(uintptr_t #count_name = obj.#count_name),
                //     quote!(#arg_0_var #arg_0_name = #to_0_values),
                //     quote!(#arg_1_var #arg_1_name = #to_1_values),
                // ]),
                property_names: CommaPunctuatedTokens::from_iter([
                    count_name.to_token_stream(),
                    arg_0_name.to_token_stream(),
                    arg_1_name.to_token_stream()
                ]),
            }
        ]);
        println!("OBJC MAP => \n{}", format_interface_implementations(&interfaces));
        Some(GenericComposerInfo::<ObjCFermentate, SPEC>::default(
            objc_name,
            &attrs,
            field_composers,
            interfaces
        ))

    }
}