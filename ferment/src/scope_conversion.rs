use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::{Ident, Item, ItemMod, ItemTrait, parse_quote, Type};
use syn::__private::TokenStream2;
use crate::context::Context;
use crate::formatter::{format_exported_dict, format_imported_dict, format_types_dict, format_used_traits};
use crate::generics::{GenericConversion, TypePathComposition};
use crate::helper::ffi_struct_name;
use crate::import_conversion::{ImportConversion, ImportType};
use crate::interface::Presentable;
use crate::item_conversion::ItemConversion;
use crate::presentation::Expansion;
use crate::scope::Scope;
use crate::type_conversion::TypeConversion;
use crate::visitor::UsageInfo;

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(Context, Item),
    Tree(Context, HashSet<GenericConversion>, HashMap<ImportType, HashSet<ImportConversion>>, HashMap<Ident, ScopeTreeExportItem>, HashMap<TypeConversion, Type>, HashMap<Scope, HashMap<Ident, ItemTrait>>),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportItem::Item(..) => f.write_str("ScopeTreeExportItem::Item"),
            ScopeTreeExportItem::Tree(context, generics, imported, exported, scope_types, used_traits) =>
                f.debug_struct("ScopeTreeExportItem::Tree")
                    .field("context", context)
                    .field("generics", generics)
                    .field("imported", &format_imported_dict(imported))
                    .field("exported", &format_exported_dict(exported))
                    .field("scope_types", &format_types_dict(scope_types))
                    .field("used_traits", &format_used_traits(used_traits))
                    .finish()
        }
    }
}
impl std::fmt::Display for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeTreeExportItem {
    pub fn single_export(ident: Ident, item: ScopeTreeExportItem) -> ScopeTreeExportItem {
        Self::Tree(Context::default(), HashSet::new(), HashMap::from([]), HashMap::from([(ident, item)]), HashMap::from([]), HashMap::from([]))
    }

    pub fn just_export_with_context(export: HashMap<Ident, ScopeTreeExportItem>, context: Context) -> ScopeTreeExportItem {
        Self::Tree(context, HashSet::new(), HashMap::from([]), export, HashMap::from([]), HashMap::from([]))
    }

    fn add_non_mod_item(&mut self, item: &ItemConversion, scope: &Scope, usage_info: &UsageInfo) {
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(context, generics, imported, exported, scope_types, scope_traits) => {
                scope_traits.extend(usage_info.used_traits_at_scopes.clone());
                if let Some(used_types) = usage_info.used_types_at_scopes.get(scope) {
                    scope_types.extend(used_types.clone());
                }
                generics.extend(item.find_generics()
                    .iter()
                    .filter_map(|TypePathComposition { 0: value, .. }| scope_types.get(&TypeConversion::from(value)))
                    .map(GenericConversion::from));
                if let Some(scope_imports) = usage_info.used_imports_at_scopes.get(scope) {
                    item.get_used_imports(scope_imports)
                        .iter()
                        .for_each(|(import_type, imports)|
                            imported.entry(import_type.clone())
                                .or_insert_with(HashSet::new)
                                .extend(imports.clone()));
                }
                exported.insert(item.ident().clone(), ScopeTreeExportItem::Item(context.clone(), item.into()));
            }
        }
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &Scope, usage_info: &UsageInfo) {
        // println!("add TREE: [{}]: {}", scope.to_token_stream(), item_mod.to_token_stream());
        match &item_mod.content {
            Some((_, items)) => {
                let ident = item_mod.ident.clone();
                let inner_scope = scope.joined(&ident);
                let mut inner_tree = ScopeTreeExportItem::Tree(Context::default(), HashSet::new(), HashMap::default(), HashMap::default(), HashMap::new(), HashMap::new());
                items.iter().for_each(|item| {
                    match ItemConversion::try_from((item, &inner_scope)) {
                        Ok(ItemConversion::Mod(item_mod, scope)) =>
                            inner_tree.add_mod_item(&item_mod, &scope, usage_info),
                        Ok(inner_item) =>
                            inner_tree.add_non_mod_item(&inner_item, &inner_scope, usage_info),
                        _ => {}
                    };
                });
                match self {
                    ScopeTreeExportItem::Item(_, _) => {},
                    ScopeTreeExportItem::Tree(_, _, _, exported, _, _) => {
                        exported.insert(ident.clone(), inner_tree);
                    }
                };
            },
            None => {}
        }
    }

    pub fn add_item(&mut self, item: ItemConversion, usage_info: &UsageInfo) {
        let scope = item.scope();
        if let ScopeTreeExportItem::Tree(..) = self {
            match &item {
                ItemConversion::Use(..) => {},
                ItemConversion::Mod(item_mod, scope) =>
                    self.add_mod_item(item_mod, scope, usage_info),
                _ =>
                    self.add_non_mod_item(&item, scope, usage_info)
            };
        }
    }
}


