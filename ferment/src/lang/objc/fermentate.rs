use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::Depunctuated;
use super::presentation::{ImplementationPresentation, InterfacePresentation};

#[derive(Clone, Debug)]
pub enum Fermentate {
    Empty,
    Item {
        header_name: String,
        imports: Depunctuated<TokenStream2>,
        interfaces: Depunctuated<InterfacePresentation>,
        implementations: Depunctuated<ImplementationPresentation>
    },
}

impl Default for Fermentate {
    fn default() -> Self {
        Self::Empty
    }
}


impl Display for Fermentate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Fermentate::Empty => {
                f.write_str("\n")
            },
            Fermentate::Item { header_name, imports, implementations, interfaces } => {
                f.write_str("#import <Foundation/Foundation.h>\n")?;
                f.write_str(format!("#import \"{}.h\"\n", header_name).as_str())?;
                for import in imports {
                    f.write_str(format!("#import \"{}.h\"\n", import).as_str())?;
                }
                f.write_str("NS_ASSUME_NONNULL_BEGIN\n")?;
                f.write_str(interfaces.to_token_stream().to_string().as_str())?;
                f.write_str(implementations.to_token_stream().to_string().as_str())?;
                f.write_str("\nNS_ASSUME_NONNULL_END\n")
            }
            // Fermentate::CrateTree() => f.write_str("")
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
                interfaces: _,
                implementations: _ } => {
                quote! {

                }
            }
            Fermentate::Empty => quote!()
        }.to_tokens(tokens)
    }
}

// impl From<super::composers::ItemComposer> for Fermentate {
//     fn from(value: super::composers::ItemComposer) -> Self {
//         todo!()
//     }
// }