use quote::ToTokens;
use crate::ast::{CommaPunctuated, DelimiterTrait, Depunctuated, SemiPunctuated};
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, FFIAspect, FFIObjectComposable, FieldsConversionComposable, GenericsComposable, InterfaceComposable, NameKindComposable, SourceAccessible, SourceComposable, SourceFermentable, ToConversionComposer, TypeAspect, VarComposer};
use crate::lang::objc::ObjCSpecification;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::ObjCFermentate;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::lang::objc::presentable::ArgPresentation;
use crate::presentable::{Expression, ScopeContextPresentable};

impl<I, SPEC> InterfaceComposable<SPEC::Interface> for crate::composer::ItemComposer<I, ObjCFermentate, SPEC>
    where I: DelimiterTrait + ?Sized,
          SPEC: ObjCSpecification,
          Self: GenericsComposable<SPEC::Gen>
            + AttrComposable<SPEC::Attr>
            + TypeAspect<SPEC::TYC>
            + NameKindComposable {
    fn compose_interfaces(&self) -> Depunctuated<SPEC::Interface> {
        let source = self.source_ref();
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let objc_name = target_type.to_token_stream();
        let c_name = ffi_type.to_token_stream();

        println!("OBJC:: ITEM FFI ASPECT TYPE: {}", ffi_type.to_token_stream());
        println!("OBJC:: ITEM TARGET ASPECT TYPE: {}", objc_name);
        println!("OBJC:: ITEM ASPECT FROM: {}", self.present_aspect(FFIAspect::From));
        println!("OBJC:: ITEM ASPECT TO: {}", self.present_aspect(FFIAspect::To));
        println!("OBJC:: ITEM ASPECT DESTROY: {}", self.present_aspect(FFIAspect::Destroy));
        println!("OBJC:: ITEM ASPECT DROP: {}", self.present_aspect(FFIAspect::Drop));
        println!("OBJC:: ITEM ASPECT OBJ: {}", self.compose_object().to_token_stream());
        println!("OBJC:: ITEM ASPECT F_FROM => {}", self.fields_from().compose(&()).present(&source));
        println!("OBJC:: ITEM ASPECT F_TO => {}", self.fields_to().compose(&()).present(&source));

        let mut property_names = CommaPunctuated::new();
        let mut vars = Depunctuated::new();
        let mut properties = SemiPunctuated::new();
        let mut to_conversions = CommaPunctuated::new();
        self.field_composers
            .iter()
            .for_each(|FieldComposer { name, kind, .. }| {
                let var = VarComposer::<ObjCFermentate, SPEC>::key_in_scope(kind.ty(), &source.scope)
                    .compose(&source);
                let to_conversion = ToConversionComposer::new(name.clone(), kind.ty().clone(), Some(Expression::ObjName(name.clone())))
                    .compose(&source)
                    .present(&source);

                property_names.push(name.to_token_stream());
                properties.push(ArgPresentation::NonatomicReadwrite { ty: var.to_token_stream(), name: name.to_token_stream() });

                to_conversions.push(to_conversion.to_token_stream());
                vars.push(var);
            });

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
                from_conversions_statements: self.present_aspect(FFIAspect::From),
                to_conversions_statements: self.present_aspect(FFIAspect::To),
                destroy_body: self.present_aspect(FFIAspect::Drop),
            },
            InterfaceImplementation::BindingsImplementation {
                objc_name,
                c_name,
                to_conversions,
                property_names,
            }
        ]);

        // println!("OBJC ITEM => \n{}", format_interface_implementations(&interfaces));
        interfaces
    }
}

impl<I, SPEC> SourceFermentable<ObjCFermentate> for crate::composer::ItemComposer<I, ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification,
          I: DelimiterTrait + ?Sized, Self: NameKindComposable {
    fn ferment(&self) -> ObjCFermentate {
        let implementations = self.compose_interfaces();
        println!("OBJC: ITEM FERMENT: \n{}", format_interface_implementations(&implementations));
        ObjCFermentate::Item {
            implementations
        }
    }
}

