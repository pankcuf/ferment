use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Item, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ParenthesizedGenericArguments, Signature, Type};
use syn::punctuated::Punctuated;
use crate::ast::{CommaPunctuated, PathHolder};
use crate::composable::{TraitDecompositionPart1, TypeComposition};
use crate::conversion::{ScopeItemConversion, TypeCompositionConversion};
use crate::ext::{collect_bounds, ResolveAttrs, ToType, ValueReplaceScenario};
use crate::presentation::Name;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ObjectConversion {
    Type(TypeCompositionConversion),
    Item(TypeCompositionConversion, ScopeItemConversion),
    Empty
}

impl ObjectConversion {
    pub fn is_type(&self, ty: &Type) -> bool {
        match self {
            ObjectConversion::Type(conversion) |
            ObjectConversion::Item(conversion, _) =>
                ty.eq(conversion.ty()),
            ObjectConversion::Empty => false
        }
    }
    pub fn is_refined(&self) -> bool {
        match self {
            ObjectConversion::Type(conversion) => conversion.is_refined(),
            _ => true
        }
    }
    pub fn maybe_callback<'a>(&'a self) -> Option<&'a ParenthesizedGenericArguments> {
        match self {
            ObjectConversion::Type(tyc) |
            ObjectConversion::Item(tyc, _) => tyc.maybe_callback(),
            ObjectConversion::Empty => None
        }
    }
    pub fn maybe_lambda_args(&self) -> Option<CommaPunctuated<Name>> {
        match self.maybe_callback() {
            Some(ParenthesizedGenericArguments { inputs, ..}) =>
                Some(CommaPunctuated::from_iter(inputs.iter().enumerate().map(|(index, _ty)| Name::UnnamedArg(index)))),
            _ => None
        }
    }
}

impl ValueReplaceScenario for ObjectConversion {
    fn should_replace_with(&self, other: &Self) -> bool {
        // println!("ObjectConversion ::: should_replace_with:::: {}: {}", self, other);
        match (self, other) {
            (_, ObjectConversion::Item(..)) => true,
            (ObjectConversion::Type(self_ty), ObjectConversion::Type(_candidate_ty)) => {
                // let should = !self_ty.is_refined() && candidate_ty.is_refined();
                let should = !self_ty.is_refined();
                // let should = !self_ty.is_refined() && candidate_ty.is_refined() || self_ty.is_tuple();
                // println!("MERGE? {} [{}]:\n\t {} [{}]: {}", should, self_ty.is_refined(), self_ty, candidate_ty.is_refined(), candidate_ty);
                should
            }
            _ => false
        }
    }

}



impl ToTokens for ObjectConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.maybe_type().to_tokens(tokens)
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
            ObjectConversion::Type(type_conversion) |
            ObjectConversion::Item(type_conversion, ..) => Some(type_conversion),
            ObjectConversion::Empty => None
        }
    }
    pub fn ty(&self) -> Option<&Type> {
        match self {
            ObjectConversion::Type(type_conversion) |
            ObjectConversion::Item(type_conversion, ..) => Some(type_conversion.ty()),
            ObjectConversion::Empty => None
        }
    }
    pub fn maybe_type_composition(&self) -> Option<&TypeComposition> {
        match self {
            ObjectConversion::Type(ty) |
            ObjectConversion::Item(ty, _) => Some(ty.ty_composition()),
            ObjectConversion::Empty => None
        }
    }
    pub fn maybe_type(&self) -> Option<Type> {
        match self {
            ObjectConversion::Type(ty) |
            ObjectConversion::Item(ty, _) => Some(ty.to_type()),
            ObjectConversion::Empty => None
        }
    }
}

impl TryFrom<(&Item, &PathHolder)> for ObjectConversion {
    type Error = ();

    fn try_from((value, scope): (&Item, &PathHolder)) -> Result<Self, Self::Error> {
        match value {
            Item::Trait(ItemTrait { ident, generics, items, supertraits, .. }) => {
                Ok(ObjectConversion::new_item(
                    TypeCompositionConversion::Trait(
                        TypeComposition::new(ident.to_type(), Some(generics.clone()), Punctuated::new()),
                        TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits)),
                    ScopeItemConversion::Item(value.clone(), scope.clone())))
            },
            Item::Struct(ItemStruct { ident, generics, .. }) => {
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(ident.to_type(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone(), scope.clone())))
            },
            Item::Enum(ItemEnum { ident, generics, .. }) => {
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(ident.to_type(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone(), scope.clone())))
            },
            Item::Type(ItemType { ident, generics, ty, .. }) => {
                let conversion = ScopeItemConversion::Item(value.clone(), scope.clone());
                let obj = match &**ty {
                    Type::BareFn(..) => {
                        ObjectConversion::Item(TypeCompositionConversion::FnPointer(TypeComposition::new(ident.to_type(), Some(generics.clone()), Punctuated::new())), conversion)
                    },
                    _ => ObjectConversion::new_obj_item(TypeComposition::new(ident.to_type(), Some(generics.clone()), Punctuated::new()), conversion)
                };
                Ok(obj)
            },
            Item::Const(ItemConst { ident, .. }) => {
                Ok(ObjectConversion::new_obj_item(
                    TypeComposition::new(ident.to_type(), None, Punctuated::new()),
                    ScopeItemConversion::Item(value.clone(), scope.clone())))
            },
            Item::Impl(ItemImpl { self_ty, generics, .. }) => {
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(*self_ty.clone(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone(), scope.clone())))
            },
            Item::Fn(ItemFn { sig: Signature { ident, generics, .. }, .. }) => {
                Ok(ObjectConversion::new_obj_item(
                        TypeComposition::new(ident.to_type(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemConversion::Item(value.clone(), scope.clone())))
                    // ScopeItemConversion::Fn(value.clone())))
            },
            Item::Mod(ItemMod { ident, .. }) => {
                Ok(ObjectConversion::new_item(
                    TypeCompositionConversion::Unknown(
                        TypeComposition::new(ident.to_type(), None, Punctuated::new())),
                    ScopeItemConversion::Item(value.clone(), scope.clone())))

            }
            _ => Err(()),
        }
    }
}

impl ResolveAttrs for ObjectConversion {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>> {
        match self {
            ObjectConversion::Item(_, item) =>
                item.resolve_attrs(),
            _ => vec![],
        }
    }
}