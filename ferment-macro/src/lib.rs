extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Generics, Lit, Meta, MetaNameValue, parse_macro_input, parse_quote, Path, Variant};
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
    input
}


#[proc_macro_attribute]
pub fn register(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}


#[proc_macro_derive(CompositionContext)]
pub fn composition_context_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let expanded = quote!(impl crate::composition::CompositionContext for #name {});

    TokenStream::from(expanded)
}

// #[proc_macro_derive(Parent)]
// pub fn composer_parent_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//
//     let name = &input.ident;
//     let expanded = quote! {
//         impl<Parent: crate::shared::SharedAccess> crate::shared::HasParent<Parent> for #name<Parent>  {
//             fn set_parent(&mut self, parent: &Parent) {
//                 self.parent = Some(parent.clone_container());
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(crate::shared::SharedAccess));
        }
    }

    generics
}

#[proc_macro_derive(Parent)]
pub fn composer_parent_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics crate::shared::HasParent<Parent> for #name #ty_generics #where_clause {
            fn set_parent(&mut self, parent: &Parent) {
                self.parent = Some(parent.clone_container());
            }
        }
    };
    TokenStream::from(expanded)
}

// #[proc_macro_derive(MethodCall, attributes(namespace, method))]
// pub fn method_call_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//
//     let namespace = input.attrs.iter().find_map(|attr| {
//         if attr.path.is_ident("namespace") {
//             match attr.parse_meta() {
//                 Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.value()),
//                 _ => None,
//             }
//         } else {
//             None
//         }
//     }).expect("namespace attribute is required");
//
//     let method = input.attrs.iter().find_map(|attr| {
//         if attr.path.is_ident("method") {
//             match attr.parse_meta() {
//                 Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.value()),
//                 _ => None,
//             }
//         } else {
//             None
//         }
//     }).expect("method attribute is required");
//
//     let namespace_tokens: TokenStream2 = namespace.parse().expect("Invalid namespace");
//     let method_tokens: TokenStream2 = method.parse().expect("Invalid method");
//
//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
//     let (methods, exprs): (Vec<TokenStream2>, Vec<TokenStream2>) = if let Data::Enum(data) = &input.data {
//         data.variants.iter()
//             .map(|Variant { ident, .. }| (
//                 quote!(Self::#ident(..) => <Self as crate::naming::MethodCall>::Method::#ident,),
//                 quote!(Self::#ident(expr) => expr,)))
//             .unzip()
//     } else {
//         return TokenStream::new();
//     };
//
//     let expanded = quote! {
//         impl #impl_generics crate::naming::MethodCall for #name #ty_generics #where_clause {
//             type Namespace = #namespace_tokens;
//             type Method = #method_tokens;
//
//             fn ns(&self) -> Self::Namespace {
//                 #namespace_tokens
//             }
//
//             fn method(&self) -> Self::Method {
//                 match self {
//                     #(#methods)*
//                 }
//             }
//
//             fn expr(&self) -> &TokenStream2 {
//                 match self {
//                     #(#exprs)*
//                 }
//             }
//         }
//
//         impl #impl_generics ToTokens for #name #ty_generics #where_clause {
//             fn to_tokens(&self, dst: &mut TokenStream2) {
//                 (self as &dyn crate::naming::MethodCall<Namespace = #namespace_tokens, Method = #method_tokens>).to_tokens(dst)
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }

