use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use quote::{quote, ToTokens};
use syn::{Ident, Item, ItemMod,parse_quote, Path,Type};
use syn::__private::TokenStream2;
use crate::generics::{GenericConversion, TypePathComposition};
use crate::helper::{ffi_struct_name, mangle_type};
use crate::interface::Presentable;
use crate::item_conversion::ItemConversion;
use crate::presentation::Expansion;
use crate::scope::Scope;
use crate::type_conversion::TypeConversion;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ImportType {
    Original,
    External,
    // full or partial import
    ExternalChunk,
    FfiType,
    FfiGeneric,
    Inner,
    None,
}

impl ImportType {

    pub fn as_path(&self) -> Path {
        match self {
            ImportType::Original => parse_quote!(ImportType::Original),
            ImportType::External => parse_quote!(ImportType::External),
            ImportType::ExternalChunk => parse_quote!(ImportType::ExternalChunk),
            ImportType::FfiType => parse_quote!(ImportType::FfiType),
            ImportType::FfiGeneric => parse_quote!(ImportType::FfiGeneric),
            ImportType::Inner => parse_quote!(ImportType::Inner),
            ImportType::None => parse_quote!(ImportType::None),
        }
    }

    pub fn get_imports_for(self, used_imports: HashSet<ImportConversion>) -> Option<(ImportType, HashSet<ImportConversion>)> {
        match self {
            ImportType::Inner | ImportType::None => None,
            _ => Some((self, used_imports))
        }
    }

    pub fn full_path_for(&self, ident: &Ident, scope: &Scope) -> Scope {
        println!("full path for: {}: {} : {}", self.as_path().to_token_stream(), ident.to_token_stream(), scope.to_token_stream());
        match self {
            ImportType::Original => scope.clone(),
            _ => scope.joined(ident),
        }
    }
}

#[derive(Clone)]
pub struct ImportConversion {
    pub ident: Ident,
    pub scope: Scope,
}

impl<'a> From<(&'a Ident, &'a Scope)> for ImportConversion {
    fn from(value: (&'a Ident, &'a Scope)) -> Self {
        Self { ident: value.0.clone(), scope: value.1.clone() }
    }
}

impl<'a> From<&'a GenericConversion> for ImportConversion {
    fn from(value: &'a GenericConversion) -> Self {
        let mangled_ident = mangle_type(&value.full_type);
        let ident = ffi_struct_name(&mangled_ident);
        ImportConversion { ident, scope: Scope::ffi_generics_scope() }
    }

}

impl std::fmt::Debug for ImportConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        f.write_str(&self.scope.to_string())?;
        f.write_str("]: ")?;
        f.write_str(&self.ident.to_token_stream().to_string())
    }
}

impl std::fmt::Display for ImportConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for ImportConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ident.to_token_stream(), self.scope.to_token_stream()];
        let other_tokens = [other.ident.to_token_stream(), other.scope.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(TokenStream2::to_string))
            .all(|(a, b)| a == b)
    }
}

impl Eq for ImportConversion {}

impl Hash for ImportConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.to_token_stream().to_string().hash(state);
        self.scope.to_token_stream().to_string().hash(state);
    }
}

#[derive(Clone)]
pub enum ScopeTreeExportItem {
    Item(Item),
    Tree(HashSet<GenericConversion>, HashMap<ImportType, HashSet<ImportConversion>>, HashMap<Ident, ScopeTreeExportItem>, HashMap<TypeConversion, Type>),
}

