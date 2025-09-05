use std::collections::{HashMap, HashSet};
use indexmap::IndexMap;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, ConstParam, Field, FnArg, GenericParam, Generics, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, Item, ItemFn, ItemImpl, ItemMod, ItemTrait, LifetimeParam, Meta, parse_quote, Path, PatType, PredicateType, ReturnType, Signature, TraitBound, TraitItem, TraitItemConst, TraitItemFn, TraitItemType, Type, TypeParam, TypeParamBound, Variant, WhereClause, WherePredicate, TypePath, PathSegment, TraitBoundModifier};
use syn::parse::Parser;
use crate::ast::{AddPunctuated, CommaPunctuated};
use crate::composable::{NestedArgument, TraitDecompositionPart1, TraitModel, TypeModel};
use crate::composer::MaybeMacroLabeled;
use crate::context::ScopeChain;
use crate::kind::{MacroKind, ObjectKind, ScopeItemKind, TypeModelKind};
use crate::ext::{Join, ToType};
use crate::ext::item::collect_bounds;
use crate::tree::Visitor;

pub trait VisitScope {
    fn join_scope(&self, scope: &ScopeChain, visitor: &mut Visitor) -> Option<ScopeChain>;
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
                self.add_to_scope(&scope, visitor);
                Some(scope.clone())
            },
            _ => None
        }
    }
    fn add_to_scope(&self, scope: &ScopeChain, visitor: &mut Visitor) {
        let self_scope = scope.self_path_ref();
        match self {
            Item::Mod(item_mod) => {
                add_inner_module_conversion(visitor, item_mod, scope);
            }
            Item::Const(_) => {
                // TODO: Const scope processing
            }
            Item::Enum(item_enum) => {
                let mut nested_arguments = CommaPunctuated::new();
                let full_ty = if !item_enum.generics.params.is_empty() || item_enum.generics.where_clause.is_some() {
                    let mut inner_args = CommaPunctuated::new();
                    item_enum.generics.params.iter().for_each(|p| match p {
                        GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                            inner_args.push(quote!(#ident));
                            let mut nested_bounds = CommaPunctuated::new();
                            bounds.iter().for_each(|pp| match pp {
                                TypeParamBound::Trait(TraitBound { path, .. }) => {
                                    // TODO: make it Unknown
                                    nested_bounds.push(NestedArgument::Object(ObjectKind::trait_model_type(TypeModel::new_default(path.to_type()))));
                                }
                                _ => {}
                            });
                            // TODO: make it Unknown
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::trait_model_type(TypeModel::new_generic(ident.to_type(), item_enum.generics.clone(), nested_bounds))));

                        }
                        GenericParam::Const(ConstParam { ident, ty: _, .. }) => {
                            inner_args.push(quote!(#ident));
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::object_model_type(TypeModel::new_generic_non_nested(ident.to_type(), item_enum.generics.clone()))))
                        },
                        GenericParam::Lifetime(LifetimeParam { lifetime, bounds: _, .. }) => {
                            inner_args.push(quote!(#lifetime));
                        },
                    });
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };

                let self_object = ObjectKind::new_generic_obj_item(
                    full_ty,
                    item_enum.generics.clone(),
                    nested_arguments,
                    ScopeItemKind::item_enum(item_enum, self_scope));
                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, &item_enum.ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, &item_enum.ident, self_object);
                visitor.add_full_qualified_trait_type_from_macro(&item_enum.attrs, scope);
                visitor.add_generic_chain(scope, &item_enum.generics, true);
                item_enum.variants.iter().for_each(|Variant { fields, .. }|
                    fields.iter().for_each(|Field { ty, .. }|
                        visitor.add_full_qualified_type_match(scope, ty, true)));

            }
            Item::Struct(item_struct) => {
                let mut nested_arguments = CommaPunctuated::new();
                let full_ty = if !item_struct.generics.params.is_empty() || item_struct.generics.where_clause.is_some() {
                    let mut inner_args = CommaPunctuated::new();
                    item_struct.generics.params.iter().for_each(|p| match p {
                        GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                            inner_args.push(quote!(#ident));
                            let mut nested_bounds = CommaPunctuated::new();
                            bounds.iter().for_each(|pp| match pp {
                                TypeParamBound::Trait(TraitBound { path, .. }) =>
                                    nested_bounds.push(NestedArgument::Object(ObjectKind::trait_model_type(TypeModel::new_default(path.to_type())))),
                                _ => {}
                            });
                            // TODO: make it Unknown
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::trait_model_type(TypeModel::new_generic(ident.to_type(), item_struct.generics.clone(), nested_bounds))));

                        }
                        GenericParam::Const(ConstParam { ident, .. }) => {
                            inner_args.push(quote!(#ident));
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::object_model_type(TypeModel::new_generic_non_nested(ident.to_type(), item_struct.generics.clone()))))
                        },
                        GenericParam::Lifetime(LifetimeParam { lifetime, .. }) =>
                            inner_args.push(quote!(#lifetime)),
                    });
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };
                let self_object = ObjectKind::new_generic_obj_item(
                    full_ty,
                    item_struct.generics.clone(),
                    nested_arguments,
                    ScopeItemKind::item_struct(item_struct, self_scope));
                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, &item_struct.ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, &item_struct.ident, self_object);
                visitor.add_full_qualified_trait_type_from_macro(&item_struct.attrs, scope);
                visitor.add_generic_chain(scope, &item_struct.generics, true);
                item_struct.fields.iter().for_each(|Field { ty, .. }|
                    visitor.add_full_qualified_type_match(scope, ty,true));
            }
            Item::Fn(ItemFn { sig, .. }) => {
                let self_object = ObjectKind::new_fn_item(
                    TypeModel::new_generic_non_nested(scope.to_type(), sig.generics.clone()),
                    ScopeItemKind::fn_ref(sig, self_scope));
                let sig_ident = &sig.ident;
                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, sig_ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, sig_ident, self_object);
                add_full_qualified_signature(visitor, sig, scope);
            }
            Item::Trait(item_trait) =>
                add_full_qualified_trait(visitor, item_trait, scope),
            Item::Type(item_type) => {
                let self_object = ObjectKind::model_item(
                    if let Type::BareFn(..) = &*item_type.ty {
                        TypeModelKind::FnPointer
                    } else {
                        TypeModelKind::Object
                    },
                    TypeModel::new_non_nested(scope.to_type(), Some(item_type.generics.clone())),
                    ScopeItemKind::item_type(item_type, self_scope));

                if let Some(parent_scope) = scope.parent_scope() {
                    add_itself_conversion(visitor, parent_scope, &item_type.ident, self_object.clone());
                }
                add_itself_conversion(visitor, scope, &item_type.ident, self_object);
                visitor.add_generic_chain(scope, &item_type.generics, true);
                visitor.add_full_qualified_type_match(scope, &item_type.ty, true);
            }
            Item::Impl(ItemImpl { generics, trait_, self_ty, items , ..}) => {
                if let Some((_, path, _)) = trait_ {
                    visitor.add_full_qualified_type_match(scope, &path.to_type(), true);
                }
                visitor.add_full_qualified_type_match(scope, self_ty, false);
                visitor.add_generic_chain(scope, generics, true);
                items.iter().for_each(|impl_item| {
                    match impl_item {
                        ImplItem::Const(ImplItemConst { ident, ty, .. }) => {
                            visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                            visitor.add_full_qualified_type_match(scope, ty, true);
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
                                visitor.add_full_qualified_type_chain(&fn_scope, visitor.create_type_chain(ty, scope), true);
                            }
                            inputs.iter().for_each(|arg| {
                                match arg {
                                    FnArg::Receiver(..) => {
                                        // visitor.add_full_qualified_type_match(scope, self_ty, false);
                                    },
                                    FnArg::Typed(PatType { ty, .. }) => {
                                        let type_chain = visitor.create_type_chain(ty, &fn_scope);
                                        let parent_type_chain = visitor.create_type_chain(ty, scope).excluding_self_and_bounds(generics);
                                        let mut type_chains = HashMap::from_iter([
                                            (fn_scope.clone(), type_chain),
                                            (scope.clone(), parent_type_chain.clone()),
                                        ]);
                                        if let Some(parent_scope) = scope.parent_scope() {
                                            type_chains.insert(parent_scope.clone(), parent_type_chain);
                                        }
                                        visitor.add_full_qualified_type_chains(type_chains);
                                    }
                                }
                            });
                            visitor.add_generic_chain(&fn_scope, generics, false);
                        },
                        ImplItem::Type(ImplItemType { ident, ty, generics, .. }) => {
                            visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                            visitor.add_full_qualified_type_match(scope, ty, true);
                            visitor.add_generic_chain(scope, generics, false);
                        },
                        _ => {}
                    }
                });
            }
            _ => {}
        }
    }
}
fn add_full_qualified_trait(visitor: &mut Visitor, item_trait: &ItemTrait, scope: &ScopeChain) {
    let ident = &item_trait.ident;
    let trait_type = ident.to_type();
    let type_compo = TypeModel::new_generic_non_nested(scope.to_type(), item_trait.generics.clone());
    let itself = ObjectKind::new_trait_item(
        TraitModel::new(type_compo, TraitDecompositionPart1::from_trait_items(ident, &item_trait.items), add_bounds(visitor, &item_trait.supertraits, scope, true)),
        ScopeItemKind::item_trait(item_trait, scope.self_path_ref()));

    // 1. Add itself to the scope as <Self, Item(Trait(..))>
    // 2. Add itself to the parent scope as <Ident, Item(Trait(..))>
    visitor.add_full_qualified_trait_match(&scope, item_trait, &itself);
    item_trait.items.iter().for_each(|trait_item|
        match trait_item {
            TraitItem::Const(TraitItemConst { ident, ty, .. }) => {
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                visitor.add_full_qualified_type_match(scope, ty, true);
            },
            TraitItem::Fn(trait_item_method) => {
                let TraitItemFn { sig, .. } = trait_item_method;
                let Signature { ident, generics, inputs, output, .. } = sig;
                let fn_scope = scope.joined(trait_item_method);
                // visitor.add_full_qualified_type_match(&fn_scope, self_ty, false);
                // let fn_scope = scope.joined(impl_method);
                // println!("ADDD IMPL METHOD: {} : {} : {}", ident.to_token_stream(), self_ty.to_token_stream(), fn_scope);
                visitor.add_full_qualified_type_match(&fn_scope, &trait_type, false);
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                if let ReturnType::Type(_, ty) = output {
                    visitor.add_full_qualified_type_chain(&fn_scope, visitor.create_type_chain(ty, scope), true);
                }
                inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
                    let mut type_chain = visitor.create_type_chain(ty, &fn_scope);
                    let parent_type_chain = visitor.create_type_chain(ty, scope).excluding_self_and_bounds(generics);
                    type_chain.insert(parse_quote!(Self), scope.self_scope().object.clone());
                    let mut type_chains = HashMap::from_iter([
                        (fn_scope.clone(), type_chain),
                        (scope.clone(), parent_type_chain.clone()),
                    ]);
                    if let Some(parent_scope) = scope.parent_scope() {
                        type_chains.insert(parent_scope.clone(), parent_type_chain);
                    }
                    visitor.add_full_qualified_type_chains(type_chains);

                });
                visitor.add_generic_chain(&fn_scope, generics, false);
            }
            TraitItem::Type(TraitItemType { ident, bounds, generics, .. }) => {
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                add_bounds(visitor, bounds, scope, true);
                visitor.add_generic_chain(scope, generics, false);
            },
            _ => {}
        });
    if let Some(parent_scope) = scope.parent_scope() {
        visitor.scope_add_one(parse_quote!(#ident), itself.clone(), parent_scope);
    }
    visitor.scope_add_one(parse_quote!(Self), itself, scope);
    visitor.add_generic_chain(&scope, &item_trait.generics, true);
}

fn add_full_qualified_signature(visitor: &mut Visitor, sig: &Signature, scope: &ScopeChain) {
    let Signature { output, inputs, generics, .. } = sig;
    if let ReturnType::Type(_, ty) = output {
        // TODO: Prevent generic bound from adding to parent here
        let type_chain = visitor.create_type_chain(ty, scope);
        visitor.add_full_qualified_type_chain(scope, type_chain, true);
    }
    inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
        // TODO: Prevent generic bound from adding to parent here
        // It's easy when arg is non-compound type, i.e. itself
        // It's hard when bound is a part of arg i.e. T: Into<U>
        // where "Into" SHOULD persist in the parent scope,
        // T: shouldn't if sig generics contain it
        // U: should if sig generics contain it
        let type_chain = visitor.create_type_chain(ty, scope);
        visitor.add_full_qualified_type_chain(scope, type_chain, true);
    });
    visitor.add_generic_chain(scope, generics, false);


    // let ty: Type = parse_quote!(#ident);
    // self.add_full_qualified_type_match(scope, &ty);
    // match scope.obj_root_chain() {
    //     Some(parent) => {
    //         let ty: TypeHolder = parse_quote!(#ident);
    //         // TODO: wrong here can be non-determined context
    //         let object = self.visit_scope_type(parent, &ty.0);
    //         self.scope_add_one(ty, object, parent);
    //
    //     },
    //     _ => {}
    // }
}

