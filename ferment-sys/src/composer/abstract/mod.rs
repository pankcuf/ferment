mod composable;
mod context;
mod iterative;
mod linked;
mod sequence;
mod sequence_mixer;

#[allow(unused)]
mod new;

use syn::{Field, Item, Meta, NestedMeta, Path, Type, Visibility, VisPublic};
use syn::token::Pub;
use crate::ast::{CommaPunctuated, PathHolder, TypeHolder};
use crate::composable::CfgAttributes;
use crate::composer::{ItemComposerWrapper, ComposerLink, SigComposer};
use crate::composer::type_alias::TypeAliasComposer;
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::MacroType;
use crate::ext::{CrateExtension, ItemExtension, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{TypeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Aspect, Expression};
use crate::presentation::RustFermentate;
pub use self::composable::*;
pub use self::context::*;
pub use self::iterative::*;
pub use self::linked::*;
pub use self::sequence::*;
pub use self::sequence_mixer::*;

pub trait MaybeMacroLabeled {
    fn maybe_macro_labeled(&self) -> Option<MacroType>;
}

pub trait MaybeComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ComposerLink<ScopeContext>) -> Option<ItemComposerWrapper<LANG, SPEC>>;
}

pub trait Composer<'a> {
    type Source;
    type Output;
    fn compose(&self, source: &'a Self::Source) -> Self::Output;
}

impl MaybeMacroLabeled for Item {
    fn maybe_macro_labeled(&self) -> Option<MacroType> {
        self.maybe_attrs()
            .and_then(|attrs| attrs.iter().find_map(|attr| {
                let path = &attr.path;
                let mut arguments = Vec::<Path>::new();
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    meta_list.nested
                        .into_iter()
                        .for_each(|meta| if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            arguments.push(path);
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
    }
}

impl<SPEC> MaybeComposer<RustFermentate, SPEC> for Item
    where SPEC: RustSpecification {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ComposerLink<ScopeContext>) -> Option<ItemComposerWrapper<RustFermentate, SPEC>> {
        self.maybe_macro_labeled()
            .and_then(|macro_type| {
                let source = scope_context.borrow();
                let crate_ident = source.scope.crate_ident_as_path();
                match (macro_type, self) {
                    (MacroType::Opaque, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::opaque_struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes()), scope_context)),
                    (MacroType::Export, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::r#struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes()), scope_context)),
                    (MacroType::Export, Item::Enum(item)) =>
                        Some(ItemComposerWrapper::r#enum(item, TypeContext::r#enum(&item.ident, item.attrs.cfg_attributes()), scope_context)),
                    (MacroType::Export, Item::Type(item)) => match &*item.ty {
                        Type::BareFn(type_bare_fn) =>
                            Some(ItemComposerWrapper::Sig(SigComposer::from_type_bare_fn(TypeContext::callback(scope.self_path().crate_named(&scope.crate_ident_as_path()), &item.ident, type_bare_fn, &item.attrs.cfg_attributes()), &item.generics, &item.attrs, scope_context))),
                        _ => {
                            let fields = CommaPunctuated::from_iter([Field {
                                vis: Visibility::Public(VisPublic { pub_token: Pub::default() }),
                                ty: *item.ty.clone(),
                                attrs: vec![],
                                ident: None,
                                colon_token: None,
                            }]);
                            Some(ItemComposerWrapper::TypeAlias(TypeAliasComposer::new(TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes()), &item.attrs, &item.generics, &fields, scope_context)))
                        }
                    },
                    (MacroType::Export, Item::Fn(item)) =>
                        Some(ItemComposerWrapper::r#fn(item, TypeContext::mod_fn(scope.self_path().crate_named(&crate_ident), item), scope_context)),
                    (MacroType::Export, Item::Trait(item)) =>
                        Some(ItemComposerWrapper::r#trait(item, TypeContext::r#trait(item), scope, scope_context)),
                    (MacroType::Export, Item::Impl(item)) => {
                        let mut full_fn_path = scope.self_path_holder();
                        if full_fn_path.is_crate_based() {
                            full_fn_path.replace_first_with(&PathHolder::from(scope.crate_ident_ref().to_path()));
                        }
                        Some(ItemComposerWrapper::r#impl(item, TypeContext::r#impl(full_fn_path.0, item.attrs.cfg_attributes()), scope, scope_context))
                    }
                    _ => None
                }
            })

    }
}