pub struct ScopeTreeCompact {
    pub(crate) context: Context,
    pub(crate) scope: Scope,
    pub(crate) generics: HashSet<GenericConversion>,
    pub(crate) imported: HashMap<ImportType, HashSet<ImportConversion>>,
    pub(crate) exported: HashMap<Ident, ScopeTreeExportItem>,
    pub(crate) scope_types: HashMap<TypeConversion, Type>,
    pub(crate) used_traits: HashMap<Scope, HashMap<Ident, ItemTrait>>
}

impl From<ScopeTreeCompact> for ScopeTreeItem {
    fn from(value: ScopeTreeCompact) -> Self {
        let name = value.scope.head();
        ScopeTreeItem::Tree {
            item: parse_quote!(pub mod #name;),
            tree: value.into()
        }
    }
}

impl ScopeTreeCompact {
    pub fn init_with(item: ScopeTreeExportItem, scope: Scope, context: Context) -> Option<Self> {
        match item {
            ScopeTreeExportItem::Item(_, _) => None,
            ScopeTreeExportItem::Tree(_, generics, imported, exported, scope_types, used_traits) => {
                Some(ScopeTreeCompact {
                    context,
                    scope,
                    generics,
                    imported,
                    exported,
                    scope_types,
                    used_traits
                })
            }
        }
    }
}

#[derive(Clone)]
pub enum ScopeTreeItem {
    Item {
        item: Item,
        scope: Scope,
        scope_types: HashMap<TypeConversion, Type>,
        scope_traits: HashMap<Scope, HashMap<Ident, ItemTrait>>
    },
    Tree {
        item: Item,
        tree: ScopeTree
    }
}

impl Presentable for ScopeTreeItem {
    fn present(self) -> TokenStream2 {
        match self {
            Self::Item { item, scope, scope_types, scope_traits } =>
                ItemConversion::try_from((&item, scope))
                    .map(|conversion| Expansion::from((conversion, scope_types, scope_traits)))
                    .map_or(quote!(), Expansion::present),
            Self::Tree { item: _, tree} =>
                tree.present()
        }
    }
}

impl ScopeTreeItem {
    pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
        match self {
            Self::Item { .. } => HashSet::from([]),
            Self::Tree { tree, .. } => tree.generic_conversions()
        }
    }
}


#[derive(Clone)]
pub struct ScopeTree {
    pub scope: Scope,
    pub imported: HashMap<ImportType, HashSet<ImportConversion>>,
    pub exported: HashMap<Ident, ScopeTreeItem>,
    pub generics: HashSet<GenericConversion>,
    pub scope_types: HashMap<TypeConversion, Type>,
    pub used_traits: HashMap<Scope, HashMap<Ident, ItemTrait>>,
}

