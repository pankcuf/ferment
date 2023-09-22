use quote::quote;
use syn::__private::TokenStream2;
use crate::interface::{doc, ffi, ffi_from_const, ffi_to_const, interface, obj, package, Presentable};

/// Root-level composer chain
pub enum Expansion {
    Empty,
    Callback {
        input: TokenStream2,
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
    },
    Function {
        input: TokenStream2,
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
    },
    Full {
        input: TokenStream2,
        comment: DocPresentation,
        ffi_presentation: FFIObjectPresentation,
        conversion: ConversionInterfacePresentation,
        drop: DropInterfacePresentation,
    },
}

pub enum DocPresentation {
    // Empty,
    Default(TokenStream2),
    Safety(TokenStream2),
}


pub enum FFIObjectPresentation {
    // Empty,
    Callback {
        name: TokenStream2,
        arguments: Vec<TokenStream2>,
        output_expression: TokenStream2,
    },
    Function {
        name_and_arguments: TokenStream2,
        input_conversions: TokenStream2,
        output_expression: TokenStream2,
        output_conversions: TokenStream2,

    },
    Full(TokenStream2)
}

pub enum ConversionInterfacePresentation {
    Empty,
    Interface {
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        from_presentation: TokenStream2,
        to_presentation: TokenStream2,
        destroy_presentation: TokenStream2
    }
}


pub enum DropInterfacePresentation {
    Empty,
    Full(TokenStream2, TokenStream2)
    // Enum(TokenStream2, TokenStream2)
}

impl Presentable for Expansion {
    fn present(self) -> TokenStream2 {
        match self {
            Self::Empty => quote!(),
            Self::Callback { input, comment, ffi_presentation } =>
                expansion(
                    input,
                    comment.present(),
                    ffi_presentation.present(),
                    ConversionInterfacePresentation::Empty.present(),
                    DropInterfacePresentation::Empty.present()),
            Self::Function { input, comment, ffi_presentation } =>
                expansion(
                    input,
                    comment.present(),
                    ffi_presentation.present(),
                    ConversionInterfacePresentation::Empty.present(),
                    DropInterfacePresentation::Empty.present()
                ),
            Self::Full { input, comment, ffi_presentation, conversion, drop} =>
                expansion(
                    input,
                    comment.present(),
                    ffi_presentation.present(),
                    conversion.present(),
                    drop.present()),
        }
    }
}

impl Presentable for DocPresentation {
    fn present(self) -> TokenStream2 {
        match self {
            // Self::Empty => quote!(),
            Self::Default(target_name) => doc(target_name.to_string()),
            Self::Safety(target_name) => {
                let doc = doc(target_name.to_string());
                quote! {
                    #doc
                    /// # Safety
                }
            }
        }
    }
}

impl Presentable for FFIObjectPresentation {
    fn present(self) -> TokenStream2 {
        match self {
            // Self::Empty => quote!(),
            Self::Callback { name, arguments, output_expression} =>
                quote! {
                    #[allow(non_camel_case_types)]
                    pub type #name = unsafe extern "C" fn(#(#arguments),*) #output_expression;
                },
            Self::Function { name_and_arguments, input_conversions, output_expression, output_conversions, } =>
                quote! {
                    #[no_mangle]
                    pub unsafe extern "C" fn #name_and_arguments -> #output_expression {
                        let obj = #input_conversions;
                        #output_conversions
                    }
                },
            Self::Full(presentation) => presentation,
        }
    }
}

impl Presentable for ConversionInterfacePresentation {
    fn present(self) -> TokenStream2 {
        match self {
            Self::Empty => quote!(),
            Self::Interface { ffi_name, target_name, from_presentation, to_presentation, destroy_presentation} => {
                let package = package();
                let interface = interface();
                let ffi = ffi();
                let obj = obj();
                // let ffi_from = ffi_from();
                let ffi_from_const = ffi_from_const();
                // let ffi_to = ffi_to();
                let ffi_to_const = ffi_to_const();
                // let ffi_from_opt = ffi_from_opt();
                // let ffi_to_opt = ffi_to_opt();
                quote! {
                    impl #package::#interface<#target_name> for #ffi_name {
                        unsafe fn #ffi_from_const(#ffi: *const #ffi_name) -> #target_name { #from_presentation }
                        unsafe fn #ffi_to_const(#obj: #target_name) -> *const #ffi_name { #to_presentation }
                        // unsafe fn #ffi_from(#ffi: *mut #ffi_name) -> #target_name { #ffi_from_conversion }
                        // unsafe fn #ffi_to(#obj: #target_name) -> *mut #ffi_name { #ffi_to_conversion }
                        // unsafe fn #ffi_from_opt(#ffi: *mut #ffi_name) -> Option<#target_name> {
                        //     (!#ffi.is_null()).then_some(<Self as #package::#interface<#target_name>>::#ffi_from(#ffi))
                        // }
                        // unsafe fn #ffi_to_opt(#obj: Option<#target_name>) -> *mut #ffi_name {
                        //     #obj.map_or(std::ptr::null_mut(), |o| <Self as #package::#interface<#target_name>>::#ffi_to(o))
                        // }
                        unsafe fn destroy(#ffi: *mut #ffi_name) { #destroy_presentation; }
                    }
                }
            },
        }
    }
}

impl Presentable for DropInterfacePresentation {
    fn present(self) -> TokenStream2 {
        match self {
            Self::Empty => quote!(),
            // Self::Full(presenter, name, code) => presenter(name, code)
            Self::Full(name, code) =>
                quote!(impl Drop for #name { fn drop(&mut self) { unsafe { #code } } })
        }
    }
}



// Vec<Vec<u32>> -> Vec_Vec_u32_FFI -> { pub count : usize, pub values : * mut * mut Vec_u32_FFI, },
// Vec<bool> -> Vec_bool_FFI -> pub count : usize, pub values : * mut bool, },
// BTreeMap<self::HashID, Vec<self::HashID>> -> Map_keys_self_HashID_values_self_HashID_FFI -> { pub count : usize, pub keys : * mut * mut self :: HashIDFFI, pub values : * mut * mut Vec_self_HashID_FFI,}
// BTreeMap<String, self::HashID>,
// Vec<u32>,
// Vec<Vec<self::HashID>>,
// BTreeMap<self::HashID, self::HashID>,
// Vec<u8>

fn expansion(
    input: TokenStream2,
    comment: TokenStream2,
    ffi_converted_input: TokenStream2,
    ffi_conversion_presentation: TokenStream2,
    drop_presentation: TokenStream2,
) -> TokenStream2 {
    let expanded = quote! {
        #input
        #comment
        #ffi_converted_input
        #ffi_conversion_presentation
        #drop_presentation
    };
    println!("{}", expanded);
    expanded
}
