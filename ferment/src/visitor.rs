use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use quote::{format_ident, ToTokens};
use syn::{Attribute, Generics, Ident, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, parse_quote, Type, UseTree};
use syn::visit::Visit;
use crate::context::{GlobalContext, ScopeChain, TypeChain};
use crate::conversion::{MacroType, ObjectConversion};
use crate::ext::{add_trait_names, CrateExtension, create_generics_chain, extract_trait_names, ItemHelper, Join, MergeInto, NestingExtension, Pop, VisitScope, VisitScopeType};
use crate::helper::ItemExtension;
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
        // println!("visit_item_mod: {}", node.to_token_stream());
        let item = Item::Mod(node.clone());
        let module = self.current_module_scope.clone();
        self.current_module_scope = self.current_module_scope.joined(&item);
        self.add_conversion(Item::Mod(node.clone()));
        // if let Some((_, ref items)) = node.content {
        //     for node in items {
        //         syn::visit::visit_item(self, node);
        //     }
        // }
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
        //println!("visit_item_use: {}", node.to_token_stream());
        // TODO: what to do with fn-level use statement?
        let scope = self.current_module_scope.clone();
        self.fold_import_tree(&scope, &node.tree, vec![]);
    }
}

impl Visitor {
    /// path: full-qualified Path for file
    pub fn new(scope: ScopeChain, context: &Arc<RwLock<GlobalContext>>) -> Self {
        //println!("Visitor::new({})", scope.self_path_holder_ref());
        Self {
            context: context.clone(),
            parent: scope.self_path_holder_ref().clone(),
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
        // print_phase!("PHASE 1: MERGE VISITORS", "{}", self.tree);
    }
    pub fn into_code_tree(mut self) -> ScopeTreeExportItem {
        self.merge_visitor_trees();
        // println!("into_code_tree: \n{}", self.tree);
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
        // println!("Visitor::add_generic_chain: {:?}", generics); // TODO: always empty
        lock.generics.extend_in_scope(scope, generics)
    }

    fn scope_add_many(&self, types: TypeChain, scope: &ScopeChain) {
        // println!("scope_add_many: {}", scope.self_path_holder_ref());
        let mut lock = self.context.write().unwrap();
        lock.scope_mut(scope)
            .add_many(types);
    }
    pub(crate) fn scope_add_one(&self, ty: TypeHolder, object: ObjectConversion, scope: &ScopeChain) {
        // println!("scope_add_one: {}", scope.self_path_holder_ref());
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
        involved_types.iter()
            .for_each(|ty|
                destination.add_one(
                    TypeHolder::from(ty),
                    ty.update_nested_generics(&(scope, &self.context.read().unwrap()))));
        // println!("TYPECHAIN: {} ---> {}", ty.to_token_stream(), destination);
        destination
    }

    pub(crate) fn add_full_qualified_type_match(&mut self, scope: &ScopeChain, ty: &Type) {
        nprint!(0, Emoji::Plus, "{} in [{}]", ty.to_token_stream(), scope);
        // println!("::: scope_items: {}: {}", ty.to_token_stream(), scope.self_path_holder_ref());
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

    fn find_scope_tree(&mut self, scope: &PathHolder) -> &mut ScopeTreeExportItem {
        // println!("find_scope_tree: {}", scope);
        let mut current_tree = &mut self.tree;
        for ident in scope.crate_less().iter().map(ScopeTreeExportID::from) {
            match current_tree {
                ScopeTreeExportItem::Item(..) =>
                    panic!("Unexpected item while traversing the scope path"),  // Handle as appropriate
                ScopeTreeExportItem::Tree(scope_context, _, exported) => {
                    // println!("find_scope_tree.1: {}: {}: {}", ident, exported.contains_key(&ident), scope_context.borrow().scope.self_path_holder_ref());
                    if !exported.contains_key(&ident) {
                        exported.insert(ident.clone(), ScopeTreeExportItem::tree_with_context_and_export(scope_context.clone(), HashMap::default()));
                    }
                    current_tree = exported.get_mut(&ident).unwrap();
                }
            }
        }
        current_tree
    }

    pub fn add_conversion(&mut self, item: Item) {
        // TODO: filter #[cfg(test)]
        let ident = item.maybe_ident();
        let current_scope = self.current_module_scope.clone();
        let self_scope = current_scope.self_scope().clone().self_scope;
        match (MacroType::try_from(&item), ObjectConversion::try_from(&item)) {
            (Ok(MacroType::Export), Ok(_object)) => {
                //println!("add_conversion.1: {}: {}", item.ident_string(), self_scope);
                if let Some(scope) = item.join_scope(&current_scope, self) {
                    self.find_scope_tree(&self_scope)
                        .add_item(item, scope);
                }
            },
            (_, Ok(_object)) if item.is_mod() => {
                //println!("add_conversion.1: {}: {}", item.ident_string(), self_scope);

                item.add_to_scope(&current_scope, self);
                self.find_scope_tree(&self_scope.popped())
                    .add_item(item, current_scope);
            },
            (Ok(MacroType::Register(path)), Ok(_object)) => {
                //println!("add_conversion.1: {}: {}", item.ident_string(), self_scope);
                if let ScopeTreeExportItem::Tree(scope_context, ..) = self.find_scope_tree(&self_scope) {
                    ident.map(|ident| {
                        scope_context.borrow()
                            .add_custom_conversion(current_scope, path, parse_quote!(#self_scope::#ident));
                    });
                }
            },
            (_, Ok(_object)) if ident.eq(&Some(&format_ident!("FFIConversion"))) => if let Item::Impl(..) = item {
                if let Some(_scope) = item.join_scope(&current_scope, self) {
                }
            },
            _ => {}
        }
        // println!("add_conversion.2: {}: {}", item.ident_string(), self_scope);

    }
}

