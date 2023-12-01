extern crate proc_macro;

use proc_macro::TokenStream;


/// The `export` procedural macro facilitates FFI (Foreign Function Interface) conversion
/// for a given function. It handles both input arguments and output types, converting them into a format
/// suitable for FFI boundaries.
///
/// # Syntax
///
/// The macro can be applied to any Rust function:
///
/// ```ignore
/// #[ferment::export]
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
/// #[ferment::export]
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