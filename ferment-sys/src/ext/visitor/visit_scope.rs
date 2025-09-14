//! VisitScope: attach syn items to scope and collect types
//!
//! This module defines the [`VisitScope`] trait used by the tree traversal `Visitor` to
//! attach parsed syn items (modules, traits, impls, structs/enums, fns, type aliases)
//! to a semantic [`ScopeChain`], and to collect/import the types they reference into the
//! global context. The collection feeds later codegen and logging.
//!
//! Key behaviors (high level):
//! - Creates child scopes for items (`join_scope`) and populates the scope register (`add_to_scope`).
//! - Resolves paths against folded imports (rename/group) and records imported kinds.
//! - Records function argument and return types; handles generics and trait/impl-specific rules.
//! - Propagates non-Self, non-method-generic types upward to parent scopes for discoverability.
//! - Keeps Self-associated paths (e.g. `Self::Item`, `<Self as Trait>::Assoc`) in trait/impl scopes
//!   while not leaking them to parents.
//! - Method generics are collected in the function scope; their trait/impl scopes see the bounds but
//!   parents do not.
//!
//! Examples
//! --------
//! Attach a struct item to a module scope and collect its field types:
//!
//! ```ignore
//! # use syn::parse_quote;
//! # use ferment_sys::tree::Visitor;
//! # use ferment_sys::context::{GlobalContext, ScopeChain};
//! # use std::rc::Rc; use std::cell::RefCell;
//! # let ctx = Rc::new(RefCell::new(GlobalContext::with_config(ferment_sys::Config::new(
//! #   "fermented", ferment_sys::lang::rust::Crate::current_with_name("my_crate"), cbindgen::Config::default()))));
//! let mod_scope = ScopeChain::crate_root_with_ident(parse_quote!(my_crate), vec![]);
//! let mut visitor = Visitor::new(&mod_scope, &[], &ctx);
//! let item: syn::Item = parse_quote!(struct S { f: u32 });
//! // Creates/returns the child scope for `S` and populates the register
//! let _child = ferment_sys::ext::VisitScope::join_scope(&item, &mod_scope, &mut visitor).unwrap();
//! ```
//!
//! For trait/impl methods, argument and return type collection follows:
//! - Full chains recorded in the function scope
//! - Trait/impl scopes receive: Self-associated entries and non-method-generic entries
//! - Parent scopes receive: non-method-generic entries only
//!
//! This strikes a balance between local accuracy (compose at trait/impl scope) and preventing
//! leakage of `Self`-anchored semantics into outer scopes.

use std::collections::{HashMap, HashSet};
use indexmap::IndexMap;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, ConstParam, Field, FnArg, GenericParam, Generics, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, Item, ItemFn, ItemImpl, ItemMod, ItemTrait, LifetimeParam, Meta, parse_quote, Path, PatType, PredicateType, ReturnType, Signature, TraitBound, TraitItem, TraitItemConst, TraitItemFn, TraitItemType, Type, TypeParam, TypeParamBound, Variant, WhereClause, WherePredicate, TypePath, PathSegment, TraitBoundModifier, ItemEnum, ItemStruct, ItemType, QSelf};
use syn::parse::Parser;
use crate::ast::{AddPunctuated, CommaPunctuated, CommaPunctuatedTokens};
use crate::composable::{NestedArgument, TraitDecompositionPart1, TraitModel, TypeModel};
use crate::composer::{CommaPunctuatedNestedArguments, MaybeMacroLabeled};
use crate::context::{GenericChain, ScopeChain};
use crate::kind::{MacroKind, ObjectKind, ScopeItemKind, TypeModelKind};
use crate::ext::{Join, MaybeTraitBound, ToType, GenericBoundKey};
use crate::ext::maybe_ident::collect_bounds;
use crate::tree::Visitor;

