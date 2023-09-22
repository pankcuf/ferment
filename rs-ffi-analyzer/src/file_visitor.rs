use std::collections::HashMap;
use quote::quote;
use syn::{Field, GenericArgument, Ident, ItemStruct, ItemUse, Path, PathArguments, PathSegment, Type, TypePath, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::__private::{TokenStream2};
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use crate::item_conversion::ItemConversion;

pub struct FileVisitor {
    /// <full-qualified path, Vec<FFI item>
    pub(crate) conversion_scopes: HashMap<Path, Vec<ItemConversion>>,
    /// syn::Path to the file
    pub(crate) parent: Path,
    matches: HashMap<Type, Type>,
    type_paths: HashMap<Ident, Path>,
}

impl<'ast> Visit<'ast> for FileVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        node.fields.iter()
            .for_each(|Field { ty, .. }| { self.matches.insert(ty.clone(),self.update_nested_generics(&ty)); });

        let mut modified = node.clone();
        modified
            .fields
            .iter_mut()
            .for_each(|Field { ty, .. }| match self.matches.get(ty) {
                Some(converted) => { *ty = converted.clone() },
                None => {}
            });

        println!("struct: original: {} modified: {}", quote!(#node), quote!(#modified));
        self.conversion_scopes
            .entry(self.parent.clone())
            .or_insert_with(Vec::new)
            .push(ItemConversion::Struct(modified));
        // conversion.expand_all_types()
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        self.fold_import_tree(&node.tree, vec![]);
        syn::visit::visit_item_use(self, node);
    }
}


impl FileVisitor {
    /// path: full-qualified Path for file
    pub(crate) fn new(parent: Path) -> Self {
        Self { parent, matches: HashMap::new(), type_paths: HashMap::new(), conversion_scopes: HashMap::new() }
    }
    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    fn fold_import_tree(&mut self, use_tree: &UseTree, mut current_path: Vec<Ident>) {
        match use_tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                current_path.push(ident.clone());
                self.fold_import_tree(&*tree, current_path);
            },
            UseTree::Name(UseName { ident, .. }) => {
                current_path.push(ident.clone());
                self.type_paths.insert(ident.clone(), Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) });
            },
            UseTree::Rename(UseRename { rename, .. }) => {
                current_path.push(rename.clone());
                self.type_paths.insert(rename.clone(), Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) });
            },
            UseTree::Group(UseGroup { items, .. }) =>
                items.iter()
                    .for_each(|tree| self.fold_import_tree(tree,current_path.clone())),
            UseTree::Glob(_) => {
                // For a glob import, we can't determine the full path statically
                // Just ignore them for now
            }
        }
    }

    /// Create a new TypePath with the updated base path and generic type parameters
    /// `BTreeMap<u32, u32>` -> `std::collections::BTreeMap<u32, u32>`,
    /// `BTreeMap<u32, BTreeMap<u32, u32>>` -> `std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>`
    fn update_nested_generics(&self, ty: &Type) -> Type {
        match ty {
            Type::Path(TypePath { qself, path, .. }) => {
                let mut segments = path.segments.clone();
                for segment in &mut segments {
                    if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                        for arg in &mut angle_bracketed_generic_arguments.args {
                            if let GenericArgument::Type(inner_type) = arg {
                                *arg = GenericArgument::Type(self.update_nested_generics(inner_type));
                            }
                        }
                    }
                }
                if let Some(replacement_path) = self.type_paths.get(&segments.last().unwrap().ident) {
                    let last_segment = segments.pop().unwrap();
                    segments.extend(replacement_path.segments.clone());
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                }
                Type::Path(TypePath {
                    qself: qself.clone(),
                    path: Path { leading_colon: path.leading_colon, segments },
                })
            },
            _ => ty.clone(),
        }
    }

    // fn print_modified_struct(&self, node: &ItemStruct) {
    //     println!("----> {}", quote!(#node));
    //     let mut modified = node.clone();
    //     modified
    //         .fields
    //         .iter_mut()
    //         .for_each(|Field { ty, .. }| match self.matches.get(ty) {
    //             Some(converted) => { *ty = converted.clone() },
    //             None => {}
    //         });
    //     println!("<---- {}", quote!(#modified));
    // }

    pub fn make_expansions(&self) -> Vec<TokenStream2> {
        let expansions = vec![];
        self.conversion_scopes.iter()
            .fold(expansions, |mut acc, (path, conversions)| {

                conversions.iter()
                    .for_each(|c| {
                        let expansions = c.expand_all_types();
                        let scope = quote! {
                            pub mod #path {
                                #(#expansions)*
                            }
                        };
                        acc.push(quote!(#scope));

                    });
                acc
            })
    }
}


impl VisitMut for FileVisitor {
    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        println!("----> {}", quote!(#node));
        node.fields.iter_mut().for_each(|field| {
            field.ty = self.update_nested_generics(&field.ty);
        });
        println!("<---- {}", quote!(#node));
        syn::visit_mut::visit_item_struct_mut(self, node);
    }
}
