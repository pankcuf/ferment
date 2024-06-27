use std::collections::HashMap;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, ConstParam, Field, FnArg, GenericParam, Generics, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemFn, ItemMod, ItemTrait, Lifetime, LifetimeDef, Meta, NestedMeta, parse_quote, Path, PatType, PredicateType, ReturnType, Signature, TraitBound, TraitItem, TraitItemConst, TraitItemMethod, TraitItemType, Type, TypeParam, TypeParamBound, Variant, WhereClause, WherePredicate};
use syn::punctuated::Punctuated;
use crate::ast::{AddPunctuated, CommaPunctuated, TypePathHolder};
use crate::composable::{NestedArgument, TraitDecompositionPart1, TypeComposition};
use crate::context::{Scope, ScopeChain, ScopeInfo};
use crate::conversion::{MacroType, ObjectConversion, ScopeItemConversion, TypeCompositionConversion};
use crate::ext::{Join, ResolveMacro, ToType};
use crate::ext::item::collect_bounds;
use crate::tree::Visitor;

pub trait VisitScope {
    fn join_scope(&self, scope: &ScopeChain, visitor: &mut Visitor) -> Option<ScopeChain>;
    fn add_to_scope(&self, scope: &ScopeChain, visitor: &mut Visitor);
}

impl VisitScope for Item {
    fn join_scope(&self, scope: &ScopeChain, visitor: &mut Visitor) -> Option<ScopeChain> {
        //println!("join_scope: {}", scope.self_path_holder_ref());
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
        //println!("add_to_scope: {}", scope.self_path_holder_ref());
        let self_scope = scope.self_path_holder_ref();
        match self {
            Item::Const(_) => {}
            Item::Enum(item_enum) => {
                let self_object = ObjectConversion::new_item(TypeCompositionConversion::Object(TypeComposition::new(scope.to_type(), Some(item_enum.generics.clone()), Punctuated::new())), ScopeItemConversion::Item(Item::Enum(item_enum.clone()), self_scope.clone()));
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), &item_enum.ident, self_object.clone());
                add_itself_conversion(visitor, scope, &item_enum.ident, self_object);
                visitor.add_full_qualified_trait_type_from_macro(&item_enum.attrs, scope);
                visitor.add_generic_chain(scope, &item_enum.generics);
                item_enum.variants.iter().for_each(|Variant { fields, .. }|
                    fields.iter().for_each(|Field { ty, .. }|
                        visitor.add_full_qualified_type_match(scope, ty)));

            }
            Item::Struct(item_struct) => {
                let mut nested_arguments = CommaPunctuated::new();
                let full_ty = if !item_struct.generics.params.is_empty() || item_struct.generics.where_clause.is_some() {
                    println!("ADDD FQ STRUCT: {}: {} ---- {}", item_struct.ident, item_struct.generics.params.to_token_stream(), item_struct.generics.where_clause.to_token_stream());
                    let mut inner_args = CommaPunctuated::new();
                    item_struct.generics.params.iter().for_each(|p| match p {
                        GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                            inner_args.push(quote!(#ident));
                            let mut nested_bounds = CommaPunctuated::new();
                            bounds.iter().for_each(|pp| match pp {
                                TypeParamBound::Trait(TraitBound { path, .. }) => {
                                    nested_bounds.push(NestedArgument::Object(ObjectConversion::Type(TypeCompositionConversion::TraitType(TypeComposition::new(parse_quote!(#path), None, CommaPunctuated::new())))));
                                }
                                TypeParamBound::Lifetime(Lifetime { .. }) => {}
                            });
                            nested_arguments.push(NestedArgument::Constraint(ObjectConversion::Type(TypeCompositionConversion::TraitType(TypeComposition::new(parse_quote!(#ident), Some(item_struct.generics.clone()), nested_bounds)))));

                        }
                        GenericParam::Const(ConstParam { ident, ty: _, .. }) => {
                            inner_args.push(quote!(#ident));
                            nested_arguments.push(NestedArgument::Constraint(ObjectConversion::Type(TypeCompositionConversion::Object(TypeComposition::new(parse_quote!(#ident), Some(item_struct.generics.clone()), CommaPunctuated::new())))))
                        },
                        GenericParam::Lifetime(LifetimeDef { lifetime, bounds: _, .. }) => {
                            inner_args.push(quote!(#lifetime));
                        },
                    });
                    parse_quote!(#scope<#inner_args>)
                } else {
                    scope.to_type()
                };
                let self_object = ObjectConversion::new_item(
                    TypeCompositionConversion::Object(TypeComposition::new(full_ty, Some(item_struct.generics.clone()), nested_arguments)),
                    ScopeItemConversion::Item(Item::Struct(item_struct.clone()), self_scope.clone()));
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), &item_struct.ident, self_object.clone());
                add_itself_conversion(visitor, scope, &item_struct.ident, self_object);
                visitor.add_full_qualified_trait_type_from_macro(&item_struct.attrs, scope);
                visitor.add_generic_chain(scope, &item_struct.generics);
                item_struct.fields.iter().for_each(|Field { ty, .. }|
                    visitor.add_full_qualified_type_match(scope, ty));
            }
            Item::Fn(ItemFn { sig, .. }) => {
                let self_object = ObjectConversion::new_item(TypeCompositionConversion::Fn(TypeComposition::new(scope.to_type(), Some(sig.generics.clone()), Punctuated::new())), ScopeItemConversion::Fn(sig.clone(), self_scope.clone()));
                let sig_ident = &sig.ident;
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), sig_ident, self_object.clone());
                add_itself_conversion(visitor, scope, sig_ident, self_object);
                add_full_qualified_signature(visitor, sig, scope);
            }
            Item::Impl(item_impl) => {
                match &item_impl.trait_ {
                    Some((_, path, _)) => {
                        let ty = path.to_type();
                        visitor.add_full_qualified_type_match(scope, &ty);
                    },
                    None => {}
                }
                visitor.add_generic_chain(scope, &item_impl.generics);
                item_impl.items.iter().for_each(|impl_item| {
                    match impl_item {
                        ImplItem::Const(ImplItemConst { ident, ty, expr: _, .. }) => {
                            add_local_type(visitor, ident, scope);
                            visitor.add_full_qualified_type_match(scope, ty);
                        },
                        ImplItem::Method(ImplItemMethod { sig, .. }) => {
                            add_local_type(visitor, &sig.ident, scope);
                            add_full_qualified_signature(visitor, sig, scope);
                        },
                        ImplItem::Type(ImplItemType { ident, ty, generics, .. }) => {
                            add_local_type(visitor, ident, scope);
                            visitor.add_full_qualified_type_match(scope, ty);
                            visitor.add_generic_chain(scope, generics);
                        },
                        _ => {}
                    }
                });
            }
            Item::Mod(item_mod) => {
                add_inner_module_conversion(visitor, item_mod, scope);
            }
            Item::Trait(item_trait) => add_full_qualified_trait(visitor, item_trait, scope),
            Item::Type(item_type) => {
                let self_object = match &*item_type.ty {
                    Type::BareFn(..) =>
                        ObjectConversion::new_item(TypeCompositionConversion::FnPointer(TypeComposition::new(scope.to_type(), Some(item_type.generics.clone()), Punctuated::new())), ScopeItemConversion::Item(Item::Type(item_type.clone()), self_scope.clone())),
                    _ => ObjectConversion::new_item(TypeCompositionConversion::Object(TypeComposition::new(scope.to_type(), Some(item_type.generics.clone()), Punctuated::new())), ScopeItemConversion::Item(Item::Type(item_type.clone()), self_scope.clone()))
                };
                // println!("ADDD TYPE: {}", self_object);
                add_itself_conversion(visitor, scope.parent_scope().unwrap(), &item_type.ident, self_object.clone());
                add_itself_conversion(visitor, scope, &item_type.ident, self_object);
                visitor.add_generic_chain(scope, &item_type.generics);
                visitor.add_full_qualified_type_match(scope, &item_type.ty);
            }
            _ => {}
        }
    }
}
fn add_full_qualified_trait(visitor: &mut Visitor, item_trait: &ItemTrait, scope: &ScopeChain) {
    // println!("add_full_qualified_trait: {}: {}", item_trait.ident, scope);
    let ident = &item_trait.ident;
    let type_compo = TypeComposition::new(scope.to_type(), Some(item_trait.generics.clone()), Punctuated::new());
    let itself = ObjectConversion::new_item(
        TypeCompositionConversion::Trait(
            type_compo,
            TraitDecompositionPart1::from_trait_items(ident, &item_trait.items),
            add_bounds(visitor, &item_trait.supertraits, scope)),
        ScopeItemConversion::Item(Item::Trait(item_trait.clone()), scope.self_path_holder()));

    // 1. Add itself to the scope as <Self, Item(Trait(..))>
    // 2. Add itself to the parent scope as <Ident, Item(Trait(..))>
    // println!("::: 1. ADD Self (local scope): <{}, {}> in [{}]", quote!(Self), itself, scope);
    // println!("::: 2. ADD Self: (parent scope) <{}, {}> in [{}]", quote!(#ident), itself, scope.parent_scope().unwrap());
    visitor.add_full_qualified_trait_match(&scope, item_trait, &itself);
    item_trait.items.iter().for_each(|trait_item|
        match trait_item {
            TraitItem::Method(TraitItemMethod { attrs, sig, .. }) => {
                let sig_ident = &sig.ident;
                let self_scope = scope.self_scope();
                let fn_self_scope = self_scope.self_scope.joined(sig_ident);
                add_local_type(visitor, sig_ident, scope);
                let object = ObjectConversion::new_item(TypeCompositionConversion::Fn(TypeComposition::new(fn_self_scope.to_type(), Some(sig.generics.clone()), Punctuated::new())), ScopeItemConversion::Fn(sig.clone(), self_scope.self_scope.clone()));
                let fn_scope = ScopeChain::Fn {
                    info: ScopeInfo {
                        attrs: attrs.clone(),
                        crate_ident: scope.crate_ident().clone(),
                        self_scope: Scope::new(fn_self_scope, object),
                    },
                    parent_scope_chain: Box::new(scope.clone())
                };
                add_full_qualified_signature(visitor, sig, &fn_scope);
                visitor.add_generic_chain(&fn_scope, &sig.generics);
            },
            TraitItem::Type(TraitItemType { ident: type_ident, bounds, generics, .. }) => {
                add_local_type(visitor, type_ident, scope);
                add_bounds(visitor, bounds, scope);
                visitor.add_generic_chain(scope, generics);
            },
            TraitItem::Const(TraitItemConst { ident, ty, .. }) => {
                add_local_type(visitor, ident, scope);
                visitor.add_full_qualified_type_match(scope, ty);
            },
            _ => {}
        });
    visitor.scope_add_one(parse_quote!(#ident), itself.clone(), scope.parent_scope().unwrap());
    visitor.scope_add_one(parse_quote!(Self), itself, scope);
    visitor.add_generic_chain(&scope, &item_trait.generics);
}

fn add_full_qualified_signature(visitor: &mut Visitor, sig: &Signature, scope: &ScopeChain) {
    let Signature { output, inputs, generics, .. } = sig;
    if let ReturnType::Type(_arrow_token, ty) = output {
        visitor.add_full_qualified_type_match(scope, ty)
    }
    inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
        visitor.add_full_qualified_type_match(scope, ty);
    });
    visitor.add_generic_chain(scope, generics);


    // let ty: Type = parse_quote!(#ident);
    // self.add_full_qualified_type_match(scope, &ty);
    // match scope.obj_root_chain() {
    //     Some(parent) => {
    //         let ty: TypeHolder = parse_quote!(#ident);
    //         // TODO: wrong here can be non-determined context
    //         let object = self.update_nested_generics(parent, &ty.0);
    //         self.scope_add_one(ty, object, parent);
    //
    //     },
    //     _ => {}
    // }
}

fn add_inner_module_conversion(visitor: &mut Visitor, item_mod: &ItemMod, scope: &ScopeChain) {
    //println!("add_inner_module_conversion: {}" , item_mod.ident);
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
                        match MacroType::try_from(item) {
                            Ok(MacroType::Export | MacroType::Opaque | MacroType::Register(_)) => {
                                //println!("add_inner_module_conversion.item: {}", item.ident_string());
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

fn add_local_type(visitor: &mut Visitor, ident: &Ident, scope: &ScopeChain) {
    let local_type = parse_quote!(Self::#ident);
    visitor.add_full_qualified_type_match(scope, &local_type);
}

fn add_bounds(visitor: &mut Visitor, bounds: &AddPunctuated<TypeParamBound>, scope: &ScopeChain) -> Vec<Path> {
    let bounds = collect_bounds(bounds);
    bounds.iter().for_each(|path| {
        let ty = path.to_type();
        visitor.add_full_qualified_type_match(scope, &ty);
    });
    bounds
}

pub fn create_generics_chain(visitor: &mut Visitor, generics: &Generics, scope: &ScopeChain) -> HashMap<TypePathHolder, Vec<Path>> {
    let mut generics_chain: HashMap<TypePathHolder, Vec<Path>> = HashMap::new();
    generics.params.iter().for_each(|generic_param| {
        match generic_param { // T: Debug + Clone
            GenericParam::Type(TypeParam { ident: generic_ident, bounds, .. }) => {
                generics_chain.insert(parse_quote!(#generic_ident), add_bounds(visitor, bounds, scope));
            },
            GenericParam::Const(ConstParam { ty, .. }) => {
                visitor.add_full_qualified_type_match(scope, ty);
            },
            GenericParam::Lifetime(_lifetime) => {},
        }
    });
    match &generics.where_clause {
        Some(WhereClause { predicates, .. }) => {
            predicates.iter().for_each(|predicate| match predicate {
                WherePredicate::Type(PredicateType { bounds, bounded_ty, .. }) => {
                    // where T: Debug + Clone, T::Item: XX,
                    println!("WherePredicate: {}", bounded_ty.to_token_stream());
                    generics_chain.insert(parse_quote!(#bounded_ty), add_bounds(visitor, bounds, scope));
                    visitor.add_full_qualified_type_match(scope, bounded_ty);
                },
                WherePredicate::Lifetime(_) => {}
                WherePredicate::Eq(_) => {}
            })
        },
        None => {}
    }
    generics_chain
}

fn add_itself_conversion(visitor: &mut Visitor, scope: &ScopeChain, ident: &Ident, object: ObjectConversion) {
    visitor.scope_add_one(parse_quote!(#ident), object, scope);
}

pub fn extract_trait_names(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();
    attrs.iter().for_each(|attr| {
        if attr.is_labeled_for_export() {
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

pub fn add_trait_names(visitor: &mut Visitor, scope: &ScopeChain, item_trait_paths: &Vec<Path>) {
    item_trait_paths.iter().for_each(|trait_name|
        visitor.add_full_qualified_type_match(scope, &trait_name.to_type()));

}
