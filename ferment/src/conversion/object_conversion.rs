use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Item, parse_quote, Type};
use syn::punctuated::Punctuated;
use crate::composition::{TraitDecompositionPart1, TypeComposition};
use crate::conversion::{ScopeItemConversion, TypeCompositionConversion};
use crate::ext::ValueReplaceScenario;
use crate::helper::collect_bounds;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ObjectConversion {
    Type(TypeCompositionConversion),
    Item(TypeCompositionConversion, ScopeItemConversion),
    Empty
}

impl ValueReplaceScenario for ObjectConversion {
    fn should_replace_with(&self, other: &Self) -> bool {
        // println!("ObjectConversion ::: should_replace_with:::: {}: {}", self, other);
        match (self, other) {
            (_, ObjectConversion::Item(..)) => true,
            (ObjectConversion::Type(self_ty), ObjectConversion::Type(candidate_ty)) => {
                let should = !self_ty.is_refined() && candidate_ty.is_refined();
                // let should = !self_ty.is_refined() && candidate_ty.is_refined() || self_ty.is_tuple();
                //println!("MERGE? {} [{}]: {} [{}]: {}", should, self_ty.is_refined(), self_ty, candidate_ty.is_refined(), candidate_ty);
                should
            }
            _ => false
        }
    }

}



impl ToTokens for ObjectConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.to_ty().to_tokens(tokens)
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
    pub fn replace_composition_type(&mut self, with_ty: Type) {
        match self {
            ObjectConversion::Type(ty) => ty.replace_composition_type(with_ty),
            // actually it has no sense since items can never be imported where they are defined
            ObjectConversion::Item(ty, _) => ty.replace_composition_type(with_ty),
            ObjectConversion::Empty => {}
        }
    }

    pub fn new_item(ty: TypeCompositionConversion, item: ScopeItemConversion) -> ObjectConversion {
        ObjectConversion::Item(ty, item)
    }
    pub fn new_obj_item(ty: TypeComposition, item: ScopeItemConversion) -> ObjectConversion {
        ObjectConversion::Item(TypeCompositionConversion::Object(ty), item)
    }
    pub fn type_conversion(&self) -> Option<&TypeCompositionConversion> {
        match self {
            ObjectConversion::Type(type_conversion) => Some(type_conversion),
            ObjectConversion::Item(scope, _item) => Some(scope),
            ObjectConversion::Empty => None
        }
    }
    // pub fn ty(&self) -> Option<&Type> {
    //     match self {
    //         ObjectConversion::Type(type_conversion) => Some(type_conversion.ty()),
    //         ObjectConversion::Item(scope, _) => Some(scope.ty()),
    //         ObjectConversion::Empty => None
    //     }
    // }
    pub fn to_ty(&self) -> Option<Type> {
        match self {
            ObjectConversion::Type(type_conversion) => Some(type_conversion.to_ty()),
            ObjectConversion::Item(scope, _) => Some(scope.to_ty()),
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
                    TypeCompositionConversion::Trait(
                        TypeComposition::new(
                            parse_quote!(#ident),
                            Some(item.generics.clone()), Punctuated::new()),
                        TraitDecompositionPart1::from_trait_items(ident, &item.items), collect_bounds(&item.supertraits)),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Struct(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Enum(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Type(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Impl(item) => {
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(*(&item.self_ty).clone(), Some(item.generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone())))
            },
            Item::Fn(item) => {
                let ident = &item.sig.ident;
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(parse_quote!(#ident), Some(item.sig.generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone())))
                    // ScopeItemConversion::Fn(value.clone())))
            },
            Item::Mod(item) => {
                let ident = &item.ident;
                Ok(ObjectConversion::new_item(
                    TypeCompositionConversion::Unknown(
                        TypeComposition::new(parse_quote!(#ident), None, Punctuated::new())),
                    ScopeItemConversion::Item(value.clone())))

            }
            _ => Err(()),
        }
    }
}