// #[proc_macro_derive(MethodCall, attributes(namespace))]
// pub fn method_call_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//     let namespace = input.attrs.iter()
//         .find(|attr| attr.path.is_ident("namespace"))
//         .and_then(|attr| match attr.parse_meta() {
//             Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.parse::<Path>().expect("Invalid namespace")),
//             _ => None
//         });
//     let expression_enum_name = format_ident!("{}Expr", name);
//     let mut expression_variants = Punctuated::<TokenStream2, Comma>::new();
//     let mut methods = Punctuated::<TokenStream2, Comma>::new();
//     let mut exprs = Punctuated::<TokenStream2, Comma>::new();
//     let mut nss = Punctuated::<TokenStream2, Comma>::new();
//     if let Data::Enum(data) = &input.data {
//         data.variants.iter()
//             .for_each(|Variant { ident, fields, .. }| {
//                 match fields {
//                     Fields::Named(FieldsNamed { named , ..}) => {
//                         expression_variants.push(quote!(#ident(#named, syn::__private::TokenStream2)));
//                         let path = named.iter().cloned().collect::<Punctuated<_, Colon2>>();
//                         nss.push(quote!(Self::#ident(#named, ..) => quote!(#namespace::)));
//                         methods.push(quote!(Self::#ident(#named, ..) => <Self as crate::naming::MethodCall>::Method::#ident));
//                     }
//                     Fields::Unnamed(FieldsUnnamed { unnamed, ..}) => {
//
//                         // let path = unnamed.iter().map(|l| ).collect::<Punctuated<syn::__private::TokenStream2, Colon2>>();
//                         methods.push(quote!(Self::#ident(#unnamed, ..) => <Self as crate::naming::MethodCall>::Method::#ident));
//                         expression_variants.push(quote!(#ident(#unnamed, syn::__private::TokenStream2)));
//                     }
//                     Fields::Unit => {
//                         methods.push(quote!(Self::#ident(..) => <Self as crate::naming::MethodCall>::Method::#ident));
//                         expression_variants.push(quote!(#ident(syn::__private::TokenStream2)));
//                     }
//                 }
//                 exprs.push(quote!(Self::#ident(.., expr) => expr))
//             });
//     }
//     let expanded = quote! {
//         #[derive(Clone, Debug)]
//         pub enum #expression_enum_name {
//             #expression_variants
//         }
//         impl crate::naming::MethodCall for #expression_enum_name {
//             fn ns(&self) -> TokenStream2 {
//                 match self { #nss }
//             }
//             fn method(&self) -> TokenStream2 {
//                 match self { #methods }
//             }
//             fn expr(&self) -> &TokenStream2 {
//                 match self { #exprs }
//             }
//         }
//         impl ToTokens for #expression_enum_name {
//             fn to_tokens(&self, dst: &mut TokenStream2) {
//                 (self as &dyn crate::naming::MethodCall).to_tokens(dst)
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }

// #[proc_macro_derive(MethodCall, attributes(namespace))]
// pub fn method_call_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//     let namespace = input.attrs.iter()
//         .find(|attr| attr.path.is_ident("namespace"))
//         .and_then(|attr| match attr.parse_meta() {
//             Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.parse::<Path>().expect("Invalid namespace")),
//             _ => None
//         })
//         .expect("namespace attribute is required");
//
//     let expression_enum_name = format_ident!("{}Expr", name);
//     let mut expression_variants = Punctuated::<TokenStream2, Comma>::new();
//     let mut methods = Punctuated::<TokenStream2, Comma>::new();
//     let mut exprs = Punctuated::<TokenStream2, Comma>::new();
//
//     if let Data::Enum(data) = &input.data {
//         for Variant { ident, fields, .. } in &data.variants {
//             expression_variants.push(quote!(#ident(syn::__private::TokenStream2)));
//             match fields {
//                 Fields::Unnamed(FieldsUnnamed { unnamed, ..}) => {
//                     if unnamed.is_empty() {
//                         exprs.push(quote!(Self::#ident(expr) => expr));
//                         methods.push(quote!(Self::#ident(..) => #name::#ident.to_token_stream()));
//                     } else {
//                         exprs.push(quote!(Self::#ident(.., expr) => expr));
//                         let args = unnamed.iter().enumerate().map(|(index, field)| format_ident!("o_{}", index).to_token_stream()).collect::<Punctuated<_, Comma>>();
//                         let args_cloned = args.iter().map(|arg| quote!(#arg.clone())).collect::<Punctuated<_, Comma>>();
//                         methods.push(quote!(Self::#ident(#args, ..) => #name::#ident(#args_cloned).to_token_stream()));
//                     }
//                 },
//                 Fields::Named(FieldsNamed { named, .. }) => {
//                     if named.is_empty() {
//                         exprs.push(quote!(Self::#ident(expr) => expr));
//                         methods.push(quote!(Self::#ident(..) => #name::#ident.to_token_stream()));
//                     } else {
//                         exprs.push(quote!(Self::#ident(.., expr) => expr));
//                         let args = named.iter().map(|field| field.ident.to_token_stream()).collect::<Punctuated<_, Comma>>();
//                         let args_cloned = args.iter().map(|arg| quote!(#arg.clone())).collect::<Punctuated<_, Comma>>();
//                         methods.push(quote!(Self::#ident(#args, ..) => #name::#ident(#args_cloned).to_token_stream()));
//                     }
//
//                 },
//                 Fields::Unit => {
//                     methods.push(quote!(Self::#ident(..) => #name::#ident.to_token_stream()));
//                     exprs.push(quote!(Self::#ident(expr) => expr));
//                 }
//             };
//         }
//     }
//
//     let expanded = quote! {
//         pub enum #expression_enum_name {
//             #expression_variants
//         }
//         impl crate::naming::MethodCall for #expression_enum_name {
//             fn method(&self) -> TokenStream2 {
//                 let method_ns = match self {
//                     #methods
//                 };
//                 quote!(#namespace::#method_ns)
//             }
//             fn expr(&self) -> &TokenStream2 {
//                 match self {
//                     #exprs
//                 }
//             }
//         }
//         impl ToTokens for #expression_enum_name {
//             fn to_tokens(&self, dst: &mut TokenStream2) {
//                 (self as &dyn crate::naming::MethodCall).to_tokens(dst)
//             }
//         }
//     };
//     TokenStream::from(expanded)
// }


// #[proc_macro_derive(MethodCall, attributes(namespace))]
// pub fn method_call_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//     let namespace = input.attrs.iter()
//         .find(|attr| attr.path.is_ident("namespace"))
//         .and_then(|attr| match attr.parse_meta() {
//             Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.parse::<Path>().expect("Invalid namespace")),
//             _ => None
//         })
//         .expect("namespace attribute is required");
//
//     let expression_enum_name = format_ident!("{}Expr", name);
//     let mut expression_variants = Punctuated::<TokenStream2, Comma>::new();
//     let mut methods = Punctuated::<TokenStream2, Comma>::new();
//     let mut exprs = Punctuated::<TokenStream2, Comma>::new();
//
//     if let Data::Enum(data) = &input.data {
//         for Variant { ident, fields, .. } in &data.variants {
//             expression_variants.push(quote!(#ident(syn::__private::TokenStream2),));
//             match fields {
//                 Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
//                     if unnamed.is_empty() {
//                         exprs.push(quote!(Self::#ident(expr) => expr));
//                         methods.push(quote!(Self::#ident(..) => #name::#ident.to_token_stream(),));
//                     } else {
//                         exprs.push(quote!(Self::#ident(.., expr) => expr));
//                         let args = unnamed.iter().enumerate().map(|(index, _)| format_ident!("o_{}", index).to_token_stream()).collect::<Punctuated<_, Comma>>();
//                         let args_cloned = args.iter().map(|arg| quote!(#arg.clone())).collect::<Punctuated<_, Comma>>();
//                         methods.push(quote!(Self::#ident(#args, ..) => #name::#ident(#args_cloned).to_token_stream(),));
//                     }
//                 }
//                 Fields::Named(FieldsNamed { named, .. }) => {
//                     if named.is_empty() {
//                         exprs.push(quote!(Self::#ident(expr) => expr));
//                         methods.push(quote!(Self::#ident(..) => #name::#ident.to_token_stream(),));
//                     } else {
//                         exprs.push(quote!(Self::#ident(.., expr) => expr));
//                         let args = named.iter().map(|field| field.ident.to_token_stream()).collect::<Punctuated<_, Comma>>();
//                         let args_cloned = args.iter().map(|arg| quote!(#arg.clone())).collect::<Punctuated<_, Comma>>();
//                         methods.push(quote!(Self::#ident(#args, ..) => #name::#ident(#args_cloned).to_token_stream(),));
//                     }
//                 }
//                 Fields::Unit => {
//                     methods.push(quote!(Self::#ident(..) => #name::#ident.to_token_stream(),));
//                     exprs.push(quote!(Self::#ident(expr) => expr));
//                 }
//             };
//         }
//     }
//
//     let expanded = quote! {
//         pub enum #expression_enum_name {
//             #expression_variants
//         }
//         impl crate::naming::MethodCall for #expression_enum_name {
//             fn method(&self) -> TokenStream2 {
//                 match self {
//                     #methods
//                 }
//             }
//             fn expr(&self) -> &TokenStream2 {
//                 match self {
//                     #exprs
//                 }
//             }
//         }
//         impl ToTokens for #expression_enum_name {
//             fn to_tokens(&self, dst: &mut TokenStream2) {
//                 (self as &dyn crate::naming::MethodCall).to_tokens(dst)
//             }
//         }
//     };
//     TokenStream::from(expanded)
// }

