use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::{Attribute, ItemUse};
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composer::SourceFermentable;
use crate::context::ScopeContext;
use crate::lang::LangFermentable;
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{BindingPresentation, DocPresentation, FFIObjectPresentation, InterfacePresentation};
use crate::tree::ScopeTree;



#[non_exhaustive]
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Fermentate {
    Empty,
    Rust(RustFermentate),
    #[cfg(feature = "objc")]
    ObjC(crate::lang::objc::ObjCFermentate),
}


impl Default for Fermentate {
    fn default() -> Self {
        Self::Empty
    }
}

// impl ToTokens for Fermentate {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         match self {
//             Fermentate::Empty => {}
//             Fermentate::Rust(fermentate) => fermentate.to_tokens(tokens),
//             #[cfg(feature = "objc")]
//             Fermentate::ObjC(fermentate) => fermentate.to_tokens(tokens)
//         }
//     }
// }
// impl ScopeContextPresentable for Fermentate {
//     type Presentation = TokenStream2;
//
//     fn present(&self, _source: &ScopeContext) -> Self::Presentation {
//         self.to_token_stream()
//     }
// }
/// A result of fermentation
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum RustFermentate {
    Empty,
    TokenStream(TokenStream2),
    Function {
        comment: DocPresentation,
        binding: BindingPresentation,
    },
    Item {
        attrs: Vec<Attribute>,
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
        conversions: Depunctuated<InterfacePresentation>,
        bindings: Depunctuated<BindingPresentation>,
        traits: Depunctuated<RustFermentate>,
    },
    // CrateTree(CrateTree),
    ScopeTree(ScopeTree),
    Mod {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        imports: SemiPunctuated<ItemUse>,
        conversions: Depunctuated<RustFermentate>
    },
    Impl {
        comment: DocPresentation,
        items: Depunctuated<RustFermentate>,
        vtable: Option<BindingPresentation>
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
    },
    StaticVTable {
        vtable: BindingPresentation,
    },
    Generic {
        attrs: Vec<Attribute>,
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
        conversions: Depunctuated<InterfacePresentation>,
        drop: InterfacePresentation,
        bindings: Depunctuated<BindingPresentation>,
        traits: Depunctuated<RustFermentate>,
    },
    Root { mods: Depunctuated<RustFermentate> }
}

impl RustFermentate {

    pub fn mod_with(attrs: Vec<Attribute>, name: TokenStream2, imports: SemiPunctuated<ItemUse>, conversions: Depunctuated<RustFermentate>) -> Self {
        Self::Mod { attrs, name, imports, conversions }
    }
    pub fn types(attrs: &[Attribute], conversions: Depunctuated<RustFermentate>) -> Self {
        Self::mod_with(attrs.to_owned(), quote!(types), SemiPunctuated::new(), conversions)
    }
    pub fn generics(attrs: &[Attribute], imports: SemiPunctuated<ItemUse>, conversions: Depunctuated<RustFermentate>) -> Self {
        Self::mod_with(attrs.to_owned(), quote!(generics), imports, conversions)
    }
}

impl Default for RustFermentate {
    fn default() -> Self {
        Self::Empty
    }
}

impl ToTokens for RustFermentate {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => {},
            Self::Root { mods} => {
                mods.to_tokens(tokens)
            },
            Self::TokenStream(stream) =>
                stream.to_tokens(tokens),
            Self::Impl { comment, items, vtable } => {
                vtable.to_tokens(tokens);
                comment.to_tokens(tokens);
                items.to_tokens(tokens);
            }
            Self::Function { comment, binding: ffi_presentation } => {
                comment.to_tokens(tokens);
                ffi_presentation.to_tokens(tokens);
            }
            Self::Item { attrs, comment, ffi_presentation, conversions, bindings, traits } => {
                let attrs = quote!(#(#attrs)*);
                comment.to_tokens(tokens);
                attrs.to_tokens(tokens);
                ffi_presentation.to_tokens(tokens);
                conversions.iter().for_each(|interface| {
                    attrs.to_tokens(tokens);
                    interface.to_tokens(tokens);
                });
                bindings.to_tokens(tokens);
                traits.to_tokens(tokens);
            },
            Self::Mod { attrs, name, imports , conversions } =>
                quote!(#(#attrs)* pub mod #name { #imports #conversions }).to_tokens(tokens),
            Self::Trait { comment, vtable, trait_object } => {
                comment.to_tokens(tokens);
                trait_object.to_tokens(tokens);
                vtable.to_tokens(tokens);
            },
            // Self::CrateTree(tree) =>
            //     <CrateTree as SourceFermentable<RustFermentate>>::ferment(tree)
            //         .to_tokens(tokens),
            Self::ScopeTree(tree) =>
                <ScopeTree as SourceFermentable<RustFermentate>>::ferment(tree)
                    .to_tokens(tokens),
            Self::TraitVTable { vtable, export, destructor } => {
                vtable.to_tokens(tokens);
                export.to_tokens(tokens);
                destructor.to_tokens(tokens);
            }
            Self::Generic { attrs: _, comment, ffi_presentation, conversions, drop, bindings, .. } => {
                comment.to_tokens(tokens);
                ffi_presentation.to_tokens(tokens);
                conversions.to_tokens(tokens);
                drop.to_tokens(tokens);
                bindings.to_tokens(tokens);
            }
            Self::StaticVTable { vtable } =>
                vtable.to_tokens(tokens),
        }
    }
}

impl LangFermentable for RustFermentate {}

impl ScopeContextPresentable for RustFermentate {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}