use quote::ToTokens;
use crate::context::ScopeContext;

pub trait ScopeContextPresentable {
    type Presentation: ToTokens;
    fn present(&self, context: &ScopeContext) -> Self::Presentation;
}

// impl<R: ToTokens, T: ScopeContextPresentable<Presentation=R>> ScopeContextPresentable for RepIteratorExt<T> {
//     type Presentation = RepIteratorExt<R>;
//
//     fn present(&self, context: &ScopeContext) -> Self::Presentation {
//         self.iter().map(|item| item.present(context)).collect()
//     }
// }
// impl<T: ScopeContextPresentable, I: RepIteratorExt<Item=T>> ScopeContextPresentable for I where T::Presentation: RepIteratorExt<Item=T::Presentation> {
//     type Presentation = Box<dyn RepIteratorExt<Item=T::Presentation>>;
//
//     fn present(&self, context: &ScopeContext) -> Self::Presentation {
//         self.iter().map(|item| item.present(context)).collect()
//     }
// }
//
//
// impl<T, I> ScopeContextPresentable for I
//     where
//         T: ScopeContextPresentable,
//         I: IntoIterator<Item = T> + 'static,
//         T::Presentation: ToTokens,
// {
//     type Presentation = Box<dyn Iterator<Item = T::Presentation>>;
//
//     fn present(&self, context: &ScopeContext) -> Self::Presentation {
//         Box::new(self.into_iter().map(move |item| item.present(context)))
//     }
// }
