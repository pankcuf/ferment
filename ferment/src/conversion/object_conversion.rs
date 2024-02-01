use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Item, parse_quote, Type};
use crate::composition::{TraitDecompositionPart1, TypeComposition};
use crate::conversion::{ScopeItemConversion, TypeConversion};
use crate::holder::PathHolder;


#[derive(Clone, PartialEq)]
pub enum ObjectConversion {
    Type(TypeConversion),
    Item(TypeConversion, ScopeItemConversion),
    Empty
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
                f.write_str(format!("Item({}, {})", scope, item).as_str()),
            ObjectConversion::Empty =>
                f.write_str("Empty"),
        }
    }
}

impl Display for ObjectConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ObjectConversion {
    pub fn type_conversion(&self) -> Option<&TypeConversion> {
        match self {
            ObjectConversion::Type(type_conversion) => Some(type_conversion),
            ObjectConversion::Item(scope, _item) => Some(scope),
            ObjectConversion::Empty => None
        }
    }
    pub fn ty(&self) -> Option<&Type> {
        match self {
            ObjectConversion::Type(type_conversion) => Some(type_conversion.ty()),
            ObjectConversion::Item(scope, _) => Some(scope.ty()),
            ObjectConversion::Empty => None
        }
    }
    pub fn as_scope(&self) -> PathHolder {
        let ty = self.ty();
        parse_quote!(#ty)
    }

}

impl TryFrom<&Item> for ObjectConversion {
    type Error = ();

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        match value {
            Item::Trait(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::Item(
                    TypeConversion::Trait(
                        TypeComposition::new(
                            parse_quote!(#ident),
                            Some(item.generics.clone())),
                        TraitDecompositionPart1::from_trait_items(ident, &item.items)),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Struct(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::Item(
                    TypeConversion::Object(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone()))),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Enum(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::Item(
                    TypeConversion::Object(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone()))),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Type(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::Item(
                    TypeConversion::Object(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone()))),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Impl(item) => {
                let ty = &item.self_ty;
                Ok(ObjectConversion::Item(
                    TypeConversion::Object(
                        TypeComposition::new(*ty.clone(), Some(item.generics.clone()))),
                    ScopeItemConversion::Item(value.clone())))
            },
            // Item::Fn(item) => {}
            // Item::Mod(_) => {}
            _ => Err(()),
        }
    }
}