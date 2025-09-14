use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{parse_quote, Item, ItemTrait, TraitItem, Type};
use crate::context::{GlobalContext, ScopeChain};
use crate::lang::rust::Crate as RustCrate;
use crate::Config;
use crate::tree::Visitor;
use crate::ext::{Join, GenericBoundKey, VisitScope};
use crate::kind::{ObjectKind, TypeModelKind};

fn test_context() -> Rc<RefCell<GlobalContext>> {
    let krate = RustCrate::current_with_name("my_crate");
    let cfg = Config::new("fermented", krate, cbindgen::Config::default());
    Rc::new(RefCell::new(GlobalContext::with_config(cfg)))
}

fn module_scope() -> ScopeChain {
    let root = ScopeChain::crate_root_with_ident(parse_quote!(my_crate), vec![]);
    ScopeChain::child_mod_attr_less(parse_quote!(my_crate), &parse_quote!(module), &root)
}

#[test]
fn trait_method_generics_inherit_parent_generics() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // trait MyTrait<T: Clone> { fn m<U: Default>(&self, t: T, u: U) -> T; }
    let trait_item: ItemTrait = parse_quote! {
        trait MyTrait<T: Clone> {
            fn m<U: Default>(&self, t: T, u: U) -> T;
        }
    };

    // Process trait into scopes
    let trait_scope = Item::Trait(trait_item.clone()).join_scope(&mod_scope, &mut visitor).expect("joined trait scope");

    // Find method and compute its fn scope
    let method = match &trait_item.items[0] { TraitItem::Fn(f) => f.clone(), _ => panic!("expected method") };
    let fn_scope = trait_scope.joined(&method);
    let context = ctx.borrow();
    // Trait generic T bound captured at trait scope
    let trait_bounds = context.generics.maybe_generic_bounds(&trait_scope, &parse_quote!(T)).expect("trait T bounds");
    let rendered = trait_bounds.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>();
    assert!(rendered.contains(&"Clone".to_string()));

    // Method generic U bound captured at method scope
    let method_bounds = context.generics.maybe_generic_bounds(&fn_scope, &parse_quote!(U)).expect("method U bounds");
    let rendered_u = method_bounds.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>();
    assert!(rendered_u.contains(&"Default".to_string()));

    // From method scope, asking for T via scope chain fallback yields the trait's bounds
    let (_gens, chain_for_t) = fn_scope.maybe_generic_bound_for_path(&GenericBoundKey::ident(&parse_quote!(T))).expect("T visible in fn scope via parent");
    let ty: Type = parse_quote!(T);
    assert!(chain_for_t.inner.get(&ty).is_some());
}

#[test]
fn trait_method_type_chain_binds_trait_generic_in_fn_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    let trait_item: ItemTrait = parse_quote! {
        trait MyTrait<T: Clone> {
            fn m(&self, t: T) -> T;
        }
    };

    // Process trait and method
    let trait_scope = Item::Trait(trait_item.clone()).join_scope(&mod_scope, &mut visitor).unwrap();
    let method = match &trait_item.items[0] { TraitItem::Fn(f) => f.clone(), _ => unreachable!() };
    let fn_scope = trait_scope.joined(&method);

    // The scope register for fn scope should include a mapping for `T` resolved as bounds
    let context = ctx.borrow();
    let reg = &context.scope_register;
    let chain = reg.get(&fn_scope).expect("fn scope chain present");
    let ty: Type = parse_quote!(T);
    let obj = chain.get(&ty).expect("T in fn scope chain");
    assert!(matches!(obj, ObjectKind::Type(TypeModelKind::Bounds(..))));
}

