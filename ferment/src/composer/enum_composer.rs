use syn::{Attribute, Generics};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use crate::composer::{AttrsComposer, Composer, constants, Depunctuated, EnumParentComposer, FFIAspect, ItemParentComposer, OwnerIteratorPostProcessingComposer, ParentComposer, VariantComposer, VariantIteratorLocalContext};
use crate::composer::basic::BasicComposer;
use crate::composer::composable::{BasicComposable, BindingComposable, ConversionComposable, DropComposable, SourceExpandable, FFIObjectComposable, NameContext};
use crate::composer::r#type::TypeComposer;
use crate::composition::{AttrsComposition, CfgAttributes};
use crate::context::{ScopeChain, ScopeContext};
use crate::presentation::context::{FieldContext, name, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::shared::{ParentLinker, SharedAccess};

pub struct EnumComposer {
    pub base: BasicComposer<ParentComposer<EnumComposer>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<EnumComposer>>,

    pub variant_composers: Vec<ItemParentComposer>,
    pub variant_presenters: Vec<(VariantComposer, VariantIteratorLocalContext)>,
}

impl NameContext for EnumComposer {
    fn name_context_ref(&self) -> &name::Context {
        self.base.name_context_ref()
    }
}

impl BasicComposable<EnumParentComposer> for EnumComposer {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
    }

    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.base.doc.compose(&()))
    }
}


impl SourceExpandable for EnumComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }

    fn expand(&self) -> Expansion {
        Expansion::Full {
            attrs: self.compose_attributes(),
            comment: self.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversion: ConversionComposable::<EnumParentComposer>::compose_conversion(self),
            drop: self.compose_drop(),
            bindings: self.compose_bindings(),
            // traits: self.base.compose_attributes()
            // TODO: migrate to specific composer chain
            traits: Depunctuated::new()
        }
    }
}

impl DropComposable for EnumComposer {
    fn compose_drop(&self) -> DropInterfacePresentation {
        let source = self.source_ref();
        DropInterfacePresentation::Full {
            attrs: self.compose_attributes().to_token_stream(),
            ty: self.base.ffi_name_aspect().present(&source),
            body: OwnerIteratorPresentationContext::MatchFields((
                FieldContext::Simple(quote!(self)).into(),
                {
                    let mut result =
                    Punctuated::from_iter(self.variant_composers
                        .iter()
                        .map(|composer| {
                            let comp = composer.borrow();
                            OwnedItemPresentableContext::Conversion(comp.compose_aspect(FFIAspect::Drop), comp.compose_attributes().to_token_stream())
                        }));
                    // TODO: make only if fields contain any conditional compilation flags
                    result.push(OwnedItemPresentableContext::Exhaustive(quote!()));
                    result
                }))
                .present(&source)
        }
    }

}

impl FFIObjectComposable for EnumComposer {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.context().borrow()))
    }
}

impl BindingComposable for EnumComposer {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        let source = self.context().borrow();
        let mut bindings = Depunctuated::new();
        bindings.extend(self.variant_composers
            .iter()
            .map(|composer| composer.borrow().ctor_composer.compose(&()).present(&source)));
        bindings.push(BindingPresentableContext::Destructor(Aspect::FFI(self.base.name_context()).present(&source), self.compose_attributes().to_token_stream()).present(&source));
        bindings
    }
}

impl<Parent> ConversionComposable<Parent> for EnumComposer where Parent: SharedAccess {
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>) {
        let (conversions_from_ffi, conversions_to_ffi) = self.variant_composers.iter().map(|composer| {
            let composer_owned = composer.borrow();
            let attrs = composer_owned.compose_attributes();
            let from = composer_owned.compose_aspect(FFIAspect::From);
            let to = composer_owned.compose_aspect(FFIAspect::To);
            (quote! {
                #attrs
                #from
            }, quote! {
                #attrs
                #to
            })
        }).unzip();
        (FromConversionPresentation::Enum(conversions_from_ffi),
         ToConversionPresentation::Enum(conversions_to_ffi),
         DestroyPresentation::Default,
         self.base.generics.clone())
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
            base: BasicComposer::new(
                AttrsComposer::new(AttrsComposition::from(attrs, target_name, scope)),
                constants::enum_composer_doc(),
                TypeComposer::new(Context::Enum {
                    ident: target_name.clone(),
                    attrs: attrs.cfg_attributes_expanded(),
                }),
                Some(generics.clone()),
                Rc::clone(context)
            ),
            variant_composers: variant_composers.0,
            variant_presenters: variant_composers.1,
            ffi_object_composer: constants::enum_composer_object(),
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &EnumParentComposer) {
        self.base.link(root);
        self.ffi_object_composer.link(root);
    }
}
