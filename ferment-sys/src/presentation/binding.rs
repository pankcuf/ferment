use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, BareFnArg, Field, Generics, parse_quote, ReturnType, Type, Visibility, Lifetime, FieldMutability};
use syn::punctuated::Punctuated;
use syn::token::{Pub, RArrow};
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens, Depunctuated};
use crate::composer::{CommaPunctuatedArgs, SemiPunctuatedArgs};
use crate::ext::{Accessory, CrateExtension, Pop, Terminated, ToPath, ToType};
use crate::presentation::{ArgPresentation, DictionaryName, InterfacePresentation, InterfacesMethodExpr};

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum BindingPresentation {
    Empty,
    Constructor {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        ty: Type,
        generics: Option<Generics>,
        ctor_arguments: CommaPunctuatedArgs,
        body_presentation: TokenStream2,
    },
    VariantConstructor {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        ty: Type,
        generics: Option<Generics>,
        ctor_arguments: CommaPunctuatedArgs,
        body_presentation: TokenStream2,
    },
    Destructor {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        ty: Type,
        generics: Option<Generics>,
    },
    Getter {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
    },
    Setter {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
    },
    GetterOpaque {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
    },
    SetterOpaque {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
    },
    ObjAsTrait {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        item_type: Type,
        trait_type: TokenStream2,
        vtable_name: TokenStream2,
    },
    ObjAsTraitDestructor {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        item_type: TokenStream2,
        trait_type: TokenStream2,
        generics: Option<Generics>,
    },
    RegularFunction {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        is_async: bool,
        arguments: CommaPunctuatedArgs,
        input_conversions: TokenStream2,
        return_type: ReturnType,
        output_conversions: TokenStream2,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
    },
    RegularFunction2 {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        is_async: bool,
        argument_names: CommaPunctuatedTokens,
        arguments: CommaPunctuatedArgs,
        full_fn_path: Type,
        input_conversions: SemiPunctuatedArgs,
        return_type: ReturnType,
        output_conversions: TokenStream2,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
    },
    Callback {
        name: Ident,
        attrs: Vec<Attribute>,
        ffi_args: CommaPunctuated<BareFnArg>,
        result: ReturnType,
        conversion: InterfacePresentation,
    },

    TraitVTableInnerFn {
        name: TokenStream2,
        name_and_args: TokenStream2,
        output_expression: ReturnType,
    },
    StaticVTableInnerFnDeclaration {
        name: TokenStream2,
        fn_name: Ident
    },
    StaticVTableInnerFn {
        name: TokenStream2,
        args: CommaPunctuatedArgs,
        output: ReturnType,
        body: TokenStream2,
    },
    StaticVTable {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        methods_declarations: CommaPunctuated<BindingPresentation>,
        methods_implementations: Depunctuated<BindingPresentation>,
        bindings: Depunctuated<BindingPresentation>,
        fq_trait_vtable: TokenStream2,
    },
}

fn present_pub_function<T: ToTokens, U: ToTokens>(
    attrs: &Vec<Attribute>,
    name: U,
    args: CommaPunctuated<T>,
    output: ReturnType,
    generics: Option<Generics>,
    lifetimes: Vec<Lifetime>,
    body: TokenStream2) -> TokenStream2 {
    present_function(attrs, Pub::default().to_token_stream(), name.to_token_stream(), args, output, generics, lifetimes, body)
}
pub fn present_function<T: ToTokens>(
    attrs: &Vec<Attribute>,
    acc: TokenStream2,
    name: TokenStream2,
    args: CommaPunctuated<T>,
    output: ReturnType,
    generics: Option<Generics>,
    lifetimes: Vec<Lifetime>,
    body: TokenStream2) -> TokenStream2 {
    match generics {
        None => {
            let comma_lifetimes = CommaPunctuated::from_iter(lifetimes.iter().filter_map(|lt| {
                if lt.ident.to_string().eq("static") {
                    None
                } else {
                    Some(lt.to_token_stream())
                }
            }));
            let lifetime_tokens = if comma_lifetimes.is_empty() {
                quote!()
            } else {
                quote!(<#comma_lifetimes>)
            };
            quote! {
               #(#attrs)*
               #[no_mangle]
               #acc unsafe extern "C" fn #name #lifetime_tokens(#args) #output {
                    #body
                }
            }
        },
        Some(Generics { params, where_clause, .. }) => {
            quote! {
               #(#attrs)*
               #[no_mangle]
               #acc unsafe extern "C" fn #name<#params>(#args) #output #where_clause {
                    #body
                }
            }
        }
    }
}

pub fn present_struct<Name: ToTokens, Impl: ToTokens>(
    ident: Name,
    attrs: &Vec<Attribute>,
    implementation: Impl
) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #(#attrs)*
        pub struct #ident #implementation
    }
}


impl ToTokens for BindingPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty =>
                quote!(),
            Self::Constructor { attrs, name, ty, generics, ctor_arguments, body_presentation} => {
                let ffi_path = ty.to_path().arg_less();
                present_pub_function(
                    attrs,
                    name,
                    ctor_arguments.clone(),
                    ReturnType::Type(RArrow::default(), ty.joined_mut().into()),
                    generics.clone(),
                    vec![],
                    InterfacesMethodExpr::Boxed(quote!(#ffi_path #body_presentation)).to_token_stream())
            },
            Self::VariantConstructor { attrs, name, ty, generics, ctor_arguments, body_presentation} => {
                let variant_path = ty.to_path();
                present_pub_function(
                    attrs,
                    name,
                    ctor_arguments.clone(),
                    ReturnType::Type(RArrow::default(), variant_path.popped().to_token_stream().joined_mut().to_type().into()),
                    generics.clone(),
                    vec![],
                    InterfacesMethodExpr::Boxed(quote!(#variant_path #body_presentation)).to_token_stream())
            },
            Self::Destructor { attrs, name, ty, generics } => {
                let ty = ty.joined_mut();
                present_pub_function(
                    attrs,
                    name,
                    CommaPunctuated::from_iter([quote!(ffi: #ty)]),
                    ReturnType::Default,
                    generics.clone(),
                    vec![],
                    InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated()
                )
            },
            Self::ObjAsTrait { name, item_type, trait_type, vtable_name, attrs } => {
                let ty = item_type.joined_const();
                present_pub_function(
                    attrs,
                    name,
                    CommaPunctuated::from_iter([quote!(obj: #ty)]),
                    ReturnType::Type(RArrow::default(), trait_type.to_type().into()),
                    None,
                    vec![],
                    quote!(#trait_type {
                        object: obj as *const (),
                        vtable: &#vtable_name
                    })
                )
            },
            BindingPresentation::ObjAsTraitDestructor { name, item_type, trait_type, attrs, generics } => {
                present_pub_function(
                    attrs,
                    name,
                    CommaPunctuated::from_iter([quote! { #(#attrs)* obj: #trait_type }]),
                    ReturnType::Default,
                    generics.clone(),
                    vec![],
                    InterfacesMethodExpr::UnboxAny(quote!(obj.object as *mut #item_type)).to_token_stream().terminated()
                )
            },
            BindingPresentation::Getter { name, field_name, obj_type, field_type, attrs, generics } |
            BindingPresentation::GetterOpaque { name, field_name, obj_type, field_type, attrs, generics } => {
                let var = obj_type.joined_const();
                present_pub_function(
                    attrs,
                    name,
                    CommaPunctuated::from_iter([quote! { obj: #var }]),
                    ReturnType::Type(RArrow::default(), field_type.clone().into()),
                    generics.clone(),
                    vec![],
                    quote!((*obj).#field_name)
                )
            },
            BindingPresentation::Setter { name, field_name, obj_type, field_type, attrs, generics } |
            BindingPresentation::SetterOpaque { name, field_name, obj_type, field_type, attrs, generics } => {
                let var = obj_type.joined_mut();
                present_pub_function(
                    attrs,
                    name,
                    CommaPunctuated::from_iter([
                        quote!(obj: #var),
                        quote!(value: #field_type),
                    ]),
                    ReturnType::Default,
                    generics.clone(),
                    vec![],
                    quote!((*obj).#field_name = value;))
            },
            BindingPresentation::RegularFunction { attrs, is_async, name, arguments, input_conversions, return_type, output_conversions, generics, lifetimes } => {
                if *is_async {
                    let mut args = Punctuated::from_iter([
                        ArgPresentation::Field(Field { attrs: vec![], vis: Visibility::Inherited, ident: Some(format_ident!("runtime")), colon_token: Default::default(), mutability: FieldMutability::None, ty: parse_quote!(*const std::os::raw::c_void) }),
                    ]);
                    args.extend(arguments.clone());
                    present_pub_function(
                        attrs,
                        name,
                        args,
                        return_type.clone(),
                        generics.clone(),
                        lifetimes.clone(),
                        quote! {
                            let rt = &*(runtime as *const tokio::runtime::Runtime);
                            let obj = rt.block_on(async {
                                #input_conversions .await
                            });
                            #output_conversions
                        }
                    )
                } else {
                    present_pub_function(
                        attrs,
                        name,
                        arguments.clone(),
                        return_type.clone(),
                        generics.clone(),
                        lifetimes.clone(),
                        quote!(let obj = #input_conversions; #output_conversions)
                    )
                }
            },
            BindingPresentation::RegularFunction2 { attrs, is_async, name, argument_names, arguments, full_fn_path, input_conversions, return_type, output_conversions, generics, lifetimes } => {
                if *is_async {
                    let mut args = Punctuated::from_iter([
                        ArgPresentation::Field(Field { attrs: vec![], vis: Visibility::Inherited, ident: Some(format_ident!("runtime")), colon_token: Default::default(), mutability: FieldMutability::None, ty: parse_quote!(*const std::os::raw::c_void) }),
                    ]);
                    args.extend(arguments.clone());
                    present_pub_function(
                        attrs,
                        name,
                        args.clone(),
                        return_type.clone(),
                        generics.clone(),
                        lifetimes.clone(),
                        quote! {
                            let rt = unsafe { &*(runtime as *const tokio::runtime::Runtime) };
                            #input_conversions;
                            let obj = rt.block_on(async {
                                #full_fn_path(#argument_names).await
                            });
                            #output_conversions
                        }
                        // quote! {
                        //     let rt = unsafe { &*(runtime as *mut tokio::runtime::Runtime) };
                        //     #input_conversions;
                        //     let obj = rt.block_on(tokio::task::spawn_blocking(move || {
                        //         tokio::runtime::Handle::current().block_on(async {
                        //             #full_fn_path(#argument_names).await
                        //         })
                        //     })).unwrap();
                        //     #output_conversions
                        // }
                    )
                } else {
                    present_pub_function(
                        attrs,
                        name,
                        arguments.clone(),
                        return_type.clone(),
                        generics.clone(),
                        lifetimes.clone(),
                        quote! {
                            #input_conversions;
                            let obj = #full_fn_path(#argument_names);
                            #output_conversions
                        }
                    )
                }
            },
            BindingPresentation::Callback { name, attrs, ffi_args, result, conversion } => {
                let result_impl = match result {
                    ReturnType::Default => quote! {},
                    ReturnType::Type(_, ref ty) => quote! { #result, destructor: unsafe extern "C" fn(result: #ty) }
                };
                let implementation = quote! {{ caller: unsafe extern "C" fn(#ffi_args) #result_impl, }};
                let definition = present_struct(name, attrs, implementation);
                quote! {
                    #definition
                    #conversion
                }
            }
            BindingPresentation::StaticVTable { attrs, name, fq_trait_vtable, methods_declarations, methods_implementations, bindings } => {
                quote! {
                    #[no_mangle]
                    #(#attrs)*
                    pub static #name: #fq_trait_vtable = {
                        #methods_implementations
                        #fq_trait_vtable {
                            #methods_declarations
                        }
                    };
                    #bindings
                }
            },
            BindingPresentation::TraitVTableInnerFn { name, name_and_args, output_expression } => {
                quote!(pub #name: #name_and_args #output_expression)
            }
            BindingPresentation::StaticVTableInnerFn { name, args, output, body } => {
                present_function(
                    &vec![],
                    quote!(),
                    name.to_token_stream(),
                    args.clone(),
                    output.clone(),
                    None,
                    vec![],
                    body.clone()
                )
            },
            BindingPresentation::StaticVTableInnerFnDeclaration { name, fn_name } =>
                quote!(#fn_name: #name),

        }.to_tokens(tokens)
     }
}
