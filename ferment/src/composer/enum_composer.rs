use syn::{Generics, parse_quote, Path};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use quote::ToTokens;
use crate::composer::{AttrsComposer, Composer, constants, ContextComposer, Depunctuated, EnumParentComposer, FFIAspect, ItemParentComposer, NameComposer, OwnerIteratorPostProcessingComposer, ParentComposer, SimpleContextComposer, VariantComposer};
use crate::composer::parent_composer::IParentComposer;
use crate::composition::AttrsComposition;
use crate::context::ScopeContext;
use crate::interface::{DEFAULT_DOC_PRESENTER, package_unboxed_root};
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation, TraitVTablePresentation};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::shared::HasParent;

pub struct EnumComposer {
    pub context: ParentComposer<ScopeContext>,
    pub target_name_composer: NameComposer<EnumParentComposer>,
    pub attrs_composer: AttrsComposer<EnumParentComposer>,
    pub doc_composer: SimpleContextComposer<EnumParentComposer>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<EnumParentComposer>,
    pub variant_composers: Vec<ItemParentComposer>,
    pub variant_presenters: Vec<(VariantComposer, Ident, Punctuated<OwnedItemPresentableContext, Comma>)>,
    pub generics: Option<Generics>,
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
        let target_name = self.target_name_composer.compose(&());
        let mut bindings = Punctuated::new();
        bindings.extend(self.variant_composers
            .iter()
            .map(|composer| composer.borrow().ctor_composer.compose(&()).present(&source)));
        bindings.push(BindingPresentableContext::Destructor(parse_quote!(#target_name)).present(&source));
        bindings
    }

    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Default(self.doc_composer.compose(&()))
    }

    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.context().borrow()))
    }

    fn compose_drop(&self) -> DropInterfacePresentation {
        DropInterfacePresentation::Full {
            name: self.target_name_composer.compose(&()),
            body: IteratorPresentationContext::EnumDropBody(self.variant_composers
                .iter()
                .map(|composer|
                    OwnedItemPresentableContext::Conversion(composer.borrow().compose_aspect(FFIAspect::Drop)))
                .collect())
                .present(&self.context().borrow()),
        }
    }

    fn compose_names(&self) -> (TokenStream2, TokenStream2) {
        let ffi_type = self.target_name_composer.compose(&());
        let target_type = self.context().borrow().full_type_for(&parse_quote!(#ffi_type));
        (ffi_type, target_type.to_token_stream())
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
        target_name: Path,
        generics: Generics,
        attrs: AttrsComposition,
        variant_composers: Vec<ItemParentComposer>,
        variant_presenters: Vec<(VariantComposer, Ident, Punctuated<OwnedItemPresentableContext, Comma>)>,
        context: &ParentComposer<ScopeContext>,
    ) -> EnumParentComposer {
        let root = Rc::new(RefCell::new(Self {
            context: Rc::clone(context),
            generics: Some(generics),
            doc_composer: ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<EnumComposer>| composer.target_name_composer.compose(&())),
            variant_composers,
            variant_presenters,
            target_name_composer: NameComposer::new(target_name),
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
        self.target_name_composer.set_parent(root);
        self.ffi_object_composer.set_parent(root);
    }
}
