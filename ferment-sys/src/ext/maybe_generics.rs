use proc_macro2::Ident;
use syn::{GenericParam, Generics, Item, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, PathSegment, Signature, TypeParam};
use crate::ext::GenericBoundKey;
use crate::kind::ScopeItemKind;

pub trait MaybeGenerics {
    fn maybe_generics(&self) -> Option<&Generics>;
    fn maybe_generic_bound_for_path(&self, path: &GenericBoundKey) -> Option<(Generics, TypeParam)> {
        self.maybe_generics()
            .and_then(|generics|
                maybe_generic_type_bound(path, generics)
                    .map(|bound| (generics.clone(), bound.clone())))
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

fn maybe_generic_type_bound<'a>(key: &'a GenericBoundKey, generics: &'a Generics) -> Option<&'a TypeParam> {
    match key {
        GenericBoundKey::Ident(ident) => maybe_generic_type_for_ident(ident, generics),
        GenericBoundKey::Path(path) => path.segments.last()
            .and_then(|PathSegment { ident, .. }| maybe_generic_type_for_ident(ident, generics)),
    }

    // TODO: where
}

fn maybe_generic_type_for_ident<'a>(ident: &'a Ident, generics: &'a Generics) -> Option<&'a TypeParam> {
    generics.params.iter().find_map(|param| match param {
        GenericParam::Type(type_param) =>
            type_param.ident.eq(ident).then_some(type_param),
        _ => None
    })
}