/// Trait implemented for syn items to attach themselves to a scope and populate the global context.
///
/// The default `Visitor` traversal calls these methods to:
/// - Determine the child scope for an item (`join_scope`).
/// - Collect referenced types, imports, and generics into the appropriate scopes (`add_to_scope`).
///
/// Semantics summary:
/// - Struct/Enum/Type/Fn: create object or fn scopes under the current scope; record field/arg/return types.
/// - Module: re-traverse inner content and attach nested items.
/// - Trait:
///   - Record trait itself in trait scope and alias in parent.
///   - For methods: record full chains in fn scope; add Self-associated and non-method-generic entries to trait scope;
///     add non-method-generic entries to the parent.
/// - Impl:
///   - For inherent/trait impl methods: record as for traits, but under impl scope.
/// - Generics: method generics are collected in the fn scope; bounds are visible in trait/impl scope; parents do not receive them.
pub trait VisitScope {
    /// Creates a child scope for `self` under `scope`, attaches/collects the item, and returns the child.
    ///
    /// Returns `None` for items that do not participate in scope creation (e.g., unsupported kinds).
    fn join_scope(&self, scope: &ScopeChain, visitor: &mut Visitor) -> Option<ScopeChain>;

    /// Populates the global context with information derived from `self` within `scope`.
    ///
    /// Collects:
    /// - Resolved field/argument/return types (including imports and unknowns)
    /// - Generics and bounds (recorded at fn/trait/impl scopes as described above)
    /// - Trait/impl method specifics (Self-associated vs non-method-generic propagation)
    fn add_to_scope(&self, scope: &ScopeChain, visitor: &mut Visitor);
}

