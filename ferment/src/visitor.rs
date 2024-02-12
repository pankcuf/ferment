use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, ConstParam, Field, FnArg, GenericArgument, GenericParam, Generics, Ident, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Path, PathArguments, PathSegment, PatType, PredicateType, QSelf, ReturnType, Signature, Token, TraitBound, TraitItem, TraitItemConst, TraitItemMethod, TraitItemType, Type, TypeParam, TypeParamBound, TypePath, TypeTraitObject, UseTree, Variant, WhereClause, WherePredicate};
use syn::punctuated::Punctuated;
use syn::token::{Add, Colon2};
use syn::visit::Visit;
use crate::composition::{QSelfComposition, TraitDecompositionPart1, TypeComposition};
use crate::context::{GlobalContext, Scope, ScopeChain};
use crate::conversion::{MacroType, ObjectConversion, ScopeItemConversion, TypeConversion};
use crate::ext::{Constraints, merge_scope_type, MergeInto, NestingExtension};
use crate::formatter::{Emoji, format_token_stream};
use crate::helper::ident_from_item;
use crate::holder::{PathHolder, TypeHolder};
use crate::nprint;
use crate::tree::{ScopeTreeExportID, ScopeTreeExportItem};

pub struct Visitor {
    pub context: Arc<RwLock<GlobalContext>>,
    pub parent: PathHolder,
    pub inner_visitors: Vec<Visitor>,
    pub tree: ScopeTreeExportItem,
    pub current_module_scope: ScopeChain,
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
        self.add_conversion(Item::Fn(node.clone()));
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        self.add_conversion(Item::Impl(node.clone()));
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        if node.ident.to_string().eq("fermented") {
            return;
        }
        let item = Item::Mod(node.clone());
        let module = self.current_module_scope.clone();
        self.current_module_scope = self.current_module_scope.joined_mod(&item);
        self.add_conversion(Item::Mod(node.clone()));
        if let Some(ref content) = node.content {
            for item in &content.1 {
                syn::visit::visit_item(self, item);
            }
        }
        self.current_module_scope = self.current_module_scope.parent_scope().cloned().unwrap_or(module);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.add_conversion(Item::Struct(node.clone()));
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        self.add_conversion(Item::Trait(node.clone()));
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        self.add_conversion(Item::Type(node.clone()));
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        // TODO: what to do with fn-level use statement?
        let scope = self.current_module_scope.clone();
        self.fold_import_tree(&scope, &node.tree, vec![]);
    }
}

impl Visitor {
    /// path: full-qualified Path for file
    pub fn new(scope: ScopeChain, context: &Arc<RwLock<GlobalContext>>) -> Self {
        Self {
            context: context.clone(),
            parent: scope.self_path_holder().clone(),
            current_module_scope: scope.clone(),
            inner_visitors: vec![],
            tree: ScopeTreeExportItem::with_global_context(scope, context.clone())
        }
    }
}

/// Global Context Facade
impl Visitor {

    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    pub(crate) fn fold_import_tree(&mut self, scope: &ScopeChain, use_tree: &UseTree, current_path: Vec<Ident>) {
        let mut lock = self.context.write().unwrap();
        lock.imports.fold_import_tree(scope, use_tree, current_path);
    }

    pub(crate) fn add_full_qualified_trait_match(&mut self, scope: &ScopeChain, item_trait: &ItemTrait, itself: &ObjectConversion) {
        let mut lock = self.context.write().unwrap();
        lock.traits.add_trait(scope, item_trait, itself);
    }
    pub(crate) fn add_full_qualified_generic_match(&mut self, scope: &ScopeChain, generics: HashMap<PathHolder, Vec<Path>>) {
        let mut lock = self.context.write().unwrap();
        lock.generics.extend_in_scope(scope, generics)
    }

