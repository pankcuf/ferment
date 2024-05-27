use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::{Attribute, parse_quote, Path, Type};
use crate::composition::{CfgAttributes, GenericBoundComposition, TypeComposition};
use crate::context::GlobalContext;
use crate::context::scope::Scope;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{CrateExtension, DictionaryType, Pop, ResolveAttrs, ToPath, ToType};
use crate::formatter::format_attrs;
use crate::helper::path_arguments_to_nested_objects;
use crate::holder::PathHolder;

#[derive(Clone, Eq)]
pub struct ScopeInfo {
    pub attrs: Vec<Attribute>,
    pub crate_ident: Ident,
    pub self_scope: Scope
}
impl PartialEq<Self> for ScopeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.self_scope.eq(&other.self_scope) &&
            self.crate_ident.eq(&other.crate_ident)
    }
}


#[derive(Clone, Eq)]
#[repr(u8)]
pub enum ScopeChain {
    CrateRoot {
        info: ScopeInfo,
    },
    Mod {
        info: ScopeInfo,
        parent_scope_chain: Box<ScopeChain>,
    },
    Trait {
        info: ScopeInfo,
        parent_scope_chain: Box<ScopeChain>,
    },
    Fn {
        info: ScopeInfo,
        parent_scope_chain: Box<ScopeChain>,
    },
    Object {
        info: ScopeInfo,
        parent_scope_chain: Box<ScopeChain>,
    },
    Impl {
        info: ScopeInfo,
        parent_scope_chain: Box<ScopeChain>,
    },
}

impl PartialEq<Self> for ScopeChain {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScopeChain::Impl { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::Impl { info: ScopeInfo { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) |
            (ScopeChain::CrateRoot { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::CrateRoot { info: ScopeInfo {crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) |
            (ScopeChain::Mod { info: ScopeInfo {crate_ident, self_scope, ..}, .. },
                ScopeChain::Mod { info: ScopeInfo {crate_ident: other_crate_ident, self_scope: other_self_scope, ..}, .. }) |
            (ScopeChain::Trait { info: ScopeInfo {crate_ident, self_scope, ..}, .. },
                ScopeChain::Trait { info: ScopeInfo {crate_ident: other_crate_ident, self_scope: other_self_scope, ..}, .. }) |
            (ScopeChain::Fn { info: ScopeInfo {crate_ident, self_scope, ..}, .. },
                ScopeChain::Fn { info: ScopeInfo {crate_ident: other_crate_ident, self_scope: other_self_scope, ..}, .. }) |
            (ScopeChain::Object { info: ScopeInfo {crate_ident, self_scope, ..}, .. },
                ScopeChain::Object { info: ScopeInfo {crate_ident: other_crate_ident, self_scope: other_self_scope, ..}, .. }) =>
                self_scope.eq(&other_self_scope) && crate_ident.eq(other_crate_ident),
            _ => false
        }
    }
}
impl Hash for ScopeChain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.variant_code().hash(state);
        self.self_path_holder_ref().hash(state);
    }
}

impl Debug for ScopeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.fmt_long().as_str())
    }
}
impl Display for ScopeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ScopeChain {
    pub fn variant_code(&self) -> u8 {
        match self {
            ScopeChain::CrateRoot { .. } => 0,
            ScopeChain::Mod { .. } => 1,
            ScopeChain::Fn { .. } => 2,
            ScopeChain::Object { .. } => 3,
            ScopeChain::Impl { .. } => 4,
            ScopeChain::Trait { .. } => 5
        }
    }