fn add_inner_module_conversion(visitor: &mut Visitor, item_mod: &ItemMod, scope: &ScopeChain) {
    if let Some((_, items)) = &item_mod.content {
        items.into_iter().for_each(|item| match item {
            Item::Use(node) =>
                visitor.fold_import_tree(scope, &node.tree, vec![]),
            Item::Mod(..) =>
                item.add_to_scope(&scope.joined(item), visitor),
            Item::Trait(..) |
            Item::Fn(..) |
            Item::Struct(..) |
            Item::Enum(..) |
            Item::Type(..) |
            Item::Impl(..) => if let Ok(..) = MacroKind::try_from(item) {
                item.add_to_scope(&scope.joined(item), visitor)
            },
            _ => {}
        })
    }
}

fn add_bounds(visitor: &mut Visitor, bounds: &AddPunctuated<TypeParamBound>, scope: &ScopeChain, add_to_parent: bool) -> Vec<Path> {
    let bounds = collect_bounds(bounds);
    bounds.iter().for_each(|path| {
        let ty =  Type::Path(TypePath { qself: None, path: path.clone() });
        visitor.add_full_qualified_type_match(scope, &ty, add_to_parent);
    });
    bounds
}

// pub fn create_generics_chain(visitor: &mut Visitor, generics: &Generics, scope: &ScopeChain, add_to_parent: bool) -> IndexMap<Type, Vec<Path>> {
//     let mut generics_chain: IndexMap<Type, Vec<Path>> = IndexMap::new();
//     let Generics { params, where_clause, .. } = generics;
//     params.iter().for_each(|generic_param| {
//         match generic_param { // T: Debug + Clone
//             GenericParam::Type(TypeParam { ident, bounds, .. }) => {
//                 generics_chain.insert(parse_quote!(#ident), add_bounds(visitor, bounds, scope, add_to_parent));
//             },
//             GenericParam::Const(ConstParam { ty, .. }) =>
//                 visitor.add_full_qualified_type_match(scope, ty, add_to_parent),
//             _ => {},
//         }
//     });
//     if let Some(WhereClause { predicates, .. }) = &where_clause {
//         predicates.iter().for_each(|predicate| match predicate {
//             WherePredicate::Type(PredicateType { bounds, bounded_ty, .. }) => {
//                 // where T: Debug + Clone, T::Item: XX,
//                 generics_chain.insert(parse_quote!(#bounded_ty), add_bounds(visitor, bounds, scope, add_to_parent));
//                 visitor.add_full_qualified_type_match(scope, bounded_ty, add_to_parent);
//             },
//             _ => {}
//         })
//     }
//     generics_chain
// }