impl VisitScope for Item {
    fn join_scope(&self, scope: &ScopeChain, visitor: &mut Visitor) -> Option<ScopeChain> {
        match self {
            Item::Struct(..) |
            Item::Enum(..) |
            Item::Fn(..) |
            Item::Trait(..) |
            Item::Type(..) |
            Item::Impl(..) => {
                let scope = scope.joined(self);
                self.add_to_scope(&scope, visitor);
                Some(scope)
            },
            |
            Item::Mod(..) => {
                self.add_to_scope(scope, visitor);
                Some(scope.clone())
            },
            _ => None
        }
    }
    fn add_to_scope(&self, scope: &ScopeChain, visitor: &mut Visitor) {
        let self_scope = scope.self_path_ref();
        match self {
            Item::Mod(item_mod) =>
                add_inner_module_conversion(visitor, item_mod, scope),
            Item::Const(_) | Item::Static(_) | Item::Union(_) => {
                // TODO: Const/Static/Union scope processing
            }
            Item::Enum(item_enum) => {
                let ItemEnum { attrs, generics, ident, variants, .. } = item_enum;
                let (nested_arguments, inner_args) = add_full_qualified_generics(visitor, generics, scope, true);
                let full_ty = if !inner_args.is_empty() {
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };
                let self_object = ObjectKind::new_generic_obj_item(
                    full_ty,
                    generics,
                    nested_arguments,
                    ScopeItemKind::item_enum(item_enum, self_scope));
                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, ident, self_object);
                visitor.add_full_qualified_trait_type_from_macro(attrs, scope);
                let generic_chain = create_generics_chain(generics);
                visitor.add_generic_chain(scope, generic_chain);

                variants.iter().for_each(|Variant { fields, .. }|
                    fields.iter().for_each(|Field { ty, .. }|
                        visitor.add_full_qualified_type_match(scope, ty, true)));

            }
            Item::Struct(item_struct) => {
                let ItemStruct { attrs, generics, ident, fields, .. } = item_struct;
                let (nested_arguments, inner_args) = add_full_qualified_generics(visitor, generics, scope, true);
                let full_ty = if !inner_args.is_empty() {
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };
                let self_object = ObjectKind::new_generic_obj_item(
                    full_ty,
                    generics,
                    nested_arguments,
                    ScopeItemKind::item_struct(item_struct, self_scope));
                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, ident, self_object);
                visitor.add_full_qualified_trait_type_from_macro(attrs, scope);
                let generic_chain = create_generics_chain(generics);
                visitor.add_generic_chain(scope, generic_chain);

                fields.iter().for_each(|Field { ty, .. }|
                    visitor.add_full_qualified_type_match(scope, ty,true));
            }
            Item::Fn(ItemFn { sig, .. }) => {
                let Signature { ident, generics, .. } = sig;
                let self_object = ObjectKind::new_fn_item(TypeModel::new_generic_scope_non_nested(scope, generics), ScopeItemKind::fn_ref(sig, self_scope));
                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, ident, self_object);
                add_full_qualified_signature(visitor, sig, scope);
            }
            Item::Trait(item_trait) =>
                add_full_qualified_trait(visitor, item_trait, scope),
            Item::Type(item_type) => {
                let ItemType { generics, ident, ty, .. } = item_type;
                let (nested_arguments, inner_args) = add_full_qualified_generics(visitor, generics, scope, true);
                let full_ty = if !inner_args.is_empty() {
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };
                let self_object = ObjectKind::model_item(
                    if let Type::BareFn(..) = &**ty {
                        TypeModelKind::FnPointer
                    } else {
                        TypeModelKind::Object
                    },
                    TypeModel::new_generic(full_ty, generics.clone(), nested_arguments),
                    ScopeItemKind::item_type(item_type, self_scope));

                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, ident, self_object);
                let generic_chain = create_generics_chain(generics);
                visitor.add_generic_chain(scope, generic_chain);

                visitor.add_full_qualified_type_match(scope, ty, true);
            }
            Item::Impl(ItemImpl { generics, trait_, self_ty, items , ..}) => {
                if let Some((_, path, _)) = trait_ {
                    visitor.add_full_qualified_type_match(scope, &path.to_type(), true);
                }
                visitor.add_full_qualified_type_match(scope, self_ty, false);
                let (_nested_arguments, _inner_args) = add_full_qualified_generics(visitor, generics, scope, true);
                let generic_chain = create_generics_chain(generics);
                visitor.add_generic_chain(scope, generic_chain);
                items.iter().for_each(|impl_item| match impl_item {
                    ImplItem::Const(ImplItemConst { ident, ty, generics, .. }) => {
                        visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                        visitor.add_full_qualified_type_match(scope, ty, true);
                        let (_nested_const_arguments, _inner_const_args) = add_full_qualified_generics(visitor, generics, scope, true);
                    },
                    ImplItem::Fn(impl_method) => {
                        let ImplItemFn { sig, .. } = impl_method;
                        let Signature { ident, inputs, output, generics, .. } = sig;
                        let fn_scope = scope.joined(impl_method);
                        if let Some((_, path, _)) = trait_ {
                            visitor.add_full_qualified_type_match(&fn_scope, &path.to_type(), false);
                        }
                        visitor.add_full_qualified_type_match(&fn_scope, self_ty, false);
                        visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                        if let ReturnType::Type(_, ty) = output {
                            // Return type: add to fn scope; add filtered sets to impl and its parent
                            let fn_chain = visitor.create_type_chain(&**ty, &fn_scope);
                            visitor.scope_add_many(fn_chain, &fn_scope);

                            let full_in_impl = visitor.create_type_chain(&**ty, scope);
                            let impl_self_assoc = full_in_impl.only_self_associated();
                            let impl_non_method_generics = visitor.create_type_chain(&**ty, scope).excluding_self_and_bounds(generics);
                            let parent_type_chain = impl_non_method_generics.clone();

                            if !impl_self_assoc.inner.is_empty() {
                                visitor.scope_add_many(impl_self_assoc, scope);
                            }
                            visitor.scope_add_many(impl_non_method_generics, scope);
                            if let Some(parent_scope) = scope.parent_scope() {
                                visitor.scope_add_many(parent_type_chain, parent_scope);
                            }
                        }
                        inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
                            // Record full chain in fn scope
                            let type_chain = visitor.create_type_chain(&**ty, &fn_scope);
                            visitor.scope_add_many(type_chain, &fn_scope);

                            // For impl scope: include non-method-generics and also Self-associated paths
                            let full_in_impl = visitor.create_type_chain(&**ty, scope);
                            let impl_self_assoc = full_in_impl.only_self_associated();
                            let impl_non_method_generics = visitor.create_type_chain(&**ty, scope).excluding_self_and_bounds(generics);

                            // Parent of impl: keep only non-method-generics
                            let parent_type_chain = impl_non_method_generics.clone();

                            // Add to impl scope
                            if !impl_self_assoc.inner.is_empty() {
                                visitor.scope_add_many(impl_self_assoc, scope);
                            }
                            visitor.scope_add_many(impl_non_method_generics, scope);

                            // Propagate to parent appropriately
                            if let Some(parent_scope) = scope.parent_scope() {
                                visitor.scope_add_many(parent_type_chain, parent_scope);
                            }
                        });
                        let (_nested_fn_arguments, _inner_fn_args) = add_full_qualified_generics(visitor, generics, &fn_scope, false);
                        // Also add method generic bounds (e.g., V: Into<...>) to the trait scope itself,
                        // so trait-level composition can resolve those paths. Do not propagate to parent.
                        let _ = add_full_qualified_generics(visitor, generics, scope, false);

                        let generic_chain = create_generics_chain(generics);
                        visitor.add_generic_chain(&fn_scope, generic_chain);

                    },
                    ImplItem::Type(ImplItemType { ident, ty, generics, .. }) => {
                        visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                        visitor.add_full_qualified_type_match(scope, ty, true);
                        let (_nested_type_arguments, _inner_type_args) =  add_full_qualified_generics(visitor, generics, scope, false);
                        let generic_chain = create_generics_chain(generics);
                        visitor.add_generic_chain(scope, generic_chain);
                    },
                    _ => {}
                });
            }
            _ => {}
        }
    }
}

