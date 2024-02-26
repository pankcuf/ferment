mod attrs;
mod conversion;
mod conversions;
mod ffi_conversions;
mod item;
mod method;
mod name;
mod owned;

use std::rc::Rc;
use proc_macro2::Ident;
use syn::__private::TokenStream2;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
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
pub use self::item::{ItemComposer, BYPASS_FIELD_CONTEXT, FIELD_TYPES_COMPOSER, composer_ctor, enum_variant_composer_ctor_unnamed, enum_variant_composer_ctor_named, enum_variant_composer_ctor_unit, struct_composer_ctor_named, struct_composer_ctor_unnamed};
pub use self::method::MethodComposer;
pub use self::name::NameComposer;
pub use self::owned::OwnedComposer;

#[derive(Clone, Debug)]
pub enum ConstructorPresentableContext {
    EnumVariant(TokenStream2, Ident, TokenStream2),
    Default(Ident)
}
// impl ScopeContextPresentable for ConstructorPresentableContext {
//     type Presentation = (Name, Box<Type>, TokenStream2);
//
//     fn present(&self, source: &ScopeContext) -> Self::Presentation {
//         match self {
//             ConstructorPresentableContext::EnumVariant(ffi_ident, ffi_variant_ident, ffi_variant_path) => {
//                 (Name::Constructor(ffi_variant_ident.clone()), parse_quote!(*mut #ffi_ident), ffi_variant_path.to_token_stream())
//             }
//             ConstructorPresentableContext::Default(ffi_ident) => {
//                 (Name::Constructor(ffi_ident.clone()), parse_quote!(*mut #ffi_ident), ffi_ident.to_token_stream())
//             }
//         }
//     }
// }

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
pub type ItemParentComposerRef = ParentComposerRef<ItemComposer>;
pub type ItemComposerPresenter<T> = ComposerPresenterByRef<ItemParentComposerRef, T>;
pub type ItemComposerTokenStreamPresenter = ItemComposerPresenter<TokenStream2>;
pub type ItemComposerLocalConversionContextPresenter = ItemComposerPresenter<LocalConversionContext>;
pub type ItemComposerFieldTypesContextPresenter = ItemComposerPresenter<FieldTypesContext>;
pub type ItemComposerBindingCtorContextPresenter = ItemComposerPresenter<BindingCtorContext>;

pub type SimpleContextComposer<Parent> = ContextComposer<TokenStream2, TokenStream2, Parent>;
pub type SimpleItemParentContextComposer = SimpleContextComposer<ItemParentComposer>;

pub type SimpleComposerPresenter = ComposerPresenter<TokenStream2, TokenStream2>;
pub type SimplePairConversionComposer = ComposerPresenter<(TokenStream2, TokenStream2), TokenStream2>;
pub type IteratorConversionComposer = ComposerPresenter<Vec<OwnedItemPresentableContext>, IteratorPresentationContext>;
pub type OwnerIteratorConversionComposer = ComposerPresenter<OwnerIteratorLocalContext, OwnerIteratorPresentationContext>;
pub type OwnerIteratorPostProcessingComposer = ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, ItemParentComposer>;

// Bindings
pub type BindingComposer<T> = ComposerPresenter<T, BindingPresentation>;
pub type BindingAccessorComposer = BindingComposer<BindingAccessorContext>;
pub type BindingCtorComposer = BindingComposer<(ConstructorPresentableContext, CtorArgsAndBodyContext)>;
// pub type BindingEnumVariantCtorComposer = BindingComposer<(ConstructorContext, CtorArgsAndBodyContext)>;
pub type BindingDtorComposer = BindingComposer<DestructorContext>;


pub type FieldTypeComposer = ComposerPresenterByRef<FieldTypeConversion, FieldTypePresentableContext>;
pub type OwnedFieldTypeComposerRef = ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentableContext>;

pub type OwnerIteratorLocalContext = (TokenStream2, Vec<OwnedItemPresentableContext>);
pub type FieldTypesContext = Vec<FieldTypeConversion>;
pub type LocalConversionContext = (TokenStream2, FieldTypesContext);
pub type BindingAccessorContext = (TokenStream2, TokenStream2, TokenStream2);
pub type BindingCtorContext = (ConstructorPresentableContext, FieldTypesContext);
pub type CtorArgsAndBodyContext = (Vec<TokenStream2>, TokenStream2);
pub type DestructorContext = (Ident, TokenStream2);

pub type FieldTypeLocalContext = (TokenStream2, FieldTypePresentableContext);
pub type FieldTypePresentationContextPassRef = ComposerPresenterByRef<FieldTypeLocalContext, FieldTypePresentableContext>;

pub type FFIConversionComposer<Parent> = ConversionComposer<
    Parent,
    LocalConversionContext,
    FieldTypeLocalContext,
    FieldTypePresentableContext,
    (TokenStream2, Vec<OwnedItemPresentableContext>),
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext>;
pub type DropConversionComposer<Parent> = ConversionComposer<
    Parent,
    FieldTypesContext,
    FieldTypeLocalContext,
    FieldTypePresentableContext,
    Vec<OwnedItemPresentableContext>,
    IteratorPresentationContext,
    OwnerIteratorPresentationContext,
    IteratorPresentationContext>;

// pub type StructConstructorConversionComposer<Parent> = ConversionComposer<
//     Parent,
//     FieldTypesContext,
//     FieldTypeConversion,
//     OwnedItemPresentableContext,
//     Vec<OwnedItemPresentableContext>,
//     IteratorPresentationContext,
//     OwnerIteratorPresentationContext,
//     BindingPresentation>;
//
//
pub type FieldsOwnedComposer<Parent> = OwnedComposer<
    Parent,
    LocalConversionContext,
    FieldTypeConversion,
    OwnedItemPresentableContext,
    (TokenStream2, Vec<OwnedItemPresentableContext>),
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

// pub type CtorOwnedComposer2<Parent, CtorContext> = OwnedComposer<
//     Parent,
//     (CtorContext, FieldTypesContext),
//     FieldTypeConversion,
//     (OwnedItemPresentableContext, OwnedItemPresentableContext),
//     (CtorContext, Vec<(OwnedItemPresentableContext, OwnedItemPresentableContext)>),
//     BindingPresentation
// >;




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