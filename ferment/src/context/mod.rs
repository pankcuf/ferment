mod context;
mod crate_type;
mod global_context;
mod scope_chain;
mod scope_context;
pub(crate) mod scope;
mod traits_resolver;
mod scope_resolver;
mod import_resolver;
mod custom_resolver;
mod generic_resolver;
mod scope_propagation;
mod type_chain;
mod multi_crate_context;
mod attrs_resolver;

// pub use self::crate_type::Crate;
// pub use self::context::Context;
pub use self::custom_resolver::CustomResolver;
pub use self::generic_resolver::GenericResolver;
pub use self::import_resolver::ImportResolver;

pub use self::global_context::GlobalContext;
pub use self::scope::Scope;
pub use self::scope_context::ScopeContext;
pub use self::scope_chain::{ScopeChain, ScopeInfo};
pub use self::scope_resolver::{ScopeRefinement, ScopeResolver};
pub use self::traits_resolver::TraitsResolver;
// pub use self::type_chain::DefaultScopePolicy;
pub use self::type_chain::EnrichScopePolicy;
pub use self::type_chain::TypeChain;
// pub use self::type_chain::TypeChainKey;
