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

use std::fmt::{Debug, Display, Formatter};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Generics, Item, Lifetime};
use crate::error;
use crate::lang::{CrateTreeConsumer, Specification};
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::presentable::TypeContext;
use crate::tree::{CrateTree, ScopeTree, ScopeTreeItem};

pub use fermentate::Fermentate as ObjCFermentate;
// #[cfg(feature = "objc")]
// pub use writer::Writer as ObjCWriter;
#[cfg(feature = "objc")]
pub use xcproj::Config as XCodeConfig;
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::{SourceComposable, GenericComposer, MaybeComposer, SourceAccessible, SourceFermentable};
use crate::kind::expand_attributes;
use crate::presentable::Expression;
use crate::presentation::{FFIVariable, Name};

#[derive(Clone, Debug)]
pub struct ObjCSpecification;

impl Specification for ObjCSpecification {
    type Attr = AttrWrapper;
    type Gen = Option<Generics>;
    type Lt = Vec<Lifetime>;
    type TYC = TypeContext;
    type Interface = InterfaceImplementation;
    type Expr = Expression<Self>;
    type Var = FFIVariable<Self, TokenStream2>;
    type Name = Name<Self>;
    type Fermentate = ObjCFermentate;
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

impl SourceFermentable<ObjCFermentate> for CrateTree {
    fn ferment(&self) -> ObjCFermentate {
        let Self { attrs: _, crates, generics_tree: ScopeTree { imported, .. }} = self;
        let source = self.source_ref();
        let reg_conversions = Depunctuated::from_iter(crates.iter().map(SourceFermentable::<ObjCFermentate>::ferment));
        let _generic_imports = SemiPunctuated::from_iter(imported.iter().cloned());
        let global = source.context.borrow();
        let config = global.config.maybe_objc_config().expect("Expected ObjC config");
        let prefix = config.class_prefix();
        let generic_conversions = Depunctuated::from_iter(
            global.refined_mixins
                .iter()
                .filter_map(|(mixin, attrs)| {
                    let attrs = expand_attributes(attrs);
                    let ty_context = TypeContext::mixin(mixin, prefix, attrs.cfg_attributes());
                    GenericComposer::<ObjCSpecification>::new(mixin, ty_context, attrs, self.context())
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
                    if let Some(composer) = <Item as MaybeComposer<ObjCSpecification>>::maybe_composer(item, scope, scope_context) {
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
