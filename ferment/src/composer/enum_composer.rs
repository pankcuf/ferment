use syn::{Attribute, Generics};
use proc_macro2::Ident;
use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use ferment_macro::BasicComposerOwner;
use crate::ast::{CommaPunctuated, DelimiterTrait, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes};
use crate::composer::{BasicComposable, BasicComposer, BindingComposable, CommaPunctuatedArgs, CommaPunctuatedOwnedItems, Composer, constants, ConversionComposable, DocsComposable, DropComposable, EnumParentComposer, FFIAspect, FFIObjectComposable, ItemComposerWrapper, Linkable, NameContext, OwnerAspectWithCommaPunctuatedItems, OwnerIteratorPostProcessingComposer, ParentComposer, SourceAccessible, SourceExpandable, VariantComposerRef};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::Terminated;
use crate::presentable::{BindingPresentableContext, Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{BindingPresentation, DictionaryExpr, DictionaryName, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, InterfacesMethodExpr};
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
            conversions: ConversionComposable::<EnumParentComposer<I>>::compose_conversions(self),
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
            attrs: self.compose_attributes(),
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
                    result.push(OwnedItemPresentableContext::Exhaustive(Vec::new()));
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
    fn compose_interface_from(&self) -> TokenStream2 {
        let source = self.source_ref();
        let conversions_from_ffi = CommaPunctuated::from_iter(self.variant_composers.iter().map(|composer| {
            let attrs = composer.compose_attributes();
            Depunctuated::from_iter([quote!(#(#attrs)*), composer.compose_aspect(FFIAspect::From).present(&source)])
                .to_token_stream()
        }));
        let ffi_ref = DictionaryName::FfiRef;
        DictionaryExpr::FromRoot(DictionaryExpr::Match(quote!(#ffi_ref { #conversions_from_ffi })).to_token_stream())
            .to_token_stream()
    }
    fn compose_interface_to(&self) -> TokenStream2 {
        let source = self.source_ref();
        let conversions_to_ffi = CommaPunctuated::from_iter(self.variant_composers.iter().map(|composer| {
            let attrs = composer.compose_attributes();
            Depunctuated::from_iter([quote!(#(#attrs)*), composer.compose_aspect(FFIAspect::To).present(&source)])
                .to_token_stream()
        }));

        InterfacesMethodExpr::Boxed(
            DictionaryExpr::Match(quote!(obj { #conversions_to_ffi, _ => unreachable!("Enum Variant unreachable") }))
                .to_token_stream())
            .to_token_stream()
    }
    fn compose_interface_destroy(&self) -> TokenStream2 {
        InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated()
    }
    // fn compose_interface_aspects(&self) -> (TokenStream2, TokenStream2, TokenStream2, Option<Generics>) {
    //     let (conversions_from_ffi, conversions_to_ffi): (CommaPunctuated<_>, CommaPunctuated<_>) = self.variant_composers.iter().map(|composer| {
    //         let source = self.source_ref();
    //         let attrs = composer.compose_attributes();
    //         (
    //             Depunctuated::from_iter([quote!(#(#attrs)*), composer.compose_aspect(FFIAspect::From).present(&source)])
    //                 .to_token_stream(),
    //             Depunctuated::from_iter([quote!(#(#attrs)*), composer.compose_aspect(FFIAspect::To).present(&source)])
    //                 .to_token_stream()
    //         )
    //     }).unzip();
    //     ({
    //         let ffi_ref = DictionaryName::FfiRef;
    //         DictionaryExpr::FromRoot(DictionaryExpr::Match(quote!(#ffi_ref { #conversions_from_ffi })).to_token_stream())
    //             .to_token_stream()
    //         },
    //      InterfacesMethodExpr::Boxed(
    //          DictionaryExpr::Match(quote!(obj { #conversions_to_ffi, _ => unreachable!("Enum Variant unreachable") }))
    //              .to_token_stream())
    //          .to_token_stream(),
    //      InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
    //      self.compose_generics())
    // }
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
                AttrsModel::from(attrs, target_name, scope),
                Context::Enum {
                    ident: target_name.clone(),
                    attrs: attrs.cfg_attributes(),
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
