use quote::ToTokens;
use crate::ast::{CommaPunctuatedTokens, DelimiterTrait, Depunctuated, SemiPunctuated};
use crate::composer::{AspectPresentable, AttrComposable, CommaPunctuatedFields, FFIAspect, FieldsComposerRef, GenericsComposable, InterfaceComposable, Linkable, SourceAccessible, SourceFermentable, TypeAspect};
use crate::lang::objc::ObjCSpecification;
use crate::lang::objc::composer::{ArgsComposer, ClassNameComposer};
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::ObjCFermentate;
use crate::lang::objc::presentable::TypeContext;
use crate::lang::Specification;
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::shared::SharedAccess;

// #[derive(BasicComposerOwner)]
pub struct ItemComposer<Parent, LANG, SPEC>
    where Parent: SharedAccess + 'static,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub parent: Option<Parent>,
    pub objc_class_name_composer: ClassNameComposer,
    pub args_composer: ArgsComposer<LANG, SPEC>,
    pub context: TypeContext,
}
impl<Parent, LANG, SPEC> Linkable<Parent> for ItemComposer<Parent, LANG, SPEC>
    where Parent: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

// impl<Parent, LANG, SPEC> TypeAspect<Context> for ItemComposer<Parent, LANG, SPEC>
//     where Parent: SharedAccess,
//           LANG: Clone,
//           SPEC: Specification<LANG> {
//     fn type_context_ref(&self) -> &Context {
//         &self.context
//     }
// }


impl<Parent, LANG, SPEC> ItemComposer<Parent, LANG, SPEC>
    where Parent: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(context: TypeContext, fields: &CommaPunctuatedFields, fields_composer: FieldsComposerRef<LANG, SPEC>) -> Self {
        Self {
            parent: None,
            context: context.clone(),
            objc_class_name_composer: ClassNameComposer { aspect: Aspect::FFI(context) },
            args_composer: ArgsComposer { fields: fields_composer(fields) }
            // c_class_name_composer: ClassNameComposer { aspect: Aspect::},
        }
    }
}


// impl<Parent, SPEC> Ferment for ItemComposer<Parent, ObjCFermentate, SPEC>
//     where Parent: SharedAccess,
//           SPEC: Specification<ObjCFermentate, Attr=AttrWrapper, Gen=Option<Generics>> {
//     fn ferment-sys(&self, scope_context: &ComposerLink<ScopeContext>) -> Depunctuated<Fermentate> {
//         let source = scope_context.borrow();
//         let global = source.context.read().unwrap();
//         let config = global.config.maybe_objc_config().unwrap();
//         let prefix = config.class_prefix();
//         let c_name = self.objc_class_name_composer.aspect.present(&source).to_token_stream();
//         let objc_name = self.objc_class_name_composer.compose(&(prefix, &source));
//
//         let fermentate = ObjCFermentate::Item {
//             header_name: config.xcode.framework_name.clone(),
//             imports: Depunctuated::new(),
//             interfaces: Depunctuated::from_iter([
//                 InterfacePresentation::Default {
//                     name: objc_name.clone(),
//                     c_type: c_name.clone(),
//                     properties: Default::default(),
//                 },
//                 InterfacePresentation::C {
//                     name: objc_name.clone(),
//                     c_type: c_name.clone()
//                 },
//                 InterfacePresentation::Rust {
//                     name: objc_name.clone(),
//                     c_type: c_name.clone()
//                 },
//                 InterfacePresentation::Args {
//                     name: objc_name.clone(),
//                     c_type: c_name.clone(),
//                     args: Default::default(),
//                 }
//             ]),
//             implementations: Depunctuated::from_iter([
//                 ImplementationPresentation::Default {
//                     objc_name: objc_name.clone(),
//                     c_type: c_name.clone(),
//                     properties_inits: self.args_composer
//                         .fields
//                         .iter()
//                         .map(Property::from)
//                         .collect(),
//                 },
//                 ImplementationPresentation::C {
//                     objc_name: objc_name.clone(),
//                     c_type: c_name.clone(),
//                     property_ctors: Default::default(),
//                     property_dtors: Default::default(),
//                 },
//                 ImplementationPresentation::Rust {
//                     objc_name: objc_name.clone(),
//                     c_type: c_name.clone(),
//                     c_var: quote!(struct #c_name *),
//                     property_names: self.args_composer.fields.iter().map(|composer| composer.name.to_token_stream()).collect(),
//                     property_ctors: Default::default(),
//                 },
//                 ImplementationPresentation::Args {
//                     objc_name: objc_name.clone(),
//                     prop_implementations: Default::default(),
//                 }
//             ])
//         };
//         println!("ObjC fermentate: {}", fermentate.to_string());
//
//         Depunctuated::from_iter([
//             Fermentate::ObjC(fermentate)
//         ])
//
//
//     }
// }

