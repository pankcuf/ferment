use quote::ToTokens;
use crate::ast::{CommaPunctuated, DelimiterTrait, Depunctuated, SemiPunctuated};
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, FFIAspect, FFIObjectComposable, FieldsConversionComposable, GenericsComposable, InterfaceComposable, NameKindComposable, SourceAccessible, SourceComposable, SourceFermentable, ToConversionFullComposer, TypeAspect, VarComposer};
use crate::lang::objc::ObjCSpecification;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::ObjCFermentate;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::lang::objc::presentable::ArgPresentation;
use crate::lang::Specification;
use crate::presentable::{Expression, ScopeContextPresentable};

impl<I> InterfaceComposable<<ObjCSpecification as Specification>::Interface> for crate::composer::ItemComposer<ObjCSpecification, I>
    where I: DelimiterTrait + ?Sized,
          Self: GenericsComposable<<ObjCSpecification as Specification>::Gen>
            + AttrComposable<<ObjCSpecification as Specification>::Attr>
            + TypeAspect<<ObjCSpecification as Specification>::TYC>
            + NameKindComposable {
    fn compose_interfaces(&self) -> Depunctuated<<ObjCSpecification as Specification>::Interface> {
        let source = self.source_ref();
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let objc_name = target_type.to_token_stream();
        let c_name = ffi_type.to_token_stream();

        let from = self.compose_aspect(FFIAspect::From).present(&source);
        let to = self.compose_aspect(FFIAspect::To).present(&source);
        let destroy = self.compose_aspect(FFIAspect::Drop).present(&source);
        let drop = self.compose_aspect(FFIAspect::Drop).present(&source);

        println!("OBJC:: ITEM FFI ASPECT TYPE: {}", ffi_type.to_token_stream());
        println!("OBJC:: ITEM TARGET ASPECT TYPE: {}", objc_name);
        println!("OBJC:: ITEM ASPECT FROM: {}", from);
        println!("OBJC:: ITEM ASPECT TO: {}", to);
        println!("OBJC:: ITEM ASPECT DESTROY: {}", destroy);
        println!("OBJC:: ITEM ASPECT DROP: {}", drop);
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
                let var = VarComposer::<ObjCSpecification>::key_in_scope(kind.ty(), &source.scope)
                    .compose(&source);
                let to_conversion = ToConversionFullComposer::<ObjCSpecification>::key_expr(name.clone(), kind.ty(), &source.scope, Some(Expression::ObjName(name.clone())))
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
                from_conversions_statements: from,
                to_conversions_statements: to,
                destroy_body: drop,
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

impl<I> SourceFermentable<ObjCFermentate> for crate::composer::ItemComposer<ObjCSpecification, I>
    where I: DelimiterTrait + ?Sized, Self: NameKindComposable {
    fn ferment(&self) -> ObjCFermentate {
        let implementations = self.compose_interfaces();
        println!("OBJC: ITEM FERMENT: \n{}", format_interface_implementations(&implementations));
        ObjCFermentate::Item {
            implementations
        }
    }
}

