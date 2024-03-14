mod attrs;
mod ffi_conversions;
mod item;
mod method;
mod name;
pub mod constants;
pub mod chain;
pub mod enum_composer;
pub mod composable;
mod r#type;
pub mod generic;

use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Field, Type};
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
pub use composable::Composable;
use crate::composer::generic::GenericComposer;
use crate::conversion::FieldTypeConversion;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{BindingPresentableContext, FieldTypePresentableContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::context::name::Aspect;
use crate::shared::{HasParent, SharedAccess};
use crate::wrapped::Void;
pub use self::attrs::{AttrsComposer, implement_trait_for_item};
pub use self::ffi_conversions::{FFIAspect, FFIComposer};
pub use self::item::ItemComposer;
pub use self::method::MethodComposer;

#[derive(Clone)]
pub enum ConstructorPresentableContext {
    EnumVariant(Type),
    Default(Type)
}
impl Debug for ConstructorPresentableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnumVariant(ty) =>
                f.write_str(format!("EnumVariant({})", ty.to_token_stream()).as_str()),
            Self::Default(ty) =>
                f.write_str(format!("Default({})", ty.to_token_stream()).as_str()),
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
pub type SharedComposerRef<'a, Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::ImmutableAccess, &'a Context>;
#[allow(unused)]
pub type SharedComposerMut<Parent, Context> = ComposerPresenterByRef<<Parent as SharedAccess>::MutableAccess, Context>;
pub type ParentComposer<T> = Rc<std::cell::RefCell<T>>;
pub type ParentComposerRef<'a, T> = std::cell::Ref<'a, T>;
pub type ItemParentComposer = ParentComposer<ItemComposer>;
pub type EnumParentComposer = ParentComposer<EnumComposer>;
pub type GenericParentComposer = ParentComposer<GenericComposer>;
pub type ItemParentComposerRef<'a> = ParentComposerRef<'a, ItemComposer>;
pub type EnumParentComposerRef<'a> = ParentComposerRef<'a, EnumComposer>;
pub type ItemComposerPresenterRef<'a, T> = ComposerPresenterByRef<ItemParentComposerRef<'a>, T>;
pub type EnumComposerPresenterRef<'a, T> = ComposerPresenterByRef<EnumParentComposerRef<'a>, T>;
pub type ItemComposerFieldTypesContextPresenter<'a> = ItemComposerPresenterRef<'a, FieldTypesContext>;
pub type NameContextComposer<Parent> = ContextComposer<Name, TokenStream2, Parent>;
pub type TypeContextComposer<Parent> = ContextComposer<Type, TokenStream2, Parent>;

// pub type SimpleComposerPresenter = ComposerPresenter<TokenStream2, TokenStream2>;
pub type OwnerIteratorConversionComposer<T> = ComposerPresenter<OwnerAspectIteratorLocalContext<T>, OwnerIteratorPresentationContext>;
pub type OwnerIteratorPostProcessingComposer<T> = ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, T>;

pub type VariantComposer = ComposerPresenterByRef<VariantIteratorLocalContext, OwnerIteratorPresentationContext>;
pub type FieldsComposer = ComposerPresenterByRef<Punctuated<Field, Comma>, Punctuated<FieldTypeConversion, Comma>>;

// Bindings
pub type BindingComposer<T> = ComposerPresenter<T, BindingPresentation>;
pub type BindingDtorComposer = BindingComposer<DestructorContext>;


pub type FieldTypeComposer = ComposerPresenterByRef<FieldTypeConversion, FieldTypePresentableContext>;
pub type OwnedFieldTypeComposerRef = ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentableContext>;

// pub enum Owner {
//     Aspect(Aspect),
//     Ident(Ident)
// }
pub type OwnerIteratorLocalContext<A, T> = (A, Punctuated<OwnedItemPresentableContext, T>);
pub type OwnerAspectIteratorLocalContext<T> = OwnerIteratorLocalContext<Aspect, T>;
pub type VariantIteratorLocalContext = OwnerAspectIteratorLocalContext<Comma>;
// pub type VariantPresenterContext = OwnerAspectIteratorLocalContext<Comma>;

pub type FieldTypesContext = Punctuated<FieldTypeConversion, Comma>;
pub type LocalConversionContext = (Aspect, FieldTypesContext);

pub type BindingAccessorContext = (Type, TokenStream2, TokenStream2);
pub type DestructorContext = Type;

pub type FieldTypeLocalContext = (TokenStream2, FieldTypePresentableContext);

pub type FieldTypePresentationContextPassRef = ComposerPresenterByRef<FieldTypeLocalContext, FieldTypePresentableContext>;

pub type FFIConversionComposer<Parent> = ConversionComposer<
    Parent,
    LocalConversionContext,
    FieldTypeLocalContext,
    FieldTypePresentableContext,
    OwnerAspectIteratorLocalContext<Comma>,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext>;
pub type DropConversionComposer<Parent> = ConversionComposer<
    Parent,
    FieldTypesContext,
    FieldTypeLocalContext,
    FieldTypePresentableContext,
    Punctuated<OwnedItemPresentableContext, Semi>,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext,
    OwnerIteratorPresentationContext>;
pub type FieldsOwnedComposer<Parent> = OwnedComposer<
    Parent,
    LocalConversionContext,
    FieldTypeConversion,
    OwnedItemPresentableContext,
    OwnerAspectIteratorLocalContext<Comma>,
    OwnerIteratorPresentationContext
>;
// pub type EnumVariantComposer<Parent> = OwnedComposer<
//     Parent,
//     VariantLocalContext,
//     FieldTypeConversion,
//     OwnedItemPresentableContext,
//     OwnerIteratorLocalContext<Comma>,
//     OwnerIteratorPresentationContext
// >;
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

pub struct IteratorComposer<IN, CTX, MAP, OUT> where IN: Clone {
    root_composer: ComposerPresenter<(IN, ComposerPresenterByRef<CTX, MAP>), OUT>,
    item_composer: ComposerPresenterByRef<CTX, MAP>
}

impl<IN, CTX, MAP, OUT> IteratorComposer<IN, CTX, MAP, OUT>
    where IN: Clone {
    pub const fn new(
        root_composer: ComposerPresenter<(IN, ComposerPresenterByRef<CTX, MAP>), OUT>,
        item_composer: ComposerPresenterByRef<CTX, MAP>,
    ) -> Self {
        Self { root_composer, item_composer }
    }
}
impl<Parent, IN, CTX, MAP, OUT> Composer<Parent> for IteratorComposer<IN, CTX, MAP, OUT>
    where Parent: SharedAccess,
          IN: Clone {
    type Source = IN;
    type Result = OUT;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        // TODO: avoid cloning
        (self.root_composer)((source.clone(), self.item_composer))
    }
}

