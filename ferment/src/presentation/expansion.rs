use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Attribute, ItemUse};
use crate::ast::{Depunctuated, Directives, SemiPunctuated};
use crate::context::ScopeContext;
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, FFIObjectPresentation, InterfacePresentation};
use crate::tree::CrateTree;

/// Root-level composer chain
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum Expansion {
    Empty,
    TokenStream(TokenStream2),
    Function {
        comment: DocPresentation,
        binding: BindingPresentation,
    },
    Full {
        attrs: Vec<Attribute>,
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
        conversion: InterfacePresentation,
        drop: DropInterfacePresentation,
        bindings: Depunctuated<BindingPresentation>,
        traits: Directives,
    },
    Root {
        tree: CrateTree,
    },
    Mod {
        attrs: Directives,
        directives: TokenStream2,
        name: TokenStream2,
        imports: SemiPunctuated<ItemUse>,
        conversions: Depunctuated<TokenStream2>
    },
    Impl {
        comment: DocPresentation,
        items: Directives,
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
            Self::TokenStream(tokens) =>
                vec![tokens.to_token_stream()],
            Self::Impl { comment, items } =>
                vec![comment.to_token_stream(), items.to_token_stream()],
            Self::Function { comment, binding: ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Full { attrs, comment, ffi_presentation, conversion, drop, bindings, traits } =>
                vec![comment.to_token_stream(), quote!(#(#attrs)*), ffi_presentation.to_token_stream(), quote!(#(#attrs)*), conversion.to_token_stream(), quote!(#(#attrs)*), drop.to_token_stream(), bindings.to_token_stream(), traits.to_token_stream()],
            Self::Mod { attrs, directives, name, imports , conversions } =>
                vec![quote!(#attrs #directives pub mod #name { #imports #conversions })],
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

impl ScopeContextPresentable for Expansion {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}