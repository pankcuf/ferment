use std::collections::HashSet;
use syn::{Attribute, Fields, FieldsNamed, FieldsUnnamed, FnArg, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, Item, ItemMod, ItemType, Meta, parse_quote, Path, PatType, ReturnType, Signature, TraitItem, TraitItemConst, TraitItemFn, TraitItemType, Type, Variant, TypeParamBound, TraitBound};
use syn::parse::Parser;
use crate::ast::{AddPunctuated, CommaPunctuated};
use crate::composable::GenericBoundsModel;
use crate::composer::MaybeMacroLabeled;
use crate::kind::ScopeItemKind;
use crate::ext::UniqueNestedItems;

#[allow(unused)]
pub struct MacroAttributes {
    pub path: Path,
    pub arguments: Vec<Path>,
}

pub trait TypeCollector {
    fn collect_compositions(&self) -> HashSet<Type>;
}
fn handle_attributes_with_handler<F: FnMut(MacroAttributes)>(attrs: &[Attribute], mut handler: F) {
    attrs.iter()
        .for_each(|attr| if attr.is_labeled_for_export() || attr.is_labeled_for_opaque_export() {
            let mut arguments = Vec::<Path>::new();
            if let Meta::List(meta_list) = &attr.meta {
                if let Ok(nested) = CommaPunctuated::<Meta>::parse_terminated.parse2(meta_list.tokens.clone()) {
                    for meta_item in nested.iter() {
                        if let Meta::Path(path) = meta_item {
                            arguments.push(path.clone());
                        }
                    }
                }
            }
            handler(MacroAttributes { path: attr.path().clone(), arguments })
        })
}

impl TypeCollector for AddPunctuated<TypeParamBound> {
    fn collect_compositions(&self) -> HashSet<Type> {
        HashSet::from_iter(self.iter().flat_map(TypeParamBound::collect_compositions))
    }
}

impl TypeCollector for TypeParamBound {
    fn collect_compositions(&self) -> HashSet<Type> {
        match self {
            TypeParamBound::Trait(trait_bound) => trait_bound.collect_compositions(),
            _ => HashSet::default()
        }
    }
}

impl TypeCollector for TraitBound {
    fn collect_compositions(&self) -> HashSet<Type> {
        self.path.collect_compositions()
    }
}

impl TypeCollector for GenericBoundsModel {
    fn collect_compositions(&self) -> HashSet<Type> {
        let mut type_and_paths = HashSet::<Type>::new();
        self.bounds.iter()
            .for_each(|bound| {
                type_and_paths.insert(parse_quote!(dyn #bound));
            });
        type_and_paths
    }
}

impl TypeCollector for Item {
    fn collect_compositions(&self) -> HashSet<Type> {
        let mut type_and_paths = HashSet::<Type>::new();
        let mut cache_fields = |fields: &Fields, _attrs: &MacroAttributes| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter().for_each(|field| {
                    type_and_paths.insert(field.ty.clone());
                }),
            Fields::Unit => {}
        };
        match self {
            Item::Mod(ItemMod { content: Some((_, items)), .. }) =>
                items.iter()
                    .for_each(|item|
                        type_and_paths.extend(item.collect_compositions())),
            Item::Struct(item_struct, ..) =>
                handle_attributes_with_handler(&item_struct.attrs, |attrs|
                    cache_fields(&item_struct.fields, &attrs)),
            Item::Enum(item_enum, ..) =>
                handle_attributes_with_handler(&item_enum.attrs, |attrs|
                    item_enum.variants.iter().for_each(|Variant { fields, .. }|
                        cache_fields(fields, &attrs))),
            Item::Type(ItemType { attrs, ty, .. }, ..) =>
                handle_attributes_with_handler(attrs, |_attrs| {
                    type_and_paths.insert(*ty.clone());
                }),
            Item::Fn(item_fn, ..) =>
                handle_attributes_with_handler(&item_fn.attrs, |_attrs| {
                    type_and_paths.extend(item_fn.sig.collect_compositions());
                }),
            Item::Impl(item_impl) => handle_attributes_with_handler(&item_impl.attrs, |_attrs| {
                item_impl.items.iter().for_each(|impl_item| match impl_item {
                    ImplItem::Const(ImplItemConst { ty, .. }) |
                    ImplItem::Type(ImplItemType { ty, .. }) => {
                        type_and_paths.insert(ty.clone());
                    }
                    ImplItem::Fn(ImplItemFn { sig, .. }) => {
                        sig.inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
                            type_and_paths.insert(*ty.clone());
                        });
                        if let ReturnType::Type(_, ty) = &sig.output {
                            type_and_paths.insert(*ty.clone());
                        }
                    },
                    _ => {}
                });
            }),
            Item::Trait(item_trait, ..) => handle_attributes_with_handler(&item_trait.attrs, |_attrs| {
                item_trait.items.iter().for_each(|trait_item| match trait_item {
                    TraitItem::Type(TraitItemType { default: Some((_, ty)), .. })  |
                    TraitItem::Const(TraitItemConst { ty, .. }) => {
                        type_and_paths.insert(ty.clone());
                    }
                    TraitItem::Fn(TraitItemFn { sig, .. }) => {
                        sig.inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
                            type_and_paths.insert(*ty.clone());
                        });
                        if let ReturnType::Type(_, ty) = &sig.output {
                            type_and_paths.insert(*ty.clone());
                        }
                    },
                    _ => {}
                });
            }),
            _ => {}
        }

        type_and_paths
    }
}


impl TypeCollector for Signature {
    fn collect_compositions(&self) -> HashSet<Type> {
        let mut type_and_paths = HashSet::<Type>::new();
        self.inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
            type_and_paths.insert(*ty.clone());
        });
        if let ReturnType::Type(_, ty) = &self.output {
            type_and_paths.insert(*ty.clone());
        }
        type_and_paths
    }
}

impl TypeCollector for Type {
    fn collect_compositions(&self) -> HashSet<Type> {
        HashSet::from_iter(self.unique_nested_items())
    }
}

impl TypeCollector for Path {
    fn collect_compositions(&self) -> HashSet<Type> {
        HashSet::from_iter(self.unique_nested_items())
    }
}

impl TypeCollector for ScopeItemKind {
    fn collect_compositions(&self) -> HashSet<Type> {
        match self {
            ScopeItemKind::Item(item, ..) => item.collect_compositions(),
            ScopeItemKind::Fn(sig, ..) => sig.collect_compositions(),
        }
    }
}

