use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use quote::{format_ident, quote, ToTokens};
use syn::{GenericArgument, Ident, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, parse_quote, Path, PathArguments, PathSegment, QSelf, Token, TraitBound, Type, TypeParamBound, TypePath, TypeTraitObject, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use crate::composition::{QSelfComposition, TypeComposition};
use crate::context::{GlobalContext, TraitCompositionPart1, VisitorContext};
use crate::conversion::{Conversion, MacroType, ItemConversion, TypeConversion};
use crate::formatter::{Emoji, format_token_stream, format_trait_decomposition_part1, format_types_dict};
use crate::holder::{PathHolder, TypeHolder, TypePathHolder};
use crate::idents::is_primitive;
use crate::nprint;
use crate::tree::ScopeTreeExportItem;

pub struct Visitor {
    pub context: Arc<RwLock<GlobalContext>>,
    pub parent: PathHolder,
    pub inner_visitors: Vec<Visitor>,
    pub tree: ScopeTreeExportItem,
    pub current_scope_stack: Vec<Ident>,
    pub current_module_scope: PathHolder,
}

impl std::fmt::Debug for Visitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Visitor")
            .field("context", &self.context)
            .field("parent", &self.parent.to_token_stream().to_string())
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

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        self.add_conversion(Item::Impl(node.clone()));
    }
}

impl Visitor {
    /// path: full-qualified Path for file
    pub(crate) fn new(scope: &PathHolder, context: &Arc<RwLock<GlobalContext>>) -> Self {
        Self {
            context: context.clone(),
            parent: scope.clone(),
            current_module_scope: scope.clone(),
            current_scope_stack: vec![],
            inner_visitors: vec![],
            tree: ScopeTreeExportItem::with_global_context(scope, context.clone())
        }
    }

    pub(crate) fn add_full_qualified_trait_match(&mut self, scope: &PathHolder, item_trait: &ItemTrait) {
        // println!("add_full_qualified_trait_match: {}: {}", format_token_stream(scope), format_token_stream(&item_trait.ident));
        let mut lock = self.context.write().unwrap();
        lock.traits_dictionary
            .entry(scope.clone())
            .or_default()
            .insert(item_trait.ident.clone(), TraitCompositionPart1::new(item_trait.clone()));
    }
    pub(crate) fn add_full_qualified_generic_match(&mut self, self_scope: &PathHolder, generics: HashMap<PathHolder, Vec<Path>>) {
        let mut lock = self.context.write().unwrap();
        lock.scope_generics_mut(&self_scope)
            .extend(generics);
    }

    pub(crate) fn add_full_qualified_type_match(&mut self, scope: &PathHolder, self_scope: &PathHolder, ty: &Type, visitor_context: &VisitorContext) {
        println!();
        nprint!(0, Emoji::Plus, "[{}] [{}] {}", scope, self_scope, format_token_stream(ty));
        let all_involved_full_types = <TypePathHolder as Conversion>::nested_items(ty, visitor_context);
        // let conversion = TypeHolder::from(ty);
        // let all_involved_full_types = conversion.get_all_type_paths_involved()
        let all_involved_full_types = all_involved_full_types
            .into_iter()
            .map(|ty| {
                let tp: TypePath = parse_quote!(#ty);
                // let ty = parse_quote!(#tp);
                let mut counter = 1;
                let type_composition = self.update_nested_generics(scope, self_scope, &ty, &mut counter, visitor_context);
                nprint!(counter,
                    Emoji::Question, "[{}] {}",
                    format_token_stream(&ty),
                    format_token_stream(&type_composition));

                (TypeHolder::from(&ty), match tp.path.segments.first().unwrap().ident.to_string().as_str() {
                    "Self" => match visitor_context {
                        VisitorContext::Trait(decomposition) => {
                            println!("===> add_full_qualified_type_match:: Trait {}:", format_token_stream(&type_composition));
                            println!("===>  {}", decomposition.as_ref().map_or(format!("None"), |f| format_trait_decomposition_part1(f)));
                            TypeConversion::Trait(type_composition, decomposition.clone().unwrap())
                        },
                        VisitorContext::Object => TypeConversion::Object(type_composition),
                        VisitorContext::Unknown => TypeConversion::Unknown(type_composition)
                    },
                    id => {
                        let lock = self.context.read().unwrap();
                        let known = lock.maybe_scope_type_or_parent_type(&ty, scope);
                        println!("check: {}: {}", id, format_token_stream(&known));
                        if let Some(known) = known {
                            known.clone()
                        } else if is_primitive(id) {
                            TypeConversion::Primitive(type_composition)
                        } else {
                            TypeConversion::Unknown(type_composition)
                        }
                    }
                })
        }).collect::<HashMap<_, _>>();
        println!("{}", format_types_dict(&all_involved_full_types));
        let mut lock = self.context.write().unwrap();
        lock.scope_types_mut(self_scope)
            .extend(all_involved_full_types);
    }

    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    pub(crate) fn fold_import_tree(&mut self, scope: &PathHolder, use_tree: &UseTree, mut current_path: Vec<Ident>) {
        match use_tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                current_path.push(ident.clone());
                self.fold_import_tree(scope,tree, current_path);
            },
            UseTree::Name(UseName { ident, .. }) |
            UseTree::Rename(UseRename { rename: ident, .. }) => {
                current_path.push(ident.clone());
                let mut lock = self.context.write().unwrap();
                lock.used_imports_at_scopes
                    .entry(scope.clone())
                    .or_default()
                    .insert(parse_quote!(#ident), Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) });
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

    fn handle_full_path(&self, scope: &PathHolder, self_scope: &PathHolder, path: &Path, qself: &Option<QSelfComposition>, counter: &mut usize, visitor_context: &VisitorContext) -> TypeComposition {
        nprint!(*counter, Emoji::Branch, "handle_full_path: {} with qself: [{}] in [{}, {}]", format_token_stream(path), qself.as_ref().map_or(format!("None"), |q| format_token_stream(&q.qself.ty)), format_token_stream(self_scope), format_token_stream(scope));

        let mut segments = path.segments.clone();
        for segment in &mut segments {
            //println!("argggg (segment): {}", segment.to_token_stream());
            if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                for arg in &mut angle_bracketed_generic_arguments.args {
                    match arg {
                        GenericArgument::Type(inner_type) => {
                            let ty_composition = self.update_nested_generics(scope, self_scope, inner_type, counter, visitor_context);
                            let ty = ty_composition.ty();
                            *arg = GenericArgument::Type(ty.clone())
                        },
                        _ => {}
                    }
                }
            }
        }

        let first_segment = &segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = &segments.last().unwrap();
        let last_ident = &last_segment.ident;
        let import_seg: PathHolder = parse_quote!(#first_ident);
        let lock = self.context.read().unwrap();

        if let Some(replacement_path) = lock.maybe_scope_import_path_or_parent(self_scope, scope, &import_seg).cloned() {
            nprint!(*counter, Emoji::Local, "(ScopeImport) {}", format_token_stream(&replacement_path));
            let last_segment = segments.pop().unwrap();
            segments.extend(replacement_path.segments.clone());
            segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
        } else if let Some(generic_bounds) = lock.maybe_generic_bounds(self_scope, &import_seg) {
            let first_bound = generic_bounds.first().unwrap();
            let first_bound_as_scope = PathHolder::from(first_bound);
            if let Some(imported) = lock.maybe_scope_import_path_or_parent(self_scope, scope, &first_bound_as_scope).cloned() {
                nprint!(*counter, Emoji::Local, "(Generic Bounds Imported) {}", format_token_stream(&segments));
                let last_segment = segments.pop().unwrap();
                segments.extend(imported.segments.clone());
                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;

            } else {
                nprint!(*counter, Emoji::Local, "(Generic Bounds Local) {}", format_token_stream(&segments));
                let last_segment = segments.pop().unwrap();
                let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#scope::#first_bound);
                segments.extend(new_segments);
                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
            }
        } else {
            nprint!(*counter, Emoji::Local, "(Local or Global) {}", format_token_stream(&segments));
            match first_ident.to_string().as_str() {
                "Self" if segments.len() <= 1 => {
                    nprint!(*counter, Emoji::Local, "(Self) {}", format_token_stream(first_ident));
                    let last_segment = segments.pop().unwrap();
                    let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#self_scope);
                    segments.extend(new_segments);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                },
                "Self" => {
                    let tail = segments.iter().skip(1).cloned().collect::<Vec<_>>();
                    let last_segment = segments.pop().unwrap();
                    nprint!(*counter, Emoji::Local, "(SELF::->) {}: {}", format_token_stream(&last_segment), format_token_stream(&last_segment.clone().into_value().arguments));
                    let new_path: Path = parse_quote!(#self_scope::#(#tail)::*);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    segments.clear();
                    segments.extend(new_path.segments);
                },
                "Send" | "Sync" | "Clone" => {
                    // nprint!(*counter, Emoji::Nothing, "(Global Trait) {}", format_token_stream(&path));
                },
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool" => {
                    // nprint!(*counter, Emoji::Nothing, "(Primitive Object) {}", format_token_stream(&path));
                },
                "str" | "String" | "Option" | "Box" | "Vec" => {
                    // nprint!(*counter, Emoji::Nothing, "(Global Object) {}", format_token_stream(&path));
                },
                "Result" if segments.len() == 1 => {},
                _ if last_ident.to_string().as_str() == "BTreeMap" || last_ident.to_string().as_str() == "HashMap" => {

                },
                _ => {
                    let len = segments.len();
                    if len == 1 {
                        nprint!(*counter, Emoji::Local, "(Local join single: {}) {} + {}", first_ident, format_token_stream(scope), format_token_stream(&path));
                        let last_segment = segments.pop().unwrap();
                        let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#scope::#path);
                        segments.extend(new_segments);
                        segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    } else {
                        let tail: Vec<_> = segments.iter().skip(1).cloned().collect();
                        if let Some(QSelfComposition { qs: _, qself: QSelf { ty, .. } }) = qself {
                            nprint!(*counter, Emoji::Local, "(Local join QSELF: {} [{}]) {} + {}", format_token_stream(ty), format_token_stream(&import_seg), format_token_stream(scope), format_token_stream(&path));
                            let tt = lock.maybe_scope_import_path_or_parent(self_scope, scope, &import_seg)
                                .cloned()
                                .unwrap_or(parse_quote!(#scope::#import_seg));
                            // let last_segment = segments.pop().unwrap();
                            let tail_path = quote!(#(#tail)::*);
                            println!("{}: <{} as {}>::{}", tail.len(), format_token_stream(ty), format_token_stream(&tt), format_token_stream(&tail_path));
                            return TypeComposition::Unknown(match len {
                                0 => parse_quote!(<#ty as #tt>),
                                _ => parse_quote!(<#ty as #tt>::#tail_path)
                            });
                        } else {
                            nprint!(*counter, Emoji::Local, "(Local join multi: {}) {} + {}", first_ident, format_token_stream(scope), format_token_stream(&path));
                            let last_segment = segments.pop().unwrap();
                            let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#(#tail)::*);
                            segments.extend(new_segments);
                            segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                        }
                    }
                },
            }
        }
        *counter += 1;
        TypeComposition::Unknown(Type::Path(TypePath { qself: qself.as_ref().map(|q| q.qself.clone()), path: Path { leading_colon: path.leading_colon, segments } }))
    }

    fn handle_qself(&self, scope: &PathHolder, self_scope: &PathHolder, qself: &Option<QSelf>, counter: &mut usize, visitor_context: &VisitorContext) -> Option<QSelfComposition> {
        qself.as_ref().map(|qself| {
            let mut new_qself = qself.clone();
            let qs = self.update_nested_generics(scope, self_scope, &qself.ty, counter, visitor_context);
            new_qself.ty = Box::new(qs.ty().clone());
            QSelfComposition { qs, qself: new_qself }
        })
    }

    fn update_type_path(&self, scope: &PathHolder, self_scope: &PathHolder, type_path: &TypePath, counter: &mut usize, visitor_context: &VisitorContext) -> TypeComposition {
        // println!("update_type_path: {}", format_token_stream(type_path));
        let qself = self.handle_qself(scope, self_scope, &type_path.qself, counter, visitor_context);

        self.handle_full_path(scope, self_scope, &type_path.path, &qself, counter, visitor_context)
    }

    fn update_type_trait_object(&self, scope: &PathHolder, self_scope: &PathHolder, type_trait_object: &TypeTraitObject, counter: &mut usize, visitor_context: &VisitorContext) -> TypeComposition {
        let TypeTraitObject { dyn_token, bounds } = type_trait_object;
        let mut bounds = bounds.clone();
        bounds.iter_mut().for_each(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => {
                let full_path = self.handle_full_path(scope, self_scope, path, &None, counter, visitor_context);
                let ty = full_path.ty();
                *path = parse_quote!(#ty);
            },
            _ => {},
        });
        TypeComposition::Unknown(Type::TraitObject(TypeTraitObject {
            dyn_token: dyn_token.clone(),
            bounds
        }))
    }

    /// Create a new Type with the updated base path and generic type parameters
    /// `BTreeMap<u32, u32>` -> `std::collections::BTreeMap<u32, u32>`,
    /// `BTreeMap<u32, BTreeMap<u32, u32>>` -> `std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>`
    fn update_nested_generics(&self, scope: &PathHolder, self_scope: &PathHolder, ty: &Type, counter: &mut usize, visitor_context: &VisitorContext) -> TypeComposition {
        nprint!(*counter, Emoji::Node, "=== {}", format_token_stream(ty));
        *counter += 1;
        match ty {
            Type::Path(type_path) => {
                self.update_type_path(scope, self_scope, type_path, counter, visitor_context)
            },
            Type::TraitObject(type_trait_object) =>
                self.update_type_trait_object(scope, self_scope, type_trait_object, counter, visitor_context),
            tttt =>
                TypeComposition::Unknown(tttt.clone())
        }
    }

    fn current_scope_for(&self, item: &Item) -> PathHolder {
        let is_mod = matches!(item, Item::Mod(..));
        match self.current_scope_stack.first() {
            Some(current_mod) if !is_mod => self.current_module_scope.joined(current_mod),
            _ => self.current_module_scope.clone()
        }
    }

    fn find_scope_tree<'a>(&'a mut self, scope: &PathHolder) -> &'a mut ScopeTreeExportItem {
        let mut current_tree = &mut self.tree;
        let path_to_traverse: Vec<Ident> = scope.0.segments.iter().skip(1).map(|segment| segment.ident.clone()).collect();
        for ident in &path_to_traverse {
            match current_tree {
                ScopeTreeExportItem::Item(..) => panic!("Unexpected item while traversing the scope path"),  // Handle as appropriate
                ScopeTreeExportItem::Tree(scope_context, _, _, exported) => {
                    if !exported.contains_key(ident) {
                        exported.insert(ident.clone(), ScopeTreeExportItem::with_scope_context(scope_context.clone()));
                    }
                    current_tree = exported.get_mut(ident).unwrap();
                }
            }
        }
        current_tree
    }

    pub fn add_conversion(&mut self, item: Item) {
        let scope = self.current_scope_for(&item);
        if let Ok(conversion) = ItemConversion::try_from((item, &scope)) {
            match conversion.macro_type() {
                Some(MacroType::Export) => {
                    let full_qualified = conversion.add_full_qualified_conversion(self);
                    self.find_scope_tree(&scope).add_item(full_qualified)
                },
                Some(MacroType::Register(ty)) => {
                    if let ScopeTreeExportItem::Tree(item_context, _, _, _) = self.find_scope_tree(&scope) {
                        let ident = conversion.ident();

                        let ffi_type = parse_quote!(#scope::#ident);
                        item_context.add_custom_conversion(scope, ty, ffi_type);
                    }
                },
                _ if conversion.is_impl() && conversion.ident() != format_ident!("FFIConversion") => {
                    // println!("IMPL :::: {}", format_token_stream(&conversion.ident()));
                    conversion.add_full_qualified_conversion(self);
                    // if let ScopeTreeExportItem::Item(item_context, _) = self.find_scope_tree(&scope) {
                    //     let ident = conversion.ident();
                    //     let ffi_type = parse_quote!(#scope::#ident);
                    //     item_context.add_custom_conversion(scope, ty, ffi_type);
                    //
                    // }
                },
                _ => {}
            }
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
    if let (ScopeTreeExportItem::Tree(_dest_context, _, _, dest_exports),
        ScopeTreeExportItem::Tree(_source_context, _, _, source_exports), ) = (destination, source) {
        // println!("merge_trees: source: {}", source_context);
        // println!("merge_trees: destination: {}", dest_context);
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
}
