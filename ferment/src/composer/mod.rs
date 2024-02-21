mod attrs;
mod conversion;
mod conversions;
mod ffi_bindings;
mod ffi_conversions;
mod fields;
mod item;
mod method;
mod name;
mod owned;

use std::rc::Rc;
use syn::__private::TokenStream2;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::field_type_presentation::FieldTypePresentationContext;
use crate::shared::{HasParent, SharedAccess};
pub use self::attrs::{AttrsComposer, implement_trait_for_item};
pub use self::conversion::ConversionComposer;
pub use self::conversions::ConversionsComposer;
pub use self::ffi_bindings::FFIBindingsComposer;
pub use self::ffi_conversions::ComposerAspect;
pub use self::ffi_conversions::FFIConversionComposer;
pub use self::fields::FieldsComposer;
pub use self::item::ItemComposer;
pub use self::method::MethodComposer;
pub use self::name::NameComposer;
pub use self::owned::OwnedComposer;

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
// pub type ItemComposerLocalContextPresenter = ItemComposerPresenter<OwnerIteratorLocalContext>;
pub type ItemComposerLocalConversionContextPresenter = ItemComposerPresenter<LocalConversionContext>;
pub type ItemComposerFieldTypesContextPresenter = ItemComposerPresenter<FieldTypesContext>;

pub type SimpleContextComposer<Parent> = ContextComposer<TokenStream2, TokenStream2, Parent>;
pub type SimpleItemParentContextComposer = SimpleContextComposer<ItemParentComposer>;

pub type SimpleComposerPresenter = ComposerPresenter<TokenStream2, TokenStream2>;
pub type SimplePairConversionComposer = ComposerPresenter<(TokenStream2, TokenStream2), TokenStream2>;
pub type IteratorConversionComposer = ComposerPresenter<Vec<OwnedItemPresenterContext>, IteratorPresentationContext>;
pub type OwnerIteratorConversionComposer = ComposerPresenter<OwnerIteratorLocalContext, OwnerIteratorPresentationContext>;
pub type OwnedFieldTypeComposer = ComposerPresenter<FieldTypeConversion, OwnedItemPresenterContext>;
pub type BindingComposer = ComposerPresenter<(TokenStream2, TokenStream2, TokenStream2), BindingPresentation>;
pub type FieldTypeComposer = ComposerPresenterByRef<FieldTypeConversion, FieldTypePresentationContext>;

pub type OwnerIteratorLocalContext = (TokenStream2, Vec<OwnedItemPresenterContext>);
pub type FieldTypesContext = Vec<FieldTypeConversion>;
pub type LocalConversionContext = (TokenStream2, FieldTypesContext);

pub trait Composer<Parent> where Self: Sized + HasParent<Parent> {
    type Item;
    type Source;
    fn compose(&self, source: &Self::Source) -> Self::Item;
}


// #[macro_export]
// macro_rules! composer_impl {
//     ($name:ident, $output:ty, $composition:expr) => {
//         impl<P, S> crate::composer::Composer<P, S> for $name where Self: Sized, P: Parent {
//             type Item = $output;
//             // type Parent = ParentComposer<ItemComposer>;
//             type Source = S;
//
//             fn set_parent(&mut self, root: &P) {
//                 self.parent = Some(root.to_inner());
//             }
//             #[allow(clippy::redundant_closure_call)]
//             fn compose(&self, source: &Self::Source) -> Self::Item {
//                 $composition(self, source)
//             }
//         }
//     }
// }

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
    type Item = Result;
    type Source = ScopeContext;

    fn compose(&self, _source: &Self::Source) -> Self::Item {
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
    type Item = Result;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let presentable = (self.root_composer)(context);
        let result = presentable.present(source);
        result
    }
}