#[test]
fn trait_associated_type_and_where_bounds_are_recorded() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    let trait_item: Item = parse_quote! {
        trait AssocTrait<T>
        where T: Clone {
            type Item: Iterator;
            fn next_of<U: Copy>(&self, _x: U) -> Option<Self::Item>;
        }
    };

    let trait_scope = trait_item.join_scope(&mod_scope, &mut visitor).unwrap();
    let context = ctx.borrow();
    // T bound recorded at trait scope via where-clause
    let t_bounds = context.generics.maybe_generic_bounds(&trait_scope, &parse_quote!(T)).expect("T bounds present");
    let rendered = t_bounds.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>();
    assert!(rendered.iter().any(|s| s.contains("Clone")));

    // Associated type recorded in trait scope register under `Self::Item`
    let ty_self_item: Type = parse_quote!(Self::Item);
    let chain = context.scope_register.get(&trait_scope).expect("trait scope chain present");
    assert!(chain.get(&ty_self_item).is_some(), "assoc type bounds");
}

#[test]
fn trait_method_arg_nested_assoc_in_fn_and_trait_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // Supporting trait with associated type
    let inner_trait: ItemTrait = parse_quote! { trait Inner { type Assoc; } };
    let _ = Item::Trait(inner_trait.clone()).join_scope(&mod_scope, &mut visitor);

    // Trait with associated type bounded by Inner and method taking <Self::Item as Inner>::Assoc
    let trait_item: ItemTrait = parse_quote! {
        trait AssocTrait { type Item: Inner; fn m(&self, a: <Self::Item as Inner>::Assoc); }
    };
    let trait_scope = Item::Trait(trait_item.clone()).join_scope(&mod_scope, &mut visitor).unwrap();
    let method = match &trait_item.items[1] { TraitItem::Fn(f) => f.clone(), _ => unreachable!() };
    let fn_scope = trait_scope.joined(&method);

    let context = ctx.borrow();
    let reg = &context.scope_register;
    let ty_assoc: Type = parse_quote!(<Self::Item as Inner>::Assoc);
    // Present in fn scope and trait scope
    assert!(reg.get(&fn_scope).unwrap().get(&ty_assoc).is_some());
    assert!(reg.get(&trait_scope).unwrap().get(&ty_assoc).is_some());
    // Not propagated to parent of trait
    if let Some(parent) = trait_scope.parent_scope() {
        assert!(reg.get(parent).and_then(|c| c.get(&ty_assoc)).is_none());
    }
}

#[test]
fn trait_method_generic_bound_on_nested_assoc_in_fn_and_trait_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // Define Inner with associated type
    let inner_trait: ItemTrait = parse_quote! { trait Inner { type Assoc; } };
    let _ = Item::Trait(inner_trait.clone()).join_scope(&mod_scope, &mut visitor);

    // Method generic V: Into<<Self::Item as Inner>::Assoc>
    let trait_item: ItemTrait = parse_quote! {
        trait AssocTrait { type Item: Inner; fn m<V: Into<<Self::Item as Inner>::Assoc>>(&self, _v: V); }
    };
    let trait_scope = Item::Trait(trait_item.clone()).join_scope(&mod_scope, &mut visitor).unwrap();
    let method = match &trait_item.items[1] { TraitItem::Fn(f) => f.clone(), _ => unreachable!() };
    let fn_scope = trait_scope.joined(&method);

    let context = ctx.borrow();
    let reg = &context.scope_register;
    let into_ty: Type = parse_quote!(Into<<Self::Item as Inner>::Assoc>);
    let assoc_ty: Type = parse_quote!(<Self::Item as Inner>::Assoc);

    // Present in fn scopebuilder
    let fn_chain = reg.get(&fn_scope).expect("fn scope chain present");
    assert!(fn_chain.get(&into_ty).is_some());
    assert!(fn_chain.get(&assoc_ty).is_some());

    // Included in trait scope, but not propagated to parent
    let tr_chain = reg.get(&trait_scope).expect("trait scope chain present");
    assert!(tr_chain.get(&into_ty).is_some());
    assert!(tr_chain.get(&assoc_ty).is_some());
    if let Some(parent) = trait_scope.parent_scope() {
        assert!(reg.get(parent).and_then(|c| c.get(&into_ty)).is_none());
        assert!(reg.get(parent).and_then(|c| c.get(&assoc_ty)).is_none());
    }
}

