use syn::{Attribute, Fields, FieldsNamed, FieldsUnnamed, FnArg, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemMod, ItemType, Meta, NestedMeta, parse_quote, Path, PatType, ReturnType, Signature, TraitItem, TraitItemConst, TraitItemMethod, TraitItemType, Type, Variant};
use crate::ast::TypeHolder;
use crate::composable::GenericBoundsModel;
use crate::composer::MaybeMacroLabeled;
use crate::conversion::ScopeItemKind;
use crate::ext::UniqueNestedItems;

#[allow(unused)]
pub struct MacroAttributes {
    pub path: Path,
    pub arguments: Vec<Path>,
}

pub trait TypeCollector {
    fn collect_compositions(&self) -> Vec<TypeHolder>;
}
fn handle_attributes_with_handler<F: FnMut(MacroAttributes)>(attrs: &[Attribute], mut handler: F) {
    attrs.iter()
        .for_each(|attr|
            if attr.is_labeled_for_export() || attr.is_labeled_for_opaque_export() {
                let mut arguments = Vec::<Path>::new();
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    meta_list.nested.iter().for_each(|meta| {
                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            arguments.push(path.clone());
                        }
                    });
                }
                handler(MacroAttributes {
                    path: attr.path.clone(),
                    arguments
                })
            }
        )
}

impl TypeCollector for GenericBoundsModel {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        let mut type_and_paths: Vec<TypeHolder> = Vec::new();
        // let mut cache_type = |ty: &Type|
        //     type_and_paths.push(TypeHolder(ty.clone()));
        // self.bounds.iter().for_each(|bound| {
        //
        // });
        self.predicates.iter().for_each(|(_ty, bounds)| {
            bounds.iter().for_each(|bound| {
                type_and_paths.push(TypeHolder(parse_quote!(#bound)))
            });
        });

        type_and_paths
    }
}

impl TypeCollector for Item {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        let mut type_and_paths: Vec<TypeHolder> = Vec::new();
        let mut cache_type = |ty: &Type|
            type_and_paths.push(TypeHolder(ty.clone()));
        let mut cache_fields = |fields: &Fields, _attrs: &MacroAttributes| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter().for_each(|field| cache_type(&field.ty)),
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
                handle_attributes_with_handler(attrs, |_attrs|
                    cache_type(ty)),
            Item::Fn(item_fn, ..) =>
                handle_attributes_with_handler(&item_fn.attrs, |_attrs| {
                    type_and_paths.extend(item_fn.sig.collect_compositions());
                }),
            Item::Impl(item_impl) => handle_attributes_with_handler(&item_impl.attrs, |_attrs| {
                item_impl.items.iter().for_each(|impl_item| match impl_item {
                    ImplItem::Const(ImplItemConst { ty, .. }) =>
                        cache_type(ty),
                    ImplItem::Method(ImplItemMethod { sig, .. }) => {
                        sig.inputs.iter().for_each(|arg|
                            if let FnArg::Typed(PatType { ty, .. }) = arg {
                                cache_type(ty);
                            });
                        if let ReturnType::Type(_, ty) = &sig.output {
                            cache_type(ty);
                        }
                    },
                    ImplItem::Type(ImplItemType { ty, .. }) =>
                        cache_type(ty),
                    _ => {}
                });
            }),
            Item::Trait(item_trait, ..) => handle_attributes_with_handler(&item_trait.attrs, |_attrs| {
                item_trait.items.iter().for_each(|trait_item| match trait_item {
                    TraitItem::Type(TraitItemType { default: Some((_, ty)), .. }) =>
                        cache_type(ty),
                    TraitItem::Method(TraitItemMethod { sig, .. }) => {
                        sig.inputs.iter().for_each(|arg|
                            if let FnArg::Typed(PatType { ty, .. }) = arg {
                                cache_type(ty);
                            });
                        if let ReturnType::Type(_, ty) = &sig.output {
                            cache_type(ty);
                        }
                    },
                    TraitItem::Const(TraitItemConst { ty, .. }) =>
                        cache_type(ty),
                    _ => {}
                });
            }),
            _ => {}
        }

        type_and_paths
    }
}


impl TypeCollector for Signature {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        let mut type_and_paths: Vec<TypeHolder> = Vec::new();
        self.inputs.iter().for_each(|arg|
            if let FnArg::Typed(PatType { ty, .. }) = arg {
                type_and_paths.push(TypeHolder(*ty.clone()));
            });
        if let ReturnType::Type(_, ty) = &self.output {
            type_and_paths.push(TypeHolder(*ty.clone()));
        }
        type_and_paths
    }
}

impl TypeCollector for Type {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        self.unique_nested_items().iter().map(TypeHolder::from).collect()
    }
}

impl TypeCollector for Path {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        self.unique_nested_items().iter().map(TypeHolder::from).collect()
    }
}

impl TypeCollector for ScopeItemKind {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        match self {
            ScopeItemKind::Item(item, ..) => item.collect_compositions(),
            ScopeItemKind::Fn(sig, ..) => sig.collect_compositions(),
        }
    }
}

