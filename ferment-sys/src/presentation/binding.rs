use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, BareFnArg, Field, Generics, parse_quote, ReturnType, Type, Visibility, FieldMutability};
use syn::punctuated::Punctuated;
use syn::token::{Pub, RArrow};
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens, Depunctuated};
use crate::composer::{CommaPunctuatedArgs, SemiPunctuatedArgs, SignatureAspect};
use crate::ext::{Accessory, CrateExtension, Pop, Terminated, ToPath, ToType};
use crate::lang::RustSpecification;
use crate::presentation::{ArgPresentation, DictionaryName, InterfacePresentation, InterfacesMethodExpr};

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum BindingPresentation {
    Empty,
    Constructor {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        ty: Type,
        ctor_arguments: CommaPunctuatedArgs,
        body_presentation: TokenStream2,
    },
    VariantConstructor {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        ty: Type,
        ctor_arguments: CommaPunctuatedArgs,
        body_presentation: TokenStream2,
    },
    Destructor {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        ty: Type,
    },
    Getter {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
    },
    Setter {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
    },
    GetterOpaque {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
    },
    SetterOpaque {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
    },
    ObjAsTrait {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        item_type: Type,
        trait_type: TokenStream2,
        vtable_name: TokenStream2,
    },
    ObjAsTraitDestructor {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        item_type: TokenStream2,
        trait_type: TokenStream2,
    },
    RegularFunction {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        is_async: bool,
        arguments: CommaPunctuatedArgs,
        input_conversions: TokenStream2,
        return_type: ReturnType,
        output_conversions: TokenStream2,
    },
    RegularFunctionWithBody {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        arguments: CommaPunctuatedArgs,
        return_type: ReturnType,
        body: TokenStream2,
    },
    RegularFunction2 {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        is_async: bool,
        argument_names: CommaPunctuatedTokens,
        arguments: CommaPunctuatedArgs,
        full_fn_path: Type,
        input_conversions: SemiPunctuatedArgs,
        return_type: ReturnType,
        output_conversions: TokenStream2,
    },
    Callback {
        aspect: SignatureAspect<RustSpecification>,
        name: Ident,
        ffi_args: CommaPunctuated<BareFnArg>,
        result: ReturnType,
        conversion: InterfacePresentation,
    },

    TraitVTableInnerFn {
        attrs: Vec<Attribute>,
        name: TokenStream2,
        name_and_args: TokenStream2,
        output_expression: ReturnType,
    },
    StaticVTableInnerFnDeclaration {
        name: TokenStream2,
        fn_name: Ident
    },
    StaticVTableInnerFn {
        aspect: SignatureAspect<RustSpecification>,
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

    Any {
        attrs: Vec<Attribute>,
        body: TokenStream2
    }
}

pub fn present_pub_function<T: ToTokens, U: ToTokens>(
    aspect: &SignatureAspect<RustSpecification>,
    name: U,
    args: CommaPunctuated<T>,
    output: ReturnType,
    body: TokenStream2
) -> TokenStream2 {
    present_function(Visibility::Public(Pub::default()), aspect, name.to_token_stream(), args, output, body)
}
pub fn present_function<T: ToTokens>(
    acc: Visibility,
    (attrs, lifetimes, generics): &SignatureAspect<RustSpecification>,
    name: TokenStream2,
    args: CommaPunctuated<T>,
    output: ReturnType,
    body: TokenStream2) -> TokenStream2 {
    let signature = match generics {
        None => {
            let comma_lifetimes = CommaPunctuated::from_iter(lifetimes.iter().filter_map(|lt| {
                if lt.ident.to_string().eq("static") {
                    None
                } else {
                    Some(lt.to_token_stream())
                }
            }));
            if comma_lifetimes.is_empty() {
                quote!(#name(#args) #output)
            } else {
                quote!(#name<#comma_lifetimes>(#args) #output)
            }
        },
        Some(Generics { params, where_clause, .. }) => {
            if params.is_empty() {
                quote!(#name(#args) #output #where_clause)
            } else {
                quote!(#name<#params>(#args) #output #where_clause)
            }
        }
    };
    quote! {
        #(#attrs)*
        #[no_mangle]
        #acc unsafe extern "C" fn #signature {
            #body
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
            Self::Constructor { aspect, name, ty, ctor_arguments, body_presentation} => {
                let ffi_path = ty.to_path().arg_less();
                present_pub_function(
                    aspect,
                    name,
                    ctor_arguments.clone(),
                    ReturnType::Type(RArrow::default(), ty.joined_mut().into()),
                    InterfacesMethodExpr::Boxed(quote!(#ffi_path #body_presentation)).to_token_stream())
            },
            Self::VariantConstructor { aspect, name, ty, ctor_arguments, body_presentation} => {
                let variant_path = ty.to_path();
                present_pub_function(
                    aspect,
                    name,
                    ctor_arguments.clone(),
                    ReturnType::Type(RArrow::default(), variant_path.popped().to_token_stream().joined_mut().to_type().into()),
                    InterfacesMethodExpr::Boxed(quote!(#variant_path #body_presentation)).to_token_stream())
            },
            Self::Destructor { aspect, name, ty } => {
                let ty = ty.joined_mut();
                present_pub_function(
                    aspect,
                    name,
                    CommaPunctuated::from_iter([quote!(ffi: #ty)]),
                    ReturnType::Default,
                    InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated()
                )
            },
            Self::ObjAsTrait { aspect, name, item_type, trait_type, vtable_name } => {
                let ty = item_type.joined_const();
                present_pub_function(
                    aspect,
                    name,
                    CommaPunctuated::from_iter([quote!(obj: #ty)]),
                    ReturnType::Type(RArrow::default(), trait_type.to_type().into()),
                    quote!(#trait_type {
                        object: obj as *const (),
                        vtable: &#vtable_name
                    })
                )
            },
            BindingPresentation::ObjAsTraitDestructor { aspect, name, item_type, trait_type } => {
                let attrs = &aspect.0;
                present_pub_function(
                    aspect,
                    name,
                    CommaPunctuated::from_iter([quote! { #(#attrs)* obj: #trait_type }]),
                    ReturnType::Default,
                    InterfacesMethodExpr::UnboxAny(quote!(obj.object as *mut #item_type)).to_token_stream().terminated()
                )
            },
            BindingPresentation::Getter { name, field_name, obj_type, field_type, aspect } |
            BindingPresentation::GetterOpaque { name, field_name, obj_type, field_type, aspect } => {
                let var = obj_type.joined_const();
                present_pub_function(
                    aspect,
                    name,
                    CommaPunctuated::from_iter([quote! { obj: #var }]),
                    ReturnType::Type(RArrow::default(), field_type.clone().into()),
                    quote!((*obj).#field_name)
                )
            },
            BindingPresentation::Setter { name, field_name, obj_type, field_type, aspect } |
            BindingPresentation::SetterOpaque { name, field_name, obj_type, field_type, aspect } => {
                let var = obj_type.joined_mut();
                present_pub_function(
                    aspect,
                    name,
                    CommaPunctuated::from_iter([
                        quote!(obj: #var),
                        quote!(value: #field_type),
                    ]),
                    ReturnType::Default,
                    quote!((*obj).#field_name = value;))
            },
            BindingPresentation::RegularFunction { aspect, is_async: true, name, arguments, input_conversions, return_type, output_conversions } => {
                let mut args = Punctuated::from_iter([
                    ArgPresentation::Field(Field { attrs: vec![], vis: Visibility::Inherited, ident: Some(format_ident!("runtime")), colon_token: Default::default(), mutability: FieldMutability::None, ty: parse_quote!(*const std::os::raw::c_void) }),
                ]);
                args.extend(arguments.clone());
                present_pub_function(
                    aspect,
                    name,
                    args,
                    return_type.clone(),
                    quote! {
                            let rt = &*(runtime as *const tokio::runtime::Runtime);
                            let obj = rt.block_on(async {
                                #input_conversions .await
                            });
                            #output_conversions
                        }
                )
            },
            BindingPresentation::RegularFunction { aspect, is_async: false, name, arguments, input_conversions, return_type, output_conversions } => {
                present_pub_function(
                    aspect,
                    name,
                    arguments.clone(),
                    return_type.clone(),
                    quote!(let obj = #input_conversions; #output_conversions)
                )
            },
            BindingPresentation::RegularFunctionWithBody { aspect, name, arguments, return_type, body } => {
                present_pub_function(
                    aspect,
                    name,
                    arguments.clone(),
                    return_type.clone(),
                    body.to_token_stream()
                )
            },
            BindingPresentation::RegularFunction2 { aspect, is_async, name, argument_names, arguments, full_fn_path, input_conversions, return_type, output_conversions } => {
                if *is_async {
                    let mut args = Punctuated::from_iter([
                        ArgPresentation::Field(Field { attrs: vec![], vis: Visibility::Inherited, ident: Some(format_ident!("runtime")), colon_token: Default::default(), mutability: FieldMutability::None, ty: parse_quote!(*const std::os::raw::c_void) }),
                    ]);
                    args.extend(arguments.clone());
                    present_pub_function(
                        aspect,
                        name,
                        args.clone(),
                        return_type.clone(),
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
                        aspect,
                        name,
                        arguments.clone(),
                        return_type.clone(),
                        quote! {
                            #input_conversions;
                            let obj = #full_fn_path(#argument_names);
                            #output_conversions
                        }
                    )
                }
            },
            BindingPresentation::Callback { aspect: (attrs, ..), name, ffi_args, result, conversion } => {
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
            BindingPresentation::TraitVTableInnerFn { attrs, name, name_and_args, output_expression } => {
                quote! {
                    #(#attrs)*
                    pub #name: #name_and_args #output_expression
                }
            }
            BindingPresentation::StaticVTableInnerFn { aspect, name, args, output, body } => {
                present_function(
                    Visibility::Inherited,
                    aspect,
                    name.to_token_stream(),
                    args.clone(),
                    output.clone(),
                    body.clone()
                )
            },
            BindingPresentation::StaticVTableInnerFnDeclaration { name, fn_name } =>
                quote!(#fn_name: #name),
            BindingPresentation::Any { attrs, body } =>
                quote! {
                    #(#attrs)*
                    #body
                }

        }.to_tokens(tokens)
     }
}
