use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::{Item, parse_quote, Path, Type};
use crate::composition::{GenericBoundComposition, TypeComposition};
use crate::context::scope::Scope;
use crate::conversion::{ObjectConversion, TypeConversion};
use crate::holder::PathHolder;

#[derive(Clone, Eq)]
#[repr(u8)]
pub enum ScopeChain {
    CrateRoot {
        self_scope: Scope
    },
    Mod {
        self_scope: Scope
    },
    Trait {
        self_scope: Scope,
        parent_scope_chain: Box<ScopeChain>,
    },
    Fn {
        self_scope: Scope,
        parent_scope_chain: Box<ScopeChain>,
    },
    Object {
        self_scope: Scope,
        parent_scope_chain: Box<ScopeChain>,
    },
    Impl {
        self_scope: Scope,
        trait_scopes: Vec<ScopeChain>,
        parent_scope_chain: Box<ScopeChain>,
    },
}

impl PartialEq<Self> for ScopeChain {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScopeChain::Impl { self_scope, .. }, ScopeChain::Impl { self_scope: other_self_scope, .. }) |
            (ScopeChain::CrateRoot { self_scope }, ScopeChain::CrateRoot { self_scope: other_self_scope }) |
            (ScopeChain::Mod { self_scope, .. }, ScopeChain::Mod { self_scope: other_self_scope, .. }) |
            (ScopeChain::Trait { self_scope, .. }, ScopeChain::Trait { self_scope: other_self_scope, .. }) |
            (ScopeChain::Fn { self_scope, .. }, ScopeChain::Fn { self_scope: other_self_scope, .. }) |
            (ScopeChain::Object { self_scope, .. }, ScopeChain::Object { self_scope: other_self_scope, .. }) =>
                self_scope.eq(&other_self_scope),
            _ => false
        }
    }
}
impl Hash for ScopeChain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ScopeChain::CrateRoot { .. } => 0,
            ScopeChain::Mod { .. } => 1,
            ScopeChain::Trait { .. } => 2,
            ScopeChain::Fn { .. } => 3,
            ScopeChain::Object { .. } => 4,
            ScopeChain::Impl { .. } => 5
        }.to_string().hash(state);
        self.self_path_holder().to_string().hash(state);
    }
}

impl Debug for ScopeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ScopeChain::CrateRoot { self_scope } =>
                format!("{} (CrateRoot)", self_scope),
            ScopeChain::Mod { self_scope } =>
                format!("{} (Mod)", self_scope),
            ScopeChain::Trait { self_scope, parent_scope_chain } =>
                format!("{} (Trait) (parent: {:?})", self_scope, parent_scope_chain),
            ScopeChain::Fn { self_scope, parent_scope_chain } =>
                format!("{} (Fn) (parent: {:?})", self_scope, parent_scope_chain),
            ScopeChain::Object { self_scope, parent_scope_chain } =>
                format!("{} (Object) (parent: {:?})", self_scope, parent_scope_chain),
            ScopeChain::Impl { self_scope, trait_scopes, parent_scope_chain } =>
                format!("{} (Impl) (parent: {:?}, trauts: {:?})", self_scope, parent_scope_chain, trait_scopes),
        }.as_str())
    }
}
impl Display for ScopeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ToTokens for ScopeChain {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.self_path_holder()
            .to_tokens(tokens)
    }
}


impl ScopeChain {

    pub fn crate_root() -> Self {
        ScopeChain::CrateRoot { self_scope: Scope::new(PathHolder::crate_root(), ObjectConversion::Empty) }
    }
    pub fn new_mod(self_scope: Scope) -> Self {
        ScopeChain::Mod { self_scope }
    }
    pub fn child_mod(name: &Ident, scope: &ScopeChain) -> Self {
        ScopeChain::Mod { self_scope: Scope::new(scope.self_path_holder().joined(name), ObjectConversion::Empty) }
    }
    pub fn self_scope(&self) -> &Scope {
        match self {
            ScopeChain::Mod { self_scope } => self_scope,
            ScopeChain::Trait { self_scope, .. } => self_scope,
            ScopeChain::Fn { self_scope, .. } => self_scope,
            ScopeChain::Object { self_scope, .. } => self_scope,
            ScopeChain::Impl { self_scope, .. } => self_scope,
            ScopeChain::CrateRoot { self_scope } => self_scope,
        }
    }

    pub fn self_path_holder(&self) -> &PathHolder {
        &self.self_scope().self_scope
    }

    pub fn self_path(&self) -> &Path {
        &self.self_path_holder().0
    }

    pub fn parent_scope(&self) -> Option<&Self> {
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

    pub fn root_chain(&self) -> Option<&ScopeChain> {
        match self {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => None,
            ScopeChain::Fn { parent_scope_chain, .. } => parent_scope_chain.root_chain(),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } => Some(parent_scope_chain),
        }
    }

