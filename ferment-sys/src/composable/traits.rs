use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::{Ident, ItemTrait, Path, Signature, TraitBound, TraitItem, TraitItemMethod, TraitItemType, Type, TypeParamBound};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, Depunctuated, PathHolder};
use crate::composable::{CfgAttributes, Composition, FnSignatureContext, GenericsComposition, TraitDecompositionPart2Context};
use crate::composer::{ComposerLink, SigComposer, SigComposerLink, SourceFermentable};
use crate::context::ScopeContext;
use crate::conversion::TypeModelKind;
use crate::ext::ToType;
use crate::formatter::{format_token_stream, format_trait_decomposition_part1};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{NameTreeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentable::Aspect;
use crate::presentation::RustFermentate;

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
    pub generics: GenericsComposition,
}

impl ToTokens for TraitTypeModel {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TraitTypeModel { ident, trait_bounds, generics: _ } = self;
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
                    TypeParamBound::Lifetime(_lt) =>
                        None
                })
                .collect(),
            generics: GenericsComposition { generics: Default::default() },
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
                TraitItem::Method(TraitItemMethod { sig, .. } ) => {
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
pub struct TraitDecompositionPart2<LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          SPEC::Expr: ScopeContextPresentable {
    // pub methods: Vec<FnSignatureComposition>,
    pub method_composers: Depunctuated<SigComposerLink<LANG, SPEC>>,
    pub types: HashMap<Ident, TraitTypeModel>,
}

impl<LANG, SPEC> TraitDecompositionPart2<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    #[allow(unused)]
    pub fn from_item_trait(item_trait: &ItemTrait, ty_context: SPEC::TYC, self_ty: Type, _scope: &PathHolder, context: &ComposerLink<ScopeContext>) -> Self {
        let trait_ident = &item_trait.ident;
        let source = context.borrow();
        let mut method_composers = Depunctuated::new();
        let mut types = HashMap::new();
        item_trait.items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Method(trait_item_method) => {

                    let name_context = ty_context.join_fn(
                        source.scope.joined_path_holder(&trait_item_method.sig.ident).0,
                        FnSignatureContext::Impl(self_ty.clone(), Some(trait_ident.to_type()), trait_item_method.sig.clone()),
                        trait_item_method.attrs.cfg_attributes()
                    );
                    // let name_context = Context::Fn {
                    //     path: source.scope.joined_path_holder(&trait_item_method.sig.ident).0,
                    //     sig_context: FnSignatureContext::Impl(self_ty.clone(), Some(trait_ident.to_type()), trait_item_method.sig.clone()),
                    //     attrs: trait_item_method.attrs.cfg_attributes()
                    // };
                    method_composers.push(SigComposer::from_trait_item_method(trait_item_method, name_context, context));
                    // method_composers.push(SigComposer::with_context(
                    //     source.scope.joined_path_holder(&sig.ident).0,
                    //     FnSignatureContext::Impl(self_ty.clone(), Some(trait_ident.to_type()), sig.clone()),
                    //     &sig.generics,
                    //     attrs,
                    //     context
                    // ));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeModel::from_item_type(trait_item_type));
                },
                _ => {}
            });
        TraitDecompositionPart2 { method_composers, types }
    }
}


impl<SPEC> Composition for TraitDecompositionPart2<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Context = TraitDecompositionPart2Context;
    type Presentation = CommaPunctuated<RustFermentate>;

    fn present(self, composition_context: Self::Context, _source: &ScopeContext) -> Self::Presentation {
        match composition_context {
            TraitDecompositionPart2Context::VTableInnerFunctions => self.method_composers
                .into_iter()
                .map(|composition|
                    composition.borrow().ferment())
                .collect()
        }

    }
}

#[derive(Clone)]
pub struct TraitModelPart1 {
    pub item: ItemTrait,
    pub implementors: Vec<TypeModelKind>,
}

impl std::fmt::Debug for TraitModelPart1 {
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
