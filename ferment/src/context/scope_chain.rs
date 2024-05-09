use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, Path, Type};
use crate::composition::{GenericBoundComposition, TypeComposition};
use crate::context::GlobalContext;
use crate::context::scope::Scope;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{CrateExtension, DictionaryType, Pop, ToPath, ToType};
use crate::helper::path_arguments_to_nested_objects;
use crate::holder::PathHolder;

#[derive(Clone, Eq)]
#[repr(u8)]
pub enum ScopeChain {
    CrateRoot {
        crate_ident: Ident,
        self_scope: Scope
    },
    Mod {
        crate_ident: Ident,
        self_scope: Scope,
        parent_scope_chain: Box<ScopeChain>,
    },
    Trait {
        crate_ident: Ident,
        self_scope: Scope,
        parent_scope_chain: Box<ScopeChain>,
    },
    Fn {
        crate_ident: Ident,
        self_scope: Scope,
        parent_scope_chain: Box<ScopeChain>,
    },
    Object {
        crate_ident: Ident,
        self_scope: Scope,
        parent_scope_chain: Box<ScopeChain>,
    },
    Impl {
        crate_ident: Ident,
        self_scope: Scope,
        trait_scopes: Vec<ScopeChain>,
        parent_scope_chain: Box<ScopeChain>,
    },
}

impl PartialEq<Self> for ScopeChain {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScopeChain::Impl { crate_ident, self_scope, .. }, ScopeChain::Impl { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }) |
            (ScopeChain::CrateRoot { crate_ident, self_scope }, ScopeChain::CrateRoot { crate_ident: other_crate_ident, self_scope: other_self_scope }) |
            (ScopeChain::Mod { crate_ident, self_scope, .. }, ScopeChain::Mod { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }) |
            (ScopeChain::Trait { crate_ident, self_scope, .. }, ScopeChain::Trait { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }) |
            (ScopeChain::Fn { crate_ident, self_scope, .. }, ScopeChain::Fn { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }) |
            (ScopeChain::Object { crate_ident, self_scope, .. }, ScopeChain::Object { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }) =>
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
            ScopeChain::CrateRoot { self_scope, .. } |
            ScopeChain::Mod { self_scope, .. } |
            ScopeChain::Fn { self_scope, .. } |
            ScopeChain::Object { self_scope, .. } |
            ScopeChain::Impl { self_scope, .. } |
            ScopeChain::Trait { self_scope, .. } =>
                format!("[{}]", self_scope.self_scope.0.to_token_stream()),
        }
    }
    #[allow(unused)]
    fn fmt_long(&self) -> String {
        match self {
            ScopeChain::CrateRoot { crate_ident, self_scope } =>
                format!("[{}] :: {} (CrateRoot)", crate_ident, self_scope),
            ScopeChain::Mod { crate_ident, self_scope, parent_scope_chain } =>
                format!("[{}] :: {} (Mod) (parent: {:?})", crate_ident, self_scope, parent_scope_chain),
            ScopeChain::Trait { crate_ident, self_scope, parent_scope_chain } =>
                format!("[{}] :: {} (Trait) (parent: {:?})", crate_ident, self_scope, parent_scope_chain),
            ScopeChain::Fn { crate_ident, self_scope, parent_scope_chain } =>
                format!("[{}] :: {} (Fn) (parent: {:?})", crate_ident, self_scope, parent_scope_chain),
            ScopeChain::Object { crate_ident, self_scope, parent_scope_chain } =>
                format!("[{}] :: {} (Object) (parent: {:?})", crate_ident, self_scope, parent_scope_chain),
            ScopeChain::Impl { crate_ident, self_scope, trait_scopes, parent_scope_chain } =>
                format!("[{}] :: {} (Impl) (parent: {:?}, traits: {:?})", crate_ident, self_scope, parent_scope_chain, trait_scopes),
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

    pub fn crate_root_with_ident(crate_ident: Ident) -> Self {
        let self_scope = Scope::new(parse_quote!(#crate_ident), ObjectConversion::Empty);
        ScopeChain::CrateRoot { crate_ident, self_scope }
    }
    pub fn crate_root(crate_ident: Ident) -> Self {
        let self_scope = Scope::new(PathHolder::crate_root(), ObjectConversion::Empty);
        ScopeChain::CrateRoot { crate_ident, self_scope }
    }
    pub fn child_mod(crate_ident: Ident, name: &Ident, scope: &ScopeChain) -> Self {
        ScopeChain::new_mod(crate_ident, Scope::new(scope.self_path_holder_ref().joined(name), ObjectConversion::Empty), scope)
    }
    fn new_mod(crate_ident: Ident, self_scope: Scope, parent_scope: &ScopeChain) -> Self {
        ScopeChain::Mod { crate_ident, self_scope, parent_scope_chain: Box::new(parent_scope.clone()) }
    }
    pub fn crate_ident(&self) -> &Ident {
        match self {
            ScopeChain::Mod { crate_ident, .. } |
            ScopeChain::Trait { crate_ident, .. } |
            ScopeChain::Fn { crate_ident, .. } |
            ScopeChain::Object { crate_ident, .. } |
            ScopeChain::Impl { crate_ident, .. } |
            ScopeChain::CrateRoot { crate_ident, .. } => crate_ident,
        }
    }
    pub fn crate_ident_as_path(&self) -> Path {
        self.crate_ident().to_path()
    }
    pub fn self_scope(&self) -> &Scope {
        match self {
            ScopeChain::Mod { self_scope, .. } |
            ScopeChain::Trait { self_scope, .. } |
            ScopeChain::Fn { self_scope, .. } |
            ScopeChain::Object { self_scope, .. } |
            ScopeChain::Impl { self_scope, .. } |
            ScopeChain::CrateRoot { self_scope, .. } => self_scope,
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
    // pub fn parent_scope2(&self) -> Option<&ScopeChain> {
    //     match self {
    //         ScopeChain::CrateRoot { .. } => None,
    //         ScopeChain::Mod { parent_scope_chain, .. } |
    //         ScopeChain::Trait { parent_scope_chain, .. } |
    //         ScopeChain::Fn { parent_scope_chain, .. } |
    //         ScopeChain::Object { parent_scope_chain, .. } |
    //         ScopeChain::Impl { parent_scope_chain, .. } => Some(parent_scope_chain),
    //     }
    // }

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
        if let ScopeChain::CrateRoot { crate_ident: _, self_scope } = self {
            self_scope.self_scope.0.segments.last().unwrap().ident == format_ident!("crate")
        } else {
            false
        }
    }

    pub fn head(&self) -> Ident {
        self.self_path_holder_ref().head()
    }

    pub fn has_same_parent(&self, other: &ScopeChain) -> bool {
        match self {
            ScopeChain::CrateRoot { crate_ident, self_scope } |
            ScopeChain::Mod { crate_ident, self_scope, .. } => crate_ident.eq(other.crate_ident()) && self_scope.eq(other.self_scope()),
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
            } else if ident.is_smart_ptr() {
                Some(TypeCompositionConversion::SmartPointer(TypeComposition::new(path.to_type(), None, nested_arguments)))
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
        // println!("maybe_generic_bound_for_path: {}: {}", format_token_stream(path), self);
        match self {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => None,
            ScopeChain::Trait { self_scope, .. } |
            ScopeChain::Object { self_scope, .. } |
            ScopeChain::Impl { self_scope, .. } =>
                self_scope.maybe_generic_bound_for_path(path),
            ScopeChain::Fn { self_scope, parent_scope_chain, .. } => {
                let maybe_generic_bound = self_scope.maybe_generic_bound_for_path(path);
                if maybe_generic_bound.is_some() {
                    // println!("Fn:::: {}", maybe_bound.to_string());
                    maybe_generic_bound
                } else {
                    // println!("Fn (or parent?) [{}]", parent_scope_chain);
                    let maybe_parent_bound = parent_scope_chain.maybe_generic_bound_for_path(path);
                    // println!("Fn (or parent?):::: {}", maybe_parent_bound.as_ref().map_or(format!("None"), |f| f.to_string()));
                    maybe_parent_bound

                }
            },
        }
    }
}

// pub fn can_fn_in_parent_scope_have_self(parent_scope: &ScopeChain) -> bool {
//     match parent_scope {
//         ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => false,
//         ScopeChain::Fn { parent_scope_chain, .. } =>
//             can_fn_in_parent_scope_have_self(parent_scope_chain),
//         ScopeChain::Trait { .. } |
//         ScopeChain::Object { .. } |
//         ScopeChain::Impl { .. } => true
//     }
// }