impl<I, SPEC> InterfaceComposable<SPEC::Interface> for crate::composer::ItemComposer<I, ObjCFermentate, SPEC>
    where I: DelimiterTrait + ?Sized,
          SPEC: ObjCSpecification,
          Self: GenericsComposable<SPEC::Gen> + AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> {
    fn compose_interfaces(&self) -> Depunctuated<SPEC::Interface> {
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let objc_name = target_type.to_token_stream();
        let c_name = ffi_type.to_token_stream();


        // let mut properties = SemiPunctuated::new();

        // self.field_composers.iter().for_each(|c| {
        //
        // });
        //
        println!("OBJC:: ITEM FFI ASPECT TYPE: {}", ffi_type.to_token_stream());
        println!("OBJC:: ITEM TARGET ASPECT TYPE: {}", objc_name);
        println!("OBJC:: ITEM ASPECT FROM: {}", self.present_aspect(FFIAspect::From));
        println!("OBJC:: ITEM ASPECT TO: {}", self.present_aspect(FFIAspect::To));
        println!("OBJC:: ITEM ASPECT DESTROY: {}", self.present_aspect(FFIAspect::Destroy));
        println!("OBJC:: ITEM ASPECT DROP: {}", self.present_aspect(FFIAspect::Drop));

        //self.field_composers.iter()

        // let def_impl_properties

        //self.fields_from().compose(&())

        let properties = SemiPunctuated::new();
        let properties_inits = SemiPunctuated::new();

        Depunctuated::from_iter([
            InterfaceImplementation::default(objc_name.clone(), c_name.clone(), properties, properties_inits),
            InterfaceImplementation::c(objc_name.clone(), c_name.clone(), SemiPunctuated::new(), SemiPunctuated::new()),
            InterfaceImplementation::rust(objc_name.clone(), c_name.clone(), CommaPunctuatedTokens::new(), SemiPunctuated::new()),
            InterfaceImplementation::args(objc_name.clone(), c_name.clone(), Depunctuated::new(), Depunctuated::new()),
        ])
        // let generics = self.compose_generics();
        // let attrs = self.compose_attributes();
        // let ffi_type = self.present_ffi_aspect();
        // let types = (ffi_type.clone(), self.present_target_aspect());
        // let from  = self.present_aspect(FFIAspect::From);
        // attrs.wrap(
        // Depunctuated::from_iter([
        // InterfacePresentation::conversion_from(&attrs, &types, from, &generics),
        // InterfacePresentation::conversion_to(&attrs, &types, self.present_aspect(FFIAspect::To), &generics),
        // InterfacePresentation::conversion_destroy(&attrs, &types, self.present_aspect(FFIAspect::Destroy), &generics),
        // InterfacePresentation::drop(&attrs, ffi_type, self.present_aspect(FFIAspect::Drop))
        // ]
        // ))
    }
}

impl<I, SPEC> SourceFermentable<ObjCFermentate> for crate::composer::ItemComposer<I, ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification,
          I: DelimiterTrait + ?Sized {
    fn ferment(&self) -> ObjCFermentate {
        let source = self.source_ref();
        let global = source.context.read().unwrap();
        let config = global.config.maybe_objc_config().unwrap();
        let interfaces = self.compose_interfaces();
        println!("OBJC: ITEM FERMENT: {}", interfaces.to_token_stream());
        ObjCFermentate::Item {
            header_name: config.xcode.framework_name.clone(),
            imports: Depunctuated::new(),
            implementations: self.compose_interfaces()
        }
        // crate::lang::objc::ObjCFermentate::Item {
        //     attrs: self.compose_attributes(),
        //     comment: self.compose_docs(),
        //     ffi_presentation: self.compose_object(),
        //     conversions: self.compose_interfaces(),
        //     bindings: self.compose_bindings().present(&self.source_ref()),
        //     traits: Depunctuated::new()
        // }
        // #[cfg(feature = "objc")]
        // fermentate.extend(self.objc_composer.ferment-sys(&self.context()));
        // fermentate
    }
}

