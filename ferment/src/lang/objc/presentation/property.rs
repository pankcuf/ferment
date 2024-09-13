use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composable::FieldComposer;
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;


#[derive(Clone, Debug)]
pub enum Property {
    // Property(),
    // Method()
    NonatomicReadwrite { ty: TokenStream2, name: TokenStream2 },
    Initializer { field_name: TokenStream2, field_initializer: TokenStream2 }
}

impl Property {
    pub fn nonatomic_readwrite(composer: &FieldComposer<ObjCFermentate, AttrWrapper>) -> Self {
        Property::NonatomicReadwrite {
            ty: composer.ty().to_token_stream(),
            name: composer.name.to_token_stream()
        }
    }
    pub fn initializer(composer: &FieldComposer<ObjCFermentate, AttrWrapper>) -> Self {
        Property::Initializer {
            field_name: composer.tokenized_name(),
            field_initializer: composer.to_token_stream()
        }
    }
}

impl ToTokens for FieldComposer<ObjCFermentate, AttrWrapper> {
    #[allow(unused_variables)]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, kind, attrs, .. } = self;

        let template = quote! {
            //#ifdef SMTH
            //#(#attrs)*
            #name: #kind
            //#endif SMTH

        };
        template.to_tokens(tokens)
    }
}



impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Property::NonatomicReadwrite { ty, name } => {
                quote! {
                    @property (nonatomic, readwrite) #ty #name
                }
            }
            Property::Initializer { field_name, field_initializer } => {
                quote! {
                    self.#field_name = #field_initializer
                }
            }
        }.to_tokens(tokens)
    }
}

// #[derive(Clone, Debug)]
// pub enum MethodDeclaration {
//     InitWith { c_type: TokenStream2 }
// }

impl From<&FieldComposer<ObjCFermentate, AttrWrapper>> for Property {
    fn from(value: &FieldComposer<ObjCFermentate, AttrWrapper>) -> Self {
        Property::NonatomicReadwrite {
            ty: value.ty().to_token_stream(),
            name: value.name.to_token_stream()
        }
    }
}