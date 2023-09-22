use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use quote::{quote, ToTokens};
use syn::{Path, Type, TypePath};
use syn::__private::TokenStream2;
use crate::path_conversion::PathConversion;
use crate::helper::mangle_path;

pub struct TypePathComposition(pub Type, pub Path);

impl PartialEq for TypePathComposition {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.0.to_token_stream(), self.1.to_token_stream()];
        let other_tokens = [other.0.to_token_stream(), other.1.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypePathComposition {}

impl Hash for TypePathComposition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
        self.1.to_token_stream().to_string().hash(state);
    }
}

pub fn add_generic_type(field_type: &Type, generics: &mut HashSet<TypePathComposition>) {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if let PathConversion::Vec(path) | PathConversion::Map(path) = PathConversion::from(path) {
            generics.insert(TypePathComposition(field_type.clone(), path.clone()));
        }
    }
}

fn vec_ffi_exp(name: TokenStream2, t: TokenStream2, mangled_t: TokenStream2, decode: TokenStream2, encode: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct #name {
            pub count: usize,
            pub values: *mut #mangled_t,
        }
        impl rs_ffi_interfaces::FFIConversion<Vec<#t>> for #name {
            unsafe fn ffi_from_const(ffi: *const #name) -> Vec<#t> {
                let ffi_ref = &*ffi;
                rs_ffi_interfaces::FFIVecConversion::decode(ffi_ref)
            }
            unsafe fn ffi_to_const(obj: Vec<#t>) -> *const #name {
                rs_ffi_interfaces::FFIVecConversion::encode(obj)
            }
            unsafe fn destroy(ffi: *mut #name) {
                rs_ffi_interfaces::unbox_any(ffi);
            }
        }
        impl rs_ffi_interfaces::FFIVecConversion for #name {
            type Value = #t;
            unsafe fn decode(&self) -> Vec<Self::Value> { #decode }
            unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self { #encode }
        }
        impl Drop for #name {
            fn drop(&mut self) {
                unsafe {
                    #drop_code
                }
            }
        }
    }
}
pub fn vec_ffi_simple_expansion(name: TokenStream2, t: &Path) -> TokenStream2 {
    let decode = quote!(rs_ffi_interfaces::from_simple_vec(self.values as *const Self::Value, self.count));
    let encode = quote!(rs_ffi_interfaces::boxed(Self { count: obj.len(), values: rs_ffi_interfaces::boxed_vec(obj) }));
    let drop_code = quote!({rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);});
    vec_ffi_exp(name, quote!(#t), quote!(#t), decode, encode, drop_code)
}

pub fn vec_ffi_complex_expansion(name: TokenStream2, t: &Path) -> TokenStream2 {
    let value_path = mangle_path(t);
    let decode = quote!({
        let count = self.count;
        let values = self.values;
        (0..count)
            .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
            .collect()
    });
    let encode = quote!({
        rs_ffi_interfaces::boxed(Self { count: obj.len(), values: rs_ffi_interfaces::complex_vec_iterator::<Self::Value, #value_path>(obj.into_iter()) })
    });
    let drop_code = quote!({rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);});
    vec_ffi_exp(name, quote!(#t), quote!(*mut #value_path), decode, encode, drop_code)
}

fn map_ffi_expansion(name: TokenStream2, map: TokenStream2, k: TokenStream2, v: TokenStream2, from: TokenStream2, to: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct #name {
            pub count: usize,
            pub keys: *mut #k,
            pub values: *mut #v,
        }
         impl rs_ffi_interfaces::FFIConversion<#map> for #name {
            unsafe fn ffi_from_const(ffi: *const #name) -> #map {
                #from
            }
            unsafe fn ffi_to_const(obj: #map) -> *const #name {
                #to
            }
            unsafe fn destroy(ffi: *mut #name) {
                rs_ffi_interfaces::unbox_any(ffi);
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                unsafe {
                    #drop_code
                }
            }
        }
    }
}

pub fn map_ffi_simple_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
    let from = quote! {
        let ffi_ref = &*ffi;
        rs_ffi_interfaces::from_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
    };
    let to = quote! {
        rs_ffi_interfaces::boxed(Self {
            count: obj.len(),
            keys: rs_ffi_interfaces::to_simple_vec(obj.keys().cloned().collect()),
            values: rs_ffi_interfaces::to_simple_vec(obj.values().cloned().collect())
        })
    };

    let drop_code = quote! {
        rs_ffi_interfaces::unbox_vec_ptr(self.keys, self.count);
        rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
    };
    map_ffi_expansion(name, quote!(#map), quote!(#k), quote!(#v), from, to, drop_code)
}

pub fn map_ffi_simple_complex_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
    let value_path = mangle_path(v);
    let from = quote! {
        let ffi_ref = &*ffi;
        rs_ffi_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
    };
    let to = quote! {
        rs_ffi_interfaces::boxed(Self {
            count: obj.len(),
            keys: rs_ffi_interfaces::to_simple_vec(obj.keys().cloned().collect()),
            values: rs_ffi_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned())
        })
    };
    let drop_code = quote! {
        rs_ffi_interfaces::unbox_vec_ptr(self.keys, self.count);
        rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
    };

    map_ffi_expansion(name, quote!(#map), quote!(#k), quote!(*mut #value_path), from, to, drop_code)
}

pub fn map_ffi_complex_simple_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
    let key_path = mangle_path(k);
    let from = quote! {
        let ffi_ref = &*ffi;
        rs_ffi_interfaces::from_complex_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
    };
    let to = quote! {
        rs_ffi_interfaces::boxed(Self {
            count: obj.len(),
            keys: rs_ffi_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()),
            values: rs_ffi_interfaces::to_simple_vec(obj.values().cloned().collect::<Vec<_>>())
        })
    };
    let drop_code = quote! {
        rs_ffi_interfaces::unbox_any_vec_ptr(self.keys, self.count);
        rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
    };

    map_ffi_expansion(name, quote!(#map), quote!(*mut #key_path), quote!(#v), from, to, drop_code)
}

pub fn map_ffi_complex_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
    let key_path = mangle_path(k);
    let value_path = mangle_path(v);
    let from = quote! {
        let ffi_ref = &*ffi;
        rs_ffi_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
    };
    let to = quote! {
        rs_ffi_interfaces::boxed(Self {
            count: obj.len(),
            keys: rs_ffi_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()),
            values: rs_ffi_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned())
        })
    };
    let drop_code = quote! {
        rs_ffi_interfaces::unbox_any_vec_ptr(self.keys, self.count);
        rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
    };

    map_ffi_expansion(name, quote!(#map), quote!(*mut #key_path), quote!(*mut #value_path), from, to, drop_code)
}