// #[proc_macro_derive(MethodCall, attributes(namespace))]
// pub fn method_call_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//     let namespace = input.attrs.iter()
//         .find(|attr| attr.path.is_ident("namespace"))
//         .and_then(|attr| match attr.parse_meta() {
//             Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.parse::<Path>().expect("Invalid namespace")),
//             _ => None
//         })
//         .expect("namespace attribute is required");
//
//     let expression_enum_name = format_ident!("{}Expression", name);
//     let mut expression_variants = Punctuated::<TokenStream2, Comma>::new();
//     let mut methods = Punctuated::<TokenStream2, Comma>::new();
//     let mut exprs = Punctuated::<TokenStream2, Comma>::new();
//
//     if let Data::Enum(data) = &input.data {
//         for Variant { ident, fields, .. } in &data.variants {
//             match fields {
//                 Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
//                     if unnamed.is_empty() {
//                         expression_variants.push(quote!(#ident(syn::__private::TokenStream2),));
//                         methods.push(quote!(Self::#ident(.., expr) => #name::#ident.to_token_stream(),));
//                         exprs.push(quote!(Self::#ident(.., expr) => expr,));
//                     } else {
//                         let additional = quote!(syn::__private::TokenStream2);
//                         let fields = unnamed.iter().map(|_| additional.clone()).collect::<Punctuated<_, Comma>>();
//                         expression_variants.push(quote!(#ident(#fields, syn::__private::TokenStream2),));
//                         methods.push(quote!(Self::#ident(#fields, .., expr) => #name::#ident(#fields).to_token_stream(),));
//                         exprs.push(quote!(Self::#ident(.., expr) => expr,));
//                     }
//                 },
//                 Fields::Named(FieldsNamed { named, .. }) => {
//                     let additional = quote!(syn::__private::TokenStream2);
//                     let fields = named.iter().map(|_| additional.clone()).collect::<Punctuated<_, Comma>>();
//                     if named.is_empty() {
//                         expression_variants.push(quote!(#ident(syn::__private::TokenStream2)));
//                         methods.push(quote!(Self::#ident(.., expr) => #name::#ident.to_token_stream()));
//                         exprs.push(quote!(Self::#ident(.., expr) => expr));
//                     } else {
//                         expression_variants.push(quote!(#ident(#fields, syn::__private::TokenStream2)));
//                         methods.push(quote!(Self::#ident(#fields, .., expr) => #name::#ident(#fields).to_token_stream()));
//                         exprs.push(quote!(Self::#ident(.., expr) => expr));
//                     }
//                 },
//                 Fields::Unit => {
//                     expression_variants.push(quote!(#ident(syn::__private::TokenStream2)));
//                     methods.push(quote!(Self::#ident(.., expr) => #name::#ident.to_token_stream()));
//                     exprs.push(quote!(Self::#ident(.., expr) => expr));
//                 }
//             }
//         }
//     }
//
//     let expanded = quote! {
//         pub enum #expression_enum_name {
//             #expression_variants
//         }
//         impl crate::naming::MethodCall for #expression_enum_name {
//             fn method(&self) -> TokenStream2 {
//                 match self {
//                     #methods
//                 }
//             }
//             fn expr(&self) -> &TokenStream2 {
//                 match self {
//                     #exprs
//                 }
//             }
//         }
//         impl ToTokens for #expression_enum_name {
//             fn to_tokens(&self, dst: &mut TokenStream2) {
//                 (self as &dyn crate::naming::MethodCall).to_tokens(dst)
//             }
//         }
//     };
//     TokenStream::from(expanded)
// }

