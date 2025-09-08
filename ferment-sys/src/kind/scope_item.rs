use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Attribute, Item, ItemEnum, ItemStruct, ItemTrait, ItemType, Path, Signature, Type};
use syn::__private::TokenStream2;
use crate::composable::{CfgAttributes, TraitDecompositionPart1, TraitModel, TypeModel};
use crate::kind::{TypeModelKind};
use crate::ext::{collect_bounds, MaybeIdent, MaybeAttrs, ResolveAttrs};
use crate::formatter::format_token_stream;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ScopeItemKind {
    Item(Item, Path),
    Fn(Signature, Path),
}

impl ToTokens for ScopeItemKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ScopeItemKind::Item(item, ..) => item.to_tokens(tokens),
            ScopeItemKind::Fn(sig, ..) => sig.to_tokens(tokens)
        }
    }
}
impl Debug for ScopeItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItemKind::Item(item, scope) =>
                f.write_fmt(format_args!("Item({}, {})", format_token_stream(item.maybe_ident()), scope.to_token_stream())),
            ScopeItemKind::Fn(Signature { ident, .. }, scope) =>
                f.write_fmt(format_args!("Fn({}, {})", format_token_stream(&ident), scope.to_token_stream())),
        }
    }
}

impl Display for ScopeItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ResolveAttrs for ScopeItemKind {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>> {
        self.maybe_attrs()
            .map(CfgAttributes::cfg_attributes_or_none)
            .unwrap_or_default()
    }
}

impl ScopeItemKind {
    pub fn item(item: Item, scope: Path) -> Self {
        Self::Item(item, scope)
    }
    pub fn r#fn(sig: Signature, scope: Path) -> Self {
        Self::Fn(sig, scope)
    }
    pub fn item_ref(item: &Item, scope: &Path) -> Self {
        Self::item(item.clone(), scope.clone())
    }
    pub fn fn_ref(sig: &Signature, scope: &Path) -> Self {
        Self::Fn(sig.clone(), scope.clone())
    }

    pub fn item_enum(item_enum: &ItemEnum, scope: &Path) -> Self {
        Self::item(Item::Enum(item_enum.clone()), scope.clone())
    }
    pub fn item_struct(item_struct: &ItemStruct, scope: &Path) -> Self {
        Self::item(Item::Struct(item_struct.clone()), scope.clone())
    }
    pub fn item_type(item_type: &ItemType, scope: &Path) -> Self {
        Self::item(Item::Type(item_type.clone()), scope.clone())
    }
    pub fn item_trait(item_trait: &ItemTrait, scope: &Path) -> Self {
        Self::item(Item::Trait(item_trait.clone()), scope.clone())
    }
}
impl ScopeItemKind {
    pub fn update_with(&self, ty_to_replace: TypeModel) -> Option<TypeModelKind> {
        match self {
            ScopeItemKind::Item(item, ..) => match item {
                Item::Trait(ItemTrait { ident, items, supertraits, .. }) =>
                    Some(TypeModelKind::Trait(TraitModel::new(ty_to_replace, TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits)))),
                Item::Enum(..) |
                Item::Struct(..) |
                Item::Fn(..) |
                Item::Impl(..) =>
                    Some(TypeModelKind::Object(ty_to_replace)),
                Item::Type(ty) => Some(match &*ty.ty {
                    Type::BareFn(..) =>
                        TypeModelKind::FnPointer(ty_to_replace),
                    _ =>
                        TypeModelKind::Object(ty_to_replace),
                }),
                _ => None
            }
            ScopeItemKind::Fn(..) => None
        }
    }
    pub fn scope(&self) -> &Path {
        match self {
            ScopeItemKind::Item(.., scope) |
            ScopeItemKind::Fn(.., scope) => scope
        }
    }
    pub fn path(&self) -> &Path {
        self.scope()
    }
}