fn add_full_qualified_generics(visitor: &mut Visitor, generics: &Generics, scope: &ScopeChain, add_to_parent: bool) -> (CommaPunctuatedNestedArguments, CommaPunctuatedTokens) {
    let Generics { params, where_clause, .. } = generics;
    let mut nested_arguments = CommaPunctuated::new();
    let mut inner_args = CommaPunctuated::new();
    params.iter().for_each(|p| match p {
        GenericParam::Type(TypeParam { ident, bounds, .. }) => {
            inner_args.push(ident.to_token_stream());
            let mut nested_type_arguments = CommaPunctuated::new();
            bounds.iter().for_each(|bound| {
                if let Some(trait_bound) = bound.maybe_trait_bound() {
                    nested_type_arguments.push(NestedArgument::trait_bound_object(trait_bound));
                    visitor.add_full_qualified_type_match(scope, &trait_bound.path.to_type(), add_to_parent);
                }
            });
            nested_arguments.push(NestedArgument::trait_model_constraint(ident, generics, nested_type_arguments));
        }
        GenericParam::Const(ConstParam { ident, ty, .. }) => {
            inner_args.push(ident.to_token_stream());
            visitor.add_full_qualified_type_match(scope, ty, add_to_parent);
            nested_arguments.push(NestedArgument::object_model_constraint(ident, generics))
        },
        GenericParam::Lifetime(LifetimeParam { lifetime, .. }) =>
            inner_args.push(lifetime.to_token_stream()),
    });
    if let Some(WhereClause { predicates, .. }) = where_clause {
        predicates.iter().for_each(|pred| if let WherePredicate::Type(PredicateType { bounds, .. }) = pred {
            bounds.iter().for_each(|bound| {
                if let Some(trait_bound) = bound.maybe_trait_bound() {
                    visitor.add_full_qualified_type_match(scope, &trait_bound.path.to_type(), add_to_parent);
                }
            });
        });
    }
    (nested_arguments, inner_args)
}

