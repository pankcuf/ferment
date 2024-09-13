use quote::{quote, ToTokens};
use crate::ast::Depunctuated;
use crate::composer::{CommaPunctuatedFields, Composer, FieldsComposerRef, Linkable, NameContext, ComposerLink};
use crate::context::ScopeContext;
use crate::conversion::Ferment;
use crate::lang::objc::composer::{ArgsComposer, ClassNameComposer};
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;
use crate::lang::objc::presentation::{ImplementationPresentation, InterfacePresentation, Property};
use crate::presentable::{Aspect, Context, ScopeContextPresentable};
use crate::presentation::Fermentate;
use crate::shared::SharedAccess;

// #[derive(BasicComposerOwner)]
pub struct ItemComposer<Parent> where Parent: SharedAccess + 'static {
    pub parent: Option<Parent>,
    pub objc_class_name_composer: ClassNameComposer,
    pub args_composer: ArgsComposer,
    pub context: Context,
}
impl<Parent> Linkable<Parent> for ItemComposer<Parent> where Parent: SharedAccess {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent> NameContext<Context> for ItemComposer<Parent> where Parent: SharedAccess {
    fn name_context_ref(&self) -> &Context {
        &self.context
    }
}


impl<Parent> ItemComposer<Parent> where Parent: SharedAccess {
    pub fn new(context: Context, fields: &CommaPunctuatedFields, fields_composer: FieldsComposerRef<ObjCFermentate, AttrWrapper>) -> Self {
        Self {
            parent: None,
            context: context.clone(),
            objc_class_name_composer: ClassNameComposer { aspect: Aspect::FFI(context) },
            args_composer: ArgsComposer { fields: fields_composer(fields) }
            // c_class_name_composer: ClassNameComposer { aspect: Aspect::},
        }
    }
}

impl<Parent> Ferment for ItemComposer<Parent> where Parent: SharedAccess {
    fn ferment(&self, scope_context: &ComposerLink<ScopeContext>) -> Depunctuated<Fermentate> {
        let source = scope_context.borrow();
        let global = source.context.read().unwrap();
        let config = global.config.maybe_objc_config().unwrap();
        let prefix = config.class_prefix();
        let c_name = self.objc_class_name_composer.aspect.present(&source).to_token_stream();
        let objc_name = self.objc_class_name_composer.compose(&(prefix, &source));

        let fermentate = ObjCFermentate::Item {
            header_name: config.xcode.framework_name.clone(),
            imports: Depunctuated::new(),
            interfaces: Depunctuated::from_iter([
                InterfacePresentation::Default {
                    name: objc_name.clone(),
                    c_type: c_name.clone(),
                    properties: Default::default(),
                },
                InterfacePresentation::C {
                    name: objc_name.clone(),
                    c_type: c_name.clone()
                },
                InterfacePresentation::Rust {
                    name: objc_name.clone(),
                    c_type: c_name.clone()
                },
                InterfacePresentation::Args {
                    name: objc_name.clone(),
                    c_type: c_name.clone(),
                    args: Default::default(),
                }
            ]),
            implementations: Depunctuated::from_iter([
                ImplementationPresentation::Default {
                    objc_name: objc_name.clone(),
                    c_type: c_name.clone(),
                    properties_inits: self.args_composer
                        .fields
                        .iter()
                        .map(Property::from)
                        .collect(),
                },
                ImplementationPresentation::C {
                    objc_name: objc_name.clone(),
                    c_type: c_name.clone(),
                    property_ctors: Default::default(),
                    property_dtors: Default::default(),
                },
                ImplementationPresentation::Rust {
                    objc_name: objc_name.clone(),
                    c_type: c_name.clone(),
                    c_var: quote!(struct #c_name *),
                    property_names: self.args_composer.fields.iter().map(|composer| composer.name.to_token_stream()).collect(),
                    property_ctors: Default::default(),
                },
                ImplementationPresentation::Args {
                    objc_name: objc_name.clone(),
                    prop_implementations: Default::default(),
                }
            ])
        };
        println!("ObjC fermentate: {}", fermentate.to_string());

        Depunctuated::from_iter([
            Fermentate::ObjC(fermentate)
        ])


    }
}