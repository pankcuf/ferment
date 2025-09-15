use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use quote::{format_ident, ToTokens};
use syn::{Attribute, Ident, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, parse_quote, Type, UseTree, Path, TypePath};
use syn::visit::Visit;
use crate::context::{GenericChain, GlobalContext, ScopeChain, TypeChain};
use crate::kind::{MacroKind, ObjectKind};
use crate::ext::{CrateExtension, extract_trait_names, MaybeIdent, ItemHelper, Join, MergeInto, UniqueNestedItems, Pop, VisitScope, VisitScopeType, ToPath, ToType};
use crate::tree::{ScopeTreeID, ScopeTreeExportItem};

pub struct Visitor {
    pub context: Rc<RefCell<GlobalContext>>,
    pub parent: Path,
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
        if node.ident.eq("fermented") {
            return;
        }
        let item = Item::Mod(node.clone());
        let module = self.current_module_scope.clone();
        self.current_module_scope = self.current_module_scope.joined(&item);
        self.add_conversion(Item::Mod(node.clone()));
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
    pub fn new(scope: &ScopeChain, attrs: &[Attribute], context: &Rc<RefCell<GlobalContext>>) -> Self {
        Self {
            context: context.clone(),
            parent: scope.to_path(),
            current_module_scope: scope.clone(),
            inner_visitors: vec![],
            tree: ScopeTreeExportItem::tree_with_context(scope, context.clone(), attrs)
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
        self.tree
    }
}

/// Global Context Facade
impl Visitor {

    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    pub(crate) fn fold_import_tree(&mut self, scope: &ScopeChain, use_tree: &UseTree, current_path: Vec<Ident>) {
        let mut lock = self.context.borrow_mut();
        lock.imports.fold_import_tree(scope, use_tree, current_path);
    }

    pub(crate) fn add_full_qualified_trait_match(&mut self, scope: &ScopeChain, item_trait: &ItemTrait, itself: &ObjectKind) {
        let mut lock = self.context.borrow_mut();
        lock.traits.add_trait(scope, item_trait, itself);
    }
    pub(crate) fn add_generic_chain(&mut self, scope: &ScopeChain, generics: GenericChain) {
        let mut lock = self.context.borrow_mut();
        lock.generics.extend_in_scope(scope, generics.inner)
    }

    pub(crate) fn scope_add_many(&self, types: TypeChain, scope: &ScopeChain) {
        let mut lock = self.context.borrow_mut();
        lock.scope_mut(scope)
            .add_many(types.inner.into_iter());
    }
    fn add_to_many_scopes(&self, types: TypeChain, scopes: &[&ScopeChain]) {
        let mut lock = self.context.borrow_mut();
        scopes.iter()
            .for_each(|scope|
                lock.scope_mut(scope)
                    .add_many(types.inner.clone().into_iter()));
    }
    pub(crate) fn scope_add_one(&self, ty: Type, object: ObjectKind, scope: &ScopeChain) {
        let mut lock = self.context.borrow_mut();
        lock.scope_mut(scope)
            .add_one(ty, object);
    }
    pub(crate) fn scope_add_self(&self, object: ObjectKind, scope: &ScopeChain) {
        self.scope_add_one(parse_quote!(Self), object, scope)
    }
    pub(crate) fn add_full_qualified_trait_type_from_macro(&mut self, item_trait_attrs: &[Attribute], scope: &ScopeChain) {
        let trait_names = extract_trait_names(item_trait_attrs);
        trait_names.iter().for_each(|trait_name| self.add_full_qualified_type_match(scope, &trait_name.to_type(), true));
        let mut lock = self.context.borrow_mut();
        lock.traits.add_used_traits(scope, trait_names)
    }