fn add_full_qualified_trait(visitor: &mut Visitor, item_trait: &ItemTrait, scope: &ScopeChain) {
    let ItemTrait { generics, ident, supertraits, items, .. } = item_trait;
    let trait_type = ident.to_type();
    let type_compo = TypeModel::new_generic_scope_non_nested(scope, generics);
    let itself = ObjectKind::new_trait_item(
        TraitModel::new(type_compo, TraitDecompositionPart1::from_trait_items(ident, items), add_bounds(visitor, supertraits, scope, true)),
        ScopeItemKind::item_trait(item_trait, scope.self_path_ref()));

    // 1. Add itself to the scope as <Self, Item(Trait(..))>
    // 2. Add itself to the parent scope as <Ident, Item(Trait(..))>
    visitor.add_full_qualified_trait_match(scope, item_trait, &itself);

    items.iter().for_each(|trait_item|
        match trait_item {
            TraitItem::Const(TraitItemConst { ident, ty, .. }) => {
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                visitor.add_full_qualified_type_match(scope, ty, true);
            },
            TraitItem::Fn(trait_item_method) => {
                let TraitItemFn { sig, .. } = trait_item_method;
                let Signature { ident, generics, inputs, output, .. } = sig;
                let fn_scope = scope.joined(trait_item_method);
                visitor.add_full_qualified_type_match(&fn_scope, &trait_type, false);
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                if let ReturnType::Type(_, ty) = output {
                    // Return type: add to fn scope; add filtered sets to trait and its parent
                    let mut fn_chain = visitor.create_type_chain(&**ty, &fn_scope);
                    let full_in_trait = visitor.create_type_chain(&**ty, scope);
                    let trait_self_assoc = full_in_trait.only_self_associated();
                    let trait_non_method_generics = visitor.create_type_chain(&**ty, scope).excluding_self_and_bounds(generics);
                    let parent_type_chain = trait_non_method_generics.clone();

                    fn_chain.add_self(scope.self_object());
                    visitor.scope_add_many(fn_chain, &fn_scope);
                    visitor.scope_add_many(trait_non_method_generics, scope);
                    if !trait_self_assoc.inner.is_empty() {
                        visitor.scope_add_many(trait_self_assoc, scope);
                    }
                    if let Some(parent_scope) = scope.parent_scope() {
                        visitor.scope_add_many(parent_type_chain, parent_scope);
                    }
                }
                inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
                    let mut type_chain = visitor.create_type_chain(&**ty, &fn_scope);
                    // For trait scope: include non-method-generics and also Self-associated paths
                    let full_in_trait = visitor.create_type_chain(&**ty, scope);
                    let trait_self_assoc = full_in_trait.only_self_associated();
                    let trait_non_method_generics = visitor.create_type_chain(&**ty, scope).excluding_self_and_bounds(generics);

                    // For parent of trait: keep only non-method-generics, exclude Self-associated
                    let parent_type_chain = trait_non_method_generics.clone();

                    type_chain.add_self(scope.self_object());
                    visitor.scope_add_many(type_chain, &fn_scope);
                    // Add both non-method-generic and Self-associated entries to trait scope
                    visitor.scope_add_many(trait_non_method_generics.clone(), scope);
                    if !trait_self_assoc.inner.is_empty() {
                        visitor.scope_add_many(trait_self_assoc, scope);
                    }
                    if let Some(parent_scope) = scope.parent_scope() {
                        visitor.scope_add_many(parent_type_chain, parent_scope);
                    }
                });
                let (_nested_arguments, _inner_args) = add_full_qualified_generics(visitor, generics, &fn_scope, false);
                // Also include method generic bounds at trait scope for composition; not to parent
                let _ = add_full_qualified_generics(visitor, generics, scope, false);

                let generic_chain = create_generics_chain(generics);
                visitor.add_generic_chain(&fn_scope, generic_chain);
            }
            TraitItem::Type(TraitItemType { ident, bounds, generics, .. }) => {
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                add_bounds(visitor, bounds, scope, true);
                let (_nested_arguments, _inner_args) = add_full_qualified_generics(visitor, generics, scope, false);
                let generic_chain = create_generics_chain(generics);
                visitor.add_generic_chain(scope, generic_chain);
            },
            _ => {}
        });
    if let Some(parent_scope) = scope.parent_scope() {
        visitor.scope_add_one(ident.to_type(), itself.clone(), parent_scope);
    }
    visitor.scope_add_self(itself, scope);
    let (_nested_arguments, _inner_args) = add_full_qualified_generics(visitor, generics, scope, true);

    let generic_chain = create_generics_chain(generics);
    visitor.add_generic_chain(scope, generic_chain);

}

fn add_full_qualified_signature(visitor: &mut Visitor, sig: &Signature, scope: &ScopeChain) {
    let Signature { output, inputs, generics, .. } = sig;
    if let ReturnType::Type(_, ty) = output {
        // TODO: Prevent generic bound from adding to parent here
        visitor.add_full_qualified_type_match(scope, ty, true);
    }
    inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
        // TODO: Prevent generic bound from adding to parent here
        // It's easy when arg is non-compound type, i.e. itself
        // It's hard when bound is a part of arg i.e. T: Into<U>
        // where "Into" SHOULD persist in the parent scope,
        // T: shouldn't if sig generics contain it
        // U: should if sig generics contain it
        visitor.add_full_qualified_type_match(scope, ty, true);
    });

    let (_nested_arguments, _inner_args) = add_full_qualified_generics(visitor, generics, scope, true);

    let generic_chain = create_generics_chain(generics);
    visitor.add_generic_chain(scope, generic_chain);
}