    fn add_types_used_in_scope(&self, types: HashMap<TypeHolder, ObjectConversion>, scope: &ScopeChain) {
        let mut lock = self.context.write().unwrap();
        types.merge_into(lock.scope_register_mut(scope))
    }
    fn add_type_used_in_scope(&self, ty: TypeHolder, object: ObjectConversion, scope: &ScopeChain) {
        let mut lock = self.context.write().unwrap();
        merge_scope_type(lock.scope_register_mut(scope), ty, object);
    }
    fn add_itself_conversion(&mut self, scope: &ScopeChain, ident: &Ident, object: ObjectConversion) {
        self.add_type_used_in_scope(parse_quote!(#ident), object, scope);
    }
    fn add_full_qualified_trait_type_from_macro(&mut self, item_trait_attrs: &[Attribute], scope: &ScopeChain) {
        let trait_names = extract_trait_names(item_trait_attrs);
        // let self_scope = scope.joined(ident);
        trait_names.iter().for_each(|trait_name|
            self.add_full_qualified_type_match(scope, &parse_quote!(#trait_name)));
        let mut lock = self.context.write().unwrap();
        lock.traits
            .add_used_traits(scope, trait_names)
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

    fn involved_types_in_scope(&self, involved_types: HashSet<Type>, scope: &ScopeChain) -> HashMap<TypeHolder, ObjectConversion> {
        let mut destination = HashMap::<TypeHolder, ObjectConversion>::new();
        for ty  in &involved_types {
            merge_scope_type(&mut destination, TypeHolder::from(ty), self.update_nested_generics(scope, ty));
        }
        destination
    }
    pub(crate) fn add_full_qualified_type_match(&mut self, scope: &ScopeChain, ty: &Type) {
        // println!("::::: add_full_qualified: {} in [{}] root: {:?}", ty.to_token_stream(), scope.self_scope(),  scope.obj_root_chain());
        nprint!(0, Emoji::Plus, "{} in [{}]", format_token_stream(ty), scope);
        let involved_types = ty.nested_items();
        match scope {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => {
                let all_involved_full_types = self.involved_types_in_scope(involved_types, scope);
                let filtered = all_involved_full_types.into_iter().filter(|(th, _oc)| th.0.has_no_self()).collect();
                self.add_types_used_in_scope(filtered, scope);
            },
            ScopeChain::Impl { parent_scope_chain, .. } => {
                let all_involved_full_types = self.involved_types_in_scope(involved_types, scope);
                self.add_types_used_in_scope(all_involved_full_types.clone(), scope);
                let filtered = all_involved_full_types.into_iter().filter(|(th, _oc)| th.0.has_no_self()).collect();
                self.add_types_used_in_scope(filtered, parent_scope_chain);
            },
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } => {
                // involved_types.insert(parse_quote!(Self));
                let all_involved_full_types = self.involved_types_in_scope(involved_types, scope);
                let self_types =  self.involved_types_in_scope(HashSet::from([parse_quote!(Self)]), scope);
                self.add_types_used_in_scope(all_involved_full_types.clone(), scope);
                self.add_types_used_in_scope(self_types, scope);
                let filtered = all_involved_full_types.into_iter().filter(|(th, _oc)| th.0.has_no_self()).collect();
                self.add_types_used_in_scope(filtered, parent_scope_chain);
            },
            ScopeChain::Fn { parent_scope_chain, .. } => {
                match &**parent_scope_chain {
                    ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => {
                        let all_involved_full_types = self.involved_types_in_scope(involved_types, scope);
                        self.add_types_used_in_scope(all_involved_full_types.clone(), scope);
                        self.add_types_used_in_scope(all_involved_full_types, parent_scope_chain);
                    },
                    ScopeChain::Trait { parent_scope_chain: parent_parent_scope_chain, .. } |
                    ScopeChain::Object { parent_scope_chain: parent_parent_scope_chain, .. } |
                    ScopeChain::Impl { parent_scope_chain: parent_parent_scope_chain, .. } => {
                        let all_involved_full_types = self.involved_types_in_scope(involved_types, scope);
                        let self_types =  self.involved_types_in_scope(HashSet::from([parse_quote!(Self)]), scope);
                        self.add_types_used_in_scope(all_involved_full_types.clone(), scope);
                        self.add_types_used_in_scope(self_types.clone(), scope);
                        self.add_types_used_in_scope(self_types, parent_scope_chain);
                        self.add_types_used_in_scope(all_involved_full_types.clone(), parent_scope_chain);

                        let filtered = all_involved_full_types.into_iter().filter(|(th, _oc)| th.0.has_no_self()).collect();
                        self.add_types_used_in_scope(filtered, parent_parent_scope_chain);
                    },
                    ScopeChain::Fn { parent_scope_chain: _parent_parent_scope_chain, .. } => {
                        // TODO: actually there are may be anything wrapped into anything like trait inside a function...
                    }

                }
            }
        }
    }


    fn handle_full_path(&self, scope: &ScopeChain, path: &Path, qself: Option<QSelfComposition>) -> ObjectConversion {
        nprint!(1, Emoji::Branch, "handle_full_path: {} with qself: [{}] in {}",
            format_token_stream(path),
            qself.as_ref().map_or(format!("None"), |q| format_token_stream(&q.qself.ty)),
            scope);
        let new_qself = qself.as_ref().map(|q| q.qself.clone());
        let mut segments = path.segments.clone();
        for segment in &mut segments {
            if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                for arg in &mut angle_bracketed_generic_arguments.args {
                    match arg {
                        GenericArgument::Type(inner_type) => {
                            let obj_conversion = self.update_nested_generics(scope,inner_type);
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

        let obj_scope = scope.obj_root_chain().unwrap_or(scope);
        let object_self_scope = obj_scope.self_scope();

        // let obj_conversion = if let Some(bounds_composition) = scope.maybe_generic_bound_for_path(&import_seg.0) {
        let obj_conversion = if let Some(dict_type_composition) = scope.maybe_dictionary_type(&import_seg.0) {
            ObjectConversion::Type(dict_type_composition)
        } else if let Some(bounds_composition) = scope.maybe_generic_bound_for_path(&import_seg.0) {
            nprint!(1, Emoji::Local, "(Local Generic Bound) {}", bounds_composition);
            ObjectConversion::Type(TypeConversion::Bounds(bounds_composition))
        } else if let Some(replacement_path) = lock.maybe_import(scope, &import_seg).cloned() {
        // let obj_conversion = if let Some(replacement_path) = lock.maybe_import(scope, &import_seg).cloned() {
            nprint!(1, Emoji::Local, "(ScopeImport) {}", format_token_stream(&replacement_path));
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
        } else if let Some(generic_bounds) = lock.generics.maybe_generic_bounds(scope, &import_seg) {
            let first_bound = generic_bounds.first().unwrap();
            let first_bound_as_scope = PathHolder::from(first_bound);
            if let Some(imported) = lock.maybe_import(scope, &first_bound_as_scope).cloned() {
                nprint!(1, Emoji::Local, "(Generic Bounds Imported) {}", format_token_stream(&segments));
                let last_segment = segments.pop().unwrap();
                let new_segments = imported.segments.clone();
                segments.extend(new_segments);
                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
            } else {
                nprint!(1, Emoji::Local, "(Generic Bounds Local) {}", format_token_stream(&segments));
                let last_segment = segments.pop().unwrap();
                let scope = scope.self_path_holder();
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
            nprint!(1, Emoji::Local, "(Local or Global ....) {}", format_token_stream(&segments));
            let self_scope_path = &object_self_scope.self_scope;
            match first_ident.to_string().as_str() {
                "Self" if segments.len() <= 1 => {
                    nprint!(1, Emoji::Local, "(Self) {}", format_token_stream(first_ident));
                    let last_segment = segments.pop().unwrap();
                    let new_segments: Punctuated<PathSegment, Token![::]> = parse_quote!(#self_scope_path);
                    segments.extend(new_segments);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    object_self_scope.object.clone()
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
                    nprint!(1, Emoji::Local, "(SELF::->) {}: {}", format_token_stream(&last_segment), format_token_stream(&last_segment.clone().into_value().arguments));
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
                "Vec" => {
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

                    let obj_parent_scope = obj_scope.parent_scope();

                    let len = segments.len();
                    if len == 1 {

                        //ScopeChain::Object (self: crate::model::snapshot::LLMQSnapshot, parent: ScopeChain::Mod (self: crate::model::snapshot))
                        match obj_parent_scope {
                            None => {
                                // Global
                                nprint!(1, Emoji::Local, "(Local join single (has no parent scope): {}) {} + {}", first_ident, scope, format_token_stream(&path));
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
                                let scope = parent.self_path_holder();
                                nprint!(1, Emoji::Local, "(Local join single (has parent scope): {}) {} + {}", first_ident, scope, format_token_stream(&path));
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
                        if let Some(QSelfComposition { qs: _, qself: QSelf { ty, .. } }) = qself.as_ref() {
                            nprint!(1, Emoji::Local, "(Local join QSELF: {} [{}]) {} + {}", format_token_stream(ty), format_token_stream(&import_seg), format_token_stream(scope), format_token_stream(&path));

                            println!("------ import local? {} in [{}]", import_seg.to_token_stream(), scope);
                            println!("------ import parent? {} in [{:?}]", import_seg.to_token_stream(), scope.parent_scope());
                            println!("------ import object? {} in [{:?}]", import_seg.to_token_stream(), obj_scope);
                            println!("------ import object parent? {} in [{:?}]", import_seg.to_token_stream(), obj_parent_scope);

                            let maybe_import = lock.maybe_scope_import_path(scope, &import_seg)
                                .or(lock.maybe_scope_import_path(obj_scope, &import_seg))
                                .or(obj_parent_scope.and_then(|obj_parent_scope|
                                    lock.maybe_scope_import_path(obj_parent_scope, &import_seg)));

                            let tt = if let Some(import) = maybe_import {
                                import.clone()
                            } else {
                                let local = obj_parent_scope.unwrap_or(scope);
                                parse_quote!(#local::#import_seg)
                            };
                            let tail_path = quote!(#(#tail)::*);
                            // println!("{}: <{} as {}>::{}", tail.len(), format_token_stream(ty), format_token_stream(&tt), format_token_stream(&tail_path));
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
                        } else {
                            nprint!(1, Emoji::Local, "(Local join multi: {}) {} + {}", first_ident, format_token_stream(scope), format_token_stream(&path));
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
                        }
                    }
                },
            }
        };
        obj_conversion
        // TypeComposition::new(Type::Path(TypePath { qself: qself.as_ref().map(|q| q.qself.clone()), path: Path { leading_colon: path.leading_colon, segments } }), None)
    }

    fn handle_qself(&self, scope: &ScopeChain, qself: &Option<QSelf>) -> Option<QSelfComposition> {
        qself.as_ref().map(|qself| {
            let mut new_qself = qself.clone();
            let qs = self.update_nested_generics(scope, &qself.ty);
            let qs = qs.type_conversion().unwrap().ty_composition().clone();
            new_qself.ty = Box::new(qs.ty.clone());
            QSelfComposition { qs, qself: new_qself }
        })
    }
    fn update_nested_generics(&self, scope: &ScopeChain, ty: &Type) -> ObjectConversion {
        nprint!(1, Emoji::Node, "=== {} [{:?}]", format_token_stream(ty), ty);
        match ty {
            Type::Path(type_path) =>
                self.handle_full_path(scope, &type_path.path, self.handle_qself(scope, &type_path.qself)),
            Type::TraitObject(type_trait_object) => {
                let TypeTraitObject { dyn_token, bounds } = type_trait_object;
                let mut bounds = bounds.clone();
                bounds.iter_mut().for_each(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) => {
                        let full_path = self.handle_full_path(scope, path, None);
                        *path = parse_quote!(#full_path);
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
        }
    }

    fn find_scope_tree(&mut self, scope: &PathHolder) -> &mut ScopeTreeExportItem {
        let mut current_tree = &mut self.tree;
        let path_to_traverse: Vec<ScopeTreeExportID> = scope.0.segments.iter().skip(1).map(|segment| ScopeTreeExportID::Ident(segment.ident.clone())).collect();
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



    fn add_full_qualified_impl(&mut self, item_impl: &ItemImpl, scope: &ScopeChain) {
        // let trait_path = item_impl.trait_.clone().map(|(_, path, _)| path);
        // let visitor_context = trait_path.map_or(VisitorContext::Object, |_| VisitorContext::Trait(None));
        // return;
        // println!("add_full_qualified_impl: {} in [{}]", quote!(#item_impl), scope);
        item_impl.items.iter().for_each(|impl_item| {
            match impl_item {
                ImplItem::Const(ImplItemConst { ty, .. }) => {
                    self.add_full_qualified_type_match(scope, ty);
                },
                ImplItem::Method(ImplItemMethod { sig, .. }) => {
                    self.add_full_qualified_signature(sig, scope)
                },
                ImplItem::Type(ImplItemType { ty, .. }) => {
                    self.add_full_qualified_type_match(scope, ty);
                },
                _ => {}
            }
        });
        match &item_impl.trait_ {
            Some((_, path, _)) => {
                let ty = parse_quote!(#path);
                self.add_full_qualified_type_match(scope, &ty);
            },
            None => {}
        }
    }

    fn add_full_qualified_type_param_bounds(&mut self, bounds: Punctuated<TypeParamBound, Add>, scope: &ScopeChain) {
        bounds.iter().for_each(|bound| {
            match bound {
                TypeParamBound::Trait(TraitBound { path, .. }) => {
                    let ty = parse_quote!(#path);
                    self.add_full_qualified_type_match(scope, &ty);
                },
                TypeParamBound::Lifetime(_lifetime) => {}
            }
        });
    }

    fn add_full_qualified_trait(&mut self, item_trait: &ItemTrait, scope: &ScopeChain) {

        println!("add_full_qualified_trait: {}: {}", item_trait.ident, scope);
        let ident = &item_trait.ident;
        let type_compo = TypeComposition::new(scope.to_type(), Some(item_trait.generics.clone()));
        let itself = ObjectConversion::new_item(
            TypeConversion::Trait(
                type_compo,
                TraitDecompositionPart1::from_trait_items(ident, &item_trait.items)),
            ScopeItemConversion::Item(Item::Trait(item_trait.clone())));

        // 1. Add itself to the scope as <Self, Item(Trait(..))>
        // 2. Add itself to the parent scope as <Ident, Item(Trait(..))>
        println!("::: 1. ADD Self (local scope): <{}, {}> in [{}]", quote!(Self), itself, scope);
        println!("::: 2. ADD Self: (parent scope) <{}, {}> in [{}]", quote!(#ident), itself, scope.parent_scope().unwrap());
        self.add_full_qualified_trait_match(&scope, item_trait, &itself);
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
                                println!("add_full_qualified_trait: (generic trait): {}: {}", format_token_stream(generic_ident), format_token_stream(&ty));
                                de_bounds.push(path.clone());
                                self.add_full_qualified_type_match(scope, &ty);
                            },
                            TypeParamBound::Lifetime(_lifetime) => {}
                        }
                    });
                    generics.insert(parse_quote!(#generic_ident), de_bounds);
                },
                GenericParam::Lifetime(_lifetime) => {},
                GenericParam::Const(ConstParam { ty, .. }) => {
                    self.add_full_qualified_type_match(scope, ty);
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
                                    self.add_full_qualified_type_match(scope, &ty);
                                },
                                TypeParamBound::Lifetime(_lifetime) => {}
                            }
                        });
                        // generics.insert(parse_quote!(#generic_ident), de_bounds);
                        self.add_full_qualified_type_match(scope, bounded_ty);
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
                    // println!("add_full_qualified_trait: (super trait): {}", format_token_stream(&ty));
                    self.add_full_qualified_type_match(scope, &ty);
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
                            self_scope.self_scope.joined(sig_ident),
                            ObjectConversion::new_item(TypeConversion::Unknown(TypeComposition::new(parse_quote!(#sig_ident), Some(sig.generics.clone()))), ScopeItemConversion::Fn(sig.clone()))),
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
                                // println!("add_full_qualified_trait: (generic in fn signature): {}", quote!(#generic_param));
                                bounds.iter().for_each(|bound| {
                                    match bound {
                                        TypeParamBound::Trait(TraitBound { path, .. }) => {
                                            let ty = parse_quote!(#path);
                                            de_bounds.push(path.clone());
                                            self.add_full_qualified_type_match(&fn_scope, &ty);

                                        },
                                        TypeParamBound::Lifetime(_lifetime) => {}
                                    }
                                });
                                item_local_generics.insert(parse_quote!(#generic_ident), de_bounds);
                            },
                            GenericParam::Lifetime(_lifetime) => {},
                            GenericParam::Const(ConstParam { ty, .. }) => {
                                self.add_full_qualified_type_match(&fn_scope, ty);
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
                                                // println!("add_full_qualified_trait: (bound in fn where): {}", quote!(#ty));
                                                // let scope = &self_scope;
                                                // let self_scope = scope.joined(&sig.ident);
                                                self.add_full_qualified_type_match(&fn_scope, &ty);
                                            },
                                            TypeParamBound::Lifetime(_lifetime) => {}
                                        }
                                    });
                                    // generics.insert(parse_quote!(#generic_ident), de_bounds);
                                    self.add_full_qualified_type_match(&fn_scope, bounded_ty);
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
                    self.add_full_qualified_type_match(scope, &local_ty);
                    // println!("add_full_qualified_trait (type): {}: {}", ident, type_ident);
                    // TODO: whether we need to preserve scope or use separate scope + trait ident?
                    // Especially when using Self::  It'll break some logics
                    bounds.iter().for_each(|bound| match bound {
                        TypeParamBound::Trait(TraitBound { path, ..}) => {
                            let ty = parse_quote!(#path);
                            self.add_full_qualified_type_match(scope, &ty);
                        },
                        _ => {},
                    });
                },
                TraitItem::Const(TraitItemConst { ty, .. }) => {
                    self.add_full_qualified_type_match(scope, ty);
                },
                _ => {}
            });
        self.add_type_used_in_scope(parse_quote!(#ident), itself, scope.parent_scope().unwrap());
        self.add_full_qualified_generic_match(&scope, generics);
    }

    fn add_full_qualified_signature(&mut self, sig: &Signature, scope: &ScopeChain) {
        if let ReturnType::Type(_arrow_token, ty) = &sig.output {
            self.add_full_qualified_type_match(scope, ty)
        }
        sig.inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
            self.add_full_qualified_type_match(scope, ty);
        });
    }

    fn add_full_qualified_struct(&mut self, item_struct: &ItemStruct, scope: &ScopeChain) {
        self.add_itself_conversion(scope.parent_scope().unwrap(), &item_struct.ident, ObjectConversion::new_item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_struct.generics.clone()))), ScopeItemConversion::Item(Item::Struct(item_struct.clone()))));
        self.add_full_qualified_trait_type_from_macro(&item_struct.attrs, scope);
        item_struct.fields.iter().for_each(|Field { ty, .. }|
            self.add_full_qualified_type_match(scope, ty));

    }

    fn add_full_qualified_enum(&mut self, item_enum: &ItemEnum, scope: &ScopeChain) {
        self.add_itself_conversion(scope.parent_scope().unwrap(), &item_enum.ident, ObjectConversion::new_item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_enum.generics.clone()))), ScopeItemConversion::Item(Item::Enum(item_enum.clone()))));
        self.add_full_qualified_trait_type_from_macro(&item_enum.attrs, scope);
        item_enum.variants.iter().for_each(|Variant { fields, .. }|
            fields.iter().for_each(|Field { ty, .. }|
                self.add_full_qualified_type_match(scope, ty)));

    }
    fn add_full_qualified_fn(&mut self, item_fn: &ItemFn, scope: &ScopeChain) {
        self.add_itself_conversion(scope.parent_scope().unwrap(), &item_fn.sig.ident, ObjectConversion::new_item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_fn.sig.generics.clone()))), ScopeItemConversion::Fn(item_fn.sig.clone())));
        self.add_full_qualified_signature(&item_fn.sig, scope);
    }
    fn add_full_qualified_type(&mut self, item_type: &ItemType, scope: &ScopeChain) {
        self.add_itself_conversion(scope.parent_scope().unwrap(), &item_type.ident, ObjectConversion::new_item(TypeConversion::Object(TypeComposition::new(scope.to_type(), Some(item_type.generics.clone()))), ScopeItemConversion::Item(Item::Type(item_type.clone()))));
        self.add_full_qualified_type_match(scope, &item_type.ty);
    }

    fn add_inner_module_conversion(&mut self, item_mod: &ItemMod, scope: &ScopeChain) {
        println!("add_inner_module_conversion: {} in [{}]", item_mod.ident, scope);
        match &item_mod.content {
            None => {},
            Some((_, items)) => {
                items.into_iter().for_each(|item| match item {
                    Item::Use(node) =>
                        self.fold_import_tree(scope, &node.tree, vec![]),
                    Item::Trait(ref item_trait) =>
                        self.add_full_qualified_trait(&item_trait, &scope.joined_trait(item)),
                    Item::Fn(ref item_fn) =>
                        self.add_full_qualified_fn(item_fn, &scope.joined_fn(item)),
                    Item::Struct(ref item_struct) =>
                        self.add_full_qualified_struct(item_struct, &scope.joined_obj(item)),
                    Item::Enum(ref item_enum) =>
                        self.add_full_qualified_enum(item_enum, &scope.joined_obj(item)),
                    Item::Type(ref item_type) =>
                        self.add_full_qualified_type(item_type, &scope.joined_obj(item)),
                    Item::Impl(item_impl) =>
                        self.add_full_qualified_impl(item_impl, &scope.joined_impl(item)),
                    Item::Mod(item_mod) =>
                        self.add_inner_module_conversion(item_mod, &scope.joined_mod(item)),
                    _ => {}
                })
            }
        }
    }

    pub fn add_full_qualified_conversion(&mut self, item: &Item, scope: ScopeChain) -> Option<ScopeChain> {
        // println!("add_full_qualified_conversion: {} in [{}]", item.ident_string(), scope);
        match item {
            Item::Struct(ref item_struct) => {
                let scope = scope.joined_obj(item);
                self.add_full_qualified_struct(item_struct, &scope);
                Some(scope)
            },
            Item::Enum(ref item_enum) => {
                let scope = scope.joined_obj(item);
                self.add_full_qualified_enum(item_enum, &scope);
                Some(scope)
            },
            Item::Fn(ref item_fn) => {
                let scope = scope.joined_fn(item);
                self.add_full_qualified_fn(item_fn, &scope);
                Some(scope)
            },
            Item::Trait(ref item_trait) => {
                let scope = scope.joined_trait(item);
                self.add_full_qualified_trait(item_trait, &scope);
                Some(scope)
            },
            Item::Type(ref item_type) => {
                let scope = scope.joined_obj(item);
                self.add_full_qualified_type(item_type, &scope);
                Some(scope)

            },
            Item::Impl(ref item_impl) => {
                let scope = scope.joined_impl(item);
                self.add_full_qualified_impl(item_impl, &scope);
                Some(scope)
            },
            Item::Mod(ref item_mod) => {
                let is_fermented_mod = {
                    let ctx = self.context.read().unwrap();
                    item_mod.ident.to_string().eq(&ctx.config.mod_name)
                };
                if scope.is_crate_root() && is_fermented_mod {
                    None
                } else {
                    let scope = scope.joined(item);
                    self.add_inner_module_conversion(item_mod, &scope);
                    Some(scope)
                }
            },
            _ => None
        }
    }

    pub fn add_conversion(&mut self, item: Item) {
        let ident = ident_from_item(&item);
        let current_scope = self.current_module_scope.clone();
        let self_scope = current_scope.self_scope().clone().self_scope;
        match (MacroType::try_from(&item), ObjectConversion::try_from(&item)) {
            (Ok(MacroType::Export), Ok(_object)) => {
                if let Some(scope) = self.add_full_qualified_conversion(&item, current_scope) {
                    self.find_scope_tree(&self_scope)
                        .add_item(item, scope);
                }
            },
            (Ok(MacroType::Register(path)), Ok(_object)) => {
                if let ScopeTreeExportItem::Tree(scope_context, ..) = self.find_scope_tree(&self_scope) {
                    ident.map(|ident| {
                        let ffi_type = parse_quote!(#self_scope::#ident);
                        let ctx = scope_context.borrow();
                        ctx.add_custom_conversion(current_scope, path, ffi_type);
                    });
                }
            },
            (_, Ok(_object)) if ident != Some(format_ident!("FFIConversion")) => if let Item::Impl(..) = item {
                if let Some(_scope) = self.add_full_qualified_conversion(&item, current_scope) {
                }
            },
            _ => {}
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

