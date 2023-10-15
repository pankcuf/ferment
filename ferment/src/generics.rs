use std::collections::HashSet;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use quote::{quote, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use syn::__private::TokenStream2;
use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::helper::{ffi_mangled_ident, mangle_type};
use crate::interface::Presentable;
use crate::scope::Scope;

#[derive(Clone)]
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
        if let PathConversion::Generic(GenericPathConversion::Vec(path)) | PathConversion::Generic(GenericPathConversion::Map(path)) = PathConversion::from(path) {
            generics.insert(TypePathComposition(field_type.clone(), path.clone()));
        }
    }
}

pub fn vec_ffi_exp(name: TokenStream2, t: TokenStream2, mangled_t: TokenStream2, decode: TokenStream2, encode: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct #name {
            pub count: usize,
            pub values: *mut #mangled_t,
        }
        impl ferment_interfaces::FFIConversion<Vec<#t>> for #name {
            unsafe fn ffi_from_const(ffi: *const #name) -> Vec<#t> {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIVecConversion::decode(ffi_ref)
            }
            unsafe fn ffi_to_const(obj: Vec<#t>) -> *const #name {
                ferment_interfaces::FFIVecConversion::encode(obj)
            }
            unsafe fn destroy(ffi: *mut #name) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl ferment_interfaces::FFIVecConversion for #name {
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

pub fn map_ffi_expansion(name: TokenStream2, map: TokenStream2, k: TokenStream2, v: TokenStream2, from: TokenStream2, to: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct #name {
            pub count: usize,
            pub keys: *mut #k,
            pub values: *mut #v,
        }
         impl ferment_interfaces::FFIConversion<#map> for #name {
            unsafe fn ffi_from_const(ffi: *const #name) -> #map {
                #from
            }
            unsafe fn ffi_to_const(obj: #map) -> *const #name {
                #to
            }
            unsafe fn destroy(ffi: *mut #name) {
                ferment_interfaces::unbox_any(ffi);
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

#[derive(Clone)]
pub struct GenericConversion {
    pub full_type: Type,
}

impl<'a> From<&'a Type> for GenericConversion {
    fn from(value: &'a Type) -> Self {
        GenericConversion::new(value.clone())
    }
}
impl std::fmt::Debug for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(mangle_type(&self.full_type).to_string().as_str())?;
        f.write_str(" => ")?;
        f.write_str(self.full_type.to_token_stream().to_string().as_str())
    }
}

impl std::fmt::Display for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for GenericConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.full_type.to_token_stream()];
        let other_tokens = [other.full_type.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for GenericConversion {}

impl Hash for GenericConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_type.to_token_stream().to_string().hash(state);
    }
}

impl Presentable for GenericConversion {
    fn present(self) -> TokenStream2 {
        let Self { full_type } = self;
        let path: Path = parse_quote!(#full_type);
        match PathConversion::from(path) {
            PathConversion::Generic(generic_conversion) => generic_conversion.expand(ffi_mangled_ident(&full_type)),
            conversion => unimplemented!("non-generic PathConversion: {}", conversion.as_path().to_token_stream())
        }
    }
}

impl ToTokens for GenericConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.full_type.to_tokens(tokens);
    }
}


impl GenericConversion {
    pub fn new(full_type: Type) -> Self {
        Self { full_type }
    }

    pub fn used_imports(&self) -> HashSet<Scope> {
        generic_imports(&self.full_type)
    }

}

fn generic_imports(ty: &Type) -> HashSet<Scope> {
    match ty {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) => segments.iter()
            .flat_map(|PathSegment { arguments, .. }| match arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => args
                    .iter()
                    .filter_map(|arg| match arg { GenericArgument::Type(ty) => Some(ty), _ => None })
                    .flat_map(generic_imports)
                    .collect(),
                _ => HashSet::new(),
            })
            .collect(),
        _ => HashSet::new(),
    }
}