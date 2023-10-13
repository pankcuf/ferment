use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::{GenericArgument, Ident, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, ItemType, ItemUse, parse_quote, Path, PathArguments, PathSegment, Type, TypePath, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use crate::generics::TypePathComposition;
use crate::item_conversion::ItemConversion;
use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::Scope;
use crate::scope_conversion::ScopeTreeExportItem;
use crate::type_conversion::TypeConversion;

pub type TypePair = (Type, Type);
pub type IdentPath = (Ident, Path);

pub struct Visitor {
    /// syn::Path to the file
    pub(crate) parent: Scope,
    pub(crate) current_scope_stack: Vec<Ident>,
    pub(crate) current_module_scope: Scope,

    pub(crate) inner_visitors: Vec<Visitor>,

    // pub(crate) scope_exports: HashMap<Ident, ScopeTreeExportItem>,
    pub(crate) needed_conversions_for_scopes: HashMap<Scope, Vec<ItemConversion>>,
    pub(crate) used_types_at_scopes: HashMap<Scope, HashMap<TypeConversion, Type>>,
    pub(crate) used_imports_at_scopes: HashMap<Scope, HashMap<Ident, Path>>,

    pub tree: ScopeTreeExportItem,
}

pub struct ScopeComposition {
    pub scope: Scope,
    pub imports: Vec<Scope>,
    pub exports: Vec<Scope>,
    pub generics: HashSet<TypePathComposition>,
    pub conversions: Vec<ItemConversion>,
}


impl std::fmt::Debug for ScopeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ScopeComposition { scope, imports, exports, generics, conversions, } = self;
        let generics = generics.iter().map(|TypePathComposition { 0: ty, 1: path}| quote!(#ty: #path));
        let exports = quote!(#(#exports),*).to_string();
        let imports = quote!(#(#imports),*).to_string();
        let generics = quote!(#(#generics),*).to_string();
        let conversions = quote!(#(#conversions),*).to_string();
        f.write_fmt(format_args!("----------- scope: {}\n", scope))?;
        f.write_fmt(format_args!("-------- generics: {}\n", generics))?;
        f.write_fmt(format_args!("--------- exports: {}\n", exports))?;
        f.write_fmt(format_args!("--------- imports: {}\n", imports))?;
        f.write_fmt(format_args!("----- conversions: {}\n", conversions))
    }
}

impl std::fmt::Display for ScopeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeComposition {

    // pub fn generate_imports(&self, scope: &Scope, use_dictionary: &HashMap<Scope, HashMap<Ident, Path>>, matches: &HashMap<Scope, HashMap<TypeConversion, Type>>) -> HashSet<Scope> {
    //     let dict: Punctuated<TokenStream2, syn::Token!(:)> = Punctuated::from_iter(use_dictionary.iter().map(|(id, paths)| {
    //         let p = paths.iter().map(|(ident, path)| quote!(#ident: #path));
    //         quote!(#id: [#(#p),*])
    //     }));
    //     let m: Punctuated<TokenStream2, syn::Token!(:)>  = Punctuated::from_iter(matches.iter().map(|(scope, m)| {
    //         let m = m.iter().map(|(ty, full_ty)| quote!(#ty -> #full_ty));
    //         quote!(#scope: [#(#m),*])
    //     }));
    //     // let ffi_imports = matches.iter().filter_map(|(ty, full_ty)| {
    //     //     quote!(#ty: #full_ty)
    //     // });
    //     // let dict = use_dictionary.iter().map(|(id, path)| quote!(#id: #path)).collect::<Punctuated<_, _>>();
    //     // let m = matches.iter().map(|(ty, full_ty)| quote!(#ty: #full_ty)).collect::<Punctuated<_, _>>();
    //     // println!("generate_imports from: scope {}", quote!(#scope));
    //     // println!("generate_imports from: imports {}", quote!(#(#dict)));
    //     // println!("generate_imports from: matches {}", quote!(#(#m)));
    //     let scope_dict = use_dictionary.get(scope);
    //     let mut scope_imports: HashSet<_> = HashSet::from_iter(self.exports.clone());
    //     let generic_imports = self.generics
    //         .iter()
    //         .filter_map(|TypePathComposition { 1: path, .. }| scope_dict.and_then(|dict| dict.get(&path.segments.last().unwrap().ident)))
    //         .map(|path| Scope::new(path.clone()));
    //
    //     let types_imports = self.imports
    //         .iter()
    //         .filter(|s| !self.exports.contains(s))
    //         .map(|s| parse_quote!(#s))
    //         .filter_map(|full_ty: Type| {
    //             // println!("Import.1 ==: {}", quote!(#full_ty));
    //             let import = Scope::ffi_type_import(&full_ty);
    //             // println!("Import ==: {}", quote!(#import));
    //             import
    //         });
    //     let generic_imports2 = self.generics.iter().map(TypePathComposition::ffi_generic_import_scope);
    //     {
    //         let e_imports = self.exports.clone();
    //         let t_imports = types_imports.clone();
    //         let g_imports = generic_imports.clone();
    //         let g2_imports = generic_imports2.clone();
    //         // println!("generate_imports: exported: {}", quote!(#(#e_imports),*));
    //         // println!("generate_imports: types: {}", quote!(#(#t_imports),*));
    //         // println!("generate_imports: generic_imports: {}", quote!(#(#g_imports),*));
    //         // println!("generate_imports: generic_imports2: {}", quote!(#(#g2_imports),*));
    //     }
    //     scope_imports.extend(types_imports);
    //     scope_imports.extend(generic_imports);
    //     scope_imports.extend(generic_imports2);
    //     // scope_imports.extend(matches.values().filter_map(Scope::ffi_type_import));
    //     // scope_imports.extend(self.exports.iter().filter_map(|scope| Scope::ffi_type_import(&parse_quote!(#scope))));
    //     scope_imports
    // }

    // pub fn expand_conversions(self) -> Vec<TokenStream2> {
    //     self.conversions
    //         .into_iter()
    //         .filter(ItemConversion::is_non_empty_mod)
    //         .map(Expansion::from)
    //         .map(Expansion::present)
    //         .collect()
    // }
}

impl std::fmt::Debug for Visitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Visitor")
            .field("parent", &self.parent.to_token_stream().to_string())
            // .field("used_types_at_scopes", {
            //     let vec = vec![];
            //     let v = self.used_types_at_scopes.iter().fold(vec, |mut acc, (k, tm)| {
            //         let tme = tm.iter().map(|(ty, full_ty)| quote!(#ty -> #full_ty));
            //         acc.push(quote!(#k: [#(#tme)*]));
            //         acc
            //     });
            //     let expanded = quote!(#(#v),*);
            //     &expanded.to_string().as_str()
            // })
            // .field("used_imports_at_scopes", {
            //     let vec = vec![];
            //     let v = self.used_imports_at_scopes.iter().fold(vec, |mut acc, (k, scope_imports)| {
            //         let si = scope_imports.iter().map(|(k, v)| quote!(#k: #v)).collect::<Vec<_>>();
            //         acc.push(quote!(#k: #(#si),*));
            //         acc
            //     });
            //     let expanded = quote!(#(#v),*);
            //     &expanded.to_string().as_str()
            // })
            // .field("needed_conversions_for_scopes", {
            //     let vec = vec![];
            //     let v = self.needed_conversions_for_scopes.iter().fold(vec, |mut acc, (k, v)| {
            //         let ck = k.as_ffi_scope();
            //         acc.push(quote!(#k (#ck): #(#v),*));
            //         acc
            //     });
            //     let expanded = quote!(#(#v),*);
            //     &expanded.to_string().as_str()
            // })
            .field("visitors", &self.inner_visitors)
            .field("tree", &self.tree)
            .finish()
    }
}

impl std::fmt::Display for Visitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<'ast> Visit<'ast> for Visitor {

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        // println!("Visit enum: {}", quote!(#node));
        // let current_module = &self.current_scope_stack;
        // println!("Visit enum: {}: {}", quote!(#(#current_module::)*), quote!(#node));
        self.add_full_qualified_conversion(Item::Enum(node.clone()));
        // syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // println!("Visit fn: {}", quote!(#node));
        // let current_module = &self.current_scope_stack;
        // println!("Visit fn: {}: {}", quote!(#(#current_module::)*), quote!(#node));
        self.add_full_qualified_conversion(Item::Fn(node.clone()));
        // syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        // let current_module = &self.current_scope_stack;
        // println!("Visit mod: {}: {}", quote!(#(#current_module::)*), quote!(#node));
        // println!("Visit mod: {}", quote!(#node));
        self.current_scope_stack.push(node.ident.clone());
        self.add_full_qualified_conversion(Item::Mod(node.clone()));
        if let Some(ref content) = node.content {
            for item in &content.1 {
                syn::visit::visit_item(self, item);
            }
        }
        self.current_scope_stack.pop();
        // syn::visit::visit_item_mod(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        // let current_module = &self.current_scope_stack;
        // println!("Visit struct: {}: {}", quote!(#(#current_module::)*), quote!(#node));
        self.add_full_qualified_conversion(Item::Struct(node.clone()));
        // syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        // let current_module = &self.current_scope_stack;
        // println!("Visit type: {}: {}", quote!(#(#current_module::)*), quote!(#node));
        // println!("Visit type: {}", quote!(#node));
        self.add_full_qualified_conversion(Item::Type(node.clone()));
        // syn::visit::visit_item_type(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        // let current_module = &self.current_scope_stack;
        // println!("Visit use: {}: {}", quote!(#(#current_module::)*), quote!(#node));
        // println!("Visit use: {}", quote!(#node));
        let item = Item::Use(node.clone());
        let scope = self.current_scope_for(&item);

        self.fold_import_tree(&scope, &node.tree, vec![]);
        // syn::visit::visit_item_use(self, node);
    }

}

// fn expand_mod(directives: TokenStream2, name: TokenStream2, expansion: TokenStream2) -> TokenStream2 {
//     quote! {
//         #directives
//         pub mod #name {
//             #expansion
//         }
//     }
// }
//

impl Visitor {
    /// path: full-qualified Path for file
    pub(crate) fn new(scope: Scope) -> Self {
        Self {
            parent: scope.clone(),
            current_scope_stack: vec![],
            current_module_scope: scope.clone(),

            used_types_at_scopes: HashMap::new(),
            used_imports_at_scopes: HashMap::new(),
            needed_conversions_for_scopes: HashMap::new(),
            inner_visitors: vec![],

            tree: ScopeTreeExportItem::Tree(HashSet::new(), HashMap::default(), HashMap::default(), HashMap::default())
        }
    }

    pub(crate) fn add_full_qualified_type_match(&mut self, scope: Scope, ty: &Type) {
        let conversion = TypeConversion::from(ty);
        let all_involved_types = conversion.get_all_type_paths_involved();
        let all_involved_full_types = all_involved_types.into_iter().map(|tp| {
            let ty: Type = parse_quote!(#tp);
            let full_ty = self.update_nested_generics(&scope, &ty);
            (TypeConversion::new(ty), full_ty)
        }).collect::<HashMap<_, _>>();
        self.used_types_at_scopes.entry(scope)
            .or_insert_with(HashMap::new)
            .extend(all_involved_full_types);
    }

    // fn build_tree_item(scope: Scope, generics: HashSet<GenericConversion>, imported: HashMap<ImportType, HashSet<Import>>, exported: HashMap<Ident, ScopeTreeExportItem>) -> ScopeTreeCompact {
    //     ScopeTreeCompact {
    //         scope,
    //         generics,
    //         imported,
    //         exported,
    //     }
    // }

    // fn merge_trees(destination: &mut ScopeTreeExportItem, source: &ScopeTreeExportItem, inner_visitors: &[Visitor]) {
    //     if let ScopeTreeExportItem::Tree(_, _, dest_exports) = destination {
    //         if let ScopeTreeExportItem::Tree(_, _, source_exports) = source {
    //             for (name, export_item) in source_exports {
    //                 dest_exports.entry(name.clone())
    //                     .and_modify(|e| { Self::merge_trees(e, export_item, inner_visitors); })
    //                     .or_insert_with(|| export_item.clone());
    //             }
    //         }
    //     }
    //     inner_visitors.iter().for_each(|v|
    //         Self::merge_trees(destination, &v.tree, &v.inner_visitors));
    // }
    //
    // fn transform(visitor: &Visitor) -> ScopeTreeExportItem {
    //     let mut result = visitor.tree.clone();
    //     for v in &visitor.inner_visitors {
    //         Self::merge_trees(&mut result, &v.tree, &v.inner_visitors);
    //     }
    //     result
    // }

    // fn merge_trees(destination: &mut ScopeTreeExportItem, source: &ScopeTreeExportItem) {
    //     if let ScopeTreeExportItem::Tree(_, _, dest_exports) = destination {
    //         if let ScopeTreeExportItem::Tree(_, _, source_exports) = source {
    //             for (name, export_item) in source_exports {
    //                 dest_exports.entry(name.clone())
    //                     .and_modify(|e| Self::merge_trees(e, &export_item))
    //                     .or_insert_with(|| export_item.clone());
    //             }
    //         }
    //     }
    // }
    //
    // fn transform(visitor: &Visitor) -> ScopeTreeExportItem {
    //     let mut result = visitor.tree.clone();
    //     for v in &visitor.inner_visitors {
    //         Self::merge_trees(&mut result, &v.tree);
    //         for inner_v in &v.inner_visitors {
    //             Self::merge_trees(&mut result, &inner_v.tree);
    //         }
    //     }
    //     result
    // }

    // fn merge_trees(destination: &mut ScopeTreeExportItem, source: &ScopeTreeExportItem) {
    //     if let ScopeTreeExportItem::Tree(_, _, dest_exports) = destination {
    //         if let ScopeTreeExportItem::Tree(_, _, source_exports) = source {
    //             for (name, source_item) in source_exports {
    //                 match dest_exports.entry(name.clone()) {
    //                     std::collections::hash_map::Entry::Occupied(mut o) => {
    //                         Self::merge_trees(o.get_mut(), &source_item);
    //                     }
    //                     std::collections::hash_map::Entry::Vacant(v) => {
    //                         v.insert(source_item.clone());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // fn transform(visitor: &Visitor) -> ScopeTreeExportItem {
    //     let mut result = visitor.tree.clone();
    //     for v in &visitor.inner_visitors {
    //         Self::merge_trees(&mut result, &v.tree);
    //         for inner_v in &v.inner_visitors {
    //             Self::merge_trees(&mut result, &inner_v.tree);
    //         }
    //     }
    //     result
    // }
    //
    //
    // fn transform(visitor: &Visitor) -> ScopeTreeExportItem {
    //     let mut result = visitor.tree.clone();
    //     for v in &visitor.inner_visitors {
    //         merge_trees(&mut result, &v.tree);
    //         for inner_v in &v.inner_visitors {
    //             merge_trees(&mut result, &inner_v.tree);
    //         }
    //     }
    //     result
    // }

    // pub fn expand(&mut self) -> TokenStream2 {
    //
    // }

    // pub fn expand(&mut self) -> TokenStream2 {
    //     let name = self.parent.ffi_name();
    //     let inner_expansions = self.inner_visitors.iter().map(Visitor::expand);
    //
    //     // Self::transform()
    //
    //     // let inner_trees = self.inner_visitors.iter().map(Visitor::merge_trees);
    //
    //
    //     let mut all_generics = HashSet::new();
    //     let mut all_imports_for_generics = HashSet::new();
    //     let mut item_expansions = vec![];
    //
    //     // let tree_item = Self::build_tree_item(scope.clone(), )
    //
    //
    //     self.needed_conversions_for_scopes.iter().for_each(|(scope, conversions)| {
    //         let mut generics = HashSet::new();
    //         let mut exports = vec![];
    //         let mut imports = vec![];
    //         conversions.iter().for_each(|conversion| {
    //             if let Some(generic_type_in_scope) = conversion.export_in_scope(scope) {
    //                 exports.push(generic_type_in_scope);
    //             }
    //             generics.extend(conversion.find_generics());
    //             if conversion.is_not_mod() {
    //                 imports.extend(conversion.find_types().into_iter().map(|ty| {
    //                     // println!("Found type for import: {}: [{} -> {}]", scope, ty.0.to_token_stream(), ty.1.to_token_stream());
    //
    //                     // scope.joined_path(ty.1)
    //                     Scope::new(ty.1)
    //                 }));
    //             }
    //         });
    //         all_generics.extend(generics.clone());
    //         all_imports_for_generics.extend(exports.clone());
    //
    //         let scope_composition = ScopeComposition {
    //             scope: scope.clone(),
    //             imports,
    //             exports,
    //             generics,
    //             conversions: conversions.clone(),
    //         };
    //         // println!("{}", &scope_composition);
    //         let scope_imports = scope_composition.generate_imports(scope, &self.used_imports_at_scopes, &self.used_types_at_scopes);
    //         let scope_conversions = scope_composition.expand_conversions();
    //         let scope_imports = Vec::from_iter(scope_imports.iter());
    //         // println!("--- scope imports: {}", quote!(#(#scope_imports),*));
    //         let expansion = quote! {
    //             #(use #scope_imports;)*
    //             #(#scope_conversions)*
    //         };
    //         let item_expansion = if scope.eq(&self.parent) {
    //             expansion
    //         } else {
    //             expand_mod(quote!(), scope.ffi_name().to_token_stream(), expansion)
    //         };
    //         item_expansions.push(item_expansion);
    //     });
    //
    //     if self.parent.is_crate() {
    //         merge_visitor_trees(&mut self);
    //
    //         println!("Expand: {:#?}", self);
    //         println!("RootTree: {}", self.tree);
    //
    //         let generic_expansions = all_generics.into_iter().map(TypePathComposition::expand_generic);
    //         let mod_directives = quote!(#[allow(dead_code, redundant_semicolons, unused_braces, unused_imports, unused_unsafe, unused_variables)]);
    //         let types_mod = expand_mod(mod_directives.clone(), quote!(#name), quote! {
    //             #(#item_expansions)*
    //             #(#inner_expansions)*
    //         });
    //         let generic_mod = expand_mod(mod_directives, quote!(generics), quote! {
    //             #(#generic_expansions)*
    //         });
    //         quote! {
    //             #types_mod
    //             #generic_mod
    //         }
    //     } else {
    //         expand_mod(quote!(), quote!(#name), quote! {
    //             #(#item_expansions)*
    //             #(#inner_expansions)*
    //         })
    //     }
    // }

    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    pub(crate) fn fold_import_tree(&mut self, scope: &Scope, use_tree: &UseTree, mut current_path: Vec<Ident>) {
        match use_tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                current_path.push(ident.clone());
                self.fold_import_tree(scope,&*tree, current_path);
            },
            UseTree::Name(UseName { ident, .. }) => {
                current_path.push(ident.clone());
                self.used_imports_at_scopes
                    .entry(scope.clone())
                    .or_insert_with(HashMap::new)
                    .insert(ident.clone(), Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) });
            },
            UseTree::Rename(UseRename { rename, .. }) => {
                current_path.push(rename.clone());
                self.used_imports_at_scopes
                    .entry(scope.clone())
                    .or_insert_with(HashMap::new)
                    .insert(rename.clone(), Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) });
            },
            UseTree::Group(UseGroup { items, .. }) =>
                items.iter()
                    .for_each(|tree| self.fold_import_tree(scope,tree,current_path.clone())),
            UseTree::Glob(_) => {
                // For a glob import, we can't determine the full path statically
                // Just ignore them for now
            }
        }
    }

    /// Create a new TypePath with the updated base path and generic type parameters
    /// `BTreeMap<u32, u32>` -> `std::collections::BTreeMap<u32, u32>`,
    /// `BTreeMap<u32, BTreeMap<u32, u32>>` -> `std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>`
    fn update_nested_generics(&self, scope: &Scope, ty: &Type) -> Type {
        match ty {
            Type::Path(TypePath { qself, path, .. }) => {
                let mut segments = path.segments.clone();
                for segment in &mut segments {
                    if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                        for arg in &mut angle_bracketed_generic_arguments.args {
                            if let GenericArgument::Type(inner_type) = arg {
                                *arg = GenericArgument::Type(self.update_nested_generics(scope, inner_type));
                            }
                        }
                    }
                }
                if let Some(scope_imports) = self.used_imports_at_scopes.get(scope) {
                    let ident = &segments.last().unwrap().ident;
                    if let Some(replacement_path) = scope_imports.get(ident) {
                        let last_segment = segments.pop().unwrap();
                        segments.extend(replacement_path.segments.clone());
                        segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    } else {
                        let local_type = match PathConversion::from(path) {
                            PathConversion::Primitive(_p) => None,
                            PathConversion::Complex(p) => {
                                match p.segments.last().unwrap().ident.to_string().as_str() {
                                    "str" | "String" | "Option" => None,
                                    _ => {
                                        // println!("update_nested_generics: join: {} + {}", scope.to_token_stream(), p.to_token_stream());
                                        Some(scope.joined_path(p))
                                    },
                                }

                            },
                            PathConversion::Generic(GenericPathConversion::Vec(_p)) |
                            PathConversion::Generic(GenericPathConversion::Map(_p)) => {
                                // println!("update_nested_generics: (no import, so it exports generic: ) [{}]: {} ", quote!(#scope), quote!(#p))
                                None
                            }
                        };
                        if let Some(local) = local_type {
                            let last_segment = segments.pop().unwrap();
                            segments.extend(local.path.segments.clone());
                            segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                            // println!("update_nested_generics: (no import, so it exports type: ) {} ", quote!(#local))
                        }

                        // let last_segment = segments.pop().unwrap();
                        // segments.extend(replacement_path.segments.clone());
                        // segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;


                    }
                }

                Type::Path(TypePath {
                    qself: qself.clone(),
                    path: Path { leading_colon: path.leading_colon, segments },
                })
            },
            _ => ty.clone(),
        }
    }

    fn current_scope_for(&self, item: &Item) -> Scope {
        let is_mod = if let Item::Mod(..) = item { true } else { false };
        match self.current_scope_stack.first() {
            Some(current_mod) if !is_mod => self.current_module_scope.joined(current_mod),
            _ => self.current_module_scope.clone()
        }
    }


    // pub fn get_tree_export_item(&mut self, scope: &Scope) -> &mut ScopeTreeExportItem {
    //     let path_to_traverse: Vec<Ident> = scope.path.segments.iter().skip(1).map(|segment| segment.ident.clone()).collect();
    //     let mut current_tree = &mut self.tree;
    //
    //     let mut current_tree_export_item: Option<&mut ScopeTreeExportItem> = None;
    //     for ident in &path_to_traverse {
    //         current_tree_export_item = match current_tree {
    //             ScopeTreeExportItem::Item(..) => None,
    //             ScopeTreeExportItem::Tree(_, _, exported) => Some(match exported.entry(ident.clone()) {
    //                 Entry::Occupied(mut occupied) =>
    //                     occupied.get_mut(),
    //                 Entry::Vacant(vacant) =>
    //                     vacant.insert(ScopeTreeExportItem::just_export(HashMap::new()))
    //             })
    //         };
    //     }
    //     current_tree_export_item.unwrap()
    // }
    //

    pub fn get_tree_export_item(&mut self, scope: &Scope) -> &mut ScopeTreeExportItem {
        let path_to_traverse: Vec<Ident> = scope.path.segments.iter().skip(1).map(|segment| segment.ident.clone()).collect();
        let mut current_tree = &mut self.tree;
        for ident in &path_to_traverse {
            match current_tree {
                ScopeTreeExportItem::Item(..) => panic!("Unexpected item while traversing the scope path"),  // Handle as appropriate
                ScopeTreeExportItem::Tree(_, _, exported, _) => {
                    if !exported.contains_key(ident) {
                        exported.insert(ident.clone(), ScopeTreeExportItem::just_export(HashMap::new()));
                    }
                    current_tree = exported.get_mut(ident).unwrap();
                }
            }
        }
        current_tree
    }

    pub fn add_full_qualified_conversion<'ast>(&mut self, item: Item) {
        let scope = self.current_scope_for(&item);
        let conversion = match &item {
            Item::Mod(item_mod) => ItemConversion::Mod(item_mod.clone(), scope.clone()),
            Item::Struct(item_struct) => ItemConversion::Struct(item_struct.clone(), scope.clone()),
            Item::Enum(item_enum) => ItemConversion::Enum(item_enum.clone(), scope.clone()),
            Item::Type(item_type) => ItemConversion::Type(item_type.clone(), scope.clone()),
            Item::Fn(item_fn) => ItemConversion::Fn(item_fn.clone(), scope.clone()),
            item => unimplemented!("add_full_qualified_conversion error: {:?} ", quote!(#item))
        };
        let converted = conversion.add_full_qualified_conversion(self);
        // println!("visitor.add scope: [{}], {}", scope, converted);
        let used_types_at_scopes = self.used_types_at_scopes.clone();
        let used_imports_at_scopes = self.used_imports_at_scopes.clone();
        self.needed_conversions_for_scopes
            .entry(scope.clone())
            .or_insert_with(Vec::new)
            .push(converted.clone());
        self.get_tree_export_item(&scope)
            .add_item(ItemConversion::try_from((&item, converted.scope())).unwrap(), &used_types_at_scopes, &used_imports_at_scopes);
    }
}


pub fn merge_visitor_trees(visitor: &mut Visitor) {
    // Merge the trees of the inner visitors first.
    for inner_visitor in &mut visitor.inner_visitors {
        merge_visitor_trees(inner_visitor);
    }

    // Now merge the trees of the inner visitors into the current visitor's tree.
    for inner_visitor in &visitor.inner_visitors {
        merge_trees(&mut visitor.tree, &inner_visitor.tree);
    }
}

fn merge_trees(destination: &mut ScopeTreeExportItem, source: &ScopeTreeExportItem) {
    match (destination, source) {
        (ScopeTreeExportItem::Tree(_, _, dest_exports, _),
            ScopeTreeExportItem::Tree(_, _, source_exports, _), ) => {
            for (name, source_tree) in source_exports.iter() {
                match dest_exports.entry(name.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut o) =>
                        merge_trees(o.get_mut(), source_tree),
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(source_tree.clone());
                    }
                }
            }
        }
        _ => {}
    }
}
