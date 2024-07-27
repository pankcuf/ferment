use quote::ToTokens;
use syn::{Attribute, Generics, Type};
use crate::ast::{DelimiterTrait, Wrapped};
use crate::composer::{CommaPunctuatedArgs, CommaPunctuatedOwnedItems};
use crate::context::ScopeContext;
use crate::presentable::{ConstructorPresentableContext, ScopeContextPresentable};
use crate::presentation::BindingPresentation;

pub type ConstructorBindingPresentableContext<I> = BindingPresentableContext<CommaPunctuatedOwnedItems, CommaPunctuatedArgs, I>;

pub enum BindingPresentableContext<S, SP, I>
    where S: ScopeContextPresentable<Presentation = SP>,
          SP: ToTokens,
          I: DelimiterTrait + ?Sized {
    Constructor(ConstructorPresentableContext, CommaPunctuatedOwnedItems, Wrapped<S, SP, I>),
    Destructor(Type, Vec<Attribute>, Option<Generics>),
}

impl<S, SP, I> ScopeContextPresentable for BindingPresentableContext<S, SP, I>
    where S: ScopeContextPresentable<Presentation = SP>, SP: ToTokens, I: DelimiterTrait + ?Sized {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            BindingPresentableContext::Constructor(context, args, body) => {
                // println!("BindingPresentableContext::Constructor: {}: \n--{}\n--{}", context, args.present(source).to_token_stream(), body.present(source));
                BindingPresentation::Constructor {
                    context: context.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation: body.present(&source),
                }
            },
            BindingPresentableContext::Destructor(ty, attrs, generics) => {
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    ty: ty.clone(),
                    generics: generics.clone()
                }
            },
        }
    }
}