    #[allow(unused)]
    fn fmt_short(&self) -> String {
        match self {
            ScopeChain::CrateRoot { info, .. } |
            ScopeChain::Mod { info, .. } |
            ScopeChain::Fn { info, .. } |
            ScopeChain::Object { info, .. } |
            ScopeChain::Impl { info, .. } |
            ScopeChain::Trait { info, .. } =>
                format!("[{}]", info.self_scope.self_scope.0.to_token_stream()),
        }
    }
    #[allow(unused)]
    fn fmt_long(&self) -> String {
        match self {
            ScopeChain::CrateRoot { info } =>
                format!("[{}] :: {} + {} (CrateRoot)", format_attrs(&info.attrs), info.crate_ident, info.self_scope),
            ScopeChain::Mod { info, parent_scope_chain } =>
                format!("[{}] :: {} + {} (Mod) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent_scope_chain),
            ScopeChain::Trait { info, parent_scope_chain } =>
                format!("[{}] :: {} + {} (Trait) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent_scope_chain),
            ScopeChain::Fn { info, parent_scope_chain } =>
                format!("[{}] :: {} + {} (Fn) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent_scope_chain),
            ScopeChain::Object { info, parent_scope_chain } =>
                format!("[{}] :: {} + {} (Object) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent_scope_chain),
            ScopeChain::Impl { info, parent_scope_chain } =>
                format!("[{}] :: {} + {} (Impl) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent_scope_chain),
        }
    }
}

impl ToTokens for ScopeChain {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.self_path_holder_ref()
            .to_tokens(tokens)
    }
}

impl ToType for ScopeChain {
    fn to_type(&self) -> Type {
        self.self_path_holder_ref().to_type()
    }
}

impl ScopeChain {

    pub fn crate_root_with_ident(crate_ident: Ident, attrs: Vec<Attribute>) -> Self {
        let self_scope = Scope::new(parse_quote!(#crate_ident), ObjectConversion::Empty);
        ScopeChain::CrateRoot { info: ScopeInfo { attrs, crate_ident, self_scope }}
    }
    pub fn crate_root(crate_ident: Ident, attrs: Vec<Attribute>) -> Self {
        let self_scope = Scope::new(PathHolder::crate_root(), ObjectConversion::Empty);
        ScopeChain::CrateRoot { info: ScopeInfo { attrs, crate_ident, self_scope }}
    }
    pub fn child_mod(crate_ident: Ident, name: &Ident, parent_scope: &ScopeChain, attrs: Vec<Attribute>) -> Self {
        ScopeChain::new_mod(crate_ident, Scope::new(parent_scope.self_path_holder_ref().joined(name), ObjectConversion::Empty), parent_scope, attrs)
    }
    fn new_mod(crate_ident: Ident, self_scope: Scope, parent_scope: &ScopeChain, attrs: Vec<Attribute>) -> Self {
        ScopeChain::Mod { info: ScopeInfo { attrs, crate_ident, self_scope }, parent_scope_chain: Box::new(parent_scope.clone()) }
    }
    pub fn crate_ident(&self) -> &Ident {
        match self {
            ScopeChain::Mod { info, .. } |
            ScopeChain::Trait { info, .. } |
            ScopeChain::Fn { info, .. } |
            ScopeChain::Object { info, .. } |
            ScopeChain::Impl { info, .. } |
            ScopeChain::CrateRoot { info, .. } => &info.crate_ident,
        }
    }
    pub fn crate_ident_as_path(&self) -> Path {
        self.crate_ident().to_path()
    }
    pub fn self_scope(&self) -> &Scope {
        match self {
            ScopeChain::Mod { info, .. } |
            ScopeChain::Trait { info, .. } |
            ScopeChain::Fn { info, .. } |
            ScopeChain::Object { info, .. } |
            ScopeChain::Impl { info, .. } |
            ScopeChain::CrateRoot { info, .. } => &info.self_scope,
        }
    }

    pub fn joined_path_holder(&self, ident: &Ident) -> PathHolder {
        let scope = self.self_path_holder_ref();
        let mut full_fn_path = scope.joined(ident);
        if scope.is_crate_based() {
            full_fn_path.replace_first_with(&PathHolder::from(self.crate_ident().to_path()))
        }
        full_fn_path
    }

    pub fn self_path_holder(&self) -> PathHolder {
        self.self_scope().self_scope.clone()
    }
    pub fn self_path_holder_ref(&self) -> &PathHolder {
        &self.self_scope().self_scope
    }

    pub fn parent_path_holder(&self) -> PathHolder {
        self.self_path_holder_ref().popped()
    }

    pub fn self_path(&self) -> &Path {
        &self.self_path_holder_ref().0
    }

    pub fn parent_scope(&self) -> Option<&ScopeChain> {
        match self {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => None,
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Fn { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } => Some(parent_scope_chain),
        }
    }

    pub fn obj_root_chain(&self) -> Option<&Self> {
        match self {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => None,
            ScopeChain::Trait { .. } |
            ScopeChain::Object { .. } |
            ScopeChain::Impl { .. } => Some(self),
            ScopeChain::Fn { parent_scope_chain, .. } => parent_scope_chain.obj_root_chain(),
        }
    }

    pub(crate) fn is_crate_root(&self) -> bool {
        if let ScopeChain::CrateRoot { info, .. } = self {
            info.self_scope.self_scope.0.segments.last().unwrap().ident == format_ident!("crate")
        } else {
            false
        }
    }

    pub fn head(&self) -> Ident {
        self.self_path_holder_ref().head()
    }

    pub fn has_same_parent(&self, other: &ScopeChain) -> bool {
        match self {
            ScopeChain::CrateRoot { info, .. } |
            ScopeChain::Mod { info, .. } => info.crate_ident.eq(other.crate_ident()) && info.self_scope.eq(other.self_scope()),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Fn { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } => other.eq(&parent_scope_chain),
        }
    }

    pub fn maybe_dictionary_type(&self, path: &Path, source: &GlobalContext) -> Option<TypeCompositionConversion> {
        path.segments.last().and_then(|last_segment| {
            let nested_arguments = path_arguments_to_nested_objects(&last_segment.arguments, &(self, source));
            let ident = &last_segment.ident;
            if ident.is_primitive() {
                Some(TypeCompositionConversion::Primitive(TypeComposition::new_non_gen(path.to_type(), None)))
            } else if ident.is_any_string() {
                Some(TypeCompositionConversion::Object(TypeComposition::new_non_gen(path.to_type(), None)))
            // } else if ident.is_smart_ptr() {
            //     Some(TypeCompositionConversion::SmartPointer(TypeComposition::new(path.to_type(), None, nested_arguments)))
            } else if ident.is_special_std_trait()  {
                Some(TypeCompositionConversion::TraitType(TypeComposition::new_non_gen(path.to_type(), None)))
            } else if matches!(ident.to_string().as_str(), "FromIterator") {
                Some(TypeCompositionConversion::TraitType(TypeComposition::new(path.to_type(), None, nested_arguments)))
            } else {
                None
            }
        })
    }

    pub fn maybe_generic_bound_for_path(&self, path: &Path) -> Option<GenericBoundComposition> {
        match self {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => None,
            ScopeChain::Trait { info, .. } |
            ScopeChain::Object { info, .. } |
            ScopeChain::Impl { info, .. } =>
                info.self_scope.maybe_generic_bound_for_path(path),
            ScopeChain::Fn { info, parent_scope_chain, .. } =>
                info.self_scope.maybe_generic_bound_for_path(path)
                    .or(parent_scope_chain.maybe_generic_bound_for_path(path)),
        }
    }
}

impl ResolveAttrs for ScopeChain {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>> {
        match self {
            ScopeChain::CrateRoot { info, .. } => info.attrs.cfg_attributes_or_none(),
            ScopeChain::Mod { info, parent_scope_chain, .. } |
            ScopeChain::Trait { info, parent_scope_chain, .. } |
            ScopeChain::Fn { info, parent_scope_chain, .. } |
            ScopeChain::Object { info, parent_scope_chain, .. } |
            ScopeChain::Impl { info, parent_scope_chain, .. } => {
                let mut inherited_attrs = parent_scope_chain.resolve_attrs();
                inherited_attrs.extend(info.attrs.cfg_attributes_or_none());
                inherited_attrs
            }
        }
    }
}