impl<Parent, IN, CTX, MAP, OUT> HasParent<Parent> for IteratorComposer<IN, CTX, MAP, OUT>
    where Parent: SharedAccess,
          IN: Clone {
    fn set_parent(&mut self, _parent: &Parent) {}
}

pub struct OwnedComposer<Parent, CTX, L1CTX, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    parent: Option<Parent>,
    root_composer: ComposerPresenter<L1OUT, OUT>,
    context_composer: SharedComposer<Parent, CTX>,
    local_context_composer: IteratorComposer<CTX, L1CTX, L1MAP, L1OUT>,
}

impl<Parent, CTX, L1CTX, L1MAP, L1OUT, OUT> OwnedComposer<Parent, CTX, L1CTX, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    pub const fn new(
        root_composer: ComposerPresenter<L1OUT, OUT>,
        context_composer: SharedComposer<Parent, CTX>,
        iterator_root_composer: ComposerPresenter<(CTX, ComposerPresenterByRef<L1CTX, L1MAP>), L1OUT>,
        iterator_item_composer: ComposerPresenterByRef<L1CTX, L1MAP>,
    ) -> Self {
        Self {
            parent: None,
            root_composer,
            context_composer,
            local_context_composer: IteratorComposer::new(
                iterator_root_composer,
                iterator_item_composer
            ),
        }
    }
}

