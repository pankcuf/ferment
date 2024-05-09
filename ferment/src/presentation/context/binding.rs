use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Type;
use crate::composer::ConstructorPresentableContext;
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresentableContext};

pub enum BindingPresentableContext {
    // Empty,
    // Constructor(ConstructorPresentableContext, Punctuated<OwnedItemPresentableContext, Comma>, Wrapped<, >),
    Constructor(ConstructorPresentableContext, Punctuated<OwnedItemPresentableContext, Comma>, IteratorPresentationContext),
    Destructor(Type),
    // Accessor(LocalConversionContext)
    // Accessor(),
    // Getter(),
    // Setter()
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
            BindingPresentableContext::Destructor(ty) => {
                let ffi_name = ty.to_token_stream();
                BindingPresentation::Destructor {
                    attrs: Default::default(),
                    name: Name::Destructor(ty.clone()),
                    ffi_name
                }
            },
            // BindingPresentableContext::Getter() => {
            //     BindingPresentation::Getter {
            //         name: Name::Getter(parse_quote!(#root_obj_type), parse_quote!(#field_name)),
            //         field_name: field_name.to_token_stream(),
            //         obj_type: root_obj_type.to_token_stream(),
            //         field_type: field_type.to_token_stream()
            //     }
            // },
            // BindingPresentableContext::Setter() => {
            //
            // }
        }
    }
}
