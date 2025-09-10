use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, BareFnArg, Generics, parse_quote, ReturnType, Type, Visibility};
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens, Depunctuated};
use crate::composer::{CommaPunctuatedArgs, SemiPunctuatedArgs, SignatureAspect};
use crate::ext::{Accessory, ArgsTransform, Pop, PunctuateOne, Terminated, ToPath, ToType};
use crate::lang::RustSpecification;
use crate::presentation::{ArgPresentation, DictionaryName, InterfacePresentation, InterfacesMethodExpr, Name};

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
        var: Type,
    },
    Getter {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_var: Type,
        field_type: Type,
    },
    Setter {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_var: Type,
        field_type: Type,
    },
    GetterOpaque {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_var: Type,
        field_type: Type,
    },
    SetterOpaque {
        aspect: SignatureAspect<RustSpecification>,
        name: TokenStream2,
        field_name: TokenStream2,
        obj_var: Type,
        field_type: Type,
    },
    ObjAsTrait {
        aspect: SignatureAspect<RustSpecification>,
        name: Name<RustSpecification>,
        item_var: Type,
        trait_type: TokenStream2,
        vtable_name: TokenStream2,
    },
    ObjAsTraitDestructor {
        aspect: SignatureAspect<RustSpecification>,
        name: Name<RustSpecification>,
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
    present_function(Visibility::Public(Default::default()), aspect, name, args, output, body)
}
pub fn present_function<T: ToTokens, N: ToTokens>(
    acc: Visibility,
    (attrs, lifetimes, generics): &SignatureAspect<RustSpecification>,
    name: N,
    args: CommaPunctuated<T>,
    output: ReturnType,
    body: TokenStream2) -> TokenStream2 {
    let signature = match generics {
        None => {
            let comma_lifetimes = CommaPunctuated::from_iter(lifetimes.iter().filter(|lt| lt.ident.ne("static")).map(ToTokens::to_token_stream));
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
    let sig = present_signature(acc, signature);
    quote! {
        #(#attrs)*
        #[no_mangle]
        #sig { #body }
    }
}

pub fn present_signature<A: ToTokens, S: ToTokens>(acc: A, signature: S) -> TokenStream2 {
    quote!(#acc unsafe extern "C" fn #signature)
}

pub fn present_struct<Name: ToTokens, Impl: ToTokens>(
    name: Name,
    attrs: &Vec<Attribute>,
    implementation: Impl
) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #(#attrs)*
        pub struct #name #implementation
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
                    ReturnType::Type(Default::default(), ty.joined_mut().into()),
                    InterfacesMethodExpr::Boxed(quote!(#ffi_path #body_presentation)).to_token_stream())
            },
            Self::VariantConstructor { aspect, name, ty, ctor_arguments, body_presentation} => {
                let variant_path = ty.to_path();
                present_pub_function(
                    aspect,
                    name,
                    ctor_arguments.clone(),
                    ReturnType::Type(Default::default(), variant_path.popped().to_type().joined_mut().into()),
                    InterfacesMethodExpr::Boxed(quote!(#variant_path #body_presentation)).to_token_stream())
            },
            Self::Destructor { aspect, name, var } =>
                present_pub_function(
                    aspect,
                    name,
                    quote!(ffi: #var).punctuate_one(),
                    ReturnType::Default,
                    InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi).to_token_stream().terminated()
                ),
            Self::ObjAsTrait { aspect, name, item_var, trait_type, vtable_name } =>
                present_pub_function(
                    aspect,
                    name,
                    quote!(obj: #item_var).punctuate_one(),
                    ReturnType::Type(Default::default(), trait_type.to_type().into()),
                    quote!(#trait_type {
                        object: obj as *const (),
                        vtable: &#vtable_name
                    })
                ),
            Self::ObjAsTraitDestructor { aspect, name, item_type, trait_type } => {
                let attrs = &aspect.0;
                present_pub_function(
                    aspect,
                    name,
                    quote!(#(#attrs)* obj: #trait_type).punctuate_one(),
                    ReturnType::Default,
                    InterfacesMethodExpr::UnboxAny(quote!(obj.object as *mut #item_type)).to_token_stream().terminated()
                )
            },
            Self::Getter { name, field_name, obj_var, field_type, aspect } |
            Self::GetterOpaque { name, field_name, obj_var, field_type, aspect } =>
                present_pub_function(
                    aspect,
                    name,
                    quote!(obj: #obj_var).punctuate_one(),
                    ReturnType::Type(Default::default(), field_type.clone().into()),
                    quote!((*obj).#field_name)
                ),
            Self::Setter { name, field_name, obj_var, field_type, aspect } |
            Self::SetterOpaque { name, field_name, obj_var, field_type, aspect } =>
                present_pub_function(
                    aspect,
                    name,
                    CommaPunctuated::from_iter([quote!(obj: #obj_var), quote!(value: #field_type)]),
                    ReturnType::Default,
                    quote!((*obj).#field_name = value;)),
            Self::RegularFunction { aspect, is_async: true, name, arguments, input_conversions, return_type, output_conversions } => {
                let mut args = ArgPresentation::Field(crate::ast::inherited_named_field(format_ident!("runtime"), parse_quote!(*const std::os::raw::c_void))).punctuate_one();
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
            Self::RegularFunction { aspect, is_async: false, name, arguments, input_conversions, return_type, output_conversions } =>
                present_pub_function(
                    aspect,
                    name,
                    arguments.clone(),
                    return_type.clone(),
                    quote!(let obj = #input_conversions; #output_conversions)
                ),
            Self::RegularFunctionWithBody { aspect, name, arguments, return_type, body } =>
                present_pub_function(
                    aspect,
                    name,
                    arguments.clone(),
                    return_type.clone(),
                    body.to_token_stream()
                ),
            Self::RegularFunction2 { aspect, is_async: true, name, argument_names, arguments, full_fn_path, input_conversions, return_type, output_conversions } => {
                let mut args = ArgPresentation::Field(crate::ast::inherited_named_field(format_ident!("runtime"), parse_quote!(*const std::os::raw::c_void))).punctuate_one();
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
            },
            Self::RegularFunction2 { aspect, is_async: false, name, argument_names, arguments, full_fn_path, input_conversions, return_type, output_conversions } =>
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
                ),
            Self::Callback { aspect: (attrs, ..), name, ffi_args, result, conversion } => {
                let result_impl = match result {
                    ReturnType::Default => Default::default(),
                    ReturnType::Type(_, ref ty) => {
                        let dtor_signature = present_signature(TokenStream2::default(), quote!((result: #ty)));
                        quote! { #result, destructor: #dtor_signature }
                    }
                };
                let caller_signature = present_signature(TokenStream2::default(), quote!((#ffi_args) #result_impl));
                let implementation = quote!({ caller: #caller_signature, });
                let definition = present_struct(name, attrs, implementation);
                quote! {
                    #definition
                    #conversion
                }
            }
            Self::StaticVTable { attrs, name, fq_trait_vtable, methods_declarations, methods_implementations, bindings } => quote! {
                #[no_mangle]
                #(#attrs)*
                pub static #name: #fq_trait_vtable = {
                    #methods_implementations
                    #fq_trait_vtable { #methods_declarations }
                };
                #bindings
            },
            Self::TraitVTableInnerFn { attrs, name, name_and_args, output_expression } =>
                quote!(#(#attrs)* pub #name: #name_and_args #output_expression),
            Self::StaticVTableInnerFn { aspect, name, args, output, body } =>
                present_function(Visibility::Inherited, aspect, name, args.clone(), output.clone(), body.clone()),
            Self::StaticVTableInnerFnDeclaration { name, fn_name } =>
                quote!(#fn_name: #name),
            Self::Any { attrs, body } =>
                quote!(#(#attrs)* #body)

        }.to_tokens(tokens)
     }
}