fn add_inner_module_conversion(visitor: &mut Visitor, item_mod: &ItemMod, scope: &ScopeChain) {
    if let Some((_, items)) = &item_mod.content {
        items.iter().for_each(|item| match item {
            Item::Use(node) =>
                visitor.fold_import_tree(scope, &node.tree, vec![]),
            Item::Mod(..) =>
                item.add_to_scope(&scope.joined(item), visitor),
            Item::Trait(..) |
            Item::Fn(..) |
            Item::Struct(..) |
            Item::Enum(..) |
            Item::Type(..) |
            Item::Impl(..) => if MacroKind::try_from(item).is_ok() {
                item.add_to_scope(&scope.joined(item), visitor)
            },
            _ => {}
        })
    }
}

fn add_bounds(visitor: &mut Visitor, bounds: &AddPunctuated<TypeParamBound>, scope: &ScopeChain, add_to_parent: bool) -> Vec<Path> {
    let bounds = collect_bounds(bounds);
    bounds.iter().for_each(|path| visitor.add_full_qualified_type_match(scope, &path.to_type(), add_to_parent));
    bounds
}

fn collect_trait_bounds(bounds: &AddPunctuated<TypeParamBound>) -> Vec<Path> {
    bounds.iter()
        .filter_map(|b|
            b.maybe_trait_bound().and_then(|TraitBound { modifier, path, .. }|
                (matches!(modifier, TraitBoundModifier::None) && !path.segments.last().map(|PathSegment { ident, .. }| ident.eq("Sized")).unwrap_or_default()).then(|| path.clone())))
        .collect()
}

/// Collects trait bounds from both type parameter bounds and where-clause predicates
/// into a single, deterministically ordered list. Only `TypeParamBound::Trait` and
/// `WherePredicate::Type(..)` with trait bounds are considered.
pub fn create_generics_chain(generics: &Generics) -> GenericChain {
    let Generics { params, where_clause, .. } = generics;
    let mut generics_chain = IndexMap::<Type, Vec<Path>>::new();
    // 1) Bounds in angle brackets: `fn foo<T: Trait, U: A + B>() {}`
    params.iter().for_each(|gp| if let GenericParam::Type(TypeParam { bounds, ident, .. }) = gp {
        generics_chain
            .entry(ident.to_type())
            .or_default()
            .extend(collect_trait_bounds(bounds));
    });
    // 2) Where clause predicates: `where T: Trait, Vec<U>: Another`
    if let Some(WhereClause { predicates, .. }) = where_clause {
        predicates.iter().for_each(|pred| if let WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) = pred {
            generics_chain
                .entry(bounded_ty.clone())
                .or_default()
                .extend(collect_trait_bounds(bounds))
        });
    }
    // Ensure each generic type parameter appears at least once; add unlimited if no restrictive bound collected
    params.iter().for_each(|gp| match gp {
        GenericParam::Type(TypeParam { ident, .. }) if !generics_chain.keys().any(|bounded_ty| ident.eq(&bounded_ty.to_token_stream().to_string())) => {
            generics_chain.entry(ident.to_type())
                .or_default();
        }
        _ => {}
    });
    // Dedup per-type trait paths by token string and order deterministically
    for trait_paths in generics_chain.values_mut() {
        let mut seen_p: HashSet<String> = HashSet::new();
        trait_paths.retain(|p| seen_p.insert(p.to_token_stream().to_string()));
        trait_paths.sort_by(|a, b| {
            let a_s = a.to_token_stream().to_string();
            let b_s = b.to_token_stream().to_string();
            let w = |s: &str| u8::from(normalize_tokens(s).starts_with("::"));
            w(&a_s)
                .cmp(&w(&b_s))
                .then_with(|| a_s.cmp(&b_s))
        });
    }
    // If a bounded type has any restrictive trait bounds, drop its unlimited entries
    let mut has_restrictive: HashMap<String, bool> = HashMap::new();
    for (bounded_ty, trait_paths) in &generics_chain {
        let ty_s = bounded_ty.to_token_stream().to_string();
        let e = has_restrictive.entry(ty_s).or_insert(false);
        if !trait_paths.is_empty() {
            *e = true;
        }
    }
    generics_chain.retain(|bounded_ty, trait_paths| if trait_paths.is_empty() {
        let ty_s = bounded_ty.to_token_stream().to_string();
        !has_restrictive.get(&ty_s).copied().unwrap_or_default()
    } else {
        true
    });
    sort_generic_chain(&mut generics_chain);
    GenericChain::new(generics_chain)
}

