use quote::quote;
use syn::{Attribute, Generics, Type};
use syn::__private::TokenStream2;
use crate::composer::{BindingAccessorContext, CommaPunctuatedOwnedItems, DestructorContext, FunctionContext};
use crate::context::ScopeContext;
use crate::ext::ToPath;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{OwnedItemPresentableContext, ScopeContextPresentable};
use crate::presentation::{BindingPresentation, Name, RustFermentate};

// pub type ConstructorBindingPresentableContext<LANG, SPEC> =
//     BindingPresentableContext<
//         // CommaPunctuatedOwnedItems<LANG, SPEC>,
//         // <CommaPunctuatedOwnedItems<LANG, SPEC> as ScopeContextPresentable>::Presentation,
//         // I,
//         LANG,
//         SPEC>;
//
pub enum BindingPresentableContext<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    Constructor(
        Type, SPEC, Gen,
        bool,
        CommaPunctuatedOwnedItems<LANG, SPEC>,
        CommaPunctuatedOwnedItems<LANG, SPEC>
    ),
        // Wrapped<CommaPunctuatedOwnedItems<LANG, SPEC>, <CommaPunctuatedOwnedItems<LANG, SPEC> as ScopeContextPresentable>::Presentation, I>),
    // Constructor(
    //     ConstructorPresentableContext<LANG, SPEC>,
    //     CommaPunctuatedOwnedItems<LANG, SPEC>,
    //     Wrapped<CommaPunctuatedOwnedItems<LANG, SPEC>, <CommaPunctuatedOwnedItems<LANG, SPEC> as ScopeContextPresentable>::Presentation, I>),
    VariantConstructor(
        Type, SPEC, Gen,
        bool,
        CommaPunctuatedOwnedItems<LANG, SPEC>,
        CommaPunctuatedOwnedItems<LANG, SPEC>),
        // Wrapped<CommaPunctuatedOwnedItems<LANG, SPEC>, <CommaPunctuatedOwnedItems<LANG, SPEC> as ScopeContextPresentable>::Presentation, I>),
    Destructor(Type, SPEC, Gen),
    Getter(Type, Type, TokenStream2, SPEC, Gen),
    Setter(Type, Type, TokenStream2, SPEC, Gen),
}

impl<LANG, SPEC, Gen> BindingPresentableContext<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn ctor(context: FunctionContext<LANG, SPEC, Gen>) -> Self {
        let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
        let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
        // Wrapped::<_, _, I>::new(names)
        BindingPresentableContext::Constructor(ffi_type, attrs, generics, is_round, args, names)
        // BindingPresentableContext::Constructor(ffi_type, attrs, generics, args, Wrapped::<_, _, I>::new(names))
    }
    // pub fn ctor(
    //     context: DestructorContext<LANG, SPEC>,
    //     items: CommaPunctuatedOwnedItems<LANG, SPEC>,
    //     wrapped: Wrapped<CommaPunctuatedOwnedItems<LANG, SPEC>, <CommaPunctuatedOwnedItems<LANG, SPEC> as ScopeContextPresentable>::Presentation, I>) -> Self {
    //     let (ffi_type, attrs, generics, ..) = context;
    //     BindingPresentableContext::Constructor(ffi_type, attrs, generics, items, wrapped)
    // }
    pub fn variant_ctor(context: FunctionContext<LANG, SPEC, Gen>) -> Self {
        let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
        let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::VariantConstructor(ffi_type, attrs, generics, is_round, args, names)
    }
    // pub fn ctor2(
    //     context: ConstructorPresentableContext<LANG, SPEC>,
    //     items: CommaPunctuatedOwnedItems<LANG, SPEC>,
    //     wrapped: Wrapped<CommaPunctuatedOwnedItems<LANG, SPEC>, <CommaPunctuatedOwnedItems<LANG, SPEC> as ScopeContextPresentable>::Presentation, I>) -> Self {
    //     BindingPresentableContext::Constructor(context, items, wrapped)
    // }
    pub fn dtor(context: DestructorContext<LANG, SPEC, Gen>) -> Self {
        let (ffi_type, attrs, generics, ..) = context;
        BindingPresentableContext::Destructor(ffi_type, attrs, generics)
    }
    // pub fn dtor(ffi_type: Type, attrs: SPEC, generics: Option<Generics>) -> Self {
    //     BindingPresentableContext::Destructor(ffi_type, attrs, generics)
    // }
    pub fn get(context: BindingAccessorContext<LANG, SPEC, Gen>) -> Self {
        let (obj_type, field_name, field_type, attrs, generics, ..) = context;
        BindingPresentableContext::Getter(obj_type, field_type, field_name, attrs, generics)
    }
    pub fn set(context: BindingAccessorContext<LANG, SPEC, Gen>) -> Self {
        let (obj_type, field_name, field_type, attrs, generics, ..) = context;
        BindingPresentableContext::Setter(obj_type, field_type, field_name, attrs, generics)
    }
}

impl ScopeContextPresentable for BindingPresentableContext<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            BindingPresentableContext::Constructor(ty, attrs, generics, is_round, args, body) => {
                let body = body.present(source);
                let body_presentation = if *is_round {
                    quote!((#body))
                } else {
                    quote!({#body})
                };

                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            BindingPresentableContext::VariantConstructor(ty, attrs, generics, is_round, args, body) => {
                let body = body.present(source);
                let body_presentation = if *is_round {
                    quote!((#body))
                } else {
                    quote!({#body})
                };

                BindingPresentation::VariantConstructor {
                    attrs: attrs.clone(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            BindingPresentableContext::Destructor(ty, attrs, generics) => {
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    ty: ty.clone(),
                    generics: generics.clone()
                }
            },
            BindingPresentableContext::Getter(obj_type, field_type, field_name, attrs, generics) => BindingPresentation::Getter {
                attrs: attrs.clone(),
                name: Name::getter(obj_type.to_path(), &field_name),
                field_name: field_name.clone(),
                obj_type: obj_type.clone(),
                field_type: field_type.clone(),
                generics: generics.clone(),
            },
            BindingPresentableContext::Setter(obj_type, field_type, field_name, attrs, generics) => BindingPresentation::Getter {
                attrs: attrs.clone(),
                name: Name::setter(obj_type.to_path(), &field_name),
                field_name: field_name.clone(),
                obj_type: obj_type.clone(),
                field_type: field_type.clone(),
                generics: generics.clone(),
            }
        }
    }
}
