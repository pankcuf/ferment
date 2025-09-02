use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Ident, ItemTrait, Path, Signature, TraitBound, TraitItem, TraitItemFn, TraitItemType, Type, TypeParamBound};
use syn::__private::TokenStream2;
use crate::ast::Depunctuated;
use crate::composable::{CfgAttributes, FnSignatureContext};
use crate::composer::{SigComposer, SigComposerLink};
use crate::context::ScopeContextLink;
use crate::kind::TypeModelKind;
use crate::ext::{Join, ToType};
use crate::formatter::{format_token_stream, format_trait_decomposition_part1};
use crate::lang::Specification;
use crate::presentable::NameTreeContext;

#[derive(Clone, Debug)]
pub struct TraitBoundDecomposition {
    pub path: Path,
}

impl ToTokens for TraitBoundDecomposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.path.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
pub struct TraitTypeModel {
    pub ident: Ident,
    pub trait_bounds: Vec<TraitBoundDecomposition>,
}

impl ToTokens for TraitTypeModel {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TraitTypeModel { ident, trait_bounds } = self;
        quote!(#ident: #(#trait_bounds),*).to_tokens(tokens)
    }
}

impl TraitTypeModel {
    pub fn from_item_type(item_type: &TraitItemType) -> Self {
        Self {
            ident: item_type.ident.clone(),
            trait_bounds: item_type.bounds.iter()
                .filter_map(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        Some(TraitBoundDecomposition { path: path.clone() }),
                    _ =>
                        None,
                })
                .collect(),
        }
    }
}
// For use in Scope Agnostic Tree
#[derive(Clone, Debug)]
pub struct TraitDecompositionPart1 {
    pub ident: Ident,
    pub consts: HashMap<Ident, Type>,
    pub methods: HashMap<Ident, Signature>,
    pub types: HashMap<Ident, TraitTypeModel>
}

impl Display for TraitDecompositionPart1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("TraitDecompositionPart1({})", format_trait_decomposition_part1(self)).as_str())
    }
}


impl TraitDecompositionPart1 {
    pub fn from_trait_items(ident: &Ident, trait_items: &[TraitItem]) -> Self {
        let mut methods = HashMap::new();
        let mut types = HashMap::new();
        let mut consts = HashMap::new();
        trait_items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Fn(TraitItemFn { sig, .. }) => {
                    methods.insert(sig.ident.clone(), sig.clone());
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeModel::from_item_type(trait_item_type));
                },
                TraitItem::Const(trait_item_const) => {
                    consts.insert(trait_item_const.ident.clone(), trait_item_const.ty.clone());
                },
                _ => {}
            });
        TraitDecompositionPart1 { ident: ident.clone(), methods, types, consts }
    }
}

// For use in Full Context Tree
#[derive(Clone)]
#[allow(unused)]
pub struct TraitVTableComposer<SPEC>
    where SPEC: Specification + 'static {
    pub method_composers: Depunctuated<SigComposerLink<SPEC>>,
    pub types: HashMap<Ident, TraitTypeModel>,
}

impl<SPEC> TraitVTableComposer<SPEC>
    where SPEC: Specification {
    #[allow(unused)]
    pub fn from_item_trait(item_trait: &ItemTrait, ty_context: SPEC::TYC, self_ty: Type, context: &ScopeContextLink) -> Self {
        let trait_ident = &item_trait.ident;
        let source = context.borrow();
        let mut method_composers = Depunctuated::new();
        let mut types = HashMap::new();
        item_trait.items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Fn(trait_item_fn) => {
                    let name_context = ty_context.join_fn(
                        source.scope.joined_path_holder(&trait_item_fn.sig.ident).0,
                        FnSignatureContext::TraitImpl(trait_item_fn.sig.clone(), self_ty.clone(), trait_ident.to_type()),
                        trait_item_fn.attrs.cfg_attributes()
                    );
                    let method_scope_context = Rc::new(RefCell::new(source.joined(trait_item_fn)));
                    method_composers.push(SigComposer::from_trait_item_method(trait_item_fn, name_context, &method_scope_context));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeModel::from_item_type(trait_item_type));
                },
                _ => {}
            });
        TraitVTableComposer { method_composers, types }
    }
}

#[derive(Clone)]
pub struct TraitModelPart1 {
    pub item: ItemTrait,
    pub implementors: Vec<TypeModelKind>,
}

impl Debug for TraitModelPart1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.implementors.iter().map(|i| format_token_stream(i)).collect::<Vec<_>>().join("\n\n");
        f.write_str(format!("{}:\n  {}", format_token_stream(&self.item.ident), s).as_str())
    }
}

impl TraitModelPart1 {
    pub fn new(item: ItemTrait) -> Self {
        Self { item, implementors: vec![] }
    }
}
