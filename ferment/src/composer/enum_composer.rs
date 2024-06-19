use syn::{Attribute, Generics};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use crate::composer::{AttrsComposer, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, Composer, constants, Depunctuated, EnumParentComposer, FFIAspect, OwnerIteratorPostProcessingComposer, ParentComposer, VariantComposer, VariantIteratorLocalContext};
use crate::composer::basic::BasicComposer;
use crate::composer::composable::{BasicComposable, BindingComposable, ConversionComposable, DropComposable, SourceExpandable, FFIObjectComposable, NameContext, SourceAccessible};
use crate::composer::generics_composer::GenericsComposer;
use crate::composer::item::ItemComposerWrapper;
use crate::composer::r#type::TypeComposer;
use crate::composition::{AttrsComposition, CfgAttributes};
use crate::context::{ScopeChain, ScopeContext};
use crate::presentation::context::{FieldContext, name, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::shared::{ParentLinker, SharedAccess};
use crate::wrapped::DelimiterTrait;

pub struct EnumComposer<I>
    where I: DelimiterTrait + ?Sized + 'static {
    pub base: BasicComposer<ParentComposer<EnumComposer<I>>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<EnumComposer<I>>>,

    pub variant_composers: Vec<ItemComposerWrapper>,
    pub variant_presenters: Vec<(VariantComposer, VariantIteratorLocalContext)>,
}

impl<I> NameContext for EnumComposer<I> where I: DelimiterTrait + ?Sized {
    fn name_context_ref(&self) -> &name::Context {
        self.base.name_context_ref()
    }
}

impl<I> BasicComposable<EnumParentComposer<I>> for EnumComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
    }

    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.base.doc.compose(&()))
    }
}

impl<I> SourceAccessible for EnumComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }
}

impl<I> SourceExpandable for EnumComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn expand(&self) -> Expansion {
        Expansion::Full {
            attrs: self.compose_attributes(),
            comment: self.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversion: ConversionComposable::<EnumParentComposer<I>>::compose_conversion(self),
            drop: self.compose_drop(),
            bindings: self.compose_bindings(),
            // traits: self.base.compose_attributes()
            // TODO: migrate to specific composer chain
            traits: Depunctuated::new()
        }
    }
}

impl<I> DropComposable for EnumComposer<I>
    where I: DelimiterTrait + ?Sized {
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
                        .map(|composer| OwnedItemPresentableContext::Conversion(composer.compose_aspect(FFIAspect::Drop), composer.compose_attributes().to_token_stream())));
                    // TODO: make only if fields contain any conditional compilation flags
                    result.push(OwnedItemPresentableContext::Exhaustive(quote!()));
                    result
                }))
                .present(&source)
        }
    }

}

impl<I> FFIObjectComposable for EnumComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.context().borrow()))
    }
}

impl<I> BindingComposable for EnumComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        let source = self.context().borrow();
        let mut bindings = Depunctuated::new();
        bindings.extend(self.variant_composers
            .iter()
            .map(|composer| composer.compose_ctor(&source)));
        bindings.push(BindingPresentableContext::<CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>::Destructor(
            Aspect::FFI(self.base.name_context()).present(&source),
            self.compose_attributes().to_token_stream(),
            self.base.generics.compose(self.context()))
            .present(&source));
        bindings
    }
}

impl<Parent, I> ConversionComposable<Parent> for EnumComposer<I>
    where Parent: SharedAccess, I: DelimiterTrait + ?Sized {
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>) {
        let (conversions_from_ffi, conversions_to_ffi) = self.variant_composers.iter().map(|composer| {
            // let composer_owned = composer.borrow();
            let attrs = composer.compose_attributes();
            let from = composer.compose_aspect(FFIAspect::From);
            let to = composer.compose_aspect(FFIAspect::To);
            (quote! { #attrs #from }, quote! { #attrs #to })
        }).unzip();
        (FromConversionPresentation::Enum(conversions_from_ffi),
         ToConversionPresentation::Enum(conversions_to_ffi),
         DestroyPresentation::Default,
         self.base.generics.compose(self.context()))
    }
}

impl<I> EnumComposer<I> where I: DelimiterTrait + ?Sized {
    pub fn new(
        target_name: &Ident,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>,
        variant_composers: (Vec<ItemComposerWrapper>, Vec<(VariantComposer, VariantIteratorLocalContext)>),
    ) -> EnumParentComposer<I> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::new(
                AttrsComposer::new(AttrsComposition::from(attrs, target_name, scope)),
                constants::enum_composer_doc(),
                TypeComposer::new(Context::Enum {
                    ident: target_name.clone(),
                    attrs: attrs.cfg_attributes_expanded(),
                }),
                GenericsComposer::new(Some(generics.clone())),
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
    fn setup_composers(&mut self, root: &EnumParentComposer<I>) {
        self.base.link(root);
        self.ffi_object_composer.link(root);
    }
}
