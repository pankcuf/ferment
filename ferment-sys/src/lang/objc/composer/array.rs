use quote::{quote, ToTokens};
use syn::parse_quote;
use crate::ast::Depunctuated;
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{ArrayComposer, AspectPresentable, AttrComposable, FFIAspect, GenericComposerInfo, SourceComposable, TypeAspect, VarComposer};
use crate::context::ScopeContext;
use crate::conversion::{GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg};
use crate::lang::{FromDictionary, Specification};
use crate::lang::objc::ObjCSpecification;
use crate::lang::objc::composer::var::objc_primitive;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::presentable::{ArgKind, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::DictionaryName;

impl SourceComposable for ArrayComposer<ObjCSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let nested_ty = self.ty.maybe_first_nested_type_ref()?;
        let nested_type_kind = TypeKind::from(nested_ty);
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let arg_0_name = <ObjCSpecification as Specification>::Name::dictionary_name(DictionaryName::Values);
        let count_name = <ObjCSpecification as Specification>::Name::dictionary_name(DictionaryName::Count);
        let from_args = quote! {
            ffi_ref->#arg_0_name #count_name: ffi_ref->#count_name
        };
        let arg_presentation = match &nested_type_kind {
            TypeKind::Primitive(arg_0_target_path) => {
                let kind = ConversionExpressionKind::PrimitiveGroup;
                GenericArgPresentation::<ObjCSpecification>::new(
                    <ObjCSpecification as Specification>::Var::direct(objc_primitive(arg_0_target_path).to_token_stream()),
                    Expression::CastConversionExprTokens(FFIAspect::Drop, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::From, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::To, kind, quote!(obj.values), ffi_type.clone(), target_type.clone())
                )
            }
            TypeKind::Complex(arg_0_target_ty) => {
                let kind = ConversionExpressionKind::ComplexGroup;
                GenericArgPresentation::<ObjCSpecification>::new(
                    <ObjCSpecification as Specification>::Var::mut_ptr(FFIVarResolve::<ObjCSpecification>::special_or_to_ffi_full_path_type(arg_0_target_ty, source).to_token_stream()),
                    Expression::CastConversionExprTokens(FFIAspect::Drop, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::From, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::To, kind, quote!(obj.values), ffi_type.clone(), target_type.clone())
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
                                }, VarComposer::<ObjCSpecification>::value(ty).compose(source))
                            }
                        }
                    } else {
                        (ConversionExpressionKind::ComplexGroup, VarComposer::<ObjCSpecification>::value(arg_0_generic_path_conversion.ty()?).compose(source))
                    }
                };
                GenericArgPresentation::<ObjCSpecification>::new(
                    arg_ty,
                    Expression::CastConversionExprTokens(FFIAspect::Drop, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::From, kind, from_args.to_token_stream(), ffi_type.clone(), target_type.clone()),
                    Expression::CastConversionExprTokens(FFIAspect::To, kind, quote!(obj.values), ffi_type.clone(), target_type.clone())
                )
            }
        };
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let c_name = ffi_type.to_token_stream();

        // let from_conversions_statements = ;

        let arg_var: <ObjCSpecification as Specification>::Var = arg_presentation.ty.joined_mut();
        let field_composers = Depunctuated::from_iter([
            FieldComposer::<ObjCSpecification>::named(count_name.clone(), FieldTypeKind::Type(parse_quote!(uintptr_t))),
            FieldComposer::<ObjCSpecification>::named(arg_0_name.clone(), FieldTypeKind::Var(arg_var.clone())),
        ]);
        let to_values = arg_presentation.to_conversion.present(source);
        let destroy_value = ArgKind::<ObjCSpecification>::AttrExpression(arg_presentation.destructor, attrs.clone()).present(source);
        let from_value = arg_presentation.from_conversion.present(source);
        let interfaces = Depunctuated::from_iter([
            // InterfaceImplementation::Default {
            //     objc_name: objc_name.clone(),
            //     properties: SemiPunctuated::from_iter(field_composers.iter()
            //         .map(Property::nonatomic_readwrite))
            // },
            // InterfaceImplementation::ConversionsDeclaration {
            //     objc_name: objc_name.clone(),
            //     c_name: c_name.clone(),
            // },
            // InterfaceImplementation::BindingsDeclaration {
            //     objc_name: objc_name.clone(),
            //     c_name: c_name.clone(),
            // },
            // InterfaceImplementation::ConversionsImplementation {
            //     objc_name: objc_name.clone(),
            //     c_name: c_name.clone(),
            //     from_conversions_statements: SemiPunctuated::from_iter([
            //         Property::Initializer {
            //             field_name: count_name.to_token_stream(),
            //             field_initializer: quote!(ffi_ref->#count_name),
            //         },
            //         Property::Initializer {
            //             field_name: arg_0_name.to_token_stream(),
            //             field_initializer: arg_presentation.from_conversion.present(source),
            //         }
            //     ]).to_token_stream(),
            //     to_conversions_statements: quote! {
            //         struct #arg_var *ffi_ref = malloc(sizeof(struct #arg_var));
            //         ffi_ref->#count_name = obj.#count_name;
            //         ffi_ref->#arg_0_name = #to_values;
            //         return ffi_ref;
            //     },
            //     destroy_body: SeqKind::StructDropBody(
            //         ((self.ffi_type_aspect(), SPEC::Gen::default()), SemiPunctuated::from_iter([
            //             ArgKind::<ObjCFermentate, SPEC>::AttrExpression(arg_presentation.destructor, attrs.clone())
            //         ])))
            //         .present(source),
            // },
            // InterfaceImplementation::BindingsImplementation {
            //     objc_name: objc_name.clone(),
            //     c_name,
            //     to_conversions: SemiPunctuated::from_iter([
            //         quote!(uintptr_t #count_name = obj.#count_name),
            //         quote!(#arg_var #arg_0_name = #to_values),
            //     ]),
            //     property_names: CommaPunctuatedTokens::from_iter([
            //         count_name.to_token_stream(),
            //         arg_0_name.to_token_stream()
            //     ]),
            // },
            InterfaceImplementation::MacroCall(quote! { FFIGroupConversion(#c_name, #arg_var, #from_value, #to_values, #destroy_value); })
        ]);
        println!("OBJC Array => \n{}", format_interface_implementations(&interfaces));

        Some(GenericComposerInfo::<ObjCSpecification>::default(
            self.target_type_aspect(),
            &attrs,
            field_composers,
            interfaces
        ))
    }
}