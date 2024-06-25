use syn::Generics;
use crate::composer::{GenericParentComposer, NameContextComposer, OwnerIteratorPostProcessingComposer, ParentComposer};
use crate::composer::r#type::TypeComposer;
use crate::context::ScopeContext;

#[allow(unused)]
pub struct GenericComposer {
    pub context: ParentComposer<ScopeContext>,
    pub doc_composer: NameContextComposer<GenericParentComposer>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<GenericParentComposer>,
    pub type_composer: TypeComposer<GenericParentComposer>,
    pub generics: Option<Generics>,
}

impl GenericComposer {}
