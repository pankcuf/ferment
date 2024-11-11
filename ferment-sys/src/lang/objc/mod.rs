pub(crate) mod composer;
pub(crate) mod composers;
pub(crate) mod constants;
pub(crate) mod conversion;
pub(crate) mod fermentate;
pub(crate) mod presentation;
#[allow(unused)]
pub(crate) mod writer;
mod xcproj;
pub(crate) mod presentable;
mod composable;
mod formatter;
mod dictionary;

use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Generics, Item, Type};
use crate::error;
use crate::lang::{CrateTreeConsumer, PresentableSpecification, Specification};
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::presentable::TypeContext;
use crate::tree::{CrateTree, ScopeTree, ScopeTreeItem};

pub use fermentate::Fermentate as ObjCFermentate;
pub use writer::Writer as ObjCWriter;
pub use xcproj::Config as XCodeConfig;
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::{SourceComposable, GenericComposer, MaybeComposer, SourceAccessible, SourceFermentable};
use crate::conversion::expand_attributes;
use crate::ext::Resolve;
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{FFIVariable, Name};

pub trait ObjCSpecification:
    PresentableSpecification<ObjCFermentate,
        Attr=AttrWrapper,
        Gen=Option<Generics>,
        Interface=InterfaceImplementation,
        TYC=TypeContext,
        Expr=Expression<ObjCFermentate, Self>,
        Name=Name<ObjCFermentate, Self>,
        Var=FFIVariable<ObjCFermentate, Self, TokenStream2>
    > where <Self::Expr as ScopeContextPresentable>::Presentation: ToTokens,
            Type: Resolve<Self::Var> {}

impl<SPEC> Specification<ObjCFermentate> for SPEC where SPEC: ObjCSpecification {
    type Attr = AttrWrapper;
    type Gen = Option<Generics>;
    type TYC = TypeContext;
    type Interface = InterfaceImplementation;
    type Expr = Expression<ObjCFermentate, SPEC>;
    type Var = FFIVariable<ObjCFermentate, SPEC, TokenStream2>;
    type Name = Name<ObjCFermentate, SPEC>;
}


#[derive(Debug, Clone)]
pub struct Config {
    pub xcode: XCodeConfig,
    // pub targets: [&'static str; 5]
}

impl Config {
    pub fn new(xcode: XCodeConfig) -> Self {
        Self {
            xcode,
            // targets: APPLE_TARGETS
        }
    }
    pub fn class_prefix(&self) -> &str {
        &self.xcode.class_prefix
    }
}


impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[objc::Config]\n\txcode: {}", self.xcode))
    }
}

impl CrateTreeConsumer for Config {
    fn generate(&self, _crate_tree: &CrateTree) -> Result<(), error::Error> {
        // let ff = ObjectPresentation::Interface { name: Name::Index(0), c_type: quote!(), properties: SemiPunctuated::new() };
        // println!("objc:: {}", ff.to_token_stream());
        Ok(())
        // unimplemented!("{:?}", crate_tree)

    }
}


// #[derive(Clone, Debug)]
// pub enum CategoryKind {
//     C,
//     Rust,
//     Args
// }
// impl ToTokens for CategoryKind {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         match self {
//             CategoryKind::C => quote!(C),
//             CategoryKind::Rust => quote!(Rust),
//             CategoryKind::Args => quote!(Args),
//         }.to_tokens(tokens)
//     }
// }

impl ObjCSpecification for ScopeTree {}
impl ObjCSpecification for CrateTree {}
impl SourceFermentable<ObjCFermentate> for CrateTree {
    fn ferment(&self) -> ObjCFermentate {
        let Self { attrs: _, crates, generics_tree: ScopeTree { imported, .. }} = self;
        let source = self.source_ref();
        let reg_conversions = Depunctuated::from_iter(crates.iter().map(SourceFermentable::<ObjCFermentate>::ferment));
        let _generic_imports = SemiPunctuated::from_iter(imported.iter().cloned());
        let global = source.context
            .read()
            .unwrap();
        let config = global.config.maybe_objc_config().unwrap();
        let prefix = config.class_prefix();
        let generic_conversions = Depunctuated::from_iter(
            global.refined_mixins
                .iter()
                .filter_map(|(mixin, attrs)| {
                    let attrs = expand_attributes(attrs);
                    let ty_context = TypeContext::mixin(mixin, prefix, attrs.cfg_attributes());
                    GenericComposer::<ObjCFermentate, CrateTree>::new(mixin, attrs, ty_context, self.context())
                })
                .flat_map(|composer| composer.borrow().compose(&source)));
        let custom_conversions = Depunctuated::from_iter(
            global.custom
                .inner
                .iter()
                .map(|(_scope_chain, _type_chain)| {
                    quote!()
                    // CustomComposer::<ObjCFermentate, CrateTree>::new()
                    // let attrs = expand_attributes(attrs);
                    // let ty_context = TypeContext::mixin(mixin, prefix, attrs.cfg_attributes());
                    // GenericComposer::<ObjCFermentate, CrateTree>::new(mixin, attrs, ty_context, self.context())
                }));
                // .flat_map(|composer| composer.borrow().compose(&source)));

        // println!("CrateTree:: OBJC: {}", reg_conversions.to_token_stream());

        ObjCFermentate::TokenStream(quote! {
            #reg_conversions
            #generic_conversions
            #custom_conversions
        })
    }
}

impl SourceFermentable<ObjCFermentate> for ScopeTree {
    fn ferment(&self) -> ObjCFermentate {
        // let source = self.source_ref();
        let mut fermentate = Depunctuated::<ObjCFermentate>::new();
        self.exported
            .values()
            .for_each(|item| match item {
                ScopeTreeItem::Item { item, scope, scope_context } =>
                    if let Some(composer) = <Item as MaybeComposer<ObjCFermentate, ScopeTree>>::maybe_composer(item, scope, scope_context) {
                        fermentate.push(composer.ferment())
                    },
                ScopeTreeItem::Tree { tree} =>
                    fermentate.push(tree.ferment())
            });
        //println!("OBJC SCOPE FERMENTATE: {}", fermentate.to_token_stream());
//         if !fermentate.is_empty() {
//             let ctx = source.context.read().unwrap();
//             let rename = ctx.config.current_crate.ident();
//             let mut imports = SemiPunctuated::from_iter([
//                 create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename }))
//             ]);
//             imports.extend(SemiPunctuated::from_iter(self.imported.iter().cloned()));
//             let name = if self.scope.is_crate_root() {
//                 self.scope.crate_ident_ref().to_token_stream()
//             } else {
//                 self.scope.head().to_token_stream()
//             };
//             // Depunctuated::from_iter([RustFermentate::mod_with(self.attrs.cfg_attributes(), name, imports, fermentate)])
//             Depunctuated::new()
//         } else {
//             Depunctuated::new()
//         }
        ObjCFermentate::TokenStream(fermentate.to_token_stream())
    }
}