impl<Parent, CTX, L1, L1MAP, L1OUT, OUT> HasParent<Parent> for OwnedComposer<Parent, CTX, L1, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    fn set_parent(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent, CTX, L1, L1MAP, L1OUT, OUT> Composer<Parent> for OwnedComposer<Parent, CTX, L1, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess,
          CTX: Clone {
    type Source = ();
    type Result = OUT;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = <IteratorComposer<CTX, L1, L1MAP, L1OUT> as Composer<Parent>>::compose(&self.local_context_composer, &context);
        let out = (self.root_composer)(local_context);
        out
    }
}

pub struct ConversionComposer<Parent, L1CTX, L2CTX, L2MAP, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        L1CTX: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    parent: Option<Parent>,
    root_composer: ComposerPresenterByRef<(CTX, LOUT), OUT>,
    context_composer: SharedComposer<Parent, CTX>,
    local_context_composer: OwnedComposer<Parent, L1CTX, L2CTX, L2MAP, L1OUT, LOUT>,
}
impl<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT> HasParent<Parent> for ConversionComposer<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        C0: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    fn set_parent(&mut self, parent: &Parent) {
        self.local_context_composer.set_parent(parent);
        self.parent = Some(parent.clone_container());
    }
}
impl<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT> Composer<Parent> for ConversionComposer<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        C0: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    type Source = ();
    type Result = OUT;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.root_composer)(&(
            self.parent.as_ref().unwrap().access(self.context_composer),
            self.local_context_composer.compose(&())))
    }
}
impl<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT> ConversionComposer<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        C0: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    pub const fn new(
        root_composer: ComposerPresenterByRef<(CTX, LOUT), OUT>,
        context_composer: SharedComposer<Parent, CTX>,
        local_root_composer: ComposerPresenter<L1OUT, LOUT>,
        local_context_composer: SharedComposer<Parent, C0>,
        local_context_iterator_item_composer: ComposerPresenterByRef<C1, C2>,
        local_context_iterator_root_composer: ComposerPresenter<(C0, ComposerPresenterByRef<C1, C2>), L1OUT>,
    ) -> Self {
        Self {
            parent: None,
            root_composer,
            context_composer,
            local_context_composer: OwnedComposer::new(
                local_root_composer,
                local_context_composer,
                local_context_iterator_root_composer,
                local_context_iterator_item_composer
            )
        }
    }
}

// pub struct PresentableContextComposer<Context, Result, Presentable: ScopeContextPresentable<Presentation = Result>, Parent: SharedAccess> {
//     parent: Option<Parent>,
//     root_composer: ComposerPresenter<Context, Presentable>,
//     context_composer: SharedComposer<Parent, Context>,
// }
// impl<Context, Result, Presentable: ScopeContextPresentable<Presentation = Result>, Parent: SharedAccess> HasParent<Parent> for PresentableContextComposer<Context, Result, Presentable, Parent>  {
//     fn set_parent(&mut self, parent: &Parent) {
//         self.parent = Some(parent.clone_container());
//     }
// }
//
// impl<Context, Result, Presentable: ScopeContextPresentable<Presentation = Result>, Parent: SharedAccess> Composer<Parent> for PresentableContextComposer<Context, Result, Presentable, Parent> {
//     type Source = ScopeContext;
//     type Result = Result;
//
//     fn compose(&self, source: &Self::Source) -> Self::Result {
//         let parent = self.parent.as_ref().unwrap();
//         let context = parent.access(self.context_composer);
//         let presentable = (self.root_composer)(context);
//         let result = presentable.present(source);
//         result
//     }
// }

// pub trait ComposerAspect {
//     type Context;
//     type Presentation;
//     fn compose_with_composer<Parent, C>(&self, composer: &C, context: &Self::Context) -> Self::Presentation
//         where Parent: SharedAccess, C: Composer<Parent> {
//         composer.compose(context)
//     }
// }

