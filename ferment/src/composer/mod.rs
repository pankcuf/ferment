mod attrs;
mod conversion;
mod conversions;
mod ffi_conversions;
mod item;
mod method;
mod name;
mod owned;
pub mod constants;
pub mod chain;
pub mod enum_composer;
pub mod parent_composer;
mod r#type;

use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
pub use constants::BYPASS_FIELD_CONTEXT;
pub use constants::composer_ctor;
pub use constants::enum_variant_composer_ctor_named;
pub use constants::enum_variant_composer_ctor_unit;
pub use constants::enum_variant_composer_ctor_unnamed;
pub use constants::FIELD_TYPES_COMPOSER;
pub use constants::struct_composer_ctor_named;
pub use constants::struct_composer_ctor_unnamed;
pub use enum_composer::EnumComposer;
pub use parent_composer::IParentComposer;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::helper::Void;
use crate::naming::Name;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::context::FieldTypePresentableContext;
use crate::shared::{HasParent, SharedAccess};
pub use self::attrs::{AttrsComposer, implement_trait_for_item};
pub use self::conversion::ConversionComposer;
pub use self::conversions::ConversionsComposer;
pub use self::ffi_conversions::FFIAspect;
pub use self::ffi_conversions::FFIComposer;
pub use self::item::ItemComposer;
pub use self::method::MethodComposer;
pub use self::name::NameComposer;
pub use self::owned::OwnedComposer;

#[derive(Clone)]
pub enum ConstructorPresentableContext {
    EnumVariant(Name, TokenStream2, TokenStream2),
    Default(Name, Name)
}
impl Debug for ConstructorPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnumVariant(ffi_variant_name, enum_path, variant_path) =>
                f.write_str(format!("EnumVariant({}, {}, {})", ffi_variant_name, enum_path, variant_path.to_token_stream()).as_str()),
            Self::Default(name, name2) =>
                f.write_str(format!("Default({}, {})", name, name2).as_str()),
        }
    }
}
impl Display for ConstructorPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}



/// Composer Context Presenters
#[allow(unused)]
pub type ComposerPresenter<Context, Presentation> = fn(context: Context) -> Presentation;
#[allow(unused)]
pub type ComposerPresenterByRef<Context, Presentation> = fn(context: &Context) -> Presentation;
#[allow(unused)]
pub type SharedComposer<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, Context>;
#[allow(unused)]
pub type SharedComposerMut<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::MutableAccess, Context>;
pub type ParentComposer<T> = Rc<std::cell::RefCell<T>>;
pub type ParentComposerRef<T> = std::cell::Ref<'static, T>;
pub type ItemParentComposer = ParentComposer<ItemComposer>;
pub type EnumParentComposer = ParentComposer<EnumComposer>;
pub type ItemParentComposerRef = ParentComposerRef<ItemComposer>;
pub type EnumParentComposerRef = ParentComposerRef<EnumComposer>;
pub type ItemComposerPresenterRef<T> = ComposerPresenterByRef<ItemParentComposerRef, T>;
pub type EnumComposerPresenterRef<T> = ComposerPresenterByRef<EnumParentComposerRef, T>;
// pub type ItemComposerTokenStreamPresenter = ItemComposerPresenterRef<TokenStream2>;
pub type ItemComposerLocalConversionContextPresenter = ItemComposerPresenterRef<LocalConversionContext>;
pub type ItemComposerFieldTypesContextPresenter = ItemComposerPresenterRef<FieldTypesContext>;
// pub type SimpleContextComposer<Parent> = ContextComposer<TokenStream2, TokenStream2, Parent>;
pub type NameContextComposer<Parent> = ContextComposer<Name, TokenStream2, Parent>;
// pub type SimpleItemParentContextComposer = SimpleContextComposer<ItemParentComposer>;

pub type SimpleComposerPresenter = ComposerPresenter<TokenStream2, TokenStream2>;
pub type SimplePairConversionComposer = ComposerPresenter<(TokenStream2, TokenStream2), TokenStream2>;
// pub type IteratorConversionComposer = ComposerPresenter<Punctuated<OwnedItemPresentableContext, Semi>, IteratorPresentationContext>;
pub type OwnerIteratorConversionComposer<T> = ComposerPresenter<OwnerIteratorLocalContext<T>, OwnerIteratorPresentationContext>;
pub type OwnerIteratorPostProcessingComposer<T> = ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, T>;

pub type VariantComposer = ComposerPresenter<OwnerIteratorLocalContext<Comma>, OwnerIteratorPresentationContext>;

