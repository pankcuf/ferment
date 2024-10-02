use quote::{quote, ToTokens};
use syn::parse_quote;
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens, Depunctuated, SemiPunctuated, SemiPunctuatedTokens};
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{SourceComposable, GenericComposerInfo, GroupComposer, AttrComposable, AspectPresentable, FFIAspect, VarComposer};
use crate::context::ScopeContext;
use crate::conversion::{GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, Mangle};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::lang::objc::presentation::Property;
use crate::presentable::{ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, Name};

impl<SPEC> SourceComposable for GroupComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCFermentate, SPEC>>;

    #[allow(unused_variables)]
    fn compose(&self, source: &Self::Source) -> Self::Output {
        let ffi_name = self.ty.mangle_ident_default();
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();

        let arg_0_name = Name::Dictionary(DictionaryName::Values);
        let count_name = Name::Dictionary(DictionaryName::Count);
        let from_args = quote! {
            ffi_ref->#arg_0_name #count_name: ffi_ref->#count_name
        };

        // let arg_0_to = |expr: Expression<ObjCFermentate, SPEC>|
        //     Expression::boxed_tokens(DictionaryExpr::SelfDestructuring(
        //         CommaPunctuated::from_iter([
        //             FieldComposer::<ObjCFermentate, SPEC>::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
        //             FieldComposer::<ObjCFermentate, SPEC>::named(arg_0_name.clone(), FieldTypeKind::Conversion(expr.present(source)))
        //         ])
        //             .to_token_stream()));
        let arg_presentation = match &self.nested_type_kind {
            TypeKind::Primitive(arg_0_target_path) => {
                let kind = ConversionExpressionKind::PrimitiveGroup;
                GenericArgPresentation::<ObjCFermentate, SPEC>::new(
                    SPEC::Var::direct(arg_0_target_path.to_token_stream()),
                    Expression::CastConversionExprTokens(FFIAspect::Destroy, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::From, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::To, kind, DictionaryExpr::ObjIntoIter.to_token_stream(), ffi_type.clone(), target_type.clone())
                )
            }
            TypeKind::Complex(arg_0_target_ty) => {
                let kind = ConversionExpressionKind::ComplexGroup;
                GenericArgPresentation::<ObjCFermentate, SPEC>::new(
                    SPEC::Var::mut_ptr(FFIVarResolve::<ObjCFermentate, SPEC>::special_or_to_ffi_full_path_type(arg_0_target_ty, source).to_token_stream()),
                    // FFIVarResolve::<ObjCFermentate, SPEC>::special_or_to_ffi_full_path_type(arg_0_target_ty, source),
                    Expression::CastConversionExprTokens(FFIAspect::Destroy, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::From, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::To, kind, DictionaryExpr::ObjIntoIter.to_token_stream(), ffi_type.clone(), target_type.clone())
                )
            }
            TypeKind::Generic(arg_0_generic_path_conversion) => {
                let (kind, arg_ty) = {
                    if let GenericTypeKind::Optional(..) = arg_0_generic_path_conversion {
                        match arg_0_generic_path_conversion.ty() {
                            None => unimplemented!("Mixin inside generic: {}", arg_0_generic_path_conversion),
                            Some(ty) => {
                                (match TypeKind::from(ty) {
                                    TypeKind::Primitive(_) =>
                                        ConversionExpressionKind::PrimitiveOptGroup,
                                    _ =>
                                        ConversionExpressionKind::ComplexOptGroup,
                                }, VarComposer::<ObjCFermentate, SPEC>::value(ty).compose(source))
                            }
                        }
                    } else {
                        (ConversionExpressionKind::ComplexGroup, VarComposer::<ObjCFermentate, SPEC>::value(arg_0_generic_path_conversion.ty().unwrap()).compose(source))
                    }
                };

                GenericArgPresentation::<ObjCFermentate, SPEC>::new(
                    arg_ty,
                    Expression::CastConversionExprTokens(FFIAspect::Destroy, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    // arg_0_composer.destroy(from_args.to_token_stream()),
                    Expression::CastConversionExprTokens(FFIAspect::From, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    // arg_0_composer.from(from_args.to_token_stream()),
                    Expression::CastConversionExprTokens(FFIAspect::To, kind, DictionaryExpr::ObjIntoIter.to_token_stream(), ffi_type.clone(), target_type.clone()

                        // arg_0_composer.to_composer.map(|c| c(DictionaryExpr::ObjIntoIter.to_token_stream())).unwrap_or(Expression::empty())
                    )
                )
            }
        };
        let attrs = self.compose_attributes();
        let expr_destroy_iterator = [
            arg_presentation.destructor.present(source)
        ];
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let objc_name = target_type.mangle_tokens_default();
        let c_name = ffi_type.to_token_stream();

        let properties_inits = SemiPunctuated::from_iter([
            Property::Initializer {
                field_name: count_name.to_token_stream(),
                field_initializer: quote!(ffi_ref->count),
            },
            Property::Initializer {
                field_name: arg_0_name.to_token_stream(),
                field_initializer: arg_presentation.from_conversion.present(source),
            }
        ]);

        let field_composers = Depunctuated::from_iter([
            FieldComposer::<ObjCFermentate, SPEC>::named(count_name.clone(), FieldTypeKind::Type(parse_quote!(uintptr_t))),
            FieldComposer::<ObjCFermentate, SPEC>::named(arg_0_name.clone(), FieldTypeKind::Var(arg_presentation.ty.joined_mut()))
        ]);
        let properties = SemiPunctuated::from_iter(field_composers.iter()
            .map(Property::nonatomic_readwrite));
        let to_values = arg_presentation.to_conversion.present(source);
        let to_conversions = CommaPunctuatedTokens::from_iter([
            quote!(obj.#count_name),
            to_values.to_token_stream()
        ]);
        let to_conversions_statements = SemiPunctuatedTokens::from_iter([
            quote!(ffi_ref->count = obj.#count_name),
            quote!(ffi_ref->#arg_0_name = #to_values)
        ]);
        // self_->o_0 = [DSArr_u8_96 ffi_to:obj.o_0];


        let interfaces = Depunctuated::from_iter([
            InterfaceImplementation::Default {
                objc_name: objc_name.clone(),
                properties
            },
            InterfaceImplementation::ConversionsDeclaration {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
            },
            InterfaceImplementation::BindingsDeclaration {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
            },
            InterfaceImplementation::ConversionsImplementation {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
                from_conversions_statements: properties_inits,
                to_conversions_statements: Default::default(),
                destroy_conversions_statements: Default::default(),
            },
            InterfaceImplementation::BindingsImplementation {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
                to_conversions,
            },
        ]);
        println!("OBJC GROUP: {}", format_interface_implementations(&interfaces));
        Some(GenericComposerInfo::<ObjCFermentate, SPEC>::default(
            objc_name,
            &attrs,
            field_composers,
            interfaces
            // Depunctuated::from_iter([
            //     InterfaceImplementation::default(objc_name.clone(), c_name.clone(), properties, properties_inits),
            //     InterfaceImplementation::c(objc_name.clone(), c_name.clone(), SemiPunctuated::new(), SemiPunctuated::new()),
            //     InterfaceImplementation::rust(objc_name.clone(), c_name.clone(), CommaPunctuatedTokens::new(), SemiPunctuated::new()),
            //     InterfaceImplementation::args(objc_name.clone(), c_name.clone(), Depunctuated::new(), Depunctuated::new()),
            // ])

        ))
        // None
    }
}