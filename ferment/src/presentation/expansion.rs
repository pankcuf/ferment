use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use crate::holder::PathHolder;
use crate::presentation::{BindingPresentation, DropInterfacePresentation, TraitVTablePresentation};
use crate::presentation::conversion_interface_presentation::ConversionInterfacePresentation;
use crate::presentation::doc_presentation::DocPresentation;
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;
use crate::tree::ScopeTree;

/// Root-level composer chain
pub enum Expansion {
    Empty,
    Callback {
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
    },
    Function {
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
    },
    Full {
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
        conversion: ConversionInterfacePresentation,
        drop: DropInterfacePresentation,
        bindings: Vec<BindingPresentation>,
        traits: Vec<TraitVTablePresentation>,
    },
    Root {
        tree: ScopeTree,
    },
    Mod {
        directives: TokenStream2,
        name: TokenStream2,
        imports: Vec<PathHolder>,
        conversions: Vec<TokenStream2>
    },
    Impl {
        comment: DocPresentation,
        items: Vec<FFIObjectPresentation>,
    },
    Use {
        comment: DocPresentation,
    },
    Trait {
        comment: DocPresentation,
        vtable: FFIObjectPresentation,
        trait_object: FFIObjectPresentation,
    }
}

impl ToTokens for Expansion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let presentations = match self {
            Self::Empty | Self::Use { comment: _ } => vec![],
            Self::Impl { comment, items } => {
                let mut full = vec![comment.to_token_stream()];
                full.extend(items.iter().map(FFIObjectPresentation::to_token_stream));
                full
            },
            Self::Callback { comment, ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Function { comment, ffi_presentation } =>
                vec![comment.to_token_stream(), ffi_presentation.to_token_stream()],
            Self::Full { comment, ffi_presentation, conversion, drop, bindings, traits } => {
                let mut full = vec![comment.to_token_stream(), ffi_presentation.to_token_stream(), conversion.to_token_stream(), drop.to_token_stream()];
                full.extend(bindings.iter().map(BindingPresentation::to_token_stream));
                full.extend(traits.iter().map(TraitVTablePresentation::to_token_stream));
                full
            },
            Self::Mod { directives, name, imports: _, conversions } =>
                vec![
                    quote! {
                        #directives
                        pub mod #name {
                            //#(use #imports;)*
                            #(#conversions)*
                        }
                    }
                ],
            Self::Trait { comment, vtable, trait_object } =>
                vec![comment.to_token_stream(), vtable.to_token_stream(), trait_object.to_token_stream()],
            Self::Root { tree } =>
                vec![tree.to_token_stream()]
        };
        let expanded = quote!(#(#presentations)*);
        // println!("{}", expanded);
        expanded.to_tokens(tokens)
    }
}
