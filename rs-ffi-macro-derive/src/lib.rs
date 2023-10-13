extern crate proc_macro;

use proc_macro::TokenStream;


/// The `impl_ffi_fn_conv` procedural macro facilitates FFI (Foreign Function Interface) conversion
/// for a given function. It handles both input arguments and output types, converting them into a format
/// suitable for FFI boundaries.
///
/// # Syntax
///
/// The macro can be applied to any Rust function:
///
/// ```ignore
/// #[impl_ffi_fn_conv]
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
/// #[impl_ffi_fn_conv]
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
// #[proc_macro_attribute]
// pub fn impl_ffi_fn_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     // TokenStream::from(quote!())
//     input
//     // Expansion::from(ItemConversion::Fn(parse_macro_input!(input as ItemFn)))
//     //     .present()
//     //     .into()
// }
//
// #[proc_macro_attribute]
// pub fn impl_ffi_conv(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // println!(">> process item {}", item);
//     // TokenStream::from(quote!())
//     // let attrs = parse_macro_input!(attr as AttributeArgs);
//     // let target_name = match attrs.first() {
//     //     Some(NestedMeta::Lit(literal)) => {
//     //         format_ident!("{}", literal.to_token_stream().to_string())
//     //     }
//     //     Some(NestedMeta::Meta(Meta::Path(path))) => path.segments.first().unwrap().ident.clone(),
//     //     _ => {
//     //         // use default rules
//     //         // for unnamed structs like UInt256 -> #target_name = [u8; 32]
//     //         // for named structs -> generate ($StructName)FFI
//     //         input.ident.clone()
//     //     }
//     // };
//     item
//     // Expansion::from(ItemConversion::from(parse_macro_input!(item as DeriveInput)))
//     //     .present()
//     //     .into()
// }
//
// #[proc_macro_attribute]
// pub fn impl_ffi_ty_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     // just marker macro
//     input
//     // TokenStream::from(quote!())
//     // Expansion::from(ItemConversion::try_from(&parse_macro_input!(input as Item)).unwrap())
//     //     .present()
//     //     .into()
// }

#[proc_macro_attribute]
pub fn ferment(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

// #[proc_macro_attribute]
// pub fn ffi_dictionary(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     let item =
//     TokenStream::from(ItemConversion::try_from(&parse_macro_input!(input as Item))
//         .map(|conversion| conversion.expand_all_types())
//         .map_or(quote!(), |expansions| quote!(#(#expansions)*)))
// }
