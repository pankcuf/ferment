use syn::Generics;
use crate::composer::{GenericParentComposer, NameContextComposer, OwnerIteratorPostProcessingComposer, ParentComposer};
use crate::composer::r#type::TypeComposer;
use crate::context::ScopeContext;

#[allow(unused)]
pub struct GenericComposer {
    pub context: ParentComposer<ScopeContext>,
    pub doc_composer: NameContextComposer<GenericParentComposer>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<GenericParentComposer>,
    pub type_composer: TypeComposer<GenericParentComposer>,
    pub generics: Option<Generics>,
}

impl GenericComposer {
    // pub fn map_composer() -> GenericParentComposer {
    //     Self::new(
    //         |(name, fields)| ,
    //     )
    // }
    // fn new(
    //     ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<Self>>,
    //     generics: Generics,
    //     context: &ParentComposer<ScopeContext>) -> GenericParentComposer {
    //     // match path_conversion {
    //     //     GenericPathConversion::Map(_) => {
    //     //
    //     //     }
    //     //     GenericPathConversion::Vec(_) => {}
    //     //     GenericPathConversion::Result(_) => {}
    //     //     GenericPathConversion::Box(_) => {}
    //     //     GenericPathConversion::AnyOther(_) => {}
    //     // }
    //     let root = Rc::new(RefCell::new(Self {
    //         context: Rc::clone(context),
    //         generics: Some(generics),
    //         doc_composer: ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<GenericComposer>| composer.type_composer.compose_aspect(r#type::FFIAspect::Target, &composer.context.borrow())),
    //         type_composer: TypeComposer::new(NameComposer::new(target_name.clone()), NameComposer::new(target_name)),
    //         ffi_object_composer,
    //     }));
    //     {
    //         let mut root_borrowed = root.borrow_mut();
    //         root_borrowed.setup_composers(&root);
    //     }
    //     root
    // }
}

// impl IParentComposer for GenericComposer {
//     fn context(&self) -> &ParentComposer<ScopeContext> {
//         &self.context
//     }
//
//     fn compose_attributes(&self) -> Depunctuated<TraitVTablePresentation> {
//         Depunctuated::new()
//     }
//
//     fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
//         todo!()
//     }
//
//     fn compose_docs(&self) -> DocPresentation {
//         todo!()
//     }
//
//     fn compose_object(&self) -> FFIObjectPresentation {
//         todo!()
//     }
//
//     fn compose_drop(&self) -> DropInterfacePresentation {
//         todo!()
//     }
//
//     fn compose_names(&self) -> (Name, Name) {
//         todo!()
//     }
//
//     fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, TokenStream2, Option<Generics>) {
//         todo!()
//     }
// }