// #[proc_macro_derive(MethodCall, attributes(namespace))]
// pub fn method_call_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//     let namespace = input.attrs.iter()
//         .find(|attr| attr.path.is_ident("namespace"))
//         .and_then(|attr| match attr.parse_meta() {
//             Ok(Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. })) => Some(lit.parse::<Path>().expect("Invalid namespace")),
//             _ => None
//         })
//         .expect("namespace attribute is required");
//
//     let expression_enum_name = format_ident!("{}Expr", name);
//     let mut expression_variants = Punctuated::<TokenStream2, Comma>::new();
//     let mut methods = Punctuated::<TokenStream2, Comma>::new();
//     let mut exprs = Punctuated::<TokenStream2, Comma>::new();
//
//     if let Data::Enum(data) = &input.data {
//         for Variant { ident, fields, .. } in &data.variants {
//             match fields {
//                 Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
//                     let field_count = unnamed.len();
//                     let field_names = (0..field_count).map(|i| format_ident!("field{}", i)).collect::<Vec<_>>();
//                     let field_types = unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>();
//                     expression_variants.push(quote!(#ident(#(#field_types,)* syn::__private::TokenStream2)));
//                     methods.push(quote!(Self::#ident(#(#field_names,)* _) => #name::#ident(#(#field_names.clone(),)*).to_token_stream()));
//                     exprs.push(quote!(Self::#ident(#(#field_names,)* expr) => expr));
//                 },
//                 Fields::Named(FieldsNamed { named, .. }) => {
//                     let field_names = named.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>();
//                     let field_types = named.iter().map(|f| &f.ty).collect::<Vec<_>>();
//                     expression_variants.push(quote!(#ident { #(#field_names: #field_types,)* expr: syn::__private::TokenStream2 }));
//                     methods.push(quote!(Self::#ident { #(#field_names,)* .. } => #name::#ident { #(#field_names: #field_names.clone(),)* }.to_token_stream()));
//                     exprs.push(quote!(Self::#ident { #(#field_names,)* expr } => expr));
//                 },
//                 Fields::Unit => {
//                     expression_variants.push(quote!(#ident(syn::__private::TokenStream2)));
//                     methods.push(quote!(Self::#ident(_) => #name::#ident.to_token_stream()));
//                     exprs.push(quote!(Self::#ident(expr) => expr));
//                 }
//             }
//         }
//     }
//
//     let expanded = quote! {
//         pub enum #expression_enum_name {
//             #expression_variants
//         }
//         impl crate::naming::MethodCall for #expression_enum_name {
//             fn method(&self) -> TokenStream2 {
//                 match self {
//                     #methods
//                 }
//             }
//             fn expr(&self) -> &TokenStream2 {
//                 match self {
//                     #exprs
//                 }
//             }
//         }
//         impl ToTokens for #expression_enum_name {
//             fn to_tokens(&self, dst: &mut TokenStream2) {
//                 (self as &dyn crate::naming::MethodCall).to_tokens(dst)
//             }
//         }
//     };
//     TokenStream::from(expanded)
// }

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
        impl crate::naming::MethodCall for #expression_enum_name {
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
                (self as &dyn crate::naming::MethodCall).to_tokens(dst)
            }
        }
    };
    TokenStream::from(expanded)
}