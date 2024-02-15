use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Item, parse_quote, Type};
use crate::composition::{TraitDecompositionPart1, TypeComposition};
use crate::conversion::{ScopeItemConversion, TypeConversion};
use crate::ext::ValueReplaceScenario;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ObjectConversion {
    Type(TypeConversion),
    Item(TypeConversion, ScopeItemConversion),
    // Constraint(TypeConversion, ScopeItemConstra),
    Empty
}

impl ValueReplaceScenario for ObjectConversion {
    fn should_replace_with(&self, other: &Self) -> bool {
        println!("ObjectConversion ::: should_replace_with:::: {}: {}", self, other);
        match (self, other) {
            // (ObjectConversion::Type(..), ObjectConversion::Item(..)) => true,
            (_, ObjectConversion::Item(..)) => true,
            _ => false
        }
    }
}



impl ToTokens for ObjectConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty().to_tokens(tokens)
    }
}
impl Debug for ObjectConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectConversion::Type(tc) =>
                f.write_str(format!("Type({})", tc).as_str()),
            ObjectConversion::Item(tc, item) =>
                f.write_str(format!("Item({}, {})", tc, item).as_str()),
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
    pub fn new_item(ty: TypeConversion, item: ScopeItemConversion) -> ObjectConversion {
        ObjectConversion::Item(ty, item)
    }
    pub fn new_obj_item(ty: TypeComposition, item: ScopeItemConversion) -> ObjectConversion {
        ObjectConversion::Item(TypeConversion::Object(ty), item)
    }
    pub fn new_type(ty: TypeConversion) -> ObjectConversion {
        ObjectConversion::Type(ty)
    }
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
}

impl TryFrom<&Item> for ObjectConversion {
    type Error = ();

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        match value {
            Item::Trait(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_item(
                    TypeConversion::Trait(
                        TypeComposition::new(
                            parse_quote!(#ident),
                            Some(item.generics.clone())),
                        TraitDecompositionPart1::from_trait_items(ident, &item.items)),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Struct(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone())),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Enum(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone())),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Type(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone())),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Impl(item) => {
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(*(&item.self_ty).clone(), Some(item.generics.clone())),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Fn(item) => {
                let ident = &item.sig.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.sig.generics.clone())),
                    ScopeItemConversion::Item(value.clone())))
                    // ScopeItemConversion::Fn(value.clone())))
            },
            Item::Mod(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_item(
                    TypeConversion::Unknown(
                        TypeComposition::new(parse_quote!(#ident), None)),
                    ScopeItemConversion::Item(value.clone())))

            }
            _ => Err(()),
        }
    }
}