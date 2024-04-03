use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Item, Path, Signature, Type};
use syn::__private::TokenStream2;
use crate::composition::{GenericConversion, ImportComposition};
use crate::context::TypeChain;
use crate::conversion::ImportConversion;
use crate::ext::NestingExtension;
use crate::formatter::format_token_stream;
use crate::helper::{GenericExtension, ItemExtension};
use crate::holder::{PathHolder, TypeHolder};
use crate::tree::ScopeTreeExportID;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ScopeItemConversion {
    Item(Item),
    Fn(Signature),
}

impl ToTokens for ScopeItemConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ScopeItemConversion::Item(item) => item.to_tokens(tokens),
            ScopeItemConversion::Fn(sig) => sig.to_tokens(tokens)
        }
    }
}
impl Debug for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItemConversion::Item(item) =>
                f.write_str(format!("Item({})", format_token_stream(item.maybe_ident())).as_str()),
            ScopeItemConversion::Fn(sig) =>
                f.write_str(format!("Fn({})", format_token_stream(&sig.ident)).as_str()),
        }
    }
}

impl Display for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl GenericExtension for ScopeItemConversion {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        match self {
            ScopeItemConversion::Item(item) => item.collect_compositions(),
            ScopeItemConversion::Fn(sig) => sig.collect_compositions(),
        }
    }
    fn find_generics_fq(&self, scope_types: &TypeChain) -> HashSet<GenericConversion> {
        match self {
            ScopeItemConversion::Item(item) => item.find_generics_fq(scope_types),
            ScopeItemConversion::Fn(sig) => sig.find_generics_fq(scope_types),
        }
    }
}

impl GenericExtension for Type {
    fn collect_compositions(&self) -> Vec<TypeHolder> {
        self.nested_items().iter().map(TypeHolder::from).collect()
        // match self {
        //     Type::Array(_) => {}
        //     Type::BareFn(_) => {}
        //     Type::Group(_) => {}
        //     Type::ImplTrait(_) => {}
        //     Type::Infer(_) => {}
        //     Type::Macro(_) => {}
        //     Type::Never(_) => {}
        //     Type::Paren(_) => {}
        //     Type::Path(_) => {}
        //     Type::Ptr(_) => {}
        //     Type::Reference(_) => {}
        //     Type::Slice(_) => {}
        //     Type::TraitObject(_) => {}
        //     Type::Tuple(_) => {}
        //     Type::Verbatim(_) => {}
        //     Type::__NonExhaustive => {}
        // }
    }
}
impl ItemExtension for ScopeItemConversion {
    fn scope_tree_export_id(&self) -> ScopeTreeExportID {
        match self {
            ScopeItemConversion::Item(item) => item.scope_tree_export_id(),
            ScopeItemConversion::Fn(sig) => sig.scope_tree_export_id()
        }
    }

    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_attrs(),
            ScopeItemConversion::Fn(sig) => sig.maybe_attrs()
        }
    }

    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_ident(),
            ScopeItemConversion::Fn(sig) => sig.maybe_ident()
        }
    }

    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_generics(),
            ScopeItemConversion::Fn(sig) => sig.maybe_generics()
        }
    }

    fn classify_imports(&self, imports: &HashMap<PathHolder, Path>) -> HashMap<ImportConversion, HashSet<ImportComposition>> {
        match self {
            ScopeItemConversion::Item(item) => item.classify_imports(imports),
            ScopeItemConversion::Fn(sig) => sig.classify_imports(imports)
        }
    }
}