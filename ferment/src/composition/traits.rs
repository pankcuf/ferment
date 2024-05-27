use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::{Ident, ItemTrait, Path, Signature, TraitBound, TraitItem, TraitItemMethod, TraitItemType, Type, TypeParamBound};
use syn::__private::TokenStream2;
use crate::composer::{CommaPunctuated, Depunctuated, ParentComposer, SigParentComposer};
use crate::composer::composable::SourceExpandable;
use crate::composer::signature::SigComposer;
use crate::formatter::{format_token_stream, format_trait_decomposition_part1};
use crate::composition::{Composition, FnSignatureContext};
use crate::composition::context::TraitDecompositionPart2Context;
use crate::composition::generic_composition::GenericsComposition;
use crate::context::ScopeContext;
use crate::conversion::TypeCompositionConversion;
use crate::ext::ToType;
use crate::holder::PathHolder;
use crate::presentation::Expansion;

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
pub struct TraitTypeDecomposition {
    pub ident: Ident,
    pub trait_bounds: Vec<TraitBoundDecomposition>,
    pub generics: GenericsComposition,
}

impl ToTokens for TraitTypeDecomposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TraitTypeDecomposition { ident, trait_bounds, generics: _ } = self;
        quote!(#ident: #(#trait_bounds),*).to_tokens(tokens)
    }
}

impl TraitTypeDecomposition {
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
    pub types: HashMap<Ident, TraitTypeDecomposition>
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
                    types.insert(trait_item_type.ident.clone(), TraitTypeDecomposition::from_item_type(trait_item_type));
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
pub struct TraitDecompositionPart2 {
    // pub methods: Vec<FnSignatureComposition>,
    pub method_composers: Depunctuated<SigParentComposer>,
    pub types: HashMap<Ident, TraitTypeDecomposition>,
}

impl TraitDecompositionPart2 {
    #[allow(unused)]
    pub fn from_item_trait(item_trait: &ItemTrait, self_ty: Type, _scope: &PathHolder, context: &ParentComposer<ScopeContext>) -> Self {
        let trait_ident = &item_trait.ident;
        let source = context.borrow();
        let mut method_composers = Depunctuated::new();
        let mut types = HashMap::new();
        item_trait.items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Method(TraitItemMethod { sig, attrs, .. } ) => {
                    let sig_context = FnSignatureContext::Impl(self_ty.clone(), Some(trait_ident.to_type()), sig.clone());
                    // let full_fn_path = scope
                    // let mut full_fn_path = scope.joined(&sig.ident);
                    // if scope.is_crate_based() {
                    //     full_fn_path.replace_first_with(&PathHolder::from(source.scope.crate_ident().to_path()))
                    // }
                    let composer = SigComposer::with_context(
                        source.scope.joined_path_holder(&sig.ident).0,
                        &sig.ident,
                        sig_context,
                        &sig.generics,
                        attrs,
                        &source.scope,
                        context
                    );
                    method_composers.push(composer);
                    // methods.push(FnSignatureComposition::from_signature(&sig_context, sig, scope, &source));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeDecomposition::from_item_type(trait_item_type));
                },
                // TraitItem::Const(TraitItemConst { attrs, const_token, ident, colon_token, ty, default, semi_token }) => {
                //
                // },
                _ => {}
            });
        TraitDecompositionPart2 { method_composers, types }
    }
}


impl Composition for TraitDecompositionPart2 {
    type Context = TraitDecompositionPart2Context;
    type Presentation = CommaPunctuated<Expansion>;

    fn present(self, composition_context: Self::Context, _source: &ScopeContext) -> Self::Presentation {
        match composition_context {
            TraitDecompositionPart2Context::VTableInnerFunctions => self.method_composers
                .into_iter()
                .map(|composition|
                    composition.borrow().expand())
                    // source.present_composition_in_context(composition, FnSignatureCompositionContext::TraitVTableInner))
                .collect()
        }

    }
}

#[derive(Clone)]
pub struct TraitCompositionPart1 {
    pub item: ItemTrait,
    pub implementors: Vec<TypeCompositionConversion>,
}

impl std::fmt::Debug for TraitCompositionPart1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.implementors.iter().map(|i| format_token_stream(i)).collect::<Vec<_>>().join("\n\n");
        f.write_str(format!("{}:\n  {}", format_token_stream(&self.item.ident), s).as_str())
    }
}

impl TraitCompositionPart1 {
    pub fn new(item: ItemTrait) -> Self {
        Self { item, implementors: vec![] }
    }
}
