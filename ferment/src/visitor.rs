use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, GenericArgument, Generics, Ident, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, parse_quote, Path, PathArguments, PathSegment, QSelf, Token, TraitBound, Type, TypeParamBound, TypePath, TypeTraitObject, UseTree};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use syn::visit::Visit;
use crate::composition::{QSelfComposition, TypeComposition};
use crate::context::{GlobalContext, ScopeChain, TypeChain};
use crate::conversion::{MacroType, ObjectConversion, TypeConversion};
use crate::ext::{add_trait_names, create_generics_chain, extract_trait_names, Join, MergeInto, NestingExtension, Visiting};
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
        self.current_module_scope = self.current_module_scope.joined(&item);
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

    pub fn merge_visitor_trees(&mut self) {
        // Merge the trees of the inner visitors first
        for inner_visitor in &mut self.inner_visitors {
            inner_visitor.merge_visitor_trees();
        }
        // Now merge the trees of the inner visitors into the current visitor's tree
        for Visitor { tree, .. } in &self.inner_visitors {
            tree.merge_into(&mut self.tree);
        }
    }
    pub fn into_code_tree(mut self) -> ScopeTreeExportItem {
        self.merge_visitor_trees();
        self.tree
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
    pub(crate) fn add_generic_chain(&mut self, scope: &ScopeChain, generics: &Generics) {
        let generics = create_generics_chain(self, generics, scope);
        let mut lock = self.context.write().unwrap();
        lock.generics.extend_in_scope(scope, generics)
    }

    fn scope_add_many(&self, types: TypeChain, scope: &ScopeChain) {
        let mut lock = self.context.write().unwrap();
        lock.scope_mut(scope)
            .add_many(types);
    }
    pub(crate) fn scope_add_one(&self, ty: TypeHolder, object: ObjectConversion, scope: &ScopeChain) {
        let mut lock = self.context.write().unwrap();
        lock.scope_mut(scope)
            .add_one(ty, object);
    }
    pub(crate) fn add_full_qualified_trait_type_from_macro(&mut self, item_trait_attrs: &[Attribute], scope: &ScopeChain) {
        let trait_names = extract_trait_names(item_trait_attrs);
        add_trait_names(self, scope, &trait_names);
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

    fn create_type_chain(&self, ty: &Type, scope: &ScopeChain) -> TypeChain {
        let involved_types = ty.nested_items();
        let mut destination = TypeChain::default();
        for ty  in &involved_types {
            let object = self.update_nested_generics(scope, ty);
            destination.add_one(TypeHolder::from(ty), object);
        }
        destination
    }

    pub(crate) fn add_full_qualified_type_match(&mut self, scope: &ScopeChain, ty: &Type) {
        nprint!(0, Emoji::Plus, "{} in [{}]", format_token_stream(ty), scope);
        // let ff: HashMap<TypeHolder, ObjectConversion> = ty.scope_items();
        // println!("::: scope_items: {}: {}", ty.to_token_stream(), format_types_dict(&ff));
        let self_obj = &scope.self_scope().object;
        let type_chain = self.create_type_chain(ty, scope);
        match scope {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => {
                self.scope_add_many(type_chain.selfless(), scope);
            },
            ScopeChain::Impl { parent_scope_chain, .. } => {
                self.scope_add_many(type_chain.selfless(), parent_scope_chain);
                self.scope_add_many(type_chain, scope);
            },
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } => {
                // println!("add_full_qualified_type_match: Obj or Trait: {} in {}", self_obj, scope);
                self.scope_add_many(type_chain.clone(), scope);
                self.scope_add_one(parse_quote!(Self), self_obj.clone(), scope);
                self.scope_add_many(type_chain.selfless(), parent_scope_chain);
            },
            ScopeChain::Fn { parent_scope_chain, .. } => {
                match &**parent_scope_chain {
                    ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => {
                        self.scope_add_many(type_chain.clone(), scope);
                        self.scope_add_many(type_chain, parent_scope_chain);
                    },
                    ScopeChain::Trait { parent_scope_chain: parent_parent_scope_chain, .. } |
                    ScopeChain::Object { parent_scope_chain: parent_parent_scope_chain, .. } |
                    ScopeChain::Impl { parent_scope_chain: parent_parent_scope_chain, .. } => {
                        self.scope_add_many(type_chain.selfless(), parent_parent_scope_chain);
                        self.scope_add_many(type_chain.clone(), scope);
                        self.scope_add_one(parse_quote!(Self), self_obj.clone(), scope);
                        // self.scope_add_one(parse_quote!(Self), self_obj.clone(), parent_scope_chain);
                        self.scope_add_many(type_chain, parent_scope_chain);

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
        // let is_crate_path = format_ident!("crate").eq(first_ident);
        let last_segment = &segments.last().unwrap();
        let last_ident = &last_segment.ident;
        let import_seg: PathHolder = parse_quote!(#first_ident);
        let lock = self.context.read().unwrap();

        let obj_scope = scope.obj_root_chain().unwrap_or(scope);
        let object_self_scope = obj_scope.self_scope();

        if let Some(dict_type_composition) = scope.maybe_dictionary_type(&import_seg.0) {
            ObjectConversion::Type(dict_type_composition)
        } else if let Some(bounds_composition) = scope.maybe_generic_bound_for_path(&import_seg.0) {
            nprint!(1, Emoji::Local, "(Local Generic Bound) {}", bounds_composition);
            ObjectConversion::Type(TypeConversion::Bounds(bounds_composition))
        } else if let Some(replacement_path) = lock.maybe_import(scope, &import_seg).cloned() {
            let last_segment = segments.pop().unwrap();
            if format_ident!("crate").eq(&replacement_path.segments.first().unwrap().ident) /*&& !lock.config.current_crate.ident().eq(crate_scope)*/ {
                nprint!(1, Emoji::Local, "(ScopeImport Local) {}", format_token_stream(&replacement_path));
                let crate_scope = scope.crate_scope();
                let replaced: Vec<_> = replacement_path.segments.iter().skip(1).collect();
                let mut new_path: Path = parse_quote!(#crate_scope::#(#replaced)::*);
                new_path.segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                ObjectConversion::Type(
                    TypeConversion::Unknown(
                        TypeComposition::new(
                            Type::Path(
                                TypePath {
                                    qself: new_qself,
                                    path: new_path }),
                            None)))
            } else {
                nprint!(1, Emoji::Local, "(ScopeImport External) {}", format_token_stream(&replacement_path));
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
            }

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
                    println!("::: new_segments: {} ", new_segments.to_token_stream());
                    segments.extend(new_segments);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    println!("::: add_obj_self: {} scope: [{}]", object_self_scope, scope);
                    // object_self_scope.object.clone()

                    ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))

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
                "Option" => {
                    ObjectConversion::Type(TypeConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: path.leading_colon, segments } }),
                        None)))
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
                            let new_segments: Punctuated<PathSegment, Colon2> = if path.leading_colon.is_none() {
                                parse_quote!(#scope::#path)
                            } else {
                                parse_quote!(#scope #path)
                            };
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
        }
    }

    fn handle_qself(&self, scope: &ScopeChain, qself: &Option<QSelf>) -> Option<QSelfComposition> {
        qself.as_ref().map(|qself| {
            let mut new_qself = qself.clone();
            let qs = self.update_nested_generics(scope, &qself.ty);
            let qs = qs.type_conversion().unwrap().ty_composition().clone();
            new_qself.ty = qs.ty.clone().into();
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

    pub fn add_conversion(&mut self, item: Item) {
        let ident = ident_from_item(&item);
        let current_scope = self.current_module_scope.clone();
        let self_scope = current_scope.self_scope().clone().self_scope;
        match (MacroType::try_from(&item), ObjectConversion::try_from(&item)) {
            (Ok(MacroType::Export), Ok(_object)) => {
                if let Some(scope) = item.join_scope(&current_scope, self) {
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
                if let Some(_scope) = item.join_scope(&current_scope, self) {
                }
            },
            _ => {}
        }
    }
}


