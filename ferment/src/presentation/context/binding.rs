use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composer::ConstructorPresentableContext;
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresentableContext};

pub enum BindingPresentableContext {
    // Empty,
    Constructor(ConstructorPresentableContext, Punctuated<OwnedItemPresentableContext, Comma>, IteratorPresentationContext),
    Destructor(Ident),
    // Accessor(),
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
            }
            BindingPresentableContext::Destructor(target_name) => {
                BindingPresentation::Destructor {
                    name: Name::Destructor(target_name.clone()),
                    ffi_name: quote!(#target_name)
                }
            }
        }
    }
}
