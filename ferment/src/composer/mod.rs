mod accessors;
mod attrs;
mod conversion;
mod conversions;
mod drop;
mod ffi_bindings;
mod ffi_context;
mod ffi_conversions;
mod fields;
mod item;
mod method;
mod name;

pub use self::attrs::{AttrsComposer, implement_trait_for_item};
pub use self::conversion::ConversionComposer;
pub use self::conversions::ConversionsComposer;
pub use self::drop::DropComposer;
pub use self::ffi_bindings::FFIBindingsComposer;
pub use self::ffi_context::FFIContextComposer;
pub use self::ffi_conversions::ComposerAspect;
pub use self::ffi_conversions::FFIConversionComposer;
pub use self::fields::FieldsComposer;
pub use self::item::ItemComposer;
pub use self::name::NameComposer;

/// Composer Context Presenters
pub type ComposerPresenter<C, P> = fn(context: &C) -> P;



pub trait Composer where Self: Sized {
    // type Item: ToTokens;
    type Item;
    fn set_parent(&mut self, root: &std::rc::Rc<std::cell::RefCell<ItemComposer>>);
    fn compose(&self, context: &crate::context::ScopeContext) -> Self::Item;
}

#[macro_export]
macro_rules! composer_impl {
    ($name:ident, $output:ty, $composition:expr) => {
        impl crate::composer::Composer for $name {
            type Item = $output;
            fn set_parent(&mut self, root: &std::rc::Rc<std::cell::RefCell<ItemComposer>>) {
                self.parent = Some(Rc::clone(root));
            }
            #[allow(clippy::redundant_closure_call)]
            fn compose(&self, context: &crate::context::ScopeContext) -> Self::Item {
                $composition(self, context)
            }
        }
    }
}
