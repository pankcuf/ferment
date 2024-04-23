extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam, Generics, Lifetime, LifetimeDef, parse_macro_input, parse_quote, WhereClause};


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