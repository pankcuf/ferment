use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Semi;
use syn::ItemUse;
use crate::composer::Depunctuated;
use crate::presentation::{BindingPresentation, DropInterfacePresentation};
use crate::presentation::conversion_interface_presentation::InterfacePresentation;
use crate::presentation::doc_presentation::DocPresentation;
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;
use crate::tree::CrateTree;

/// Root-level composer chain
pub enum Expansion {
    Empty,
    Function {
        comment: DocPresentation,
        binding: BindingPresentation,
    },
    Full {
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
        conversion: InterfacePresentation,
        drop: DropInterfacePresentation,
        bindings: Depunctuated<BindingPresentation>,
        traits: Depunctuated<Expansion>,
    },
    Root {
        tree: CrateTree,
    },
    Mod {
        directives: TokenStream2,
        name: TokenStream2,
        imports: Punctuated<ItemUse, Semi>,
        conversions: Depunctuated<TokenStream2>
    },
    Impl {
        comment: DocPresentation,
        items: Depunctuated<Expansion>,
    },
    Trait {
        comment: DocPresentation,
        vtable: FFIObjectPresentation,
        trait_object: FFIObjectPresentation,
    },
    TraitVTable {
        vtable: BindingPresentation,
        export: BindingPresentation,
        destructor: BindingPresentation,
    }
}

impl ToTokens for Expansion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let presentations = match self {
            Self::Empty => vec![],
            Self::Impl { comment, items } =>
                vec![comment.to_token_stream(), items.to_token_stream()],
            Self::Function { comment, binding: ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Full { comment, ffi_presentation, conversion, drop, bindings, traits } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream(), conversion.to_token_stream(), drop.to_token_stream(), bindings.to_token_stream(), traits.to_token_stream()],
            Self::Mod { directives, name, imports , conversions } =>
                vec![quote!(#directives pub mod #name { #imports #conversions })],
            Self::Trait { comment, vtable, trait_object } =>
                vec![comment.to_token_stream(), vtable.to_token_stream(), trait_object.to_token_stream()],
            Self::Root { tree } =>
                vec![tree.to_token_stream()],
            Expansion::TraitVTable { vtable, export, destructor } =>
                vec![vtable.to_token_stream(), export.to_token_stream(), destructor.to_token_stream()]
        };
        quote!(#(#presentations)*).to_tokens(tokens)
    }
}
