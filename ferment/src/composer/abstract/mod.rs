mod composable;
mod context;
mod iterative;
mod linked;
mod sequence;
mod sequence_mixer;

#[allow(unused)]
mod new;
#[allow(unused)]
mod new_const;

use syn::{Item, Meta, NestedMeta, Path};
use crate::ast::TypeHolder;
use crate::composer::{ItemComposerWrapper, ComposerLink};
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::MacroType;
use crate::ext::{ItemExtension, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
pub use self::composable::*;
pub use self::context::*;
pub use self::iterative::*;
pub use self::linked::*;
pub use self::sequence::*;
pub use self::sequence_mixer::*;

pub trait MaybeComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ComposerLink<ScopeContext>) -> Option<ItemComposerWrapper<LANG, SPEC, Gen>>;
}

pub trait Composer<'a> {
    type Source;
    type Output;
    fn compose(&self, source: &'a Self::Source) -> Self::Output;
}

impl<LANG, SPEC, Gen> MaybeComposer<LANG, SPEC, Gen> for Item
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ComposerLink<ScopeContext>) -> Option<ItemComposerWrapper<LANG, SPEC, Gen>> {
        self.maybe_attrs()
            .and_then(|attrs| attrs.iter().find_map(|attr| {
                let path = &attr.path;
                let mut arguments = Vec::<Path>::new();
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    meta_list.nested.iter().for_each(|meta| {
                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            arguments.push(path.clone());
                        }
                    });
                }
                match path.segments.last().unwrap().ident.to_string().as_str() {
                    "export" =>
                        Some(MacroType::Export),
                    "opaque" =>
                        Some(MacroType::Opaque),
                    "register" =>
                        Some(MacroType::Register(TypeHolder(arguments.first().unwrap().to_type()))),
                    _ =>
                        None
                }
            }))
            .and_then(|macro_type| match (macro_type, self) {
                (MacroType::Opaque, Item::Struct(item)) =>
                    Some(ItemComposerWrapper::opaque_struct(item, scope_context)),
                (MacroType::Export, Item::Struct(item)) =>
                    Some(ItemComposerWrapper::r#struct(item, scope_context)),
                (MacroType::Export, Item::Enum(item)) =>
                    Some(ItemComposerWrapper::r#enum(item, scope_context)),
                (MacroType::Export, Item::Type(item)) =>
                    Some(ItemComposerWrapper::r#type(item, scope, scope_context)),
                (MacroType::Export, Item::Fn(item)) =>
                    Some(ItemComposerWrapper::r#fn(item, scope, scope_context)),
                (MacroType::Export, Item::Trait(item)) =>
                    Some(ItemComposerWrapper::r#trait(item, scope, scope_context)),
                (MacroType::Export, Item::Impl(item)) =>
                    Some(ItemComposerWrapper::r#impl(item, scope, scope_context)),
                _ => None
            })

    }
}