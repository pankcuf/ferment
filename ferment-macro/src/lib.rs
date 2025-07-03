extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, FnArg, ItemFn, Lit, Meta, MetaNameValue, parse_macro_input, parse_quote, Path, PatType, Signature, Variant};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;


/// The `export` procedural macro facilitates FFI (Foreign Function Interface) conversion
/// for a given function. It handles both input arguments and output types, converting them into a format
/// suitable for FFI boundaries.
///
/// # Syntax
///
/// The macro can be applied to any Rust function:
///
/// ```ignore
/// #[ferment_macro::export]
/// pub fn my_function(arg1: MyType1, arg2: MyType2) -> MyReturnType {
///     // function implementation
/// }
/// ```
///
/// # Output
///
/// The macro will automatically generate additional FFI-compatible code around the annotated function.
/// It converts the function into a form that can be easily invoked from C/C++ code.
///
/// ## Safety
///
/// This macro generates safety documentation specific to the function, covering the expectations
/// and constraints of the FFI boundary.
///
/// ## Function Conversion
///
/// The macro processes the function's input arguments and return type, performing necessary transformations
/// like memory allocation/deallocation, pointer conversion, etc., to make them FFI-compatible.
///
/// # Panics
///
/// - The macro will panic if any of the function's argument types are not supported for conversion.
/// - The macro will also panic if the function's return type is not supported for conversion.
///
/// # Example
///
/// ```ignore
/// #[ferment_macro::export]
/// pub fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// ```
///
/// After applying the macro, the function can be safely invoked from C/C++ code.
///
/// # Note
///
/// This macro is intended for internal use and should be used cautiously,
/// understanding the risks associated with FFI calls.
///
/// # See Also
///
/// # Limitations
///
/// - The macro currently does not support Rust async functions.
/// - Nested data structures may not be fully supported.
///
#[proc_macro_attribute]
pub fn export(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // input
    let input = TokenStream2::from(input);
    let expanded = quote! {
        #[doc = "@ferment::export"]
        #input
    };
    TokenStream::from(expanded)

}


#[proc_macro_attribute]
pub fn register(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ty = syn::parse::<Path>(attr).expect("Expected a path");
    let ty_str = quote!(#ty).to_string();
    let input = TokenStream2::from(input);

    let expanded = quote! {
        #[doc = concat!("@ferment::register(", #ty_str, ")")]
        #[repr(C)]
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn opaque(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    // let expanded = quote! {
    //     #[repr(C)]
    //     #input
    // };
    // input
    // TokenStream::from(expanded)

    let input = TokenStream2::from(input);
    let expanded = quote! {
        #[doc = "@ferment::opaque"]
        #input
    };
    TokenStream::from(expanded)

}


#[proc_macro_derive(CompositionContext)]
pub fn composition_context_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let expanded = quote!(impl crate::composable::CompositionContext for #name {});

    TokenStream::from(expanded)
}

#[proc_macro_derive(MethodCall, attributes(namespace))]
pub fn method_call_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let namespace = input.attrs.iter()
        .find(|attr| attr.path.is_ident("namespace"))
        .and_then(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.parse::<Path>().expect("Invalid namespace")),
            _ => None
        })
        .expect("namespace attribute is required");

    let expression_enum_name = format_ident!("{}Expr", name);
    let mut expression_variants = Punctuated::<TokenStream2, Comma>::new();
    let mut methods = Punctuated::<TokenStream2, Comma>::new();
    let mut exprs = Punctuated::<TokenStream2, Comma>::new();

    if let Data::Enum(data) = &input.data {
        for Variant { ident, fields, .. } in &data.variants {
            match fields {
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let field_count = unnamed.len();
                    let field_names = (0..field_count).map(|i| format_ident!("field{}", i)).collect::<Vec<_>>();
                    let field_types = unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>();
                    expression_variants.push(quote!(#ident(#(#field_types,)* syn::__private::TokenStream2)));
                    methods.push(quote!(#expression_enum_name::#ident(#(#field_names,)* _) => #name::#ident(#(#field_names.clone(),)*).to_token_stream()));
                    exprs.push(quote!(#expression_enum_name::#ident(#(#field_names,)* expr) => expr));
                },
                Fields::Named(FieldsNamed { named, .. }) => {
                    let field_names = named.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>();
                    let field_types = named.iter().map(|f| &f.ty).collect::<Vec<_>>();
                    expression_variants.push(quote!(#ident { #(#field_names: #field_types,)* expr: syn::__private::TokenStream2 }));
                    methods.push(quote!(#expression_enum_name::#ident { #(#field_names,)* .. } => #name::#ident { #(#field_names: #field_names.clone(),)* }.to_token_stream()));
                    exprs.push(quote!(#expression_enum_name::#ident { #(#field_names,)* expr } => expr));
                },
                Fields::Unit => {
                    expression_variants.push(quote!(#ident(syn::__private::TokenStream2)));
                    methods.push(quote!(#expression_enum_name::#ident(_) => #name::#ident.to_token_stream()));
                    exprs.push(quote!(#expression_enum_name::#ident(expr) => expr));
                }
            }
        }
    }

    let expanded = quote! {
        #[derive(Clone, Debug)]
        pub enum #expression_enum_name {
            #expression_variants
        }
        impl crate::presentation::MethodCall for #expression_enum_name {
            fn method(&self) -> TokenStream2 {
                let mut tokens = TokenStream2::new();
                let method = match self {
                    #methods
                };
                let ns = syn::punctuated::Punctuated::<_, syn::token::Colon2>::from_iter([quote!(#namespace), method]);
                tokens.append_all(vec![ns.to_token_stream()]);
                tokens
            }
            fn expr(&self) -> &TokenStream2 {
                match self {
                    #exprs
                }
            }
        }
        impl ToTokens for #expression_enum_name {
            fn to_tokens(&self, dst: &mut TokenStream2) {
                (self as &dyn crate::presentation::MethodCall).to_tokens(dst)
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Display)]
pub fn to_string_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("#[derive(ToString)] is only defined for enums"),
    };
    let match_arms = data.variants.iter().map(|Variant { ident, fields, .. } | {
        match fields {
            Fields::Named(fields) => quote! { Self::#ident { .. } => format!("{}{}", stringify!(#ident), stringify!(#fields)), },
            Fields::Unnamed(fields) => quote! { Self::#ident(..) => format!("{}{}", stringify!(#ident), stringify!(#fields)), },
            Fields::Unit => quote! { Self::#ident => format!("{}", stringify!(#ident)), }
        }
        // match fields {
        //     Fields::Named(_) => quote! { Self::#ident { .. } => stringify!(#ident).to_string(), },
        //     Fields::Unnamed(_) => quote! { Self::#ident(..) => stringify!(#ident).to_string(), },
        //     Fields::Unit => quote! { Self::#ident => stringify!(#ident).to_string(), }
        // }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // impl #impl_generics crate::composer::AttrComposable<SPEC::Attr> for #ident #ty_generics #where_clause {

    let expanded = quote! {
        impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    #(#match_arms)*
                }.as_str())
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(BasicComposerOwner)]
pub fn basic_composer_owner_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, generics, .. } = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics crate::composer::BasicComposerOwner<crate::presentable::Context, LANG, SPEC, Gen> for #ident #ty_generics #where_clause {
            fn base(&self) -> &crate::composer::BasicComposer<crate::composer::ParentComposer<Self>, LANG> {
                &self.base
            }
        }
    };
    TokenStream::from(expanded)
}
#[proc_macro_derive(ComposerBase)]
pub fn composer_base_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, generics, .. } = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics crate::composer::BasicComposerOwner<LANG, SPEC> for #ident #ty_generics #where_clause {
            fn base(&self) -> &crate::composer::BasicComposerLink<LANG, SPEC, Self> {
                &self.base
            }
        }
        impl #impl_generics crate::composer::AttrComposable<SPEC::Attr> for #ident #ty_generics #where_clause {
            fn compose_attributes(&self) -> SPEC::Attr {
                self.base().compose_attributes()
            }
        }
        impl #impl_generics crate::composer::GenericsComposable<SPEC::Gen> for #ident #ty_generics #where_clause {
            fn compose_generics(&self) -> SPEC::Gen {
                self.base().compose_generics()
            }
        }
        impl #impl_generics crate::composer::LifetimesComposable<SPEC::Lt> for #ident #ty_generics #where_clause {
            fn compose_lifetimes(&self) -> SPEC::Lt {
                self.base().compose_lifetimes()
            }
        }
        impl #impl_generics crate::composer::SourceAccessible for #ident #ty_generics #where_clause {
            fn context(&self) -> &ComposerLink<crate::context::ScopeContext> {
                self.base().context()
            }
        }
        impl #impl_generics crate::composer::TypeAspect<SPEC::TYC> for #ident #ty_generics #where_clause {
            fn type_context_ref(&self) -> &SPEC::TYC {
                self.base().type_context_ref()
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn debug_io(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn { sig: Signature { ident: ref method_name, ref inputs, .. }, ref block, .. } = input;
    let args: Vec<_> = inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { pat, .. }) = arg {
            Some(quote! { #pat })
        } else {
            None
        }
    }).collect();
    let fn_name = format!("{}", method_name);

    let args_str = args.iter().map(ToTokens::to_token_stream).collect::<Vec<_>>();
    let new_block = quote! {{
        let debug_str = #fn_name;
        let result = {
            #block
        };
        println!("{}({:?}) -> {:?}", debug_str, #(#args_str),*, result);
        result
    }};
    let mut output = input.clone();

    output.block = parse_quote!(#new_block);
    TokenStream::from(quote! { #output })
}