impl From<ScopeTreeCompact> for ScopeTree {
    fn from(value: ScopeTreeCompact) -> Self {
        let ScopeTreeCompact { context, scope, generics, imported, exported, scope_types, used_traits } = value;
        let mut new_imported = imported.clone();
        let generics = HashSet::from_iter(generics);
        if let Some(used_originals) = imported.get(&ImportType::Original) {
            new_imported.entry(ImportType::FfiType)
                .or_insert_with(HashSet::new)
                .extend(used_originals.iter().filter_map(|ImportConversion { ident, scope}| {
                    match ident.to_string().as_str() {
                        "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" | "VarInt" => None,
                        _ => {
                            let ty = Scope::ffi_type_converted_or_same(&parse_quote!(#scope));
                            Some(ImportConversion {
                                ident: ffi_struct_name(ident),
                                scope: parse_quote!(#ty)
                            })
                        }
                    }
                }));
        }
        // external fermented crates
        if let Some(used_external_fermented) = imported.get(&ImportType::External) {
            new_imported.entry(ImportType::FfiExternal)
                .or_insert_with(HashSet::new)
                .extend(used_external_fermented.iter().filter_map(|ImportConversion { ident, scope}| match ident.to_string().as_str() {
                    "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" | "VarInt" => None,
                    _ if context.contains_fermented_crate(&scope.root_ident()) => {
                        let ty = Scope::ffi_external_type_converted_or_same(&parse_quote!(#scope), &context);
                        Some(ImportConversion {
                            ident: ffi_struct_name(ident),
                            scope: parse_quote!(#ty)
                        })
                    },
                    _ => None
                }
                ));
        }
        new_imported.entry(ImportType::Original)
            .or_insert_with(HashSet::new)
            .extend(exported.iter().filter_map(|(ident, tree_item_raw)| match tree_item_raw {
                ScopeTreeExportItem::Item(..) => Some(ImportConversion { ident: ident.clone(), scope: scope.joined(ident) }),
                _ => None
            }));
        new_imported.entry(ImportType::FfiGeneric)
            .or_insert_with(HashSet::new)
            .extend(generics.iter()
                .map(ImportConversion::from));

        let exported = exported.into_iter().map(|(ident, tree_item_raw)| {
            let scope = scope.joined(&ident);
            (ident, match tree_item_raw {
                ScopeTreeExportItem::Item(_, item) =>
                    ScopeTreeItem::Item { item, scope, scope_types: scope_types.clone(), scope_traits: used_traits.clone()  },
                ScopeTreeExportItem::Tree(context, generics, imported, exported, scope_types, used_traits) => ScopeTreeCompact {
                    context,
                    scope,
                    generics,
                    imported,
                    exported,
                    scope_types,
                    used_traits
                }.into(),
            })
        }).collect();
        ScopeTree {
            scope,
            imported: new_imported,
            exported,
            generics,
            scope_types: scope_types.clone(),
            used_traits: used_traits.clone()
        }
    }
}

impl ScopeTree {

    pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
        let mut generics = self.generics.clone();
        generics.extend(self.inner_generics());
        generics
    }
    fn inner_generics(&self) -> HashSet<GenericConversion> {
        self.exported.iter()
            .flat_map(|(_, tree)| tree.generic_conversions())
            .collect()
    }
}

impl Presentable for ScopeTree {
    fn present(self) -> TokenStream2 {
        let scope_imports = self.imported.iter()
            .flat_map(|(import_type, imports)|
                imports.iter()
                    .map(move |import| import.present(import_type)));
        if self.scope.is_crate() {
            // For root tree only
            let mut generics: HashSet<GenericConversion> = HashSet::from_iter(self.generics);
            let scope_conversions = self.exported.into_values().map(|tree_item| {
                generics.extend(tree_item.generic_conversions());
                tree_item.present()
            }).collect::<Vec<_>>();
            let mut generic_imports = HashSet::new();
            let mut generic_conversions = vec![];
            for generic in generics {
                generic_imports.extend(generic.used_imports());
                generic_conversions.push(generic.present());
            }
            let types_expansion = Expansion::Mod {
                directives: quote!(#[allow(clippy::all, dead_code, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables)]),
                name: quote!(types),
                imports: scope_imports.collect(),
                conversions: scope_conversions
            }
                .present();
            let generics_expansion = Expansion::Mod {
                directives: quote!(#[allow(clippy::all, dead_code, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables)]),
                name: quote!(generics),
                imports: generic_imports.into_iter().collect(),
                conversions: generic_conversions
            }
                .present();
            quote! {
                #types_expansion
                #generics_expansion
            }
        } else {
            Expansion::Mod {
                directives: quote!(),
                name: self.scope.head().to_token_stream(),
                imports: scope_imports.collect(),
                conversions: self.exported.into_values().map(ScopeTreeItem::present).collect()
            }.present()
        }
    }
}