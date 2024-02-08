use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::{Item, parse_quote, Path, Type};
use crate::composition::{GenericBoundComposition, TypeComposition};
use crate::context::scope::Scope;
use crate::conversion::{ObjectConversion, ScopeItemConversion, TypeConversion};
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
        self.self_scope().self_scope.to_string().hash(state);
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
        self.self_scope()
            .self_scope
            .to_tokens(tokens)
    }
}


impl ScopeChain {

    pub fn crate_root() -> Self {
        ScopeChain::CrateRoot { self_scope: Scope::new(PathHolder::crate_root(), ObjectConversion::Empty) }
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
    // fn fn_root_chain(&self) -> &ScopeChain {
    //     match self {
    //         ScopeChain::CrateRoot { .. } => self,
    //         ScopeChain::Mod { .. } => self,
    //         ScopeChain::Trait { parent_scope_chain, .. } => parent_scope_chain,
    //         ScopeChain::Fn { parent_scope_chain, .. } => parent_scope_chain.fn_root_chain(),
    //         ScopeChain::Object { parent_scope_chain, .. } => parent_scope_chain,
    //         ScopeChain::Impl { parent_scope_chain, .. } => parent_scope_chain,
    //     }
    // }

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
        self.self_scope().self_scope.to_type()
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

    pub fn head(&self) -> Ident {
        self.self_scope().self_scope.head()
    }



    // pub fn joined_impl(&self, ident: &Ident) -> ScopeChain {
    //     ScopeChain::Impl {
    //         self_scope: self.self_scope().joined(ident),
    //         parent_scope_chain: Box::new(self.clone()),
    //         trait_scopes: vec![],
    //     }
    // }

    pub fn joined_obj(&self, item: &Item) -> ScopeChain {
        // println!(":::: joined_obj.1: {} in [{}] ", item.ident_string(), self);
        let self_scope = self.self_scope().joined(item);
        // println!(":::: joined_obj.2: [{}] --> [{}]", self.self_scope(), self_scope);
        ScopeChain::Object { self_scope, parent_scope_chain: Box::new(self.clone()) }
    }

    pub fn joined_fn(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        // println!(":::: joined_fn: {} in [{}] --> [{}] ", item.maybe_ident().map_or(format!("None"),Ident::to_string), self.self_scope(), self_scope);
        ScopeChain::Fn { self_scope, parent_scope_chain: Box::new(self.clone()) }
    }

    pub fn joined_trait(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        // println!(":::: joined_trait: {} in [{}] --> [{}] ", item.maybe_ident().map_or(format!("None"),Ident::to_string), self.self_scope(), self_scope);
        ScopeChain::Trait { self_scope, parent_scope_chain: Box::new(self.clone()) }
    }

    pub fn joined_mod(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        // println!(":::: joined_mod: {} in [{}] --> [{}] ", item.maybe_ident().map_or(format!("None"),Ident::to_string), self.self_scope(), self_scope);
        ScopeChain::Mod { self_scope }
    }

    pub fn joined_impl(&self, item: &Item) -> ScopeChain {
        let self_scope = self.self_scope().joined(item);
        // println!(":::: joined_impl: {} in [{}] --> [{}] ", item.maybe_ident().map_or(format!("None"),Ident::to_string), self.self_scope(), self_scope);
        ScopeChain::Impl {
            // self_scope: Scope::new(self_scope, ObjectConversion::Item(TypeConversion::Object(TypeComposition::new(*self_ty.clone(), Some(item.generics.clone()))), ScopeItemConversion::Item(Item::Impl(item_impl.clone())))),
            self_scope,
            trait_scopes: vec![],
            parent_scope_chain: Box::new(self.clone()),
        }
    }


    pub fn joined(&self, item: &Item) -> Self {
        //println!("joubed: {} in [{}]", item.ident_string(), self);
        match item {
            Item::Const(..) =>
                self.joined_obj(item),
            Item::Type(..) =>
                self.joined_obj(item),
            Item::Enum(..) =>
                self.joined_obj(item),
            Item::Struct(..) =>
                self.joined_obj(item),
            Item::Trait(..) =>
                self.joined_obj(item),
            Item::Fn(..) =>
                self.joined_fn(item),
            Item::Impl(item_impl) => {
                let self_ty = &item_impl.self_ty;
                let self_scope: PathHolder = parse_quote!(#self_ty);
                ScopeChain::Impl {
                    self_scope: Scope::new(
                        self_scope,
                        ObjectConversion::Item(
                            TypeConversion::Object(TypeComposition::new(*self_ty.clone(), Some(item_impl.generics.clone()))),
                            ScopeItemConversion::Item(Item::Impl(item_impl.clone())))),
                    trait_scopes: vec![],
                    parent_scope_chain: Box::new(self.clone()) }
            },
            Item::Mod(..) => ScopeChain::Mod {
                self_scope: self.self_scope().joined(item)
            },
            _ => self.clone()
        }
    }

    pub fn has_same_parent(&self, other: &ScopeChain) -> bool {
        // self.parent_scope()
        //     .map(|parent| parent.self_scope().self_scope.eq(other))
        //     .unwrap_or(false)
        match self {
            ScopeChain::CrateRoot { self_scope } |
            ScopeChain::Mod { self_scope} => self_scope.eq(other.self_scope()),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Fn { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } => other.eq(parent_scope_chain),
        }
        //
        // match self {
        //     (ScopeChain::CrateRoot { self_scope }, ScopeChain::CrateRoot { self_scope: other_self_scope }) |
        //     (ScopeChain::Mod { self_scope }, ScopeChain::Mod { self_scope: other_self_scope }) =>
        //         self_scope.eq(&other_self_scope),
        //     (ScopeChain::Trait { parent_scope_chain: parent1, .. }, ScopeChain::Trait { parent_scope_chain: parent2, .. }) |
        //     (ScopeChain::Fn { parent_scope_chain: parent1, .. }, ScopeChain::Fn { parent_scope_chain: parent2, .. }) |
        //     (ScopeChain::Object { parent_scope_chain: parent1, .. }, ScopeChain::Object { parent_scope_chain: parent2, .. }) |
        //     (ScopeChain::Impl { parent_scope_chain: parent1, .. }, ScopeChain::Impl { parent_scope_chain: parent2, .. }) => parent1 == parent2,
        //     _ => false
        // }
    }

    pub fn maybe_dictionary_type(&self, path: &Path) -> Option<TypeConversion> {
        path.get_ident().and_then(|ident| {
            let ident = ident.to_string();
            let ident = ident.as_str();
            if matches!(ident, "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool") {
                // println!("maybe_dictionary_type (found primitive):  {}", quote!(#path));
                Some(TypeConversion::Primitive(TypeComposition::new(parse_quote!(#path), None)))
            } else if matches!(ident, "String" | "str") {
                // println!("maybe_dictionary_type (found string):  {}", quote!(#path));
                Some(TypeConversion::Object(TypeComposition::new(parse_quote!(#path), None)))
            } else if matches!(ident, "Box" | "Arc" | "Rc" | "Cell" | "RefCell" | "Mutex" | "RwLock")  {
                // println!("maybe_dictionary_type (found smart pointer):  {}", quote!(#path));
                Some(TypeConversion::SmartPointer(TypeComposition::new(parse_quote!(#path), None)))
            } else {
                None
            }
        })
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

