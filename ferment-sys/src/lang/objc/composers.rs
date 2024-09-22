use std::fmt::{Display, Formatter};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Attribute;


#[derive(Clone, Debug, Default)]
pub struct AttrWrapper {
    pub attrs: Vec<Attribute>
    // #if defined(MYAPP_RELEASE) && defined(MyApp_Staging)
    // // ...
    // #else
    // // ...
    // #endif

    // #if DEBUG || RELEASE
    // let URL = "https://www.example.com/beta"
    // #elseif APPSTORE
    // let URL = "https://www.example.com/prod"
    // #endif
}

impl AttrWrapper {
    pub fn wrap<T: ToTokens>(&self, tokens: T) -> T {
        // feature stuff only
        match self.attrs.len() {
            0 => tokens,
            1 => {
                // let attr = self.attrs.first().unwrap();
                tokens
            }
            _ => tokens
        }
    }
}

impl ToTokens for AttrWrapper {
    fn to_tokens(&self, _tokens: &mut TokenStream) {

    }
}

impl Display for AttrWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let attrs = &self.attrs;
        f.write_str(format!("AttrWrapper({})", quote!(#(#attrs)*)).as_str())
    }
}

impl From<Vec<Attribute>> for AttrWrapper {
    fn from(attrs: Vec<Attribute>) -> Self {
        AttrWrapper { attrs }
    }
}

// impl LangAttrSpecification<ObjCFermentate> for AttrWrapper {
//     fn to_attrs(&self) -> AttrWrapper {
//         self.clone()

//     }
// }


// impl<'a> Composer<'a> for ArgsComposer {
//     type Source = (&'a str, &'a ScopeContext);
//     type Result = Depunctuated<super::presentation::Arg>;
//
//     fn compose(&self, (class_prefix, source): &'a Self::Source) -> Self::Result {
//         Depunctuated::new()
//     }
// }

// impl<'a> Composer<'a> for ArgsComposer {
//     type Source = (&'a str, &'a ScopeContext);
//     type Result = Depunctuated<super::presentation::Property>;
//
//     fn compose(&self, (class_prefix, source): &'a Self::Source) -> Self::Result {
//         Depunctuated::new()
//     }
// }



