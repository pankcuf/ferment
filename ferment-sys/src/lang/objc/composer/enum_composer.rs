use quote::{format_ident, quote, ToTokens};
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composer::{AspectPresentable, AttrComposable, EnumComposer, FFIAspect, GenericsComposable, InterfaceComposable, ItemComposerWrapper, SourceAccessible, SourceFermentable, TypeAspect};
use crate::ext::{Mangle, ToPath};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::lang::objc::presentable::{ArgPresentation, TypeContext};
use crate::presentable::{PresentableArgument, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, Name};
fn to_snake_case(input: &str) -> String {
    let mut snake_case = String::new();
    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                snake_case.push('_');
            }
            snake_case.push(ch.to_ascii_lowercase());
        } else {
            snake_case.push(ch);
        }
    }
    snake_case
}

impl<SPEC> InterfaceComposable<SPEC::Interface> for EnumComposer<ObjCFermentate, SPEC>
where SPEC: ObjCSpecification,
      Self: SourceAccessible
      + TypeAspect<TypeContext>
      + AttrComposable<SPEC::Attr>
      + GenericsComposable<SPEC::Gen> {
    fn compose_interfaces(&self) -> Depunctuated<SPEC::Interface> {
        let source = self.source_ref();
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let objc_name = target_type.to_token_stream();
        let c_name = ffi_type.to_token_stream();
        let from_variant_composer = |composer: &ItemComposerWrapper<ObjCFermentate, SPEC>|
            PresentableArgument::AttrSequence(composer.compose_aspect(FFIAspect::From), composer.compose_attributes());
        let to_variant_composer = |composer: &ItemComposerWrapper<ObjCFermentate, SPEC> |
            PresentableArgument::AttrSequence(composer.compose_aspect(FFIAspect::To), composer.compose_attributes());
        let drop_variant_composer = |composer: &ItemComposerWrapper<ObjCFermentate, SPEC>|
            PresentableArgument::AttrSequence(composer.compose_aspect(FFIAspect::Drop), composer.compose_attributes());

        println!("OBJC:: ENUM FFI ASPECT TYPE: {}", ffi_type.to_token_stream());
        println!("OBJC:: ENUM TARGET ASPECT TYPE: {}", objc_name);

        let mut property_names = CommaPunctuated::new();
        let mut properties = SemiPunctuated::new();
        let tag_name = Name::<ObjCFermentate, SPEC>::EnumTag(ffi_type.mangle_ident_default());
        properties.push(ArgPresentation::NonatomicAssign {
            ty: quote!(enum #tag_name),
            name: DictionaryName::Tag.to_token_stream()
        });

        let mut from_conversions = Depunctuated::new();
        let mut to_conversions = Depunctuated::new();
        let mut destroy_conversions = Depunctuated::new();

        self.variant_composers.iter()
            .for_each(|variant_composer| {


                from_conversions.push(from_variant_composer(variant_composer));
                to_conversions.push(to_variant_composer(variant_composer));
                destroy_conversions.push(drop_variant_composer(variant_composer));
                // properties.push();
            });

        self.variant_presenters.iter()
            .for_each(|(c, ((aspect, generics), args))| {
                args.iter().for_each(|arg| {
                    let asp = aspect.present(&source);
                    let path = asp.to_path();
                    let last_ident = &path.segments.last().unwrap().ident;
                    let snake_case = to_snake_case(&last_ident.to_string());

                    let presentation = arg.present(&source);
                    // OBJC ENUM VAR ARG: example_simple_errors_context_ContextProviderError :: InvalidDataContract --> NSString *
                    // -> invalid_data_contract
                        println!("OBJC ENUM VAR ARG: {} --> {snake_case} -> {}", aspect.present(&source).to_token_stream(), presentation);

                    properties.push(ArgPresentation::NonatomicReadwrite {
                        ty: presentation.to_token_stream(),
                        name: format_ident!("{snake_case}").to_token_stream(),
                    });
                });
            });


        let from_body = DictionaryExpr::SwitchFields(quote!(ffi_ref), from_conversions.present(&source));
        let to_body = DictionaryExpr::SwitchFields(quote!(obj), to_conversions.present(&source));
        let drop_body = DictionaryExpr::SwitchFields(quote!(self), destroy_conversions.present(&source));

        let mut to_conversions = CommaPunctuated::new();

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
                from_conversions_statements: from_body.to_token_stream(),
                to_conversions_statements: to_body.to_token_stream(),
                destroy_body: drop_body.to_token_stream(),
            },
            InterfaceImplementation::BindingsImplementation {
                objc_name,
                c_name,
                to_conversions,
                property_names,
            }
        ]);
        interfaces
    }
}
impl<SPEC> SourceFermentable<ObjCFermentate> for EnumComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    fn ferment(&self) -> ObjCFermentate {
        let implementations = self.compose_interfaces();
        println!("OBJC: ENUM FERMENT: \n{}", format_interface_implementations(&implementations));
        ObjCFermentate::Item {
            implementations
        }
    }
}



