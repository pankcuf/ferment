use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use crate::presentation::{BindingPresentation, FFIObjectPresentation};

pub enum TraitVTablePresentation {
    Full {
        vtable: FFIObjectPresentation,
        export: BindingPresentation,
        destructor: BindingPresentation,
    },
    // StaticVTable {
    //     name: Name,
    //     fq_trait_vtable: TokenStream2,
    //     methods_names: Vec<Ident>,
    //     methods_signatures: Vec<TokenStream2>,
    //     methods_implementations: Vec<TraitVTablePresentation>,
    //     methods_declarations: Vec<TraitVTablePresentation>,
    // },
    // MethodDeclaration {
    //     name: Ident,
    //     sig_name: Ident,
    // },
    // Method {
    //     fn_name: Ident,
    //     sig_name: Ident,
    //     argument_names: TokenStream2,
    //     name_and_args: TokenStream2,
    //     output_expression: TokenStream2,
    //     item_type: Type,
    //     trait_type: Type,
    //
    //     // input_conversions: TokenStream2,
    //     output_conversions: TokenStream2,
    // },
}

impl ToTokens for TraitVTablePresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            TraitVTablePresentation::Full { vtable, export, destructor } => quote! {
                #vtable
                #export
                #destructor
            },
            // TraitVTablePresentation::Method { fn_name, sig_name, argument_names, name_and_args, output_expression, item_type, trait_type, output_conversions } => {
            //     let input_conversions = quote! {
            //         let cast_obj = &(*(obj as *const #item_type));
            //         let obj = <#item_type as #trait_type>::#fn_name #argument_names;
            //     };
            //     quote!(#name_and_args -> #output_expression { #input_conversions #output_conversions})
            // },
            // TraitVTablePresentation::MethodDeclaration { name, sig_name } => {
            //     quote!(#name: #sig_name)
            // },
            // TraitVTablePresentation::StaticVTable { name, fq_trait_vtable, methods_names, methods_signatures, methods_implementations, methods_declarations } => {
            //     // let implementations =
            //     quote! {
            //         static #name: #fq_trait_vtable = {
            //             #(#methods_implementations,)*
            //             #fq_trait_vtable {
            //                 #(#methods_names)*: #(#methods_signatures)*
            //             }
            //         };
            //     }
            // }

        }.to_tokens(tokens)
    }
}