impl std::fmt::Debug for ScopeTreeExportItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeExportItem::Item(..) => {
                f.write_str("ScopeTreeExportItem::Item")
            }
            ScopeTreeExportItem::Tree(generics, imported, exported, scope_types) => {
                f.debug_struct("ScopeTreeExportItem::Tree")
                    .field("generics", generics)
                    .field("imported", &{
                        let debug_imports = imported.iter().map(|(i, p)| {
                            let import = i.as_path();
                            let ppp = p.iter().map(|ImportConversion { ident: i, scope: p}| quote!(#i: #p)).collect::<Vec<_>>();
                            quote!(#import: #(#ppp,)*)
                        }).collect::<Vec<_>>();
                        // println!("collect_imports_map (scope_imports): {}", quote!());
                        let all = quote!(#(#debug_imports,)*);
                        all.to_string()
                    })
                    .field("exported", {
                        &exported.iter().map(|(ident, tree_item)| (quote!(#ident).to_string(), tree_item)).collect::<Vec<_>>()
                    })
                    .field("scope_types", {
                        &scope_types.iter()
                            .map(|(tc, full_ty)| quote!(#tc: #full_ty).to_string())
                            .collect::<Vec<_>>()
                    })
                    .finish()
            }
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
        Self::Tree(HashSet::new(), HashMap::from([]), HashMap::from([(ident, item)]), HashMap::from([]))
    }
    pub fn just_export(export: HashMap<Ident, ScopeTreeExportItem>) -> ScopeTreeExportItem {
        Self::Tree(HashSet::new(), HashMap::from([]), export, HashMap::from([]))
    }

    fn non_mod_generic_full_types(item: &ItemConversion, scope_types: &HashMap<TypeConversion, Type>) -> HashSet<GenericConversion> {
        let generics = item.find_generics();
        // let dbg1 = scope_types.iter().map(|(TypeConversion { 0: conversion_ty }, ty)| quote!(#ty: #conversion_ty)).collect::<Vec<_>>();
        // let dbg2 = generics.iter().map(|TypePathComposition {0:ty, 1: path }| {
        //     let path_str = path.to_token_stream().to_string().split_whitespace().collect::<String>();
        //     let ty_str = ty.to_token_stream().to_string().split_whitespace().collect::<String>();
        //     let s = path_str.to_owned() + "," + &ty_str;
        //     quote!(#s)
        // }).collect::<Vec<_>>();
        // println!("non_mod_generic_full_types: {}", item.ident().to_token_stream());
        // println!("               scope_types: {}", quote!(#(#dbg1;)*));
        // println!("                  generics: {}", quote!(#(#dbg2;)*));
        generics
            .iter()
            .filter_map(|TypePathComposition { 0: value, .. }| {
                let ss = scope_types.get(&TypeConversion::from(value));
                // let ssss = ss.map_or(quote!(), |tu| quote!(#tu));
                // println!("        {}", quote!(#value ==> #ssss));
                // println!("GET:::: {}", ss.map_or(quote!(), |ty| quote!(#ty)));
                ss
            })
            .map(GenericConversion::from)
            .collect()
    }

    fn add_non_mod_item(&mut self, item: &ItemConversion, scope_type_dictionary: Option<&HashMap<TypeConversion, Type>>, scope_imports: Option<&HashMap<Ident, Path>>) {
        // println!("add ITEM [{}]: {} : {:?}", item.scope().to_token_stream(), item.ident().to_token_stream(), scope_type_dictionary);
        match self {
            ScopeTreeExportItem::Item(..) => panic!("Can't add item to non-tree item"),
            ScopeTreeExportItem::Tree(generics, imported, exported, scope_types) => {
                if let Some(used_types) = scope_type_dictionary {
                    scope_types.extend(used_types.clone());
                }
                generics.extend(Self::non_mod_generic_full_types(&item, scope_types));
                if let Some(used_imports) = scope_imports {
                    imported.extend(item.get_used_imports(used_imports))
                }
                // println!("add ITEM (scope_types) {:?}", scope_type_dictionary);
                exported.insert(item.ident().clone(), ScopeTreeExportItem::Item(item.into()));
            }
        }
    }

    fn add_mod_item(&mut self, item_mod: &ItemMod, scope: &Scope, used_types_at_scopes: &HashMap<Scope, HashMap<TypeConversion, Type>>, used_imports_at_scopes: &HashMap<Scope, HashMap<Ident, Path>>) {
        // println!("add TREE: [{}]: {}", scope.to_token_stream(), item_mod.to_token_stream());
        match &item_mod.content {
            Some((_, items)) => {
                let ident = item_mod.ident.clone();
                let inner_scope = scope.joined(&ident);
                let inner_used_types = used_types_at_scopes.get(&inner_scope);
                let inner_used_types_init = if let Some(inner_used_types) = inner_used_types {
                    inner_used_types.clone()
                } else {
                    HashMap::new()
                };
                let mut inner_tree = ScopeTreeExportItem::Tree(HashSet::new(), HashMap::default(), HashMap::default(), inner_used_types_init);
                items.iter().for_each(|item| {
                    match ItemConversion::try_from((item, &inner_scope)) {
                        Ok(ItemConversion::Mod(item_mod, scope)) =>
                            inner_tree.add_mod_item(&item_mod, &scope, used_types_at_scopes, used_imports_at_scopes),
                        Ok(inner_item) =>
                            {
                                // Self::print_used_types_at_scopes("inner tree add item", used_types_at_scopes, scope);
                                inner_tree.add_non_mod_item(&inner_item, used_types_at_scopes.get(&inner_scope), used_imports_at_scopes.get(&inner_scope))
                            },
                        _ => {}
                    };
                });
                match self {
                    ScopeTreeExportItem::Item(_) => {},
                    ScopeTreeExportItem::Tree(_, _, exported, _) => {
                        exported.insert(ident.clone(), inner_tree);
                    }
                };
            },
            None => {  }
        }
    }

    pub fn print_used_types_at_scopes(prefix: &str, used_types_at_scopes: &HashMap<Scope, HashMap<TypeConversion, Type>>, scope: &Scope) {
        let all = used_types_at_scopes.iter().map(|(scope, map)| {
            let conversions = map.iter()
                .map(|(tc, full_ty)| quote!(#tc: #full_ty))
                .collect::<Vec<_>>();
            quote! {
                #scope: [
                    #(#conversions;)*
                ]
            }
        }).collect::<Vec<_>>();
        println!("{}: {}", prefix, scope.to_token_stream());
        println!("                      {}", quote!(#(#all;)*));
    }
    pub fn add_item(&mut self, item: ItemConversion, used_types_at_scopes: &HashMap<Scope, HashMap<TypeConversion, Type>>, used_imports_at_scopes: &HashMap<Scope, HashMap<Ident, Path>>) {
        let scope = item.scope();
        match self {
            ScopeTreeExportItem::Tree(..) => {
                match &item {
                    ItemConversion::Use(..) => {},
                    ItemConversion::Mod(item_mod, scope) =>
                        self.add_mod_item(item_mod, scope, used_types_at_scopes, used_imports_at_scopes),
                    _ =>
                        {
                            // Self::print_used_types_at_scopes("self tree add item", used_types_at_scopes, scope);
                            self.add_non_mod_item(&item, used_types_at_scopes.get(scope), used_imports_at_scopes.get(scope))
                        }
                };
            },
            _ => {}
        }
    }
}


pub struct ScopeTreeCompact {
    pub(crate) scope: Scope,
    pub(crate) generics: HashSet<GenericConversion>,
    pub(crate) imported: HashMap<ImportType, HashSet<ImportConversion>>,
    pub(crate) exported: HashMap<Ident, ScopeTreeExportItem>,
    pub(crate) scope_types: HashMap<TypeConversion, Type>
}

impl Into<ScopeTreeItem> for ScopeTreeCompact {
    fn into(self) -> ScopeTreeItem {
        let name = self.scope.head();
        ScopeTreeItem::Tree {
            item: parse_quote!(pub mod #name;),
            tree: self.into()
        }
    }
}
impl ScopeTreeCompact {
    pub fn init_with(item: ScopeTreeExportItem, scope: Scope) -> Option<Self> {
        println!("Merged TREE: {}", item);
        match item {
            ScopeTreeExportItem::Item(_) => None,
            ScopeTreeExportItem::Tree(generics, imported, exported, scope_types) => {
                Some(ScopeTreeCompact {
                    scope,
                    generics,
                    imported,
                    exported,
                    scope_types
                })
            }
        }
    }
    pub fn init(scope: Scope) -> Self {
        Self::new(scope, HashSet::new(), HashMap::new(), HashMap::new(), HashMap::new())
    }
    pub fn new(scope: Scope, generics: HashSet<GenericConversion>, imported: HashMap<ImportType, HashSet<ImportConversion>>, exported: HashMap<Ident, ScopeTreeExportItem>, scope_types: HashMap<TypeConversion, Type>) -> Self {
        Self { scope, generics, imported, exported, scope_types }
    }
}

#[derive(Clone, Debug)]
pub enum ScopeTreeItem {
    Item {
        item: Item,
        scope: Scope,
        scope_types: HashMap<TypeConversion, Type>,
    },
    Tree {
        item: Item,
        tree: ScopeTree
    }
}

impl Presentable for ScopeTreeItem {
    fn present(self) -> TokenStream2 {
        match self {
            Self::Item { item, scope, scope_types } => {
                // println!("ScopeTreeItem::present ITEM: [{}]: {}", scope, item.to_token_stream());
                // println!("ScopeTreeItem::present ITEM: {:?}", scope_types);
                ItemConversion::try_from((&item, scope))
                    .map(|conversion| Expansion::from((conversion, scope_types)))
                    .map_or(quote!(), Expansion::present)
            }
            Self::Tree { item: _, tree} => {
                // println!("ScopeTreeItem::present TREE: [{}]", tree.scope);
                let name = tree.scope.head();
                let imports = tree.imported.iter().map(|(import_type, imports)| {
                    // println!("make import.1: {}: [{:?}]", import_type.as_path().to_token_stream(), imports);
                    let unique_imports: Vec<Scope> = imports.iter().map(|ImportConversion { ident, scope }| {
                        let unique = match import_type {
                            ImportType::External => scope.clone(),
                            ImportType::Original => scope.clone(),
                            ImportType::FfiType => scope.clone(),
                            ImportType::ExternalChunk => scope.popped(),
                            _ => scope.joined(ident)
                        };
                        // println!("make import.2: {}: {} [{}] -> {}", import_type.as_path().to_token_stream(), ident.to_token_stream(), scope.to_token_stream(), unique.to_token_stream());
                        unique
                    }).collect();
                    let imports = unique_imports.iter().map(|import| quote!(use #import)).collect::<Vec<_>>();
                    quote!(#(#imports;)*)
                });
                let conversions = tree.exported.into_iter().map(|(_, tree_item)| tree_item.present());
                quote!(pub mod #name {
                    #(#imports)*
                    #(#conversions)*
                })
            }
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


#[derive(Debug, Clone)]
pub struct ScopeTree {
    pub scope: Scope,
    pub imported: HashMap<ImportType, HashSet<ImportConversion>>,
    pub exported: HashMap<Ident, ScopeTreeItem>,
    pub generics: HashSet<GenericConversion>,
    pub scope_types: HashMap<TypeConversion, Type>,
}



impl Into<ScopeTree> for ScopeTreeCompact {
    fn into(self) -> ScopeTree {
        let ScopeTreeCompact { scope, generics, imported, exported, scope_types } = self;
        println!("ScopeTree: {}", quote!(#scope));
        let debug_gen = generics.iter().map(|g| quote!(#g)).collect::<Vec<_>>();
        let debug_tp = scope_types.iter().map(|(tc, full_ty)| quote!(#tc: #full_ty)).collect::<Vec<_>>();
        println!(" generics: {}", quote!(#(#debug_gen;)*));
        println!("         : {:?}", imported);
        println!(" scope_types : {}", quote!(#(#debug_tp;)*));
        let mut new_imported = imported.clone();
        let generics = HashSet::from_iter(generics.into_iter());
        if let Some(used_originals) = imported.get(&ImportType::Original) {
            new_imported.entry(ImportType::FfiType)
                .or_insert_with(HashSet::new)
                .extend(used_originals.iter().map(|ImportConversion { ident, scope}| {
                    let ty = Scope::ffi_type_converted_or_same(&parse_quote!(#scope));
                    ImportConversion {
                        ident: ffi_struct_name(ident),
                        scope: parse_quote!(#ty)
                    }
                }));
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

        println!("         : {:?}", new_imported);
        let exported = exported.into_iter().map(|(ident, tree_item_raw)| (ident.clone(), {
            // println!("into tree join.2: {} + {}", scope.to_token_stream(), ident.to_token_stream());
            let scope = scope.joined(&ident);
            match tree_item_raw {
                ScopeTreeExportItem::Item(item) => ScopeTreeItem::Item { item, scope, scope_types: scope_types.clone() },
                ScopeTreeExportItem::Tree(generics, imported, exported, scope_types) => ScopeTreeCompact {
                    scope,
                    generics,
                    imported,
                    exported,
                    scope_types
                }.into(),
            }
        })).collect();
        ScopeTree {
            scope,
            imported: new_imported,
            exported,
            generics,
            scope_types: scope_types.clone()
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

// For root tree only
impl Presentable for ScopeTree {
    fn present(self) -> TokenStream2 {
        let scope_imports = self.imported.iter()
            .flat_map(|(import_type, vec)| {
                vec.iter().map(move |ImportConversion { ident, scope }| {
                    // println!("scope import: {}: {}: {}", import_type.as_path().to_token_stream(), ident.to_token_stream(), scope.to_token_stream());
                    let import_path = match import_type {
                        ImportType::Original => scope.clone(),
                        _ => scope.joined(ident),
                    };
                    // quote!(use #scope::#ident;)
                    quote!(use #import_path;)
                })
            });
        let mut generics: HashSet<GenericConversion> = HashSet::from_iter(self.generics.into_iter());
        let scope_conversions = self.exported.into_iter().map(|(_, tree_item)| {
            generics.extend(tree_item.generic_conversions());
            tree_item.present()
        }).collect::<Vec<_>>();
        let mut generic_imports = HashSet::new();
        let mut generic_conversions = vec![];
        for generic in generics {
            generic_imports.extend(generic.used_imports());
            generic_conversions.push(generic.present());
        }
        let generic_unique_imports = Vec::from_iter(generic_imports.iter());
        quote! {
            #[allow(dead_code, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables)]
            pub mod types {
                #(#scope_imports)*
                #(#scope_conversions)*
            }
            #[allow(dead_code, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables)]
            pub mod generics {
                #(use #generic_unique_imports;)*
                #(#generic_conversions)*
            }
        }
    }
}