    pub fn to_type(&self) -> Type {
        self.self_path_holder().to_type()
    }

    pub(crate) fn is_crate_root(&self) -> bool {
        if let ScopeChain::CrateRoot { self_scope } = self {
            self_scope.self_scope.0.segments.last().unwrap().ident == format_ident!("crate")
        } else {
            false
        }
    }
    pub fn is_mod_level(&self) -> bool {
        match self {
            ScopeChain::Mod { .. } |
            ScopeChain::CrateRoot { .. } => true,
            _ => false
        }
    }

    pub fn can_have_self(&self) -> bool {
        match self {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => false,
            ScopeChain::Fn { parent_scope_chain, .. } =>
                can_fn_in_parent_scope_have_self(parent_scope_chain),
            _ => true
        }
    }

    pub fn head(&self) -> Ident {
        self.self_path_holder().head()
    }

    pub fn joined_obj(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        ScopeChain::Object { self_scope, parent_scope_chain: Box::new(self.clone()) }
    }

    pub fn joined_fn(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        ScopeChain::Fn { self_scope, parent_scope_chain: Box::new(self.clone()) }
    }

    pub fn joined_trait(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        ScopeChain::Trait { self_scope, parent_scope_chain: Box::new(self.clone()) }
    }

    pub fn joined_mod(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        ScopeChain::Mod { self_scope }
    }

    pub fn joined_impl(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        ScopeChain::Impl {
            self_scope,
            trait_scopes: vec![],
            parent_scope_chain: Box::new(self.clone()),
        }
    }

    pub fn joined(&self, item: &Item) -> Self {
        match item {
            Item::Const(..) |
            Item::Type(..) |
            Item::Enum(..) |
            Item::Struct(..) |
            Item::Trait(..) =>
                self.joined_obj(item),
            Item::Fn(..) =>
                self.joined_fn(item),
            Item::Impl(..) =>
                self.joined_impl(item),
            Item::Mod(..) =>
                self.joined_mod(item),
            _ => self.clone()
        }
    }

    pub fn has_same_parent(&self, other: &ScopeChain) -> bool {
        match self {
            ScopeChain::CrateRoot { self_scope } |
            ScopeChain::Mod { self_scope} => self_scope.eq(other.self_scope()),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Fn { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } => other.eq(parent_scope_chain),
        }
    }

    pub fn maybe_dictionary_type(&self, path: &Path) -> Option<TypeConversion> {
        path.get_ident().and_then(|ident| {
            let ident = ident.to_string();
            let ident = ident.as_str();
            if matches!(ident, "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool") {
                // println!("maybe_dictionary_type (found primitive):  {}", quote!(#path));
                Some(TypeConversion::Primitive(TypeComposition::new(parse_quote!(#path), None)))
            } else if matches!(ident, "String" | "str" ) {
                println!("maybe_dictionary_type (found {}):  {}", ident, path.to_token_stream());
                Some(TypeConversion::Object(TypeComposition::new(parse_quote!(#path), None)))
            } else if matches!(ident, "Box" | "Arc" | "Rc" | "Cell" | "RefCell" | "Mutex" | "RwLock")  {
                // println!("maybe_dictionary_type (found smart pointer):  {}", quote!(#path));
                Some(TypeConversion::SmartPointer(TypeComposition::new(parse_quote!(#path), None)))
            } else if matches!(ident, "Send" | "Sync" | "Clone" | "Sized")  {
                // println!("maybe_dictionary_type (found smart pointer):  {}", quote!(#path));
                Some(TypeConversion::TraitType(TypeComposition::new(parse_quote!(#path), None)))
            } else {
                None
            }
        })
    }

    pub fn maybe_self_type(&self, path: &Path) -> Option<TypeConversion> {
        if path.get_ident().map_or(false, |ident| ident.to_string().as_str().eq("Self")) {
            Some(TypeConversion::Object(TypeComposition::new(parse_quote!(#path), None)))
        } else {
            None
        }
    }


    pub fn maybe_generic_bound_for_path(&self, path: &Path) -> Option<GenericBoundComposition> {
        // println!("maybe_generic_bound_for_path: {}: {}", format_token_stream(path), self);
        match self {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } =>
                None,
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

pub fn can_fn_in_parent_scope_have_self(parent_scope: &ScopeChain) -> bool {
    match parent_scope {
        ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => false,
        ScopeChain::Fn { parent_scope_chain, .. } =>
            can_fn_in_parent_scope_have_self(parent_scope_chain),
        ScopeChain::Trait { .. } |
        ScopeChain::Object { .. } |
        ScopeChain::Impl { .. } => true
    }
}
