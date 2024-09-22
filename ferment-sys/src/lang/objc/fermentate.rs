use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuatedTokens, Depunctuated, SemiPunctuated, SemiPunctuatedTokens};
use crate::composer::SourceFermentable;
use crate::lang::objc::ObjCFermentate;
use crate::tree::{CrateTree, ScopeTree};
use super::presentation::{ArgPresentation, ImplementationPresentation, InterfacePresentation, Property};

#[derive(Clone, Debug)]
pub struct InterfaceImplementation {
    pub interface: InterfacePresentation,
    pub implementation: ImplementationPresentation,
}

impl InterfaceImplementation {
    pub fn default(
        objc_name: TokenStream2,
        c_name: TokenStream2,
        properties: SemiPunctuated<Property>,
        properties_inits: SemiPunctuated<Property>
    ) -> Self {
        Self {
            interface: InterfacePresentation::Default {
                name: objc_name.clone(),
                c_type: c_name.clone(),
                properties,
            },
            implementation: ImplementationPresentation::Default {
                objc_name: objc_name.clone(),
                c_type: c_name.clone(),
                properties_inits,
            },
        }
    }
    pub fn c(
        objc_name: TokenStream2,
        c_name: TokenStream2,
        property_ctors: SemiPunctuatedTokens,
        property_dtors: SemiPunctuatedTokens
    ) -> Self {
        Self {
            interface: InterfacePresentation::C {
                name: objc_name.clone(),
                c_type: c_name.clone()
            },
            implementation: ImplementationPresentation::C {
                objc_name: objc_name.clone(),
                c_type: c_name.clone(),
                property_ctors,
                property_dtors
            },
        }
    }
    pub fn rust(
        objc_name: TokenStream2,
        c_name: TokenStream2,
        property_names: CommaPunctuatedTokens,
        property_ctors: SemiPunctuatedTokens
    ) -> Self {
        Self {
            interface: InterfacePresentation::Rust {
                name: objc_name.clone(),
                c_type: c_name.clone()
            },
            implementation: ImplementationPresentation::Rust {
                objc_name: objc_name.clone(),
                c_type: c_name.clone(),
                c_var: quote!(struct #c_name *),
                property_names,
                property_ctors,
            },
        }
    }
    pub fn args(
        objc_name: TokenStream2,
        c_name: TokenStream2,
        args: Depunctuated<ArgPresentation>,
        prop_implementations: Depunctuated<TokenStream2>
    ) -> Self {
        InterfaceImplementation {
            interface: InterfacePresentation::Args {
                name: objc_name.clone(),
                c_type: c_name.clone(),
                args,
            },
            implementation: ImplementationPresentation::Args {
                objc_name: objc_name.clone(),
                prop_implementations,
            },
        }
    }
}

impl ToTokens for InterfaceImplementation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.interface.to_tokens(tokens);
        self.implementation.to_tokens(tokens);
    }
}

impl Display for InterfaceImplementation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.interface.to_token_stream().to_string().as_str())?;
        f.write_str(self.implementation.to_token_stream().to_string().as_str())
    }
}


#[derive(Clone, Debug)]
pub enum Fermentate {
    Empty,
    TokenStream(TokenStream2),
    Item {
        header_name: String,
        imports: Depunctuated<TokenStream2>,
        implementations: Depunctuated<InterfaceImplementation>
    },
    ScopeTree(ScopeTree),
    CrateTree(CrateTree),
}

impl Default for Fermentate {
    fn default() -> Self {
        Self::Empty
    }
}


impl Display for Fermentate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => {
                f.write_str("\n")
            },
            Self::Item { header_name, imports, implementations } => {
                f.write_str("#import <Foundation/Foundation.h>\n")?;
                f.write_str(format!("#import \"{}.h\"\n", header_name).as_str())?;
                for import in imports {
                    f.write_str(format!("#import \"{}.h\"\n", import).as_str())?;
                }
                f.write_str("NS_ASSUME_NONNULL_BEGIN\n")?;
                for i in implementations {
                    f.write_str(i.to_string().as_str())?
                }
                f.write_str("\nNS_ASSUME_NONNULL_END\n")
            }
            Self::TokenStream(token_stream) =>
                f.write_str(token_stream.to_string().as_str()),
            Self::ScopeTree(tree) =>
                f.write_str(<ScopeTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_token_stream().to_string().as_str()),
            Self::CrateTree(tree) =>
                f.write_str(<CrateTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_token_stream().to_string().as_str()),

        }
    }
}
impl Fermentate {
    pub fn objc_files(&self) -> Vec<String> {
        vec!["objc_wrapper.m".to_string(), "objc_wrapper.h".to_string()]
    }
}

impl ToTokens for Fermentate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Fermentate::Item {
                header_name: _,
                imports: _,
                implementations: _ } => {
                // quote! {
                //
                // }
            }
            Fermentate::Empty => {},
            Fermentate::TokenStream(token_stream) =>
                token_stream
                    .to_tokens(tokens),
            Fermentate::ScopeTree(tree) =>
                <ScopeTree as SourceFermentable<ObjCFermentate>>::ferment(tree)
                    .to_tokens(tokens),
            Fermentate::CrateTree(tree) =>
                <CrateTree as SourceFermentable<ObjCFermentate>>::ferment(tree)
                    .to_tokens(tokens),
        }
    }
}

// impl From<super::composers::ItemComposer> for Fermentate {
//     fn from(value: super::composers::ItemComposer) -> Self {
//         todo!()
//     }
// }