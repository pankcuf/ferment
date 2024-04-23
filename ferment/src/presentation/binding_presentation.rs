use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{parse_quote, ReturnType};
use syn::punctuated::Punctuated;
use syn::token::{Comma, RArrow};
use crate::composer::{ConstructorPresentableContext, Depunctuated};
use crate::ext::{Mangle, Pop, ToPath};
use crate::naming::Name;

// pub enum Fn

#[derive(Debug)]
pub enum BindingPresentation {
    Empty,
    Constructor {
        context: ConstructorPresentableContext,
        ctor_arguments: Punctuated<TokenStream2, Comma>,
        body_presentation: TokenStream2,
    },
    Destructor {
        name: Name,
        ffi_name: TokenStream2,
    },
    Getter {
        name: Name,
        field_name: TokenStream2,
        obj_type: TokenStream2,
        field_type: TokenStream2
    },
    Setter {
        name: Name,
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
    },
    RegularFunction {
        name: Name,
        is_async: bool,
        arguments: Punctuated<TokenStream2, Comma>,
        input_conversions: TokenStream2,
        return_type: ReturnType,
        output_conversions: TokenStream2,
    },
    Callback {
        name: TokenStream2,
        arguments: Punctuated<TokenStream2, Comma>,
        output_expression: ReturnType,
    },
    TraitVTableInnerFn {
        name: Name,
        name_and_args: TokenStream2,
        output_expression: ReturnType,
    },
    StaticVTableInnerFnDeclaration {
        name: Name,
        fn_name: Ident
    },
    StaticVTableInnerFn {
        name: Name,
        args: Punctuated<TokenStream2, Comma>,
        output: ReturnType,
        body: TokenStream2,
    },
    StaticVTable {
        name: Name,
        methods_declarations: Punctuated<BindingPresentation, Comma>,
        methods_implementations: Depunctuated<BindingPresentation>,
        fq_trait_vtable: TokenStream2,
    },
}

fn present_pub_function<T: ToTokens>(
    name: TokenStream2,
    args: Punctuated<T, Comma>,
    output: ReturnType,
    body: TokenStream2) -> TokenStream2 {
    present_function(quote!(pub), name, args, output, body)
}
pub fn present_function<T: ToTokens>(
    acc: TokenStream2,
    name: TokenStream2,
    args: Punctuated<T, Comma>,
    output: ReturnType,
    body: TokenStream2) -> TokenStream2 {
    quote! {
       #[no_mangle]
       #acc unsafe extern "C" fn #name(#args) #output {
            #body
        }
    }
}

impl ToTokens for BindingPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty =>
                quote!(),
            Self::Constructor { context, ctor_arguments, body_presentation} => {
                match context {
                    ConstructorPresentableContext::EnumVariant(ffi_type) => {
                        let ffi_variant_name = Name::Constructor(ffi_type.clone());
                        let variant_path = ffi_type.to_path();
                        let enum_path = variant_path.popped();
                        present_pub_function(
                            ffi_variant_name.mangle_ident_default().to_token_stream(),
                            ctor_arguments.clone(),
                            ReturnType::Type(RArrow::default(), parse_quote!(*mut #enum_path)),
                            quote!(ferment_interfaces::boxed(#variant_path #body_presentation)))
                    }
                    ConstructorPresentableContext::Default(ffi_type) => {
                        let name = Name::Constructor(ffi_type.clone());
                        present_pub_function(
                            name.mangle_ident_default().to_token_stream(),
                            ctor_arguments.clone(),
                            ReturnType::Type(RArrow::default(), parse_quote!(*mut #ffi_type)),
                            quote!(ferment_interfaces::boxed(#ffi_type #body_presentation)))
                    }
                }
            },
            Self::Destructor { name, ffi_name, } => {
                present_pub_function(
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([quote!(ffi: *mut #ffi_name)]),
                    ReturnType::Default,
                    quote!(ferment_interfaces::unbox_any(ffi);)
                )
            },
            Self::ObjAsTrait { name, item_type, trait_type, vtable_name, .. } => {
                present_pub_function(
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([quote!(obj: *const #item_type)]),
                    ReturnType::Type(RArrow::default(), parse_quote!(#trait_type)),
                    quote!(#trait_type { object: obj as *const (), vtable: &#vtable_name })
                )
            },
            BindingPresentation::ObjAsTraitDestructor { name, item_type, trait_type, } => {
                present_pub_function(
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([quote!(obj: #trait_type)]),
                    ReturnType::Default,
                    quote!(ferment_interfaces::unbox_any(obj.object as *mut #item_type);))
            },
            BindingPresentation::Getter { name, field_name, obj_type, field_type } => {
                present_pub_function(
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([quote!(obj: *const #obj_type)]),
                    ReturnType::Type(RArrow::default(), parse_quote!(#field_type)),
                    quote!((*obj).#field_name)
                )
            },
            BindingPresentation::Setter { name, field_name, obj_type, field_type } => {
                present_pub_function(
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([quote!(obj: *mut #obj_type), quote!(value: #field_type)]),
                    ReturnType::Default,
                    quote!((*obj).#field_name = value;))
            },
            BindingPresentation::RegularFunction { is_async, name, arguments, input_conversions, return_type, output_conversions } => {
                if *is_async {
                    let mut args = Punctuated::from_iter([quote!(runtime: *mut std::os::raw::c_void)]);
                    args.extend(arguments.clone());
                    present_pub_function(
                        name.mangle_ident_default().to_token_stream(),
                        args,
                        return_type.clone(),
                        quote! {
                            let rt = unsafe { &*(runtime as *mut tokio::runtime::Runtime) };
                            let obj = rt.block_on(async { #input_conversions .await });
                            #output_conversions
                        }
                    )
                } else {
                    present_pub_function(
                        name.mangle_ident_default().to_token_stream(),
                        arguments.clone(),
                        return_type.clone(),
                        quote!(let obj = #input_conversions; #output_conversions)
                    )
                }
            },
            BindingPresentation::Callback { name, arguments, output_expression: return_type } =>
                quote!(pub type #name = unsafe extern "C" fn(#arguments) #return_type;),
            BindingPresentation::StaticVTable { name, fq_trait_vtable, methods_declarations, methods_implementations } => {
                println!("BindingPresentation::StaticVTable:: {:?} [{}]", name, fq_trait_vtable);
                quote! {
                    static #name: #fq_trait_vtable = {
                        #methods_implementations
                        #fq_trait_vtable {
                            #methods_declarations
                        }
                    };
                }
            },
            BindingPresentation::TraitVTableInnerFn { name, name_and_args, output_expression } => {
                quote!(pub #name: #name_and_args #output_expression)
            }
            BindingPresentation::StaticVTableInnerFn { name, args, output, body } => {
                present_function(
                    quote!(),
                    name.to_token_stream(),
                    args.clone(),
                    output.clone(),
                    body.clone()
                )
            },
            BindingPresentation::StaticVTableInnerFnDeclaration { name, fn_name } =>
                quote!(#fn_name: #name),

        }.to_tokens(tokens)
     }
}
