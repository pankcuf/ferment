use syn::{Generics, parse_quote};
use proc_macro2::{TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use crate::composer::{AttrsComposer, Composer, constants, ContextComposer, Depunctuated, EnumParentComposer, FFIAspect, ItemParentComposer, NameComposer, NameContextComposer, OwnerIteratorPostProcessingComposer, ParentComposer, r#type, VariantComposer};
use crate::composer::parent_composer::IParentComposer;
use crate::composer::r#type::TypeComposer;
use crate::composition::AttrsComposition;
use crate::context::ScopeContext;
use crate::interface::{DEFAULT_DOC_PRESENTER, package_unboxed_root};
use crate::naming::Name;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation, TraitVTablePresentation};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::shared::HasParent;

pub struct EnumComposer {
    pub context: ParentComposer<ScopeContext>,
    pub attrs_composer: AttrsComposer<ParentComposer<Self>>,
    pub doc_composer: NameContextComposer<ParentComposer<Self>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<Self>>,
    pub type_composer: TypeComposer<ParentComposer<Self>>,
    pub generics: Option<Generics>,

    pub variant_composers: Vec<ItemParentComposer>,
    pub variant_presenters: Vec<(VariantComposer, Name, Punctuated<OwnedItemPresentableContext, Comma>)>,
}

impl IParentComposer for EnumComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        &self.context
    }

    fn compose_attributes(&self) -> Depunctuated<TraitVTablePresentation> {
        self.attrs_composer.compose(&self.context().borrow())
    }

    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        let source = self.context().borrow();
        let mut bindings = Punctuated::new();
        bindings.extend(self.variant_composers
            .iter()
            .map(|composer| composer.borrow().ctor_composer.compose(&()).present(&source)));
        bindings.push(BindingPresentableContext::Destructor(self.type_composer.compose_aspect(r#type::FFIAspect::Target, &self.context.borrow())).present(&source));
        bindings
    }

    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.doc_composer.compose(&()))
    }

    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.context().borrow()))
    }

    fn compose_drop(&self) -> DropInterfacePresentation {
        DropInterfacePresentation::Full {
            name: self.type_composer.compose_aspect(r#type::FFIAspect::Target, &self.context.borrow()),
            body: IteratorPresentationContext::EnumDropBody(self.variant_composers
                .iter()
                .map(|composer|
                    OwnedItemPresentableContext::Conversion(composer.borrow().compose_aspect(FFIAspect::Drop)))
                .collect())
                .present(&self.context().borrow()),
        }
    }

    fn compose_names(&self) -> (Name, Name) {
        let ffi_type = self.type_composer.compose_aspect(r#type::FFIAspect::Target, &self.context.borrow());
        let target_type = self.context().borrow().full_type_for(&parse_quote!(#ffi_type));
        (ffi_type, Name::Type(target_type))
    }

    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, TokenStream2, Option<Generics>) {
        let (conversions_from_ffi, conversions_to_ffi) = self.variant_composers.iter().map(|composer| {
            let composer_owned = composer.borrow();
            (composer_owned.compose_aspect(FFIAspect::From), composer_owned.compose_aspect(FFIAspect::To))
        }).unzip();
        (FromConversionPresentation::Enum(conversions_from_ffi),
         ToConversionPresentation::Enum(conversions_to_ffi),
         package_unboxed_root(),
         self.generics.clone())
    }
}

impl EnumComposer {
    pub fn new(
        target_name: Name,
        generics: Generics,
        attrs: AttrsComposition,
        variant_composers: Vec<ItemParentComposer>,
        variant_presenters: Vec<(VariantComposer, Name, Punctuated<OwnedItemPresentableContext, Comma>)>,
        context: &ParentComposer<ScopeContext>,
    ) -> EnumParentComposer {
        let root = Rc::new(RefCell::new(Self {
            context: Rc::clone(context),
            generics: Some(generics),
            doc_composer: ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<EnumComposer>| composer.type_composer.compose_aspect(r#type::FFIAspect::Target, &composer.context.borrow())),
            variant_composers,
            variant_presenters,
            type_composer: TypeComposer::new(NameComposer::new(target_name.clone()), NameComposer::new(target_name)),
            attrs_composer: AttrsComposer::new(attrs),
            ffi_object_composer: constants::enum_composer_object(),
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &EnumParentComposer) {
        self.attrs_composer.set_parent(root);
        self.doc_composer.set_parent(root);
        self.type_composer.set_parent(root);
        self.ffi_object_composer.set_parent(root);
    }
}