#[test]
fn impl_method_arg_nested_assoc_in_fn_and_impl_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // trait Inner { type Assoc; }
    let inner_trait: ItemTrait = parse_quote! { trait Inner { type Assoc; } };
    let _ = Item::Trait(inner_trait.clone()).join_scope(&mod_scope, &mut visitor);
    // struct S; impl Inner for S { type Assoc = u32; }
    let s_decl: Item = parse_quote!(struct S;);
    let _ = s_decl.join_scope(&mod_scope, &mut visitor);
    let impl_inner: Item = parse_quote!(impl Inner for S { type Assoc = u32; });
    let _ = impl_inner.join_scope(&mod_scope, &mut visitor);

    // impl S { fn m(&self, a: <Self as Inner>::Assoc) {} }
    let impl_inherent: Item = parse_quote!(impl S { fn m(&self, a: <Self as Inner>::Assoc) {} });
    let impl_scope = impl_inherent.join_scope(&mod_scope, &mut visitor).unwrap();

    // Find method scope
    let method_scope = if let Item::Impl(impl_block) = &impl_inherent {
        let method = &impl_block.items.iter().find_map(|it| if let syn::ImplItem::Fn(f) = it { Some(f.clone()) } else { None }).unwrap();
        impl_scope.joined(method)
    } else { unreachable!() };

    let context = ctx.borrow();
    let reg = &context.scope_register;
    let assoc_ty: Type = parse_quote!(<Self as Inner>::Assoc);
    // Present in fn scope and impl scope
    assert!(reg.get(&method_scope).unwrap().get(&assoc_ty).is_some());
    assert!(reg.get(&impl_scope).unwrap().get(&assoc_ty).is_some());
    // Not propagated to parent of impl
    if let Some(parent) = impl_scope.parent_scope() {
        assert!(reg.get(parent).and_then(|c| c.get(&assoc_ty)).is_none());
    }
}

#[test]
fn impl_method_generic_bound_on_nested_assoc_in_fn_and_impl_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // trait Inner { type Assoc; }
    let inner_trait: ItemTrait = parse_quote! { trait Inner { type Assoc; } };
    let _ = Item::Trait(inner_trait.clone()).join_scope(&mod_scope, &mut visitor);
    // struct S; impl Inner for S { type Assoc = u32; }
    let s_decl: Item = parse_quote!(struct S;);
    let _ = s_decl.join_scope(&mod_scope, &mut visitor);
    let impl_inner: Item = parse_quote!(impl Inner for S { type Assoc = u32; });
    let _ = impl_inner.join_scope(&mod_scope, &mut visitor);

    // impl S { fn m<V: Into<<Self as Inner>::Assoc>>(&self, _v: V) {} }
    let impl_inherent: Item = parse_quote!(impl S { fn m<V: Into<<Self as Inner>::Assoc>>(&self, _v: V) {} });
    let impl_scope = impl_inherent.join_scope(&mod_scope, &mut visitor).unwrap();

    // Find method scope
    let method_scope = if let Item::Impl(impl_block) = &impl_inherent {
        let method = &impl_block.items.iter().find_map(|it| if let syn::ImplItem::Fn(f) = it { Some(f.clone()) } else { None }).unwrap();
        impl_scope.joined(method)
    } else { unreachable!() };

    let context = ctx.borrow();
    let reg = &context.scope_register;
    let into_ty: Type = parse_quote!(Into<<Self as Inner>::Assoc>);
    let assoc_ty: Type = parse_quote!(<Self as Inner>::Assoc);

    // Present in fn scope
    let fn_chain = reg.get(&method_scope).expect("fn scope chain present");
    assert!(fn_chain.get(&into_ty).is_some());
    assert!(fn_chain.get(&assoc_ty).is_some());

    // Included in impl scope, but not propagated to parent
    let impl_chain = reg.get(&impl_scope).expect("impl scope chain present");
    assert!(impl_chain.get(&into_ty).is_some());
    assert!(impl_chain.get(&assoc_ty).is_some());
    if let Some(parent) = impl_scope.parent_scope() {
        assert!(reg.get(parent).and_then(|c| c.get(&into_ty)).is_none());
        assert!(reg.get(parent).and_then(|c| c.get(&assoc_ty)).is_none());
    }
}

