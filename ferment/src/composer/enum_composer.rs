use syn::{Attribute, Generics};
use proc_macro2::Ident;
use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use ferment_macro::BasicComposerOwner;
use crate::ast::{CommaPunctuated, DelimiterTrait, Depunctuated};
use crate::composable::{AttrsComposition, CfgAttributes};
use crate::composer::{BasicComposable, BasicComposer, BindingComposable, CommaPunctuatedArgs, CommaPunctuatedOwnedItems, Composer, constants, ConversionComposable, DocsComposable, DropComposable, EnumParentComposer, FFIAspect, FFIObjectComposable, ItemComposerWrapper, Linkable, NameContext, OwnerAspectWithCommaPunctuatedItems, OwnerIteratorPostProcessingComposer, ParentComposer, SourceAccessible, SourceExpandable, VariantComposerRef};
use crate::context::{ScopeChain, ScopeContext};
use crate::presentable::{BindingPresentableContext, Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{BindingPresentation, DestroyPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation};
use crate::shared::SharedAccess;

#[derive(BasicComposerOwner)]
pub struct EnumComposer<I>
    where I: DelimiterTrait + ?Sized + 'static {
    pub base: BasicComposer<ParentComposer<EnumComposer<I>>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<EnumComposer<I>>>,

    pub variant_composers: Vec<ItemComposerWrapper>,
    pub variant_presenters: Vec<(VariantComposerRef, OwnerAspectWithCommaPunctuatedItems)>,
}

impl<I> DocsComposable for EnumComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.base.doc.compose(&()))
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
            ty: self.compose_ffi_name(),
            body: SequenceOutput::MatchFields((
                Expression::Simple(quote!(self)).into(),
                {
                    let mut result =
                    CommaPunctuated::from_iter(self.variant_composers
                        .iter()
                        .map(|composer|
                            OwnedItemPresentableContext::SequenceOutput(composer.compose_aspect(FFIAspect::Drop), composer.compose_attributes())));
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
            .present(&self.source_ref()))
    }
}

impl<I> BindingComposable for EnumComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        let source = self.source_ref();
        let mut bindings = Depunctuated::new();
        bindings.extend(self.variant_composers
            .iter()
            .map(|composer| composer.compose_ctor(&source)));
        bindings.push(BindingPresentableContext::<CommaPunctuatedOwnedItems, CommaPunctuatedArgs, I>::Destructor(
            self.compose_ffi_name(),
            self.compose_attributes(),
            self.compose_generics())
            .present(&source));
        bindings
    }
}

impl<Parent, I> ConversionComposable<Parent> for EnumComposer<I>
    where Parent: SharedAccess, I: DelimiterTrait + ?Sized {
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>) {
        let (conversions_from_ffi, conversions_to_ffi) = self.variant_composers.iter().map(|composer| {
            let source = self.source_ref();
            let attrs = composer.compose_attributes();
            let from = composer.compose_aspect(FFIAspect::From).present(&source);
            let to = composer.compose_aspect(FFIAspect::To).present(&source);

            (quote! { #attrs #from }, quote! { #attrs #to })
        }).unzip();
        (FromConversionPresentation::Enum(conversions_from_ffi),
         ToConversionPresentation::Enum(conversions_to_ffi),
         DestroyPresentation::Default,
         self.compose_generics())
    }
}

impl<I> EnumComposer<I> where I: DelimiterTrait + ?Sized {
    pub fn new(
        target_name: &Ident,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>,
        variant_composers: (Vec<ItemComposerWrapper>, Vec<(VariantComposerRef, OwnerAspectWithCommaPunctuatedItems)>),
    ) -> EnumParentComposer<I> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                AttrsComposition::from(attrs, target_name, scope),
                Context::Enum {
                    ident: target_name.clone(),
                    attrs: attrs.cfg_attributes_expanded(),
                },
                Some(generics.clone()),
                constants::composer_doc(),
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
