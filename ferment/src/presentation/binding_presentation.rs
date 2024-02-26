use proc_macro2::{TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, ReturnType};
use syn::token::RArrow;
use crate::composer::ConstructorPresentableContext;
use crate::naming::Name;

pub enum BindingPresentation {
    Constructor {
        context: ConstructorPresentableContext,
        ctor_arguments: Vec<TokenStream2>,
        body_presentation: TokenStream2,
    },
    Destructor {
        ffi_name: TokenStream2,
        destructor_ident: Name
    },
    Getter {
        field_name: TokenStream2,
        obj_type: TokenStream2,
        field_type: TokenStream2
    },
    Setter {
        field_name: TokenStream2,
        obj_type: TokenStream2,
        field_type: TokenStream2
    },
    ObjAsTrait {
        name: Name,
        item_type: TokenStream2,
        trait_type: TokenStream2,
        vtable_name: Name,
    },
    ObjAsTraitDestructor {
        name: Name,
        item_type: TokenStream2,
        trait_type: TokenStream2,
    }
}

fn present_function<T: ToTokens, I: Iterator<Item = T>>(name: TokenStream2, args: I, output: ReturnType, body: TokenStream2) -> TokenStream2 {
    quote! {
       /// # Safety
       #[no_mangle]
       pub unsafe extern "C" fn #name(#(#args),*) #output {
            #body
        }
    }
}

impl ToTokens for BindingPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Constructor { context, ctor_arguments, body_presentation} => {
                match context {
                    ConstructorPresentableContext::EnumVariant(ffi_ident, ffi_variant_ident, ffi_variant_path) => {
                        present_function(
                            Name::Constructor(ffi_variant_ident.clone()).to_token_stream(),
                            ctor_arguments.iter(),
                            ReturnType::Type(RArrow::default(), parse_quote!(*mut #ffi_ident)),
                            quote!(ferment_interfaces::boxed(#ffi_variant_path #body_presentation)))
                    }
                    ConstructorPresentableContext::Default(ffi_ident) => {
                        present_function(
                            Name::Constructor(ffi_ident.clone()).to_token_stream(),
                            ctor_arguments.iter(),
                            ReturnType::Type(RArrow::default(), parse_quote!(*mut #ffi_ident)),
                            quote!(ferment_interfaces::boxed(#ffi_ident #body_presentation)))
                    }
                }
            },
            Self::Destructor { ffi_name, destructor_ident } => {
                present_function(
                    destructor_ident.to_token_stream(),
                    vec![quote!(ffi: *mut #ffi_name)].iter(),
                    ReturnType::Default,
                    quote!(ferment_interfaces::unbox_any(ffi);)
                )
            },
            Self::ObjAsTrait { name, item_type, trait_type, vtable_name, .. } => {
                present_function(
                    name.to_token_stream(),
                    vec![quote!(obj: *const #item_type)].iter(),
                    ReturnType::Type(RArrow::default(), parse_quote!(#trait_type)),
                    quote!(#trait_type { object: obj as *const (), vtable: &#vtable_name })
                )
            },
            BindingPresentation::ObjAsTraitDestructor { name, item_type, trait_type, } => {
                present_function(
                    name.to_token_stream(),
                    vec![quote!(obj: #trait_type)].iter(),
                    ReturnType::Default,
                    quote!(ferment_interfaces::unbox_any(obj.object as *mut #item_type);))
            },
            BindingPresentation::Getter { field_name, obj_type, field_type } => {
                present_function(
                    format_ident!("{}_get_{}", obj_type.to_string(), field_name.to_string()).to_token_stream(),
                    vec![quote!(obj: *const #obj_type)].iter(),
                    ReturnType::Type(RArrow::default(), parse_quote!(#field_type)),
                    quote!((*obj).#field_name)
                )
            },
            BindingPresentation::Setter { field_name, obj_type, field_type } => {
                present_function(
                    format_ident!("{}_set_{}", obj_type.to_string(), field_name.to_string()).to_token_stream(),
                    vec![quote!(obj: *mut #obj_type), quote!(value: #field_type)].iter(),
                    ReturnType::Default,
                    quote!((*obj).#field_name = value;))
            },

        }.to_tokens(tokens)
     }
}