#[test]
fn trait_non_self_arg_propagates_to_parent() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // trait with non-Self arg type Vec<u8>
    let trait_item: ItemTrait = parse_quote! {
        trait WithVecArg { fn m(&self, v: Vec<u8>); }
    };
    let trait_scope = Item::Trait(trait_item.clone()).join_scope(&mod_scope, &mut visitor).unwrap();

    let context = ctx.borrow();
    let reg = &context.scope_register;
    // Parent of trait (module scope) should have Vec<u8> recorded
    if let Some(parent) = trait_scope.parent_scope() {
        let parent_chain = reg.get(parent).expect("parent scope chain present");
        let vec_ty: Type = parse_quote!(Vec<u8>);
        assert!(parent_chain.get(&vec_ty).is_some());
    } else {
        panic!("trait has no parent scope");
    }
}

#[test]
fn trait_return_nested_assoc_in_fn_and_trait_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // trait Inner { type Assoc; }
    let inner_trait: ItemTrait = parse_quote! { trait Inner { type Assoc; } };
    let _ = Item::Trait(inner_trait.clone()).join_scope(&mod_scope, &mut visitor);

    // trait AssocTrait { type Item: Inner; fn r(&self) -> <Self::Item as Inner>::Assoc; }
    let trait_item: ItemTrait = parse_quote! { trait AssocTrait { type Item: Inner; fn r(&self) -> <Self::Item as Inner>::Assoc; } };
    let trait_scope = Item::Trait(trait_item.clone()).join_scope(&mod_scope, &mut visitor).unwrap();
    let method = match &trait_item.items[1] { TraitItem::Fn(f) => f.clone(), _ => unreachable!() };
    let fn_scope = trait_scope.joined(&method);

    let ty_assoc: Type = parse_quote!(<Self::Item as Inner>::Assoc);
    let context = ctx.borrow();
    let reg = &context.scope_register;
    // Present in fn scope and trait scope; not in parent
    assert!(reg.get(&fn_scope).unwrap().get(&ty_assoc).is_some());
    assert!(reg.get(&trait_scope).unwrap().get(&ty_assoc).is_some());
    if let Some(parent) = trait_scope.parent_scope() {
        assert!(reg.get(parent).and_then(|c| c.get(&ty_assoc)).is_none());
    }
}

#[test]
fn impl_return_nested_assoc_in_fn_and_impl_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // trait Inner { type Assoc; }
    let inner_trait: ItemTrait = parse_quote! { trait Inner { type Assoc; } };
    let _ = Item::Trait(inner_trait.clone()).join_scope(&mod_scope, &mut visitor);
    // struct S; impl Inner for S { type Assoc = u32; }
    let s_decl: Item = parse_quote!(struct S;);
    let _ = s_decl.join_scope(&mod_scope, &mut visitor);
    let impl_inner: Item = parse_quote!(impl Inner for S { type Assoc = u32; });
    let _ = impl_inner.join_scope(&mod_scope, &mut visitor);

    // impl S { fn r(&self) -> <Self as Inner>::Assoc { unimplemented!() } }
    let impl_inherent: Item = parse_quote!(impl S { fn r(&self) -> <Self as Inner>::Assoc { unimplemented!() } });
    let impl_scope = impl_inherent.join_scope(&mod_scope, &mut visitor).unwrap();
    let method_scope = if let Item::Impl(impl_block) = &impl_inherent {
        let method = &impl_block.items.iter().find_map(|it| if let syn::ImplItem::Fn(f) = it { Some(f.clone()) } else { None }).unwrap();
        impl_scope.joined(method)
    } else { unreachable!() };

    let assoc_ty: Type = parse_quote!(<Self as Inner>::Assoc);
    let context = ctx.borrow();
    let reg = &context.scope_register;
    // Present in fn scope and impl scope; not in parent
    assert!(reg.get(&method_scope).unwrap().get(&assoc_ty).is_some());
    assert!(reg.get(&impl_scope).unwrap().get(&assoc_ty).is_some());
    if let Some(parent) = impl_scope.parent_scope() {
        assert!(reg.get(parent).and_then(|c| c.get(&assoc_ty)).is_none());
    }
}

#[test]
fn trait_return_non_self_propagates_to_parent() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    let trait_item: ItemTrait = parse_quote! { trait R { fn r(&self) -> Vec<u8>; } };
    let trait_scope = Item::Trait(trait_item.clone()).join_scope(&mod_scope, &mut visitor).unwrap();
    if let Some(parent) = trait_scope.parent_scope() {
        let context = ctx.borrow();
        let parent_chain = context.scope_register.get(parent).expect("parent chain");
        let vec_ty: Type = parse_quote!(Vec<u8>);
        assert!(parent_chain.get(&vec_ty).is_some());
    }
}

#[test]
fn trait_impl_return_nested_assoc_in_fn_and_impl_scope() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    // trait Inner { type Assoc; }
    let inner_trait: ItemTrait = parse_quote! { trait Inner { type Assoc; } };
    let _ = Item::Trait(inner_trait.clone()).join_scope(&mod_scope, &mut visitor);

    // trait R: Inner { fn r(&self) -> <Self as Inner>::Assoc; }
    let trait_r: ItemTrait = parse_quote! { trait R: Inner { fn r(&self) -> <Self as Inner>::Assoc; } };
    let _ = Item::Trait(trait_r.clone()).join_scope(&mod_scope, &mut visitor);

    // struct S; impl Inner for S { type Assoc = u32; } impl R for S { fn r(&self) -> <Self as Inner>::Assoc { unimplemented!() } }
    let s_decl: Item = parse_quote!(struct S;);
    let _ = s_decl.join_scope(&mod_scope, &mut visitor);
    let impl_inner: Item = parse_quote!(impl Inner for S { type Assoc = u32; });
    let _ = impl_inner.join_scope(&mod_scope, &mut visitor);

    let impl_r: Item = parse_quote!(impl R for S { fn r(&self) -> <Self as Inner>::Assoc { unimplemented!() } });
    let impl_scope = impl_r.join_scope(&mod_scope, &mut visitor).unwrap();

    // Find method scope
    let method_scope = if let Item::Impl(impl_block) = &impl_r {
        let method = &impl_block.items.iter().find_map(|it| if let syn::ImplItem::Fn(f) = it { Some(f.clone()) } else { None }).unwrap();
        impl_scope.joined(method)
    } else { unreachable!() };

    let assoc_ty: Type = parse_quote!(<Self as Inner>::Assoc);
    let context = ctx.borrow();
    let reg = &context.scope_register;
    // Present in fn scope and impl scope; not in parent
    assert!(reg.get(&method_scope).unwrap().get(&assoc_ty).is_some());
    assert!(reg.get(&impl_scope).unwrap().get(&assoc_ty).is_some());
    if let Some(parent) = impl_scope.parent_scope() {
        assert!(reg.get(parent).and_then(|c| c.get(&assoc_ty)).is_none());
    }
}

#[test]
fn trait_impl_return_non_self_propagates_to_parent() {
    let ctx = test_context();
    let mod_scope = module_scope();
    let mut visitor = Visitor::new(&mod_scope, &[], &ctx);

    let trait_r: ItemTrait = parse_quote! { trait R { fn r(&self) -> Vec<u8>; } };
    let _ = Item::Trait(trait_r.clone()).join_scope(&mod_scope, &mut visitor);

    let s_decl: Item = parse_quote!(struct S;);
    let _ = s_decl.join_scope(&mod_scope, &mut visitor);
    let impl_r: Item = parse_quote!(impl R for S { fn r(&self) -> Vec<u8> { unimplemented!() } });
    let impl_scope = impl_r.join_scope(&mod_scope, &mut visitor).unwrap();

    // Vec<u8> should propagate to parent of impl scope
    if let Some(parent) = impl_scope.parent_scope() {
        let context = ctx.borrow();
        let parent_chain = context.scope_register.get(parent).expect("parent chain");
        let vec_ty: Type = parse_quote!(Vec<u8>);
        assert!(parent_chain.get(&vec_ty).is_some());
    }
}