// Bindings
pub type BindingComposer<T> = ComposerPresenter<T, BindingPresentation>;
pub type BindingDtorComposer = BindingComposer<DestructorContext>;


pub type FieldTypeComposer = ComposerPresenterByRef<FieldTypeConversion, FieldTypePresentableContext>;
pub type OwnedFieldTypeComposerRef = ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentableContext>;

pub type OwnerIteratorLocalContext<T> = (Name, Punctuated<OwnedItemPresentableContext, T>);
pub type FieldTypesContext = Vec<FieldTypeConversion>;
pub type LocalConversionContext = (Name, FieldTypesContext);
pub type BindingAccessorContext = (Name, TokenStream2, TokenStream2);
pub type DestructorContext = (Name, Name);

pub type FieldTypeLocalContext = (TokenStream2, FieldTypePresentableContext);
pub type FieldTypePresentationContextPassRef = ComposerPresenterByRef<FieldTypeLocalContext, FieldTypePresentableContext>;

pub type FFIConversionComposer<Parent> = ConversionComposer<
    Parent,
    LocalConversionContext,
    FieldTypeLocalContext,
    FieldTypePresentableContext,
    OwnerIteratorLocalContext<Comma>,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext>;
pub type DropConversionComposer<Parent> = ConversionComposer<
    Parent,
    FieldTypesContext,
    FieldTypeLocalContext,
    FieldTypePresentableContext,
    Punctuated<OwnedItemPresentableContext, Semi>,
    IteratorPresentationContext,
    OwnerIteratorPresentationContext,
    IteratorPresentationContext>;
pub type FieldsOwnedComposer<Parent> = OwnedComposer<
    Parent,
    LocalConversionContext,
    FieldTypeConversion,
    OwnedItemPresentableContext,
    OwnerIteratorLocalContext<Comma>,
    OwnerIteratorPresentationContext
>;
pub type CtorOwnedComposer<Parent> = OwnedComposer<
    Parent,
    (ConstructorPresentableContext, FieldTypesContext), // FieldTypesContext,
    FieldTypeConversion,
    (OwnedItemPresentableContext, OwnedItemPresentableContext),
    (ConstructorPresentableContext, Vec<(OwnedItemPresentableContext, OwnedItemPresentableContext)>),
    BindingPresentableContext
>;

pub type Depunctuated<T> = Punctuated<T, Void>;


pub trait Composer<Parent> where Self: Sized + HasParent<Parent> {
    type Source;
    type Result;
    fn compose(&self, source: &Self::Source) -> Self::Result;
}

pub struct ContextComposer<Context, Result, Parent> where Parent: SharedAccess {
    parent: Option<Parent>,
    root_presenter: ComposerPresenter<Context, Result>,
    context_composer: SharedComposer<Parent, Context>,
}

impl<Context, Result, Parent> ContextComposer<Context, Result, Parent> where Parent: SharedAccess {
    pub const fn new(
        root_presenter: ComposerPresenter<Context, Result>,
        context_composer: SharedComposer<Parent, Context>) -> Self {
        Self { parent: None, root_presenter, context_composer }
    }
}

impl<Context, Result, Parent> HasParent<Parent> for ContextComposer<Context, Result, Parent> where Parent: SharedAccess {
    fn set_parent(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Context, Result, Parent> Composer<Parent> for ContextComposer<Context, Result, Parent> where Parent: SharedAccess {
    type Source = ();
    type Result = Result;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        (self.root_presenter)(context)
    }
}

pub struct PresentableContextComposer<Context, Result, Presentable: ScopeContextPresentable<Presentation = Result>, Parent: SharedAccess> {
    parent: Option<Parent>,
    root_composer: ComposerPresenter<Context, Presentable>,
    context_composer: SharedComposer<Parent, Context>,
}
impl<Context, Result, Presentable: ScopeContextPresentable<Presentation = Result>, Parent: SharedAccess> HasParent<Parent> for PresentableContextComposer<Context, Result, Presentable, Parent>  {
    fn set_parent(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Context, Result, Presentable: ScopeContextPresentable<Presentation = Result>, Parent: SharedAccess> Composer<Parent> for PresentableContextComposer<Context, Result, Presentable, Parent> {
    type Source = ScopeContext;
    type Result = Result;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let presentable = (self.root_composer)(context);
        let result = presentable.present(source);
        result
    }
}

// pub trait ComposerAspect {
//     type Context;
//     type Presentation;
//     fn compose_with_composer<Parent, C>(&self, composer: &C, context: &Self::Context) -> Self::Presentation
//         where Parent: SharedAccess, C: Composer<Parent> {
//         composer.compose(context)
//     }
// }