use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, ConstParam, Field, FnArg, GenericArgument, GenericParam, Ident, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Path, PathArguments, PathSegment, PatType, PredicateType, QSelf, ReturnType, Signature, Token, TraitBound, TraitItem, TraitItemConst, TraitItemMethod, TraitItemType, Type, TypeParam, TypeParamBound, TypePath, TypeTraitObject, UseGroup, UseName, UsePath, UseRename, UseTree, Variant, WhereClause, WherePredicate};
use syn::punctuated::Punctuated;
use syn::token::{Add, Colon2};
use syn::visit::Visit;
use crate::composition::{QSelfComposition, TraitDecompositionPart1, TraitCompositionPart1, TypeComposition};
use crate::context::{GlobalContext, Scope, ScopeChain};
use crate::conversion::{Conversion, ItemConversion, MacroType, ObjectConversion, ScopeItemConversion, TypeConversion};
use crate::formatter::{Emoji, format_token_stream, format_types_dict, generic_bounds_dict};
use crate::helper::ident_from_item;
use crate::holder::{PathHolder, TypeHolder, TypePathHolder};
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
        // TODO: what to do with fn-level use statement?
        let scope = ScopeChain::Mod {
            self_scope: Scope::new(self.current_scope_for(&item), ObjectConversion::Empty)
        };
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
    pub(crate) fn new(scope: ScopeChain, context: &Arc<RwLock<GlobalContext>>) -> Self {
        Self {
            context: context.clone(),
            parent: scope.self_scope().self_scope.clone(),
            current_module_scope: scope.self_scope().self_scope.clone(),
            current_scope_stack: vec![],
            inner_visitors: vec![],
            tree: ScopeTreeExportItem::with_global_context(scope, context.clone())
        }
    }

    pub(crate) fn add_full_qualified_trait_match(&mut self, scope: &ScopeChain, item_trait: &ItemTrait, itself: &ObjectConversion) {
        println!("add_full_qualified_trait_match: {}: {}", format_token_stream(scope), format_token_stream(&item_trait.ident));
        let mut lock = self.context.write().unwrap();
        lock.traits_dictionary
            .entry(scope.self_scope().self_scope.clone())
            .or_default()
            .insert(item_trait.ident.clone(), TraitCompositionPart1::new(item_trait.clone()));
    }
    pub(crate) fn add_full_qualified_generic_match(&mut self, scope: &ScopeChain, generics: HashMap<PathHolder, Vec<Path>>) {
        println!("add_full_qualified_generic_match: [{}]: {}", scope, generic_bounds_dict(&generics).join("\n"));
        let mut lock = self.context.write().unwrap();
        lock.scope_generics_mut(&scope.self_scope().self_scope)
            .extend(generics);
    }

    // fn self_type_conversion(visitor_context: VisitorContext, type_composition: TypeComposition) -> ObjectConversion {
    //     match visitor_context {
    //         VisitorContext::Trait(decomposition) |
    //         VisitorContext::TraitFn(decomposition) =>
    //             ObjectConversion::Type(TypeConversion::Trait(type_composition, decomposition.unwrap())),
    //         VisitorContext::Object =>
    //             ObjectConversion::Type(TypeConversion::Object(type_composition)),
    //         VisitorContext::Unknown =>
    //             ObjectConversion::Type(TypeConversion::Unknown(type_composition)),
    //     }
    // }

    pub(crate) fn add_full_qualified_type_match(&mut self, scope: ScopeChain, ty: &Type) {
        println!();
        nprint!(0, Emoji::Plus, "{} in [{}]", format_token_stream(ty), scope);
        let all_involved_full_types = <TypePathHolder as Conversion>::nested_items(ty, &scope);

        let all_involved_full_types = all_involved_full_types
            .into_iter()
            .map(|ty| {
                // let tp: TypePath = parse_quote!(#ty);
                // let path_holder = PathHolder::from(&tp.path);
                let mut counter = 1;
                let obj_conversion = self.update_nested_generics(&scope, &ty, &mut counter);
                // let type_composition = self.update_nested_generics(&scope, &ty, &mut counter);
                nprint!(counter, Emoji::Question, "[{}] {}", format_token_stream(&ty), obj_conversion);
                // let first_ident = &tp.path.segments.first().unwrap().ident;
                (TypeHolder::from(&ty), obj_conversion)
                // (TypeHolder::from(&ty), match first_ident.to_string().as_str() {
                //     "Self" => obj_conversion,
                //         // match scope.obj_root_chain() {
                //         //     Some(ScopeChain::Trait { .. }) =>
                //         //         ObjectConversion::Type(TypeConversion::Trait(type_composition)),
                //         //     Some(ScopeChain::Object { .. }) => ObjectConversion::Type(TypeConversion::Object(type_composition)),
                //         //     Some(ScopeChain::Impl { .. }) => ObjectConversion::Type(TypeConversion::Object(type_composition)),
                //         //     _ => ObjectConversion::Empty,
                //         // }
                //
                //         // Self::self_type_conversion(scope.clone(), type_composition),
                //     id => {
                //         let lock = self.context.read().unwrap();
                //         let known = lock.maybe_type(&ty, &scope).cloned();
                //         // let known = lock.maybe_scope_type_or_parent_type(&ty, scope);
                //         println!("check as scope/parent type: {}: {}", id, format_token_stream(&known));
                //         let known_generics = lock.maybe_generic_bounds(&scope, &path_holder);
                //         println!("check as scope/parent generic: {}: {}", id, known_generics.map_or(format!("None"), |known| format_path_vec(known)));
                //         if let Some(known) = known {
                //             known.clone()
                //         } else if matches!(id, "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool") {
                //             ObjectConversion::Type(TypeConversion::Primitive(type_composition))
                //         } else if matches!(id, "String" | "str") {
                //             ObjectConversion::Type(TypeConversion::Object(type_composition))
                //         } else {
                //             ObjectConversion::Type(TypeConversion::Unknown(type_composition))
                //         }
                //     }
                // })
        }).collect::<HashMap<_, _>>();
        println!("{}", format_types_dict(&all_involved_full_types));
        let mut lock = self.context.write().unwrap();
        lock.scope_types_mut(&scope.self_scope().self_scope)
            .extend(all_involved_full_types);
    }

    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    pub(crate) fn fold_import_tree(&mut self, scope: &ScopeChain, use_tree: &UseTree, mut current_path: Vec<Ident>) {
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
                    .entry(scope.self_scope().self_scope.clone())
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

    fn handle_full_path(&self, scope: &ScopeChain, path: &Path, qself: &Option<QSelfComposition>, counter: &mut usize) -> ObjectConversion {
        // nprint!(*counter, Emoji::Branch, "handle_full_path: {} with qself: [{}] in {}",
        //     format_token_stream(path),
        //     qself.as_ref().map_or(format!("None"), |q| format_token_stream(&q.qself.ty)),
        //     scope);
        let new_qself = qself.as_ref().map(|q| q.qself.clone());
        let mut segments = path.segments.clone();
        for segment in &mut segments {
            //println!("argggg (segment): {}", segment.to_token_stream());
            if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                for arg in &mut angle_bracketed_generic_arguments.args {
                    match arg {
                        GenericArgument::Type(inner_type) => {
                            let obj_conversion = self.update_nested_generics(scope,inner_type, counter);
                            println!("nested :::: {}", obj_conversion);
                            *arg = GenericArgument::Type(obj_conversion.ty().cloned().unwrap())
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

        // println!("handle_full_path.2: {}", scope);
        let self_scope = scope.self_scope();



        // let obj_conversion = if let Some(bounds_composition) = scope.maybe_generic_bound_for_path(&import_seg.0) {
        let obj_conversion = if let Some(dict_type_composition) = scope.maybe_dictionary_type(&import_seg.0) {
            ObjectConversion::Type(dict_type_composition)
        } else if let Some(bounds_composition) = scope.maybe_generic_bound_for_path(&import_seg.0) {
            nprint!(*counter, Emoji::Local, "(Local Generic Bound) {}", bounds_composition);
            ObjectConversion::Type(TypeConversion::Bounds(bounds_composition))
        } else if let Some(replacement_path) = lock.maybe_import(scope, &import_seg).cloned() {
        // let obj_conversion = if let Some(replacement_path) = lock.maybe_import(scope, &import_seg).cloned() {
            nprint!(*counter, Emoji::Local, "(ScopeImport) {}", format_token_stream(&replacement_path));
            let last_segment = segments.pop().unwrap();
            segments.extend(replacement_path.segments.clone());
            segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
            ObjectConversion::Type(
                TypeConversion::Unknown(
                    TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
        } else if let Some(generic_bounds) = lock.maybe_generic_bounds(scope, &import_seg) {
            let first_bound = generic_bounds.first().unwrap();
            let first_bound_as_scope = PathHolder::from(first_bound);
            if let Some(imported) = lock.maybe_import(scope, &first_bound_as_scope).cloned() {
                nprint!(*counter, Emoji::Local, "(Generic Bounds Imported) {}", format_token_stream(&segments));
                let last_segment = segments.pop().unwrap();
                segments.extend(imported.segments.clone());
                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
            } else {
                nprint!(*counter, Emoji::Local, "(Generic Bounds Local) {}", format_token_stream(&segments));
                let last_segment = segments.pop().unwrap();
                let scope = &scope.self_scope().self_scope;
                let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#scope::#first_bound);
                segments.extend(new_segments);
                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
            }
            ObjectConversion::Type(
                TypeConversion::TraitType(
                    TypeComposition::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
        } else {
            // if let Some(same_mod_defined_obj) = lock.mayb
            nprint!(*counter, Emoji::Local, "(Local or Global ....) {}", format_token_stream(&segments));
            let self_scope_path = &self_scope.self_scope;
            match first_ident.to_string().as_str() {
                "Self" if segments.len() <= 1 => {
                    nprint!(*counter, Emoji::Local, "(Self) {}", format_token_stream(first_ident));
                    let last_segment = segments.pop().unwrap();
                    let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#self_scope_path);
                    segments.extend(new_segments);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    self_scope.object.clone()
                    // match scope.obj_root_chain() {
                    //     Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                    //         ObjectConversion::Type(
                    //             TypeConversion::Object(
                    //                 TypeComposition::new(
                    //                     Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: path.leading_colon, segments } }),
                    //                     None))),
                    //     Some(ScopeChain::Trait { .. }) =>
                    //         ObjectConversion::Type(
                    //             TypeConversion::TraitType(
                    //                 TypeComposition::new(
                    //                     Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: path.leading_colon, segments } }),
                    //                     None))),
                    //     _ => panic!("Unexpected scope obj root chain")
                    // }
                },
                "Self" => {
                    let tail = segments.iter().skip(1).cloned().collect::<Vec<_>>();
                    let last_segment = segments.pop().unwrap();
                    nprint!(*counter, Emoji::Local, "(SELF::->) {}: {}", format_token_stream(&last_segment), format_token_stream(&last_segment.clone().into_value().arguments));
                    let new_path: Path = parse_quote!(#self_scope_path::#(#tail)::*);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    segments.clear();
                    segments.extend(new_path.segments);

                    match scope.obj_root_chain() {
                        Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                            ObjectConversion::Type(
                                TypeConversion::Object(
                                    TypeComposition::new(
                                        Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: path.leading_colon, segments } }),
                                        None))),
                        Some(ScopeChain::Trait { .. }) =>
                            ObjectConversion::Type(
                                TypeConversion::TraitType(
                                    TypeComposition::new(
                                        Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: path.leading_colon, segments } }),
                                        None))),
                        _ => panic!("Unexpected scope obj root chain")
                    }

                },
                "Send" | "Sync" | "Clone" | "Sized" => {
                    ObjectConversion::Type(TypeConversion::TraitType(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
                    // nprint!(*counter, Emoji::Nothing, "(Global Trait) {}", format_token_stream(&path));
                },
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool" => {
                    ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
                    // nprint!(*counter, Emoji::Nothing, "(Primitive Object) {}", format_token_stream(&path));
                },
                "str" | "String" | "Option" | "Box" | "Vec" => {
                    ObjectConversion::Type(TypeConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
                    // nprint!(*counter, Emoji::Nothing, "(Global Object) {}", format_token_stream(&path));
                },
                "Result" if segments.len() == 1 => {
                    ObjectConversion::Type(TypeConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
                },
                _ if last_ident.to_string().as_str() == "BTreeMap" || last_ident.to_string().as_str() == "HashMap" => {
                    ObjectConversion::Type(TypeConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
                },
                _ => {


                    let len = segments.len();
                    if len == 1 {

                        //ScopeChain::Object (self: crate::model::snapshot::LLMQSnapshot, parent: ScopeChain::Mod (self: crate::model::snapshot))
                        let parent_scope = scope.parent_scope();
                        match parent_scope {
                            None => {
                                // Global
                                nprint!(*counter, Emoji::Local, "(Local join single (has no parent scope): {}) {} + {}", first_ident, scope, format_token_stream(&path));
                                let last_segment = segments.pop().unwrap();
                                let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#scope::#path);
                                segments.extend(new_segments);
                                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                                ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new(
                                    Type::Path(
                                        TypePath {
                                            qself: new_qself,
                                            path: Path { leading_colon: path.leading_colon, segments } }),
                                    None)))
                            },
                            Some(parent) => {
                                // match parent.obj_root_chain() {
                                //     ScopeChain::CrateRoot { .. } => {}
                                //     ScopeChain::Mod { .. } => {}
                                //     ScopeChain::Trait { .. } => {}
                                //     ScopeChain::Fn { .. } => {}
                                //     ScopeChain::Object { .. } => {}
                                //     ScopeChain::Impl { .. } => {}
                                // }
                                let scope = &parent.self_scope().self_scope;
                                nprint!(*counter, Emoji::Local, "(Local join single (has parent scope): {}) {} + {}", first_ident, scope, format_token_stream(&path));
                                let last_segment = segments.pop().unwrap();
                                let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#scope::#path);
                                segments.extend(new_segments);
                                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                                ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new(
                                    Type::Path(
                                        TypePath {
                                            qself: new_qself,
                                            path: Path { leading_colon: path.leading_colon, segments } }),
                                    None)))
                            }
                        }

                    } else {
                        let tail: Vec<_> = segments.iter().skip(1).cloned().collect();
                        if let Some(QSelfComposition { qs: _, qself: QSelf { ty, .. } }) = qself {
                            nprint!(*counter, Emoji::Local, "(Local join QSELF: {} [{}]) {} + {}", format_token_stream(ty), format_token_stream(&import_seg), format_token_stream(scope), format_token_stream(&path));
                            let tt = lock.maybe_scope_import_path_or_parent(&self_scope_path, scope, &import_seg)
                                .cloned()
                                .unwrap_or(parse_quote!(#scope::#import_seg));
                            let tail_path = quote!(#(#tail)::*);
                            println!("{}: <{} as {}>::{}", tail.len(), format_token_stream(ty), format_token_stream(&tt), format_token_stream(&tail_path));
                            match scope.obj_root_chain() {
                                Some(ScopeChain::Trait { .. }) =>
                                    ObjectConversion::Type(TypeConversion::TraitType(TypeComposition {
                                        ty: match len {
                                            0 => parse_quote!(<#ty as #tt>),
                                            _ => parse_quote!(<#ty as #tt>::#tail_path)
                                        },
                                        generics: None,
                                    })),
                                Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                                    ObjectConversion::Type(TypeConversion::Object(TypeComposition {
                                        ty: match len {
                                            0 => parse_quote!(<#ty as #tt>),
                                            _ => parse_quote!(<#ty as #tt>::#tail_path)
                                        },
                                        generics: None,
                                    })),
                                _ => ObjectConversion::Type(TypeConversion::Unknown(TypeComposition {
                                    ty: match len {
                                        0 => parse_quote!(<#ty as #tt>),
                                        _ => parse_quote!(<#ty as #tt>::#tail_path)
                                    },
                                    generics: None,
                                }))
                            }

                            // ObjectConversion::Type(TypeConversion)
                            // return TypeComposition {
                            //     ty: match len {
                            //         0 => parse_quote!(<#ty as #tt>),
                            //         _ => parse_quote!(<#ty as #tt>::#tail_path)
                            //     },
                            //     generics: None,
                            // };
                        } else {
                            nprint!(*counter, Emoji::Local, "(Local join multi: {}) {} + {}", first_ident, format_token_stream(scope), format_token_stream(&path));
                            let last_segment = segments.last().cloned().unwrap();
                            let new_segments: Punctuated<PathSegment, Colon2> = parse_quote!(#scope::#path);
                            segments.clear();
                            segments.extend(new_segments);
                            segments.last_mut().unwrap().arguments = last_segment.arguments;

                            ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new(
                                Type::Path(
                                    TypePath {
                                        qself: new_qself,
                                        path: Path { leading_colon: path.leading_colon, segments } }),
                                None)))
                            // let last_segment = segments.pop().unwrap();
                            // let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#(#tail)::*);
                            // segments.extend(new_segments);
                            // segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                        }
                    }
                },
            }
        };
        *counter += 1;
        obj_conversion
        // TypeComposition::new(Type::Path(TypePath { qself: qself.as_ref().map(|q| q.qself.clone()), path: Path { leading_colon: path.leading_colon, segments } }), None)
    }

    fn handle_qself(&self, scope: &ScopeChain, qself: &Option<QSelf>, counter: &mut usize) -> Option<QSelfComposition> {
        qself.as_ref().map(|qself| {
            let mut new_qself = qself.clone();
            let qs = self.update_nested_generics(scope, &qself.ty, counter);
            let qs = qs.type_conversion().unwrap().ty_composition().clone();
            new_qself.ty = Box::new(qs.ty.clone());
            QSelfComposition { qs, qself: new_qself }
        })
    }

    /// Create a new Type with the updated base path and generic type parameters
    /// `BTreeMap<u32, u32>` -> `std::collections::BTreeMap<u32, u32>`,
    /// `BTreeMap<u32, BTreeMap<u32, u32>>` -> `std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>`
    fn update_nested_generics(&self, scope: &ScopeChain, ty: &Type, counter: &mut usize) -> ObjectConversion {
        nprint!(*counter, Emoji::Node, "=== {} ({:?})", format_token_stream(ty), ty);
        *counter += 1;
        match ty {
            Type::Path(type_path) => {
                let qself = self.handle_qself(scope, &type_path.qself, counter);
                self.handle_full_path(scope, &type_path.path, &qself, counter)
            },
            Type::TraitObject(type_trait_object) => {
                let TypeTraitObject { dyn_token, bounds } = type_trait_object;
                let mut bounds = bounds.clone();
                bounds.iter_mut().for_each(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) => {
                        let full_path = self.handle_full_path(scope, path, &None, counter);
                        let ty = full_path.ty().unwrap();
                        *path = parse_quote!(#ty);
                    },
                    _ => {},
                });
                ObjectConversion::Type(TypeConversion::TraitType(TypeComposition::new(Type::TraitObject(TypeTraitObject {
                    dyn_token: dyn_token.clone(),
                    bounds
                }), None)))
            },
            tttt =>
                ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new(tttt.clone(), None)))
                // TypeComposition::new(tttt.clone(), None)
        }
    }

    fn current_scope_for(&self, item: &Item) -> PathHolder {
        if self.current_scope_stack.is_empty() || matches!(item, &Item::Mod(..)) {
            self.current_module_scope.clone()
        } else {
            self.current_module_scope.joined_chunk(&self.current_scope_stack)
        }
    }

    fn find_scope_tree(&mut self, scope: &PathHolder) -> &mut ScopeTreeExportItem {
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


    fn add_itself_conversion(&mut self, scope: &ScopeChain, ident: &Ident, ty: ObjectConversion) {
        let mut lock = self.context.write().unwrap();
        lock.scope_types_mut(&scope.self_scope().self_scope)
            .insert(parse_quote!(#ident), ty);
    }
    fn add_full_qualified_trait_type_from_macro(&mut self, item_trait_attrs: &[Attribute], scope: &ScopeChain) {
        let trait_names = extract_trait_names(item_trait_attrs);
        // let self_scope = scope.joined(ident);
        trait_names.iter().for_each(|trait_name|
            self.add_full_qualified_type_match(scope.clone(), &parse_quote!(#trait_name)));
        let mut lock = self.context.write().unwrap();
        lock.used_traits_dictionary
            .entry(scope.self_scope().self_scope.clone())
            .or_default()
            .extend(trait_names.iter().map(|trait_name| PathHolder::from(trait_name)));
        // let trait_names = extract_trait_names(item_trait_attrs);
        // let self_scope = scope.joined(ident);
        // trait_names.iter().for_each(|trait_name|
        //     self.add_full_qualified_type_match(&scope, &self_scope,&parse_quote!(#trait_name), &VisitorContext::Object));
        // let mut lock = self.context.write().unwrap();
        // lock.used_traits_dictionary
        //     .entry(self_scope)
        //     .or_default()
        //     .extend(trait_names.iter().map(|trait_name| PathHolder::from(trait_name)));
    }
    #[allow(unused)]
    fn add_full_qualified_impl(&mut self, item_impl: &ItemImpl, scope: &ScopeChain) {
        // let trait_path = item_impl.trait_.clone().map(|(_, path, _)| path);
        // let visitor_context = trait_path.map_or(VisitorContext::Object, |_| VisitorContext::Trait(None));
        // return;
        item_impl.items.iter().for_each(|impl_item| {
            match impl_item {
                ImplItem::Const(ImplItemConst { ty, .. }) => {
                    self.add_full_qualified_type_match(scope.clone(), ty);
                },
                ImplItem::Method(ImplItemMethod { sig, .. }) => {
                    self.add_full_qualified_signature(sig, scope)
                },
                ImplItem::Type(ImplItemType { ty, .. }) => {
                    self.add_full_qualified_type_match(scope.clone(), ty);
                },
                _ => {}
            }
        });
    }

    fn add_full_qualified_type_param_bounds(&mut self, bounds: Punctuated<TypeParamBound, Add>, scope: &ScopeChain) {
        bounds.iter().for_each(|bound| {
            match bound {
                TypeParamBound::Trait(TraitBound { path, .. }) => {
                    let ty = parse_quote!(#path);
                    self.add_full_qualified_type_match(scope.clone(), &ty);
                },
                TypeParamBound::Lifetime(_lifetime) => {}
            }
        });
    }

    fn add_full_qualified_trait(&mut self, item_trait: &ItemTrait, scope: ScopeChain) {
        println!("add_full_qualified_trait: {}: {}", item_trait.ident, scope);
        let ident = &item_trait.ident;
        let type_compo = TypeComposition::new(scope.to_type(), Some(item_trait.generics.clone()));
        let itself = ObjectConversion::Item(
            TypeConversion::Trait(
                type_compo,
                TraitDecompositionPart1::from_trait_items(ident, &item_trait.items)),
            ScopeItemConversion::Item(Item::Trait(item_trait.clone())));

        self.add_full_qualified_trait_match(&scope, item_trait, &itself);
        // let de_trait = TraitDecompositionPart1::from_trait_items(ident, &item_trait.items);
        // let de_trait_context = VisitorContext::Trait(Some(de_trait.clone()));
        let mut generics: HashMap<PathHolder, Vec<Path>> = HashMap::new();
        item_trait.generics.params.iter().for_each(|generic_param| {
            // println!("add_full_qualified_trait: generic: {}", quote!(#generic_param));
            match generic_param {
                GenericParam::Type(TypeParam { ident: generic_ident, bounds, .. }) => {
                    let mut de_bounds: Vec<Path> =  vec![];
                    bounds.iter().for_each(|bound| {
                        match bound {
                            TypeParamBound::Trait(TraitBound { path, .. }) => {
                                let ty = parse_quote!(#path);
                                // println!("add_full_qualified_trait: (generic trait): {}: {}", format_token_stream(generic_ident), format_token_stream(&ty));
                                de_bounds.push(path.clone());
                                self.add_full_qualified_type_match(scope.clone(), &ty);

                            },
                            TypeParamBound::Lifetime(_lifetime) => {}
                        }
                    });
                    generics.insert(parse_quote!(#generic_ident), de_bounds);
                },
                GenericParam::Lifetime(_lifetime) => {},
                GenericParam::Const(ConstParam { ty, .. }) => {
                    self.add_full_qualified_type_match(scope.clone(), ty);
                },
            }
        });
        match &item_trait.generics.where_clause {
            Some(WhereClause { predicates, .. }) => {
                predicates.iter().for_each(|predicate| match predicate {
                    WherePredicate::Type(PredicateType { bounds, bounded_ty, .. }) => {
                        let mut de_bounds: Vec<Path> =  vec![];
                        bounds.iter().for_each(|bound| {
                            match bound {
                                TypeParamBound::Trait(TraitBound { path, .. }) => {
                                    let ty = parse_quote!(#path);
                                    de_bounds.push(path.clone());
                                    self.add_full_qualified_type_match(scope.clone(), &ty);
                                },
                                TypeParamBound::Lifetime(_lifetime) => {}
                            }
                        });
                        // generics.insert(parse_quote!(#generic_ident), de_bounds);
                        self.add_full_qualified_type_match(scope.clone(), bounded_ty);
                    },
                    WherePredicate::Lifetime(_) => {}
                    WherePredicate::Eq(_) => {}
                })
            },
            None => {}
        }
        item_trait.supertraits.iter().for_each(|bound| {
            match bound {
                TypeParamBound::Trait(TraitBound { path, .. }) => {
                    let ty = parse_quote!(#path);
                    println!("add_full_qualified_trait: (super trait): {}", format_token_stream(&ty));
                    self.add_full_qualified_type_match(scope.clone(), &ty);
                },
                TypeParamBound::Lifetime(_lifetime) => {}
            }
        });

        item_trait.items.iter().for_each(|trait_item|
            match trait_item {
                TraitItem::Method(TraitItemMethod { sig, .. }) => {
                    // FnSignatureComposition::from_signature(sig), ::new(sig.clone(), &scope, &generics)
                    // let fn_scope = scope.joined_fn(&sig.ident);
                    let sig_ident = &sig.ident;
                    let self_scope = scope.self_scope();
                    let fn_scope = ScopeChain::Fn {
                        self_scope: Scope::new(
                            self_scope.self_scope.joined(&sig.ident),
                            ObjectConversion::Item(TypeConversion::Unknown(TypeComposition::new(parse_quote!(#sig_ident), Some(sig.generics.clone()))), ScopeItemConversion::Fn(sig.clone()))),
                        parent_scope_chain: Box::new(scope.clone())
                    };

                    self.add_full_qualified_signature(sig, &fn_scope);
                    // let mut de_bounds: Vec<Path> =  vec![];
                    // let scope = &self_scope;
                    // let self_scope = scope.joined(&sig.ident);
                    let mut item_local_generics: HashMap<PathHolder, Vec<Path>> = HashMap::new();
                    let _ = &sig.generics.params.iter().for_each(|generic_param| {
                        match generic_param {
                            GenericParam::Type(TypeParam { ident: generic_ident, bounds, .. }) => {
                                let mut de_bounds: Vec<Path> =  vec![];
                                println!("add_full_qualified_trait: (generic in fn signature): {}", quote!(#generic_param));
                                bounds.iter().for_each(|bound| {
                                    match bound {
                                        TypeParamBound::Trait(TraitBound { path, .. }) => {
                                            let ty = parse_quote!(#path);
                                            de_bounds.push(path.clone());
                                            self.add_full_qualified_type_match(fn_scope.clone(), &ty);

                                        },
                                        TypeParamBound::Lifetime(_lifetime) => {}
                                    }
                                });
                                item_local_generics.insert(parse_quote!(#generic_ident), de_bounds);
                            },
                            GenericParam::Lifetime(_lifetime) => {},
                            GenericParam::Const(ConstParam { ty, .. }) => {
                                self.add_full_qualified_type_match(fn_scope.clone(), ty);
                            },
                        }
                    });

                    match &sig.generics.where_clause {
                        Some(WhereClause { predicates, .. }) => {
                            predicates.iter().for_each(|predicate| match predicate {
                                WherePredicate::Type(PredicateType { bounds, bounded_ty, .. }) => {
                                    let mut de_bounds: Vec<Path> =  vec![];
                                    bounds.iter().for_each(|bound| {
                                        match bound {
                                            TypeParamBound::Trait(TraitBound { path, .. }) => {
                                                let ty = parse_quote!(#path);
                                                de_bounds.push(path.clone());
                                                println!("add_full_qualified_trait: (bound in fn where): {}", quote!(#ty));
                                                // let scope = &self_scope;
                                                // let self_scope = scope.joined(&sig.ident);
                                                self.add_full_qualified_type_match(fn_scope.clone(), &ty);
                                            },
                                            TypeParamBound::Lifetime(_lifetime) => {}
                                        }
                                    });
                                    // generics.insert(parse_quote!(#generic_ident), de_bounds);
                                    self.add_full_qualified_type_match(fn_scope.clone(), bounded_ty);
                                },
                                WherePredicate::Lifetime(_) => {}
                                WherePredicate::Eq(_) => {}
                            })
                        },
                        None => {}
                    }

                    // self.add_full_qualified_generic_match(&self_scope.joined(&sig.ident), item_local_generics);
                    self.add_full_qualified_generic_match(&fn_scope, item_local_generics);

                    // generics.insert(parse_quote!(#generic_ident), de_bounds);
                    // sig.generics.sig.generics.sig.generics.
                },
                TraitItem::Type(TraitItemType { ident: type_ident, bounds, ..}) => {
                    let local_ty = parse_quote!(Self::#type_ident);
                    self.add_full_qualified_type_match(scope.clone(), &local_ty);
                    println!("add_full_qualified_trait (type): {}: {}", ident, type_ident);
                    // TODO: whether we need to preserve scope or use separate scope + trait ident?
                    // Especially when using Self::  It'll break some logics
                    bounds.iter().for_each(|bound| match bound {
                        TypeParamBound::Trait(TraitBound { path, ..}) => {
                            let ty = parse_quote!(#path);
                            self.add_full_qualified_type_match(scope.clone(), &ty);
                        },
                        _ => {},
                    });
                },
                TraitItem::Const(TraitItemConst { ty, .. }) => {
                    self.add_full_qualified_type_match(scope.clone(), ty);
                },
                _ => {}
            });
        self.add_itself_conversion(&scope, ident, itself);
        self.add_full_qualified_generic_match(&scope, generics);
    }

    fn add_full_qualified_signature(&mut self, sig: &Signature, scope: &ScopeChain) {
        println!("add_full_qualified_signature: {}: {}", scope, format_token_stream(sig));
        if let ReturnType::Type(_arrow_token, ty) = &sig.output {
            self.add_full_qualified_type_match(scope.clone(), ty)
        }
        sig.inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
            self.add_full_qualified_type_match(scope.clone(), ty);
        });
        // self.add_full_qualified_generics()
        // sig.generics
        // sig.generics.
    }

    // fn add_full_qualified_generics(&mut self, scope: &ScopeChain, generics: Generics, visitor_context: &VisitorContext) {
    //     let mut item_local_generics: HashMap<PathHolder, Vec<Path>> = HashMap::new();
    //     let _ = &generics.params.iter().for_each(|generic_param| {
    //         match generic_param {
    //             GenericParam::Type(TypeParam { ident: generic_ident, bounds, .. }) => {
    //                 let mut de_bounds: Vec<Path> =  vec![];
    //                 println!("add_full_qualified_trait: (generic in fn signature): {}", quote!(#generic_param));
    //                 bounds.iter().for_each(|bound| {
    //                     match bound {
    //                         TypeParamBound::Trait(TraitBound { path, .. }) => {
    //                             let ty = parse_quote!(#path);
    //                             de_bounds.push(path.clone());
    //                             self.add_full_qualified_type_match(fn_scope.clone(), &ty, &fn_context);
    //
    //                         },
    //                         TypeParamBound::Lifetime(_lifetime) => {}
    //                     }
    //                 });
    //                 item_local_generics.insert(parse_quote!(#generic_ident), de_bounds);
    //             },
    //             GenericParam::Lifetime(_lifetime) => {},
    //             GenericParam::Const(ConstParam { ty, .. }) => {
    //                 self.add_full_qualified_type_match(fn_scope.clone(), ty, &fn_context);
    //             },
    //         }
    //     });
    //
    //     match &generics.where_clause {
    //         Some(WhereClause { predicates, .. }) => {
    //             predicates.iter().for_each(|predicate| match predicate {
    //                 WherePredicate::Type(PredicateType { bounds, bounded_ty, .. }) => {
    //                     let mut de_bounds: Vec<Path> =  vec![];
    //                     bounds.iter().for_each(|bound| {
    //                         match bound {
    //                             TypeParamBound::Trait(TraitBound { path, .. }) => {
    //                                 let ty = parse_quote!(#path);
    //                                 de_bounds.push(path.clone());
    //                                 println!("add_full_qualified_trait: (bound in fn where): {}", quote!(#ty));
    //                                 // let scope = &self_scope;
    //                                 // let self_scope = scope.joined(&sig.ident);
    //                                 self.add_full_qualified_type_match(scope.clone(), &ty, &de_trait_context);
    //                             },
    //                             TypeParamBound::Lifetime(_lifetime) => {}
    //                         }
    //                     });
    //                     // generics.insert(parse_quote!(#generic_ident), de_bounds);
    //                     self.add_full_qualified_type_match(scope.clone(), bounded_ty, &de_trait_context);
    //                 },
    //                 WherePredicate::Lifetime(_) => {}
    //                 WherePredicate::Eq(_) => {}
    //             })
    //         },
    //         None => {}
    //     }
    //     self.add_full_qualified_generic_match(&scope, item_local_generics);
    // }

    fn add_full_qualified_type_from_struct(&mut self, item_struct: &ItemStruct, scope: &ScopeChain) {
        item_struct.fields.iter().for_each(|Field { ty, .. }|
            self.add_full_qualified_type_match(scope.clone(), ty));
    }

    fn add_full_qualified_type_from_enum(&mut self, item_enum: &ItemEnum, scope: &ScopeChain) {
        item_enum.variants.iter().for_each(|Variant { fields, .. }|
            fields.iter().for_each(|Field { ty, .. }|
                self.add_full_qualified_type_match(scope.clone(), ty)));
    }

    fn add_full_qualified_struct(&mut self, item_struct: &ItemStruct, scope: ScopeChain) {
        self.add_itself_conversion(&scope, &item_struct.ident, ObjectConversion::Item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_struct.generics.clone()))), ScopeItemConversion::Item(Item::Struct(item_struct.clone()))));
        self.add_full_qualified_trait_type_from_macro(&item_struct.attrs, &scope);
        self.add_full_qualified_type_from_struct(&item_struct, &scope);
    }

    fn add_full_qualified_enum(&mut self, item_enum: &ItemEnum, scope: ScopeChain) {
        self.add_itself_conversion(&scope, &item_enum.ident, ObjectConversion::Item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_enum.generics.clone()))), ScopeItemConversion::Item(Item::Enum(item_enum.clone()))));
        self.add_full_qualified_trait_type_from_macro(&item_enum.attrs, &scope);
        self.add_full_qualified_type_from_enum(&item_enum, &scope);
    }
    fn add_full_qualified_fn(&mut self, item_fn: &ItemFn, scope: ScopeChain) {
        self.add_itself_conversion(&scope, &item_fn.sig.ident, ObjectConversion::Item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_fn.sig.generics.clone()))), ScopeItemConversion::Fn(item_fn.sig.clone())));
        self.add_full_qualified_signature(&item_fn.sig, &scope);
    }
    fn add_full_qualified_type(&mut self, item_type: &ItemType, scope: ScopeChain) {
        self.add_itself_conversion(&scope, &item_type.ident, ObjectConversion::Item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_type.generics.clone()))), ScopeItemConversion::Item(Item::Type(item_type.clone()))));
        self.add_full_qualified_type_match(scope.clone(), &item_type.ty);
    }

    pub fn add_full_qualified_conversion(&mut self, item: Item, scope: ScopeChain) -> Option<ItemConversion> {
        match item {
            Item::Struct(ref item_struct) => {
                let scope = scope.joined_obj(&item);
                self.add_full_qualified_struct(&item_struct, scope.clone());
                Some(ItemConversion::Struct(item_struct.clone(), scope))
            },
            Item::Enum(ref item_enum) => {
                let scope = scope.joined_obj(&item);
                self.add_full_qualified_enum(&item_enum, scope.clone());
                Some(ItemConversion::Enum(item_enum.clone(), scope))
            },
            Item::Fn(ref item_fn) => {
                let scope = scope.joined_fn(&item);
                self.add_full_qualified_fn(&item_fn, scope.clone());
                Some(ItemConversion::Fn(item_fn.clone(), scope))
            },
            Item::Trait(ref item_trait) => {
                let scope = scope.joined_trait(&item);
                self.add_full_qualified_trait(&item_trait, scope.clone());
                Some(ItemConversion::Trait(item_trait.clone(), scope))
            },
            Item::Type(ref item_type) => {
                let scope = scope.joined_obj(&item);
                self.add_full_qualified_type(&item_type, scope.clone());
                Some(ItemConversion::Type(item_type.clone(), scope))

            },
            Item::Impl(item_impl) => {
                // TODO: How it's better to determine scope chain here ??
                // TODO: how to build trait_scopes
                let self_ty = &item_impl.self_ty;
                let self_path: Path = parse_quote!(#self_ty);
                let self_scope = PathHolder::from(self_path);
                // item_impl.items.iter().for_each(|impl_item| impl_item)
                // let self_ty = &*item_impl.self_ty;
                // let ident = type_ident(self_ty);
                // let self_scope = scope.joined(&ident);
                //Self::add_full_qualified_impl(visitor, &item_impl, &scope, &self_scope);
                let scope = ScopeChain::Impl {
                    self_scope: Scope::new(self_scope, ObjectConversion::Item(TypeConversion::Object(TypeComposition::new(*self_ty.clone(), Some(item_impl.generics.clone()))), ScopeItemConversion::Item(Item::Impl(item_impl.clone())))),
                    trait_scopes: vec![],
                    parent_scope_chain: Box::new(scope.clone()),
                };
                Some(ItemConversion::Impl(item_impl, scope))
            },
            Item::Use(item_use) => Some(ItemConversion::Use(item_use, scope)),
            Item::Mod(ref item_mod) => {
                let inner_scope = scope.joined(&item);
                match &item_mod.content {
                    None => {},
                    Some((_, items)) => {
                        items.clone().into_iter().for_each(|item| match item {
                            Item::Use(node) =>
                                self.fold_import_tree(&inner_scope, &node.tree, vec![]),
                            Item::Trait(ref item_trait) => {
                                let scope = inner_scope.joined_trait(&item);
                                self.add_full_qualified_trait(&item_trait, scope)
                            },
                            Item::Fn(ref item_fn) => {
                                let scope = inner_scope.joined_fn(&item);
                                self.add_full_qualified_fn(item_fn, scope);
                            },
                            Item::Struct(ref item_struct) => {
                                let scope = inner_scope.joined_obj(&item);
                                self.add_full_qualified_struct(item_struct, scope);
                            },
                            Item::Enum(ref item_enum) => {
                                let scope = inner_scope.joined_obj(&item);
                                self.add_full_qualified_enum(item_enum, scope);
                            },
                            Item::Type(ref item_type) => {
                                let scope = inner_scope.joined_obj(&item);
                                self.add_full_qualified_type(item_type, scope);
                            },
                            // Item::Impl(item_impl) => {
                            //     let self_ty = &item_impl.self_ty;
                            //     let path = parse_quote!(#self_ty);
                            //     let self_scope = scope.joined_path(path);
                            //     Self::add_full_qualified_impl(visitor, &item_impl, &inner_scope, &self_scope);
                            // },
                            _ => {}
                        })
                    }
                }
                Some(ItemConversion::Mod(item_mod.clone(), scope))
            },
            _ => None
        }
    }

    pub fn add_conversion(&mut self, item: Item) {
        let ident = ident_from_item(&item);
        let self_scope = self.current_scope_for(&item);
        //let scope = ScopeChain::Mod { self_scope: Scope { self_scope, item: ObjectConversion::Item(TypeConversion::Object()) } };

        match (MacroType::try_from(&item), ObjectConversion::try_from(&item)) {
            (Ok(MacroType::Export), Ok(object)) => {
                let scope_chain = ScopeChain::Mod { self_scope: Scope::new(self_scope.clone(), object) };
                if let Some(conversion) = self.add_full_qualified_conversion(item, scope_chain) {
                    self.find_scope_tree(&self_scope)
                        .add_item(conversion);
                }
            },
            (Ok(MacroType::Register(path)), Ok(object)) => {
                let scope_chain = ScopeChain::Mod { self_scope: Scope::new(self_scope.clone(), object) };

                if let ScopeTreeExportItem::Tree(scope_context, ..) = self.find_scope_tree(&self_scope) {
                    ident.map(|ident| {
                        let ffi_type = parse_quote!(#self_scope::#ident);
                        let ctx = scope_context.borrow();
                        ctx.add_custom_conversion(scope_chain, path, ffi_type);
                    });
                }
            },
            (_, Ok(object)) if ident != Some(format_ident!("FFIConversion")) => if let Item::Impl(..) = item {
                let scope_chain = ScopeChain::Mod { self_scope: Scope::new(self_scope, object) };
                self.add_full_qualified_conversion(item, scope_chain);
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

fn extract_trait_names(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();
    attrs.iter().for_each(|attr| {
        if attr.path.segments
            .iter()
            .any(|segment| segment.ident == format_ident!("export")) {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                meta_list.nested.iter().for_each(|meta| {
                    if let NestedMeta::Meta(Meta::Path(path)) = meta {
                        paths.push(path.clone());
                    }
                });
            }
        }
    });
    paths
}

