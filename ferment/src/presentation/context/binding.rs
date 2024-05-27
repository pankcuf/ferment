use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Type;
use crate::composer::{CommaPunctuated, ConstructorPresentableContext};
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresentableContext};

pub enum BindingPresentableContext {
    Constructor(ConstructorPresentableContext, CommaPunctuated<OwnedItemPresentableContext>, IteratorPresentationContext),
    Destructor(Type, TokenStream2),
}

impl ScopeContextPresentable for BindingPresentableContext {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            BindingPresentableContext::Constructor(context, args, field_names) => {
                BindingPresentation::Constructor {
                    context: context.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation: field_names.present(&source),
                }
            },
            BindingPresentableContext::Destructor(ty, attrs) => {
                BindingPresentation::Destructor {
                    attrs: attrs.to_token_stream(),
                    name: Name::Destructor(ty.clone()),
                    ffi_name: ty.clone()
                }
            },
        }
    }
}
