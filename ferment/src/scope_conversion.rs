use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::{Ident, Item, ItemMod, parse_quote};
use syn::__private::TokenStream2;
use crate::context::Context;
use crate::formatter::{format_tree_exported_dict, format_imported_dict, format_types_dict, format_used_traits, format_tree_item_dict};
use crate::generics::GenericConversion;
use crate::import_conversion::{ImportConversion, ImportType};
use crate::item_conversion::{ItemContext, ItemConversion, trait_items_from_attributes};
use crate::presentation::Expansion;
use crate::scope::Scope;
use crate::visitor::UsageInfo;

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(Context, Item),
    Tree(HashSet<GenericConversion>, HashMap<ImportType, HashSet<ImportConversion>>, HashMap<Ident, ScopeTreeExportItem>, ItemContext),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportItem::Item(..) => f.write_str("ScopeTreeExportItem::Item"),
            ScopeTreeExportItem::Tree(generics, imported, exported, ItemContext { context ,scope_types, traits_dictionary: used_traits } ) =>
                f.debug_struct("ScopeTreeExportItem::Tree")
                    .field("context", context)
                    .field("generics", generics)
                    .field("imported", &format_imported_dict(imported))
                    .field("exported", &format_tree_exported_dict(exported))
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
        Self::Tree(HashSet::new(), HashMap::from([]), HashMap::from([(ident, item)]), ItemContext::default())
    }

    pub fn just_export_with_context(export: HashMap<Ident, ScopeTreeExportItem>, context: Context) -> ScopeTreeExportItem {
        Self::Tree(HashSet::new(), HashMap::from([]), export, ItemContext::with_context(context))
    }

    fn add_non_mod_item(&mut self, item: &ItemConversion, scope: &Scope, usage_info: &UsageInfo) {
        // println!("add_non_mod_item: {}", item.ident().to_token_stream());
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(
                // context,
                generics,
                imported,
                exported,
                item_context) => {
                item_context.traits_dictionary.extend(usage_info.used_traits_at_scopes.clone());
                if let Some(used_types) = usage_info.used_types_at_scopes.get(scope) {
                    item_context.scope_types.extend(used_types.clone());
                }
                // TODO: here we should also compose imports for used traits by this item
                let scope_generics = item.find_generics_fq(&item_context.scope_types);
                //println!("find scope generics: {:?}", scope_generics);
                generics.extend(scope_generics);
                if let Some(scope_imports) = usage_info.used_imports_at_scopes.get(scope) {
                    item.get_used_imports(scope_imports)
                        .iter()
                        .for_each(|(import_type, imports)|
                            imported.entry(import_type.clone())
                                .or_insert_with(HashSet::new)
                                .extend(imports.clone()));
                }
                let trait_items = trait_items_from_attributes(item.attrs(), item_context);
                trait_items.into_iter().for_each(|(item_trait, trait_scope)| {
                    let scope_imports = usage_info.used_imports_at_scopes.get(&trait_scope);
                    let trait_item = ItemConversion::Trait(item_trait, trait_scope);
                    if let Some(scope_imports) = scope_imports {
                        trait_item.get_used_imports(scope_imports)
                            .iter()
                            .for_each(|(import_type, imports)|
                                imported.entry(import_type.clone())
                                    .or_insert_with(HashSet::new)
                                    .extend(imports.clone()));
                    }
                    // TODO: replace scope_types with global scope dictionary or we have everything inside?
                    let trait_item_generics = trait_item.find_generics_fq(&item_context.scope_types);
                    //println!("find trait generics: {}: {:?}", item.ident().to_token_stream(), trait_item_generics);
                    generics.extend(trait_item_generics);

                });
                exported.insert(item.ident().clone(), ScopeTreeExportItem::Item(item_context.context.clone(), item.into()));
            }
        }
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &Scope, usage_info: &UsageInfo) {
        // println!("add TREE: [{}]: {}", scope.to_token_stream(), item_mod.to_token_stream());
        match &item_mod.content {
            Some((_, items)) => {
                let ident = item_mod.ident.clone();
                let inner_scope = scope.joined(&ident);
                let mut inner_tree = ScopeTreeExportItem::Tree( HashSet::new(), HashMap::default(), HashMap::default(), ItemContext::default());
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
                    ScopeTreeExportItem::Tree(_, _, exported, _) => {
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
    // pub(crate) context: Context,
    pub(crate) scope: Scope,
    pub(crate) generics: HashSet<GenericConversion>,
    pub(crate) imported: HashMap<ImportType, HashSet<ImportConversion>>,
    pub(crate) exported: HashMap<Ident, ScopeTreeExportItem>,
    pub item_context: ItemContext,
    // pub(crate) scope_types: HashMap<TypeConversion, Type>,
    // pub(crate) used_traits: HashMap<Scope, HashMap<Ident, ItemTrait>>
}
impl std::fmt::Debug for ScopeTreeCompact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopeTreeCompact")
            // .field("context", &self.context)
            .field("generics", &self.generics)
            .field("imported", &format_imported_dict(&self.imported))
            .field("exported", &format_tree_exported_dict(&self.exported))
            .field("scope_types", &format_types_dict(&self.item_context.scope_types))
            .field("used_traits", &format_used_traits(&self.item_context.traits_dictionary))
            .finish()
    }
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
    pub fn init_with(item: ScopeTreeExportItem, scope: Scope) -> Option<Self> {
        match item {
            ScopeTreeExportItem::Item(_, _) => None,
            ScopeTreeExportItem::Tree(generics, imported, exported, item_context) => {
                Some(ScopeTreeCompact {
                    // context,
                    scope,
                    generics,
                    imported,
                    exported,
                    item_context
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
        item_context: ItemContext,
        // scope_types: HashMap<TypeConversion, Type>,
        // traits_dictionary: HashMap<Scope, HashMap<Ident, ItemTrait>>
    },
    Tree {
        item: Item,
        tree: ScopeTree
    }
}

impl ToTokens for ScopeTreeItem {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Item { item, scope, item_context } =>
                ItemConversion::try_from((item, scope))
                    .map(|conversion| conversion.make_expansion(item_context.clone()))
                    .map_or(quote!(), Expansion::into_token_stream),
            Self::Tree { item: _, tree} =>
                tree.to_token_stream()
        }.to_tokens(tokens)
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
    pub generics: HashSet<GenericConversion>,
    pub imported: HashMap<ImportType, HashSet<ImportConversion>>,
    pub exported: HashMap<Ident, ScopeTreeItem>,
    pub item_context: ItemContext,
    // pub scope_types: HashMap<TypeConversion, Type>,
    // pub used_traits: HashMap<Scope, HashMap<Ident, ItemTrait>>,
}

impl std::fmt::Debug for ScopeTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopeTreeCompact")
            .field("scope", &self.scope)
            .field("generics", &self.generics)
            .field("imported", &format_imported_dict(&self.imported))
            .field("exported", &format_tree_item_dict(&self.exported))
            .field("scope_types", &format_types_dict(&self.item_context.scope_types))
            .field("used_traits", &format_used_traits(&self.item_context.traits_dictionary))
            .finish()
    }
}

impl ToTokens for ScopeTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let scope_imports = self.imported.iter()
            .flat_map(|(import_type, imports)|
                imports.iter()
                    .map(move |import| import.present(import_type)));

        if self.scope.is_crate() {
            // For root tree only
            let mut generics: HashSet<GenericConversion> = HashSet::from_iter(self.generics.iter().cloned());
            let scope_conversions = self.exported.iter().map(|(_, tree_item)| {
                generics.extend(tree_item.generic_conversions());
                tree_item.to_token_stream()
            }).collect::<Vec<_>>();
            let mut generic_imports = HashSet::new();
            let mut generic_conversions = vec![];
            for generic in &generics {
                generic_imports.extend(generic.used_imports());
                generic_conversions.push(generic.to_token_stream());
            }
            let directives = quote!(#[allow(clippy::let_and_return, clippy::redundant_field_names, dead_code, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables, unused_qualifications)]);
            let types_expansion = Expansion::Mod {
                directives: directives.clone(),
                name: quote!(types),
                imports: scope_imports.collect(),
                conversions: scope_conversions
            }
                .to_token_stream();
            let generics_expansion = Expansion::Mod {
                directives,
                name: quote!(generics),
                imports: generic_imports.into_iter().collect(),
                conversions: generic_conversions
            }
                .to_token_stream();
            quote! {
                #types_expansion
                #generics_expansion
            }
        } else {
            Expansion::Mod {
                directives: quote!(),
                name: self.scope.head().to_token_stream(),
                imports: scope_imports.collect(),
                conversions: self.exported.iter().map(|(_, tree_item)| tree_item.to_token_stream()).collect()
            }.to_token_stream()
        }.to_tokens(tokens)
    }}


impl From<ScopeTreeCompact> for ScopeTree {
    fn from(value: ScopeTreeCompact) -> Self {
        let ScopeTreeCompact { scope, generics, imported, exported, item_context } = value;
        //println!("ScopeTreeCompact:::: [{}]: {:#?}", quote!(#scope), item_context);
        let new_imported = imported.clone();
        // // TODO: add types in implemented traits
        // let generics = HashSet::from_iter(generics);
        // if let Some(used_originals) = imported.get(&ImportType::Original) {
        //     new_imported.entry(ImportType::FfiType)
        //         .or_insert_with(HashSet::new)
        //         .extend(used_originals.iter().filter_map(|ImportConversion { ident, scope}| {
        //             match ident.to_string().as_str() {
        //                 "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" | "VarInt" => None,
        //                 _ => {
        //                     let ty = Scope::ffi_type_converted_or_same(&parse_quote!(#scope));
        //                     Some(ImportConversion {
        //                         ident: ffi_struct_name(ident),
        //                         scope: parse_quote!(#ty)
        //                     })
        //                 }
        //             }
        //         }));
        // }
        // // external fermented crates
        // if let Some(used_external_fermented) = imported.get(&ImportType::External) {
        //     new_imported.entry(ImportType::FfiExternal)
        //         .or_insert_with(HashSet::new)
        //         .extend(used_external_fermented.iter().filter_map(|ImportConversion { ident, scope}| match ident.to_string().as_str() {
        //             "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" | "VarInt" => None,
        //             _ if context.contains_fermented_crate(&scope.root_ident()) => {
        //                 let ty = Scope::ffi_external_type_converted_or_same(&parse_quote!(#scope), &context);
        //                 Some(ImportConversion {
        //                     ident: ffi_struct_name(ident),
        //                     scope: parse_quote!(#ty)
        //                 })
        //             },
        //             _ => None
        //         }
        //         ));
        // }
        // new_imported.entry(ImportType::Original)
        //     .or_insert_with(HashSet::new)
        //     .extend(exported.iter().filter_map(|(ident, tree_item_raw)| match tree_item_raw {
        //         ScopeTreeExportItem::Item(..) => Some(ImportConversion { ident: ident.clone(), scope: scope.joined(ident) }),
        //         _ => None
        //     }));
        // new_imported.entry(ImportType::FfiGeneric)
        //     .or_insert_with(HashSet::new)
        //     .extend(generics.iter()
        //         .map(ImportConversion::from));

        let exported = exported.into_iter().map(|(ident, tree_item_raw)| {
            let scope = scope.joined(&ident);
            (ident, match tree_item_raw {
                ScopeTreeExportItem::Item(_, item) =>
                    ScopeTreeItem::Item { item, scope, item_context: item_context.clone()  },
                ScopeTreeExportItem::Tree(generics, imported, exported, item_context) => ScopeTreeCompact {
                    // context,
                    scope,
                    generics,
                    imported,
                    exported,
                    item_context
                }.into(),
            })
        }).collect();
        let tree = ScopeTree {
            scope,
            imported: new_imported,
            exported,
            generics,
            item_context,
        };
        // println!("ScopeTree:::: {:?}", tree);

        tree
    }
}

impl ScopeTree {

    pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
        let mut generics = self.generics.clone();
        generics.extend(self.inner_generics());
        generics
    }
    fn inner_generics(&self) -> HashSet<GenericConversion> {
        self.exported.values()
            .flat_map(|tree| tree.generic_conversions())
            .collect()
    }
}