use syn::{Attribute, Generics};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use std::rc::Rc;
use std::cell::RefCell;
use quote::quote;
use crate::composer::{AttrsComposer, Composer, constants, Depunctuated, EnumParentComposer, FFIAspect, ItemParentComposer, OwnerIteratorPostProcessingComposer, ParentComposer, TypeContextComposer, VariantComposer, VariantIteratorLocalContext};
use crate::composer::composable::Composable;
use crate::composer::r#type::TypeComposer;
use crate::composition::AttrsComposition;
use crate::context::{ScopeChain, ScopeContext};
use crate::presentation::context::{FieldTypePresentableContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation, TraitVTablePresentation};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::shared::HasParent;

pub struct EnumComposer {
    pub context: ParentComposer<ScopeContext>,
    pub attrs_composer: AttrsComposer<ParentComposer<EnumComposer>>,
    pub doc_composer: TypeContextComposer<ParentComposer<EnumComposer>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<EnumComposer>>,
    pub type_composer: TypeComposer<ParentComposer<EnumComposer>>,
    pub generics: Option<Generics>,

    pub variant_composers: Vec<ItemParentComposer>,
    pub variant_presenters: Vec<(VariantComposer, VariantIteratorLocalContext)>,
}

impl Composable for EnumComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        &self.context
    }

    fn name_context_ref(&self) -> &Context {
        &self.type_composer.context
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
        bindings.push(BindingPresentableContext::Destructor(Aspect::FFI(self.name_context()).present(&source)).present(&source));
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
        let source = self.as_source_ref();
        DropInterfacePresentation::Full {
            ty: self.ffi_name_aspect().present(&source),
            body: OwnerIteratorPresentationContext::MatchFields((
                FieldTypePresentableContext::Simple(quote!(self)).into(),
                self.variant_composers
                    .iter()
                    .map(|composer|
                        OwnedItemPresentableContext::Conversion(composer.borrow().compose_aspect(FFIAspect::Drop)))
                    .collect()))
                .present(&source)
        }
    }

    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>) {
        let (conversions_from_ffi, conversions_to_ffi) = self.variant_composers.iter().map(|composer| {
            let composer_owned = composer.borrow();
            (composer_owned.compose_aspect(FFIAspect::From),
             composer_owned.compose_aspect(FFIAspect::To))
        }).unzip();
        (FromConversionPresentation::Enum(conversions_from_ffi),
         ToConversionPresentation::Enum(conversions_to_ffi),
         DestroyPresentation::Default,
         self.generics.clone())
    }
}

impl EnumComposer {
    pub fn new(
        target_name: &Ident,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>,
        variant_composers: (Vec<ItemParentComposer>, Vec<(VariantComposer, VariantIteratorLocalContext)>),
    ) -> EnumParentComposer {
        let root = Rc::new(RefCell::new(Self {
            context: Rc::clone(context),
            generics: Some(generics.clone()),
            doc_composer: constants::enum_composer_doc(),
            variant_composers: variant_composers.0,
            variant_presenters: variant_composers.1,
            type_composer: TypeComposer::new(Context::Enum { ident: target_name.clone() }),
            attrs_composer: AttrsComposer::new(AttrsComposition::from(attrs, target_name, scope)),
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
