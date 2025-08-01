use std::collections::HashMap;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, ConstParam, Field, FnArg, GenericParam, Generics, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, Item, ItemFn, ItemImpl, ItemMod, ItemTrait, LifetimeParam, Meta, parse_quote, Path, PatType, PredicateType, ReturnType, Signature, TraitBound, TraitItem, TraitItemConst, TraitItemFn, TraitItemType, Type, TypeParam, TypeParamBound, Variant, WhereClause, WherePredicate, TypePath};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use crate::ast::{AddPunctuated, CommaPunctuated, TypePathHolder};
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
        let self_scope = scope.self_path_holder_ref();
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
                    //println!("ADDD FQ STRUCT: {}: {} ---- {}", item_struct.ident, item_struct.generics.params.to_token_stream(), item_struct.generics.where_clause.to_token_stream());
                    let mut inner_args = CommaPunctuated::new();
                    item_enum.generics.params.iter().for_each(|p| match p {
                        GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                            inner_args.push(quote!(#ident));
                            let mut nested_bounds = CommaPunctuated::new();
                            bounds.iter().for_each(|pp| match pp {
                                TypeParamBound::Trait(TraitBound { path, .. }) => {
                                    // TODO: make it Unknown
                                    nested_bounds.push(NestedArgument::Object(ObjectKind::Type(TypeModelKind::TraitType(TypeModel::new(path.to_type(), None, CommaPunctuated::new())))));
                                }
                                _ => {}
                            });
                            // TODO: make it Unknown
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::Type(TypeModelKind::TraitType(TypeModel::new(ident.to_type(), Some(item_enum.generics.clone()), nested_bounds)))));

                        }
                        GenericParam::Const(ConstParam { ident, ty: _, .. }) => {
                            inner_args.push(quote!(#ident));
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::Type(TypeModelKind::Object(TypeModel::new(ident.to_type(), Some(item_enum.generics.clone()), CommaPunctuated::new())))))
                        },
                        GenericParam::Lifetime(LifetimeParam { lifetime, bounds: _, .. }) => {
                            inner_args.push(quote!(#lifetime));
                        },
                    });
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };

                let self_object = ObjectKind::new_item(TypeModelKind::Object(TypeModel::new(full_ty, Some(item_enum.generics.clone()), nested_arguments)), ScopeItemKind::Item(Item::Enum(item_enum.clone()), self_scope.clone()));
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), &item_enum.ident, self_object.clone());
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
                                TypeParamBound::Trait(TraitBound { path, .. }) => {
                                    // TODO: make it Unknown
                                    nested_bounds.push(NestedArgument::Object(ObjectKind::Type(TypeModelKind::TraitType(TypeModel::new(path.to_type(), None, CommaPunctuated::new())))));
                                }
                                _ => {}
                            });
                            // TODO: make it Unknown
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::Type(TypeModelKind::TraitType(TypeModel::new(ident.to_type(), Some(item_struct.generics.clone()), nested_bounds)))));

                        }
                        GenericParam::Const(ConstParam { ident, ty: _, .. }) => {
                            inner_args.push(quote!(#ident));
                            nested_arguments.push(NestedArgument::Constraint(ObjectKind::Type(TypeModelKind::Object(TypeModel::new(ident.to_type(), Some(item_struct.generics.clone()), CommaPunctuated::new())))))
                        },
                        GenericParam::Lifetime(LifetimeParam { lifetime, bounds: _, .. }) => {
                            inner_args.push(quote!(#lifetime));
                        },
                    });
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };
                let self_object = ObjectKind::new_item(
                    TypeModelKind::Object(TypeModel::new(full_ty, Some(item_struct.generics.clone()), nested_arguments)),
                    ScopeItemKind::Item(Item::Struct(item_struct.clone()), self_scope.clone()));
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), &item_struct.ident, self_object.clone());
                add_itself_conversion(visitor, scope, &item_struct.ident, self_object);
                visitor.add_full_qualified_trait_type_from_macro(&item_struct.attrs, scope);
                visitor.add_generic_chain(scope, &item_struct.generics, true);
                item_struct.fields.iter().for_each(|Field { ty, .. }|
                    visitor.add_full_qualified_type_match(scope, ty,true));
            }
            Item::Fn(ItemFn { sig, .. }) => {
                let self_object = ObjectKind::new_item(TypeModelKind::Fn(TypeModel::new(scope.to_type(), Some(sig.generics.clone()), Punctuated::new())), ScopeItemKind::Fn(sig.clone(), self_scope.clone()));
                let sig_ident = &sig.ident;
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), sig_ident, self_object.clone());
                add_itself_conversion(visitor, scope, sig_ident, self_object);
                add_full_qualified_signature(visitor, sig, scope);
            }
            Item::Trait(item_trait) => add_full_qualified_trait(visitor, item_trait, scope),
            Item::Type(item_type) => {
                let self_object = match &*item_type.ty {
                    Type::BareFn(..) =>
                        ObjectKind::new_item(
                            TypeModelKind::FnPointer(TypeModel::new_non_gen(scope.to_type(), Some(item_type.generics.clone()))), ScopeItemKind::Item(Item::Type(item_type.clone()), self_scope.clone())),
                    _ => {
                        ObjectKind::new_item(TypeModelKind::Object(TypeModel::new_non_gen(scope.to_type(), Some(item_type.generics.clone()))), ScopeItemKind::Item(Item::Type(item_type.clone()), self_scope.clone()))
                    }
                };
                // println!("ADDD TYPE: {}", self_object);
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), &item_type.ident, self_object.clone());
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
                        ImplItem::Const(ImplItemConst { ident, ty, expr: _, .. }) => {
                            visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                            visitor.add_full_qualified_type_match(scope, ty, true);
                        },
                        ImplItem::Fn(impl_method) => {
                            let ImplItemFn { sig, .. } = impl_method;
                            let Signature { ident, inputs, output, generics, .. } = sig;
                            let fn_scope = scope.joined(impl_method);
                            visitor.add_full_qualified_type_match(&fn_scope, self_ty, false);
                            visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                            if let ReturnType::Type(_arrow_token, ty) = output {
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
                                        visitor.add_full_qualified_type_chains(HashMap::from_iter([
                                            (fn_scope.clone(), type_chain),
                                            (scope.clone(), parent_type_chain.clone()),
                                            (scope.parent_scope().unwrap().clone(), parent_type_chain.clone()),
                                        ]))
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
    let type_compo = TypeModel::new(scope.to_type(), Some(item_trait.generics.clone()), Punctuated::new());
    let itself = ObjectKind::new_item(
        TypeModelKind::Trait(TraitModel::new(type_compo, TraitDecompositionPart1::from_trait_items(ident, &item_trait.items), add_bounds(visitor, &item_trait.supertraits, scope, true))),
        ScopeItemKind::Item(Item::Trait(item_trait.clone()), scope.self_path_holder()));

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
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#ident), true);
                if let ReturnType::Type(_arrow_token, ty) = output {
                    visitor.add_full_qualified_type_chain(&fn_scope, visitor.create_type_chain(ty, scope), true);
                }
                inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
                    let mut type_chain = visitor.create_type_chain(ty, &fn_scope);
                    let parent_type_chain = visitor.create_type_chain(ty, scope).excluding_self_and_bounds(generics);
                    type_chain.insert(parse_quote!(Self), scope.self_scope().object.clone());
                    visitor.add_full_qualified_type_chains(HashMap::from_iter([
                        (fn_scope.clone(), type_chain),
                        (scope.clone(), parent_type_chain.clone()),
                        (scope.parent_scope().unwrap().clone(), parent_type_chain.clone()),
                    ]))
                });
                visitor.add_generic_chain(&fn_scope, generics, false);
            }
            TraitItem::Type(TraitItemType { ident: type_ident, bounds, generics, .. }) => {
                visitor.add_full_qualified_type_match(scope, &parse_quote!(Self::#type_ident), true);
                add_bounds(visitor, bounds, scope, true);
                visitor.add_generic_chain(scope, generics, false);
            },
            _ => {}
        });
    visitor.scope_add_one(parse_quote!(#ident), itself.clone(), scope.parent_scope().unwrap());
    visitor.scope_add_one(parse_quote!(Self), itself, scope);
    visitor.add_generic_chain(&scope, &item_trait.generics, true);
}

fn add_full_qualified_signature(visitor: &mut Visitor, sig: &Signature, scope: &ScopeChain) {
    let Signature { output, inputs, generics, .. } = sig;
    if let ReturnType::Type(_arrow_token, ty) = output {
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
    match &item_mod.content {
        None => {},
        Some((_, items)) => {
            items.into_iter().for_each(|item| {
                match item {
                    Item::Use(node) =>
                        visitor.fold_import_tree(scope, &node.tree, vec![]),
                    Item::Mod(..) =>
                        item.add_to_scope(&scope.joined(item), visitor),
                    Item::Trait(..) |
                    Item::Fn(..) |
                    Item::Struct(..) |
                    Item::Enum(..) |
                    Item::Type(..) |
                    Item::Impl(..) => {
                        match MacroKind::try_from(item) {
                            Ok(MacroKind::Export | MacroKind::Opaque | MacroKind::Register(_)) => {
                                item.add_to_scope(&scope.joined(item), visitor)
                            }
                            Err(_) => {}
                        }
                    },
                    _ => {}
                }
            })
        }
    }
}

fn add_bounds(visitor: &mut Visitor, bounds: &AddPunctuated<TypeParamBound>, scope: &ScopeChain, add_to_parent: bool) -> Vec<Path> {
    let bounds = collect_bounds(bounds);
    bounds.iter().for_each(|path| {
        // let type_path: TypePath = parse_quote!(#path);
        let ty =  Type::Path(TypePath { qself: None, path: path.clone() });
        // TypePath::parse(path.to_token_stream());
        println!("add_bounds: {}: {:?}", path.to_token_stream(), path);
        // let type_path: TypePath = parse_quote!(#path);
        // let ty = path.to_type();
        visitor.add_full_qualified_type_match(scope, &ty, add_to_parent);
    });
    bounds
}

pub fn create_generics_chain(visitor: &mut Visitor, generics: &Generics, scope: &ScopeChain, add_to_parent: bool) -> HashMap<TypePathHolder, Vec<Path>> {
    let mut generics_chain: HashMap<TypePathHolder, Vec<Path>> = HashMap::new();
    generics.params.iter().for_each(|generic_param| {
        match generic_param { // T: Debug + Clone
            GenericParam::Type(TypeParam { ident: generic_ident, bounds, .. }) => {
                println!("create_generics_chain: param: {} /// {}", generic_ident.to_token_stream(), bounds.to_token_stream());
                generics_chain.insert(parse_quote!(#generic_ident), add_bounds(visitor, bounds, scope, add_to_parent));
            },
            GenericParam::Const(ConstParam { ty, .. }) => {
                visitor.add_full_qualified_type_match(scope, ty, add_to_parent);
            },
            _ => {},
        }
    });
    match &generics.where_clause {
        Some(WhereClause { predicates, .. }) => {
            predicates.iter().for_each(|predicate| match predicate {
                WherePredicate::Type(PredicateType { bounds, bounded_ty, .. }) => {
                    // where T: Debug + Clone, T::Item: XX,
                    println!("create_generics_chain: where: {} /// {}", bounded_ty.to_token_stream(), bounds.to_token_stream());
                    generics_chain.insert(parse_quote!(#bounded_ty), add_bounds(visitor, bounds, scope, add_to_parent));
                    visitor.add_full_qualified_type_match(scope, bounded_ty, add_to_parent);
                },
                _ => {}
            })
        },
        None => {}
    }
    generics_chain
}

fn add_itself_conversion(visitor: &mut Visitor, scope: &ScopeChain, ident: &Ident, object: ObjectKind) {
    visitor.scope_add_one(parse_quote!(#ident), object, scope);
}

pub fn extract_trait_names(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();
    attrs.iter().for_each(|attr| {
        if attr.is_labeled_for_export() {

            // attr.parse_nested_meta()
            // if let Ok(Meta::List(ref meta_list)) = attr.meta {
            //     meta_list.
            //     paths.push(path.clone());
            // }
            // if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            //     meta_list.nested.iter().for_each(|meta| {
            //         if let NestedMeta::Meta(Meta::Path(path)) = meta {
            //             paths.push(path.clone());
            //         }
            //     });
            // }
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

pub fn add_trait_names(visitor: &mut Visitor, scope: &ScopeChain, item_trait_paths: &Vec<Path>, add_to_parent: bool) {
    item_trait_paths.iter().for_each(|trait_name|
        visitor.add_full_qualified_type_match(scope, &trait_name.to_type(), add_to_parent));

}
