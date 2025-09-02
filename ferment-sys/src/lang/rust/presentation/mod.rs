mod ffi_full_path;
mod naming;
mod ffi_variable;

pub use self::ffi_variable::{resolve_type_variable_via_ffi_full_path, resolve_type_variable_via_maybe_object, resolve_type_variable_via_type, resolve_type_variable};