use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::{GenericArgument, Ident, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, parse_quote, Path, PathArguments, PathSegment, Type, TypePath, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use crate::Config;
use crate::context::Context;
use crate::item_conversion::ItemConversion;
use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::scope_conversion::ScopeTreeExportItem;
use crate::scope::Scope;
use crate::type_conversion::TypeConversion;

pub struct Visitor {
    /// syn::Path to the file
    pub(crate) context: Context,
    pub(crate) parent: Scope,
    pub(crate) current_scope_stack: Vec<Ident>,
    pub(crate) current_module_scope: Scope,

    pub(crate) inner_visitors: Vec<Visitor>,

    pub(crate) used_traits_at_scopes: HashMap<Scope, HashMap<Ident, ItemTrait>>,
    pub(crate) used_types_at_scopes: HashMap<Scope, HashMap<TypeConversion, Type>>,
    pub(crate) used_imports_at_scopes: HashMap<Scope, HashMap<Ident, Path>>,

    pub tree: ScopeTreeExportItem,
}

impl std::fmt::Debug for Visitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Visitor")
            .field("context", &self.context)
            .field("parent", &self.parent.to_token_stream().to_string())
            // .field("used_traits_at_scopes", {
            //     let vec = vec![];
            //     let v = self.used_traits_at_scopes.iter().fold(vec, |mut acc, (k, tm)| {
            //         let tme = tm.iter().map(|(ty, ItemTrait { ident, .. })| quote!(#ty -> #ident));
            //         acc.push(quote!(#k: [#(#tme)*]));
            //         acc
            //     });
            //     let expanded = quote!(#(#v),*);
            //     &expanded.to_string().as_str()
            // })
            .field("used_types_at_scopes", {
                let vec = vec![];
                let v = self.used_types_at_scopes.iter().fold(vec, |mut acc, (k, tm)| {
                    let tme = tm.iter().map(|(ty, full_ty)| quote!(#ty -> #full_ty));
                    acc.push(quote!(#k: [#(#tme)*]));
                    acc
                });
                let expanded = quote!(#(#v),*);
                &expanded.to_string().as_str()
            })
            .field("used_imports_at_scopes", {
                let vec = vec![];
                let v = self.used_imports_at_scopes.iter().fold(vec, |mut acc, (k, scope_imports)| {
                    let si = scope_imports.iter().map(|(k, v)| quote!(#k: #v)).collect::<Vec<_>>();
                    acc.push(quote!(#k: #(#si),*));
                    acc
                });
                let expanded = quote!(#(#v),*);
                &expanded.to_string().as_str()
            })
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
        self.add_conversion(Item::Enum(node.clone()));
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        //println!("visit_item_fn: {}: {:?}", node.sig.ident.to_token_stream(), node.attrs);
        self.add_conversion(Item::Fn(node.clone()));
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        self.current_scope_stack.push(node.ident.clone());
        self.add_conversion(Item::Mod(node.clone()));
        if let Some(ref content) = node.content {
            for item in &content.1 {
                syn::visit::visit_item(self, item);
            }
        }
        self.current_scope_stack.pop();
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        //println!("visit_item_struct: {}: {:?}", node.ident.to_token_stream(), node.attrs);
        self.add_conversion(Item::Struct(node.clone()));
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        //println!("visit_item_type: {}: {:?}", node.ident.to_token_stream(), node.attrs);
        self.add_conversion(Item::Type(node.clone()));
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        let item = Item::Use(node.clone());
        let scope = self.current_scope_for(&item);
        self.fold_import_tree(&scope, &node.tree, vec![]);
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        self.add_conversion(Item::Trait(node.clone()));
    }
}

impl Visitor {
    /// path: full-qualified Path for file
    pub(crate) fn new(scope: Scope, config: &Config) -> Self {
        Self {
            context: Context::new(config.crate_names.clone()),
            parent: scope.clone(),
            current_scope_stack: vec![],
            current_module_scope: scope.clone(),
            used_traits_at_scopes: HashMap::new(),
            used_types_at_scopes: HashMap::new(),
            used_imports_at_scopes: HashMap::new(),
            inner_visitors: vec![],
            tree: ScopeTreeExportItem::Tree(Context::default(), HashSet::new(), HashMap::default(), HashMap::default(), HashMap::default(), HashMap::default()),
        }
    }

    pub(crate) fn add_full_qualified_trait_match(&mut self, scope: Scope, item_trait: &ItemTrait) {
        println!("add_full_qualified_trait_match: {}: -> {}", quote!(#scope), quote!(#item_trait));
        self.used_traits_at_scopes.entry(scope.clone())
            .or_insert_with(HashMap::new)
            .insert(item_trait.ident.clone(), item_trait.clone());
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

    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    pub(crate) fn fold_import_tree(&mut self, scope: &Scope, use_tree: &UseTree, mut current_path: Vec<Ident>) {
        match use_tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                current_path.push(ident.clone());
                self.fold_import_tree(scope,&*tree, current_path);
            },
            UseTree::Name(UseName { ident, .. }) |
            UseTree::Rename(UseRename { rename: ident, .. }) => {
                current_path.push(ident.clone());
                self.used_imports_at_scopes
                    .entry(scope.clone())
                    .or_insert_with(HashMap::new)
                    .insert(ident.clone(), Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) });
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

    pub fn get_tree_export_item(&mut self, scope: &Scope, context: Context) -> &mut ScopeTreeExportItem {
        let path_to_traverse: Vec<Ident> = scope.path.segments.iter().skip(1).map(|segment| segment.ident.clone()).collect();
        let mut current_tree = &mut self.tree;
        for ident in &path_to_traverse {
            match current_tree {
                ScopeTreeExportItem::Item(..) => panic!("Unexpected item while traversing the scope path"),  // Handle as appropriate
                ScopeTreeExportItem::Tree(inner_context, _, _, exported, _, _) => {
                    inner_context.merge(&context);
                    if !exported.contains_key(ident) {
                        exported.insert(ident.clone(), ScopeTreeExportItem::just_export_with_context(HashMap::new(), context.clone()));
                    }
                    current_tree = exported.get_mut(ident).unwrap();
                }
            }
        }
        current_tree
    }

    pub fn add_conversion<'ast>(&mut self, item: Item) {
        let scope = self.current_scope_for(&item);
        match ItemConversion::try_from((&item, &scope)) {
            Ok(conversion) => {
                if conversion.has_macro_attribute() {
                    // println!("handle_attributes_with_handler: {}", path.to_token_stream());
                    let full_qualified = conversion.add_full_qualified_conversion(self);
                    let used_traits_at_scopes = self.used_traits_at_scopes.clone();
                    let used_types_at_scopes = self.used_types_at_scopes.clone();
                    let used_imports_at_scopes = self.used_imports_at_scopes.clone();
                    self.get_tree_export_item(&scope, self.context.clone())
                        .add_item(full_qualified, Some(&used_traits_at_scopes), &used_types_at_scopes, &used_imports_at_scopes);
                }
            },
            _ => {}
        }
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
        (ScopeTreeExportItem::Tree(_, _, _, dest_exports, _, _),
            ScopeTreeExportItem::Tree(_, _, _, source_exports, _, _), ) => {
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