    pub(crate) fn create_type_chain<N>(&self, ty: &N, scope: &ScopeChain) -> TypeChain
    where N: UniqueNestedItems<Item = Type> {
        let context = self.context.borrow();
        TypeChain::from(
            ty.unique_nested_items()
                .into_iter()
                .map(|ty| {
                    let object = ty.visit_scope_type(&(scope, &context));
                    (ty, object)
                }))
    }
    pub(crate) fn add_full_qualified_type_chain(&mut self, scope: &ScopeChain, type_chain: TypeChain, add_to_parent: bool) {
        match scope {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } =>
                self.scope_add_many(type_chain.selfless(), scope),
            ScopeChain::Impl { parent, .. } => {
                if add_to_parent {
                    self.scope_add_many(type_chain.selfless(), parent);
                }
                self.scope_add_many(type_chain, scope);
            },
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Object { parent, .. } => {
                self.scope_add_many(type_chain.clone(), scope);
                self.scope_add_one(parse_quote!(Self), scope.self_object(), scope);
                if add_to_parent {
                    self.scope_add_many(type_chain.selfless(), parent);
                }
            },
            ScopeChain::Fn { parent, .. } => match &**parent {
                ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => if add_to_parent {
                    self.add_to_many_scopes(type_chain, &[scope, parent])
                } else {
                    self.scope_add_many(type_chain, scope);
                },
                ScopeChain::Trait { parent: parent_parent_scope_chain, .. } |
                ScopeChain::Object { parent: parent_parent_scope_chain, .. } |
                ScopeChain::Impl { parent: parent_parent_scope_chain, .. } => {
                    self.scope_add_one(parse_quote!(Self), scope.self_object(), scope);
                    self.scope_add_many(type_chain.clone(), scope);
                    if add_to_parent {
                        self.scope_add_many(type_chain.selfless(), parent_parent_scope_chain);
                        self.scope_add_many(type_chain, parent);
                    }
                },
                ScopeChain::Fn { parent: _parent_parent_scope_chain, .. } => {
                    // TODO: actually there are may be anything wrapped into anything like trait inside a function...
                }
            }
        }
    }

    pub(crate) fn add_full_qualified_type_match(&mut self, scope: &ScopeChain, ty: &Type, add_to_parent: bool) {
        // Filter out macro marker paths like `ferment_macro::export/register/opaque`
        let mut chain = self.create_type_chain(ty, scope);
        chain.inner.retain(|t, _| match t {
            Type::Path(TypePath { qself: None, path, .. }) =>
                !matches!(path.to_token_stream().to_string().replace(' ', "").as_str(), "ferment_macro::export" | "ferment_macro::register" | "ferment_macro::opaque"),
            _ => true,
        });
        self.add_full_qualified_type_chain(scope, chain, add_to_parent)
    }

    fn find_scope_tree(&mut self, scope: &Path) -> &mut ScopeTreeExportItem {
        let mut current_tree = &mut self.tree;
        for ident in scope.segments.crate_less().iter().map(ScopeTreeID::from) {
            if let ScopeTreeExportItem::Tree(scope_context, _, exported, attrs) = current_tree {
                if !exported.contains_key(&ident) {
                    exported.insert(ident.clone(), ScopeTreeExportItem::tree_with_context_and_exports(scope_context.clone(), attrs));
                }
                current_tree = exported.get_mut(&ident).unwrap();
            }
        }
        current_tree
    }

    pub fn add_conversion(&mut self, item: Item) {
        // TODO: filter out #[cfg(test)]
        let ident = item.maybe_ident();
        let current_scope = self.current_module_scope.clone();
        let self_scope = current_scope.to_path();
        match (MacroKind::try_from(&item), ObjectKind::try_from((&item, &self_scope))) {
            (Ok(MacroKind::Export | MacroKind::Opaque), Ok(_)) => if let Some(scope) = item.join_scope(&current_scope, self) {
                self.find_scope_tree(&self_scope)
                    .add_item(item, scope);
            },
            (_, Ok(_)) if item.is_mod() => {
                item.add_to_scope(&current_scope, self);
                self.find_scope_tree(&self_scope.popped())
                    .add_item(item, current_scope);
            },
            (Ok(MacroKind::Register(custom_type)), Ok(_)) => if let ScopeTreeExportItem::Tree(scope_context, ..) = self.find_scope_tree(&self_scope) {
                let scope_context_borrowed = scope_context.borrow();
                scope_context_borrowed.add_custom_conversion(current_scope, custom_type, parse_quote!(#self_scope::#ident));
            },
            (_, Ok(_)) => if ident.eq(&Some(&format_ident!("FFIConversionFrom"))) || ident.eq(&Some(&format_ident!("FFIConversionTo"))) || ident.eq(&Some(&format_ident!("FFIConversionDestroy"))) {
                if let Item::Impl(..) = item {
                    if let Some(_scope) = item.join_scope(&current_scope, self) {
                        // self.find_scope_tree(&self_scope)
                        //     .add_item(item, scope);
                    }
                }
            } /*else if let Item::Impl(..) = item {
                if let Some(scope) = item.join_scope(&current_scope, self) {
                    self.find_scope_tree(&self_scope)
                        .add_item(item, scope);
                }
            }*/,

            _ => {
            }
        }
    }
}
