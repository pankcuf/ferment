mod item_conversion;
mod type_composition_conversion;
mod macro_conversion;
mod object_conversion;
mod scope_item_conversion;
mod local_type_conversion;
mod generic_type_conversion;
mod type_conversion;
mod opaque_conversion;
mod dictionary_conversion;

use crate::ast::Depunctuated;
use crate::composer::ComposerLink;
use crate::context::ScopeContext;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentation::Fermentate;
pub use self::generic_type_conversion::*;
pub use self::macro_conversion::*;
pub use self::object_conversion::*;
pub use self::scope_item_conversion::*;
pub use self::type_conversion::*;
pub use self::type_composition_conversion::*;

pub trait Ferment {
    fn ferment(&self, scope_context: &ComposerLink<ScopeContext>) -> Depunctuated<Fermentate>;
}


#[allow(unused)]
pub trait Specification: Clone + Default {
    type Fermentate: Clone;
    type Attr: LangAttrSpecification<Self::Fermentate>;
    type Gen: LangGenSpecification<Self::Fermentate>;
}