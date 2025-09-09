use syn::{Attribute, Item, Signature};
use crate::kind::ScopeItemKind;

pub trait MaybeAttrs {
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>>;
}

impl MaybeAttrs for Item {
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            Item::Const(item) => Some(&item.attrs),
            Item::Enum(item) => Some(&item.attrs),
            Item::ExternCrate(item) => Some(&item.attrs),
            Item::Fn(item) => Some(&item.attrs),
            Item::ForeignMod(item) => Some(&item.attrs),
            Item::Impl(item) => Some(&item.attrs),
            Item::Macro(item) => Some(&item.attrs),
            Item::Mod(item) => Some(&item.attrs),
            Item::Static(item) => Some(&item.attrs),
            Item::Struct(item) => Some(&item.attrs),
            Item::Trait(item) => Some(&item.attrs),
            Item::TraitAlias(item) => Some(&item.attrs),
            Item::Type(item) => Some(&item.attrs),
            Item::Union(item) => Some(&item.attrs),
            Item::Use(item) => Some(&item.attrs),
            _ => None,
        }
    }
}

impl MaybeAttrs for Signature {
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        None
    }
}


impl MaybeAttrs for ScopeItemKind {
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            ScopeItemKind::Item(item, ..) => item.maybe_attrs(),
            ScopeItemKind::Fn(sig, ..) => sig.maybe_attrs()
        }
    }
}
