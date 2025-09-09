use syn::{Generics, Item, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, Signature};
use crate::context::GenericChain;
use crate::ext::{create_generics_chain_exact, GenericBoundKey};
use crate::kind::ScopeItemKind;

pub trait MaybeGenerics {
    fn maybe_generics(&self) -> Option<&Generics>;
    fn maybe_generic_bound_for_path(&self, path: &GenericBoundKey) -> Option<(Generics, GenericChain)> {
        self.maybe_generics()
            .and_then(|generics| {
                let chain = create_generics_chain_exact(generics, path);
                (!chain.inner.is_empty()).then(|| (generics.clone(), chain))
            })
    }
}
impl MaybeGenerics for Signature {
    fn maybe_generics(&self) -> Option<&Generics> {
        Some(&self.generics)
    }
}

impl MaybeGenerics for Item {
    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            Item::Enum(ItemEnum { generics, .. }) |
            Item::Fn(ItemFn { sig: Signature { generics, .. }, .. }) |
            Item::Impl(ItemImpl { generics, .. }) |
            Item::Struct(ItemStruct { generics, .. }) |
            Item::Trait(ItemTrait { generics, .. }) |
            Item::TraitAlias(ItemTraitAlias { generics, .. }) |
            Item::Type(ItemType { generics, .. }) |
            Item::Union(ItemUnion { generics, .. }) =>
                Some(generics),
            _ => None
        }
    }
}

impl MaybeGenerics for ScopeItemKind {
    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            ScopeItemKind::Item(item, ..) => item.maybe_generics(),
            ScopeItemKind::Fn(sig, ..) => sig.maybe_generics()
        }
    }
}