/// Deterministic order: first by bounded type, then by trait path (both token strings)
fn sort_generic_chain(chain: &mut IndexMap<Type, Vec<Path>>) {
    chain.sort_by(|a_ty, a_paths, b_ty, b_paths| {
        // Prefer simple type parameters (single-segment, no leading ::) before concrete/qualified types
        let type_weight = |t: &Type| match t {
            // simple ident like `T`, `U`
            Type::Path(TypePath { qself: None, path: Path { leading_colon: None, segments } }) if segments.len() == 1 => 0,
            _ => 1
        };
        let a_ty_s = a_ty.to_token_stream().to_string();
        let a_tr_s = Vec::from_iter(a_paths.iter().map(|p| p.to_token_stream().to_string())).join(" + ");
        let b_ty_s = b_ty.to_token_stream().to_string();
        let b_tr_s = Vec::from_iter(b_paths.iter().map(|p| p.to_token_stream().to_string())).join(" + ");
        let a_tw = type_weight(a_ty);
        let b_tw = type_weight(b_ty);
        // Prefer bare trait names over fully-qualified ones; unlimited last
        let trait_weight = |s: &str| {
            if s == "<unlimited>" { 2 }
            else if normalize_tokens(s).starts_with("::") { 1 }
            else { 0 }
        };
        a_tw
            .cmp(&b_tw)
            .then_with(|| a_ty_s.cmp(&b_ty_s))
            .then_with(|| trait_weight(&a_tr_s).cmp(&trait_weight(&b_tr_s)))
            .then_with(|| a_tr_s.cmp(&b_tr_s))
    });
}

fn normalize_tokens<S: AsRef<str>>(s: S) -> String {
    s.as_ref().replace(' ', "")
}

fn anchor_string_of_bounded_ty(ty: &Type) -> String {
    match ty {
        // For qualified paths like `<Self::Item::Value as Trait>::Assoc`,
        // anchor on the inner `ty` (e.g. `Self::Item::Value`).
        Type::Path(TypePath { qself: Some(QSelf { ty, .. }), .. }) => ty.to_token_stream().to_string(),
        _ => ty.to_token_stream().to_string(),
    }
}

/// Filters the generics chain to constraints related to the provided key.
/// Related means the bounded type is the key itself or an associated path stemming
/// from it (e.g., `Self::Item`, `Self::Item::Key`, `<Self::Item::Value as Trait>::Assoc`).
#[allow(unused)]
pub fn create_generics_chain_for(generics: &Generics, key: &GenericBoundKey) -> GenericChain {
    let mut full = create_generics_chain(generics).inner;
    let key_s = normalize_tokens(key.to_token_stream().to_string());
    full.retain(|bounded_ty, _| {
        let anchor = normalize_tokens(anchor_string_of_bounded_ty(bounded_ty));
        anchor == key_s || anchor.starts_with(&(key_s.clone() + "::"))
    });
    sort_generic_chain(&mut full);
    GenericChain::new(full)
}

/// Filters the generics chain to only the exact key match (no associated descendants).
#[allow(unused)]
pub fn create_generics_chain_exact(generics: &Generics, key: &GenericBoundKey) -> GenericChain {
    let mut full = create_generics_chain(generics).inner;
    let key_s = normalize_tokens(key.to_token_stream().to_string());
    full.retain(|bounded_ty, _| normalize_tokens(anchor_string_of_bounded_ty(bounded_ty)) == key_s);
    sort_generic_chain(&mut full);
    GenericChain::new(full)
}

fn add_itself_conversion(visitor: &mut Visitor, scope: &ScopeChain, ident: &Ident, object: ObjectKind) {
    visitor.scope_add_one(ident.to_type(), object, scope);
}

pub fn extract_trait_names(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();
    attrs.iter().for_each(|attr| {
        if attr.is_labeled_for_export() {
            if let Meta::List(meta_list) = &attr.meta {
                if let Ok(nested) = CommaPunctuated::<Meta>::parse_terminated.parse2(meta_list.tokens.clone()) {
                    for meta_item in nested.iter() {
                        if let Meta::Path(path) = meta_item {
                            paths.push(path.clone());
                        }
                    }
                }
            }

        }
    });
    paths
}
