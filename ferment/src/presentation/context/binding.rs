use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Generics, Type};
use crate::composer::{CommaPunctuatedOwnedItems, ConstructorPresentableContext};
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::wrapped::{DelimiterTrait, Wrapped};

pub enum BindingPresentableContext<S, SP, I>
    where S: ScopeContextPresentable<Presentation = SP>,
          SP: ToTokens,
          I: DelimiterTrait + ?Sized {
    Constructor(ConstructorPresentableContext, CommaPunctuatedOwnedItems, Wrapped<S, SP, I>),
    Destructor(Type, TokenStream2, Option<Generics>),
}

impl<S, SP, I> ScopeContextPresentable for BindingPresentableContext<S, SP, I>
    where S: ScopeContextPresentable<Presentation = SP>, SP: ToTokens, I: DelimiterTrait + ?Sized {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            BindingPresentableContext::Constructor(context, args, body) => {
                BindingPresentation::Constructor {
                    context: context.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation: body.present(&source),
                }
            },
            BindingPresentableContext::Destructor(ty, attrs, generics) => {
                BindingPresentation::Destructor {
                    attrs: attrs.to_token_stream(),
                    name: Name::Destructor(ty.clone()),
                    ffi_name: ty.clone(),
                    generics: generics.clone()
                }
            },
        }
    }
}
