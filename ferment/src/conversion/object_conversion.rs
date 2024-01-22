use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Item, parse_quote, Type};
use crate::conversion::{TypeConversion};
use crate::formatter::format_token_stream;
use crate::holder::PathHolder;

#[derive(Clone)]
pub enum ObjectConversion {
    Type(TypeConversion),
    Item(TypeConversion, Item),
}


impl ToTokens for ObjectConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty().to_tokens(tokens)
    }
}
impl Debug for ObjectConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectConversion::Type(ty) =>
                f.write_str(format!("Type({})", ty).as_str()),
            ObjectConversion::Item(scope, item) =>
                f.write_str(format!("Item({}, {})", scope, format_token_stream(item)).as_str()),
        }
    }
}

impl Display for ObjectConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ObjectConversion {
    pub fn type_conversion(&self) -> &TypeConversion {
        match self {
            ObjectConversion::Type(type_conversion) => type_conversion,
            ObjectConversion::Item(scope, _item) => scope
        }
    }
    pub fn ty(&self) -> &Type {
        match self {
            ObjectConversion::Type(type_conversion) => type_conversion.ty(),
            ObjectConversion::Item(scope, _) => scope.ty()
        }
    }
    pub fn as_scope(&self) -> PathHolder {
        let ty = self.ty();
        parse_quote!(#ty)
    }
}
