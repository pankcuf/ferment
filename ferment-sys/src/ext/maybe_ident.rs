use syn::{Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemImpl, ItemMacro, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, Path, Signature, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::ToTokens;
use crate::ast::AddPunctuated;
use crate::kind::ScopeItemKind;

pub trait MaybeIdent {
    fn maybe_ident(&self) -> Option<&Ident>;
    fn ident_string(&self) -> String {
        self.maybe_ident().map_or("(None)".to_string(), Ident::to_string)
    }
}
impl MaybeIdent for Item {
    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            Item::Const(ItemConst { ident, .. }) |
            Item::Enum(ItemEnum { ident, .. }) |
            Item::ExternCrate(ItemExternCrate { ident, .. }) |
            Item::Fn(ItemFn { sig: Signature { ident, .. }, .. }) |
            Item::Mod(ItemMod { ident, .. }) |
            Item::Struct(ItemStruct { ident, ..  }) |
            Item::Static(ItemStatic { ident, ..  }) |
            Item::Trait(ItemTrait { ident, ..  }) |
            Item::TraitAlias(ItemTraitAlias { ident, ..  }) |
            Item::Type(ItemType { ident, .. }) |
            Item::Union(ItemUnion { ident, .. }) => Some(ident),
            Item::Macro(ItemMacro { ident, .. }) => ident.as_ref(),
            Item::Impl(ItemImpl { self_ty, .. }) => type_ident_ref(self_ty),
            _ => None
        }
    }
}
impl MaybeIdent for Signature {
    fn maybe_ident(&self) -> Option<&Ident> {
        Some(&self.ident)
    }
}
impl MaybeIdent for ScopeItemKind {
    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            ScopeItemKind::Item(item, ..) => item.maybe_ident(),
            ScopeItemKind::Fn(sig, ..) => sig.maybe_ident()
        }
    }
}


pub fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}

pub fn collect_bounds(bounds: &AddPunctuated<TypeParamBound>) -> Vec<Path> {
    bounds.iter().filter_map(|bound| match bound {
        TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.clone()),
        _ => None
    }).collect()
}
fn path_ident_ref(path: &Path) -> Option<&Ident> {
    path.segments.last().map(|last_segment| &last_segment.ident)
}

fn type_ident_ref(ty: &Type) -> Option<&Ident> {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path_ident_ref(path),
        Type::Reference(TypeReference { elem, .. }) |
        Type::Ptr(TypePtr { elem, .. }) =>
            type_ident_ref(elem),
        Type::TraitObject(TypeTraitObject { bounds, .. }) => bounds.iter().find_map(|b| match b {
            TypeParamBound::Trait(TraitBound { path, ..}) => path_ident_ref(path),
            _ => None
        }),
        Type::Array(TypeArray { elem, .. }) => type_ident_ref(elem),
        _ => None
    }
}

