use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, Generics, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemImpl, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, Path, PathSegment, Signature, Type};
use crate::context::{Scope, ScopeChain, ScopeInfo};
use crate::ext::Join;
use crate::kind::ScopeItemKind;

pub trait GetScopeTreeID {
    fn scope_tree_id(&self) -> ScopeTreeID;
}

impl GetScopeTreeID for Item {
    fn scope_tree_id(&self) -> ScopeTreeID {
        match self {
            Item::Const(ItemConst { ident, .. }, ..) |
            Item::Enum(ItemEnum { ident, .. }, ..) |
            Item::ExternCrate(ItemExternCrate { ident, .. }) |
            Item::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) |
            Item::Mod(ItemMod { ident, .. }, ..) |
            Item::Struct(ItemStruct { ident, .. }, ..) |
            Item::Static(ItemStatic { ident, .. }, ..) |
            Item::Trait(ItemTrait { ident, .. }, ..) |
            Item::TraitAlias(ItemTraitAlias { ident, ..  }) |
            Item::Type(ItemType { ident, .. }, ..) |
            Item::Union(ItemUnion { ident, .. }) =>
                ScopeTreeID::Ident(ident.clone()),
            Item::Impl(ItemImpl { self_ty, trait_, generics, .. }, ..) =>
                ScopeTreeID::Impl(*self_ty.clone(), trait_.clone().map(|(_, path, _)| path), generics.clone()),
            item => panic!("ScopeTreeExportID Not supported for {}", quote!(#item)),
        }
    }
}
impl GetScopeTreeID for Signature {
    fn scope_tree_id(&self) -> ScopeTreeID {
        ScopeTreeID::Ident(self.ident.clone())
    }
}

impl GetScopeTreeID for ScopeItemKind {
    fn scope_tree_id(&self) -> ScopeTreeID {
        match self {
            ScopeItemKind::Item(item, ..) => item.scope_tree_id(),
            ScopeItemKind::Fn(sig, ..) => sig.scope_tree_id()
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ScopeTreeID {
    Ident(Ident),
    Impl(Type, Option<Path>, Generics)
}

impl From<&PathSegment> for ScopeTreeID {
    fn from(value: &PathSegment) -> Self {
        Self::from_ident(&value.ident)
    }
}

impl std::fmt::Debug for ScopeTreeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident) =>
                f.write_str(format!("{ident}").as_str()),
            Self::Impl(ty, path, generics) =>
                f.write_str(format!("Impl({}, {}, {})", ty.to_token_stream(), path.to_token_stream(), generics.to_token_stream()).as_str())
        }
    }
}

impl std::fmt::Display for ScopeTreeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeTreeID {
    pub fn from_ident(ident: &Ident) -> Self {
        Self::Ident(ident.clone())
    }

    pub fn create_child_scope(&self, scope: &ScopeChain, attrs: Vec<Attribute>) -> ScopeChain {
        match &self {
            Self::Ident(ident) =>
                ScopeChain::r#mod(ScopeInfo::new(attrs, scope.crate_ident_ref().clone(), Scope::empty(scope.self_path_ref().joined(ident))), scope.clone()),
            Self::Impl(..) =>
                panic!("impl not implemented")
        }
    }
}

