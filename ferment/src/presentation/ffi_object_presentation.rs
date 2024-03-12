use quote::{quote, ToTokens};
use proc_macro2::{TokenStream as TokenStream2};
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composer::Depunctuated;
use crate::composition::TraitVTableMethodComposition;
use crate::interface::create_struct;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, DropInterfacePresentation};
use crate::presentation::conversion_interface_presentation::ConversionInterfacePresentation;

pub enum FFIObjectPresentation {
    Empty,
    StaticVTable {
        name: Name,
        methods_compositions: Vec<TraitVTableMethodComposition>,
        // methods_names: Vec<Ident>,
        // methods_signatures: Vec<TokenStream2>,
        fq_trait_vtable: TokenStream2,
        // methods_implementations: Vec<TraitVTablePresentation>,
        // methods_declarations: Vec<TraitVTablePresentation>,
    },
    TraitVTable {
        name: Name,
        fields: Punctuated<BindingPresentation, Comma>
    },
    // TraitVTableInnerFn {
    //     name: Name,
    //     name_and_args: TokenStream2,
    //     output_expression: ReturnType,
    // },
    TraitObject {
        name: Name,
        vtable_name: Name,
    },
    Full(TokenStream2),
    Generic {
        object_presentation: TokenStream2,
        conversion_presentation: ConversionInterfacePresentation,
        drop_presentation: DropInterfacePresentation,
        bindings: Depunctuated<BindingPresentation>,
        special: Option<TokenStream2>
    },
}


impl ToTokens for FFIObjectPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Full(presentation) => quote!(#presentation),
            Self::TraitVTable { name, fields } =>
                create_struct(&parse_quote!(#name), quote!({ #fields })),
            Self::TraitObject { name, vtable_name } =>
                create_struct(&parse_quote!(#name), quote!({ pub object: *const (), pub vtable: *const #vtable_name })),
            FFIObjectPresentation::Generic {
                object_presentation,
                conversion_presentation,
                drop_presentation,
                bindings,
                special
            } => {
                let special_presentation = special.as_ref().map_or(quote!(), |special| quote!(#special));
                quote! {
                    #object_presentation
                    #conversion_presentation
                    #special_presentation
                    #drop_presentation
                    #bindings
                }

            },
            FFIObjectPresentation::Empty => { /* Box<T> */
                quote!()
            },
            FFIObjectPresentation::StaticVTable { name, fq_trait_vtable, methods_compositions } => {
                let (methods_implementations, methods_declarations): (Depunctuated<TokenStream2>, Punctuated<TokenStream2, Comma>) = methods_compositions
                    .iter()
                    .map(|TraitVTableMethodComposition { fn_name, ffi_fn_name, item_type, trait_type, argument_names, name_and_args, output_expression, output_conversions }| {
                        (quote!(#fn_name: #ffi_fn_name), {
                            let input_conversions = quote! {
                                let cast_obj = &(*(obj as *const #item_type));
                                let obj = <#item_type as #trait_type>::#fn_name #argument_names;
                            };
                            quote!(#name_and_args -> #output_expression { #input_conversions #output_conversions})
                        })
                    })
                    .unzip();
                quote! {
                    static #name: #fq_trait_vtable = {
                        #methods_implementations
                        #fq_trait_vtable {
                            #methods_declarations
                        }
                    };
                }
            }
        }.to_tokens(tokens)
    }
}
// # [doc = r" # Safety"]
// # [no_mangle]
// pub unsafe extern "C" fn Status_as_CanRetry_can_retry (obj: * const Status) -> bool {
//     let obj = ferment_interfaces::FFIConversion::ffi_from_const(obj);
//     let result = <crate::transport::transport_request::Status as crate::transport::transport_request::CanRetry>::can_retry(&obj);
//     result
// }