fn collect_trait_bounds(visitor: &mut Visitor, bounds: &AddPunctuated<TypeParamBound>, scope: &ScopeChain, add_to_parent: bool) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();
    bounds.iter().for_each(|b| match b {
        TypeParamBound::Trait(TraitBound { modifier, path, .. }) => if let Some(path) = (matches!(modifier, TraitBoundModifier::None) && !path.segments.last().map(|PathSegment { ident, .. }| ident.eq("Sized")).unwrap_or_default()).then(|| path.clone()) {
            paths.push(path.clone());
            let ty =  Type::Path(TypePath { qself: None, path });
            visitor.add_full_qualified_type_match(scope, &ty, add_to_parent);
        },
        _ => {}
    });
    paths
}

/// Collects trait bounds from both type parameter bounds and where-clause predicates
/// into a single, deterministically ordered list. Only `TypeParamBound::Trait` and
/// `WherePredicate::Type(..)` with trait bounds are considered.
pub fn create_generics_chain(visitor: &mut Visitor, generics: &Generics, scope: &ScopeChain, add_to_parent: bool) -> IndexMap<Type, Vec<Path>> {
    let Generics { params, where_clause, .. } = generics;
    let mut generics_chain = IndexMap::<Type, Vec<Path>>::new();
    // 1) Bounds in angle brackets: `fn foo<T: Trait, U: A + B>() {}`
    params.iter().for_each(|gp| match gp {
        GenericParam::Type(TypeParam { bounds, ident, .. }) => {
            generics_chain.entry(parse_quote!(#ident))
                .or_default()
                .extend(collect_trait_bounds(visitor, bounds, scope, add_to_parent));
        },
        GenericParam::Const(ConstParam { ty, .. }) =>
            visitor.add_full_qualified_type_match(scope, ty, add_to_parent),
        _ => {}
    });
    // 2) Where clause predicates: `where T: Trait, Vec<U>: Another`
    if let Some(WhereClause { predicates, .. }) = where_clause {
        predicates.iter().for_each(|pred| match pred {
            WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) =>
                generics_chain.entry(bounded_ty.clone())
                    .or_default()
                    .extend(collect_trait_bounds(visitor, bounds, scope, add_to_parent)),
            _ => {}
        });
    }
    // Ensure each generic type parameter appears at least once; add unlimited if no restrictive bound collected
    params.iter().for_each(|gp| match gp {
        GenericParam::Type(TypeParam { ident, .. }) if !generics_chain.keys().any(|bounded_ty| bounded_ty.to_token_stream().to_string() == ident.to_string()) => {
            generics_chain.entry(parse_quote!(#ident))
                .or_default();
        }
        _ => {}
    });
    // Dedup per-type trait paths by token string and order deterministically
    for (_, trait_paths) in generics_chain.iter_mut() {
        let mut seen_p: HashSet<String> = HashSet::new();
        trait_paths.retain(|p| seen_p.insert(p.to_token_stream().to_string()));
        trait_paths.sort_by(|a, b| {
            let a_s = a.to_token_stream().to_string();
            let b_s = b.to_token_stream().to_string();
            let norm = |s: &str| s.replace(' ', "");
            let w = |s: &str| if norm(s).starts_with("::") { 1 } else { 0 };
            w(&a_s).cmp(&w(&b_s)).then_with(|| a_s.cmp(&b_s))
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
    generics_chain.retain(|bounded_ty, trait_paths| {
        if !trait_paths.is_empty() { return true; }
        let ty_s = bounded_ty.to_token_stream().to_string();
        !has_restrictive.get(&ty_s).copied().unwrap_or_default()
    });
    // Deterministic order: first by bounded type, then by trait path (both token strings)
    generics_chain.sort_by(|a_ty, a_paths, b_ty, b_paths| {
        // Prefer simple type parameters (single-segment, no leading ::) before concrete/qualified types
        let type_weight = |t: &Type| match t {
            // simple ident like `T`, `U`
            Type::Path(TypePath { qself: None, path: Path { leading_colon: None, segments } }) if segments.len() == 1 => 0,
            _ => 1
        };
        let a_ty_s = a_ty.to_token_stream().to_string();
        let a_tr_s = a_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + ");
        let b_ty_s = b_ty.to_token_stream().to_string();
        let b_tr_s = b_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + ");
        let a_tw = type_weight(a_ty);
        let b_tw = type_weight(b_ty);
        // Prefer bare trait names over fully-qualified ones; unlimited last
        let trait_weight = |s: &str| {
            if s == "<unlimited>" { 2 }
            else if s.replace(' ', "").starts_with("::") { 1 }
            else { 0 }
        };
        a_tw
            .cmp(&b_tw)
            .then_with(|| a_ty_s.cmp(&b_ty_s))
            .then_with(|| trait_weight(&a_tr_s).cmp(&trait_weight(&b_tr_s)))
            .then_with(|| a_tr_s.cmp(&b_tr_s))
    });
    generics_chain
}

fn add_itself_conversion(visitor: &mut Visitor, scope: &ScopeChain, ident: &Ident, object: ObjectKind) {
    visitor.scope_add_one(parse_quote!(#ident), object, scope);
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
