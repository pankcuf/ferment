use quote::{quote, ToTokens};
use proc_macro2::{TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composer::Depunctuated;
use crate::ext::ToPath;
use crate::interface::create_struct;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, DropInterfacePresentation, Expansion};
use crate::presentation::conversion_interface_presentation::InterfacePresentation;

pub enum FFIObjectPresentation {
    Empty,
    // StaticVTable {
    //     name: Name,
    //     methods_declarations: Punctuated<TokenStream2, Comma>,
    //     methods_implementations: Depunctuated<BindingPresentation>,
    //     // methods_compositions: Vec<TraitVTableMethodComposition>,
    //     // methods_names: Vec<Ident>,
    //     // methods_signatures: Vec<TokenStream2>,
    //     fq_trait_vtable: TokenStream2,
    //     // methods_implementations: Vec<TraitVTablePresentation>,
    //     // methods_declarations: Vec<TraitVTablePresentation>,
    // },
    TraitVTable {
        name: Name,
        fields: Punctuated<Expansion, Comma>
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
        interface_presentations: Depunctuated<InterfacePresentation>,
        drop_presentation: DropInterfacePresentation,
        bindings: Depunctuated<BindingPresentation>,
    },
}


impl ToTokens for FFIObjectPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Full(presentation) => quote!(#presentation),
            Self::TraitVTable { name, fields } => {
                println!("FFIObjectPresentation::TraitVTable:: {:?} [{}]", name, fields.to_token_stream());
                create_struct(&name.to_path(), quote!({ #fields }))
            },
            Self::TraitObject { name, vtable_name } => {
                println!("FFIObjectPresentation::TraitObject:: {:?} [{}]", name, vtable_name.to_token_stream());
                create_struct(&name.to_path(), quote!({ pub object: *const (), pub vtable: *const #vtable_name }))
            },
            Self::Generic {
                object_presentation,
                interface_presentations,
                drop_presentation,
                bindings
            } => quote! {
                #object_presentation
                #interface_presentations
                #drop_presentation
                #bindings
            },
            Self::Empty => { /* Box<T> */
                quote!()
            },
/*            Self::StaticVTable { name, fq_trait_vtable, methods_declarations, methods_implementations } => {
                println!("FFIObjectPresentation::StaticVTable:: {:?} [{}]", name, fq_trait_vtable);
                quote! {
                    static #name: #fq_trait_vtable = {
                        #methods_implementations
                        #fq_trait_vtable {
                            #methods_declarations
                        }
                    };
                }
            }
*/        }.to_tokens(tokens)
    }
}
// # [doc = r" # Safety"]
// # [no_mangle]
// pub unsafe extern "C" fn Status_as_CanRetry_can_retry (obj: * const Status) -> bool {
//     let obj = ferment_interfaces::FFIConversion::ffi_from_const(obj);
//     let result = <crate::transport::transport_request::Status as crate::transport::transport_request::CanRetry>::can_retry(&obj);
//     result
// }
