use std::collections::HashSet;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use quote::{quote, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use syn::__private::TokenStream2;
use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::helper::{ffi_struct_name, mangle_type};
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

impl TypePathComposition {

    pub fn ffi_generic_import_scope(&self) -> Scope {
        Scope::ffi_generic_import(&self.0)
    }
    // pub fn expand_generic(self) -> TokenStream2 {
    //     let TypePathComposition { 0:ty, 1: path } = self;
    //     // let mangled_type = ffi_type_converted
    //     let mangled_type = mangle_type(&ty);
    //     let ffi_name = ffi_struct_name(&mangled_type).to_token_stream();
    //     // println!("expand_generic: {} < ==== > {}", quote!(#ty), quote!(#path));
    //     let PathSegment { ident, arguments} = path.segments.last().unwrap();
    //     // TODO: handle abstract generic types
    //     match ident.to_string().as_str() {
    //         "Vec" | "BTreeMap" | "HashMap" => match &path_arguments_to_path_conversions(arguments)[..] {
    //             [PathConversion::Primitive(value_path)] =>
    //                 GENERIC_VEC_SIMPLE_PRESENTER((ffi_name, value_path)),
    //             [PathConversion::Complex(value_path) | PathConversion::Generic(GenericPathConversion::Vec(value_path)) | PathConversion::Generic(GenericPathConversion::Map(value_path))] =>
    //                 GENERIC_VEC_COMPLEX_PRESENTER((ffi_name, value_path)),
    //             [PathConversion::Primitive(key_path),
    //             PathConversion::Primitive(value_path)] =>
    //                 GENERIC_MAP_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
    //             [PathConversion::Primitive(key_path),
    //             PathConversion::Complex(value_path) | PathConversion::Generic(GenericPathConversion::Vec(value_path)) | PathConversion::Generic(GenericPathConversion::Map(value_path))] =>
    //                 GENERIC_MAP_SIMPLE_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
    //             [PathConversion::Complex(key_path) | PathConversion::Generic(GenericPathConversion::Vec(key_path)) | PathConversion::Generic(GenericPathConversion::Map(key_path)),
    //             PathConversion::Primitive(value_path)] =>
    //                 GENERIC_MAP_COMPLEX_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
    //             [PathConversion::Complex(key_path) | PathConversion::Generic(GenericPathConversion::Vec(key_path)) | PathConversion::Generic(GenericPathConversion::Map(key_path)),
    //             PathConversion::Complex(value_path) | PathConversion::Generic(GenericPathConversion::Vec(value_path)) | PathConversion::Generic(GenericPathConversion::Map(value_path))] =>
    //                 GENERIC_MAP_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
    //             _ => unimplemented!("Generic path arguments conversion error"),
    //         },
    //         _ => quote!(),
    //     }
    //
    // }
}

pub fn add_generic_type(field_type: &Type, generics: &mut HashSet<TypePathComposition>) {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if let PathConversion::Generic(GenericPathConversion::Vec(path)) | PathConversion::Generic(GenericPathConversion::Map(path)) = PathConversion::from(path) {
            generics.insert(TypePathComposition(field_type.clone(), path.clone()));
        }
    }
}
// pub fn add_generic_type_2(field_type: &Type, generics: &mut HashSet<GenericConversion>) {
//     if let Type::Path(TypePath { path, .. }) = field_type {
//         if let PathConversion::Generic(generic_path_conversion) = PathConversion::from(path) {
//             generics.insert(GenericConversion::from(field_type));
//             generics.extend(generic_path_conversion.generic_types()
//                 .into_iter()
//                 .map(GenericConversion::from))
//         }
//     }
// }

pub fn vec_ffi_exp(name: TokenStream2, t: TokenStream2, mangled_t: TokenStream2, decode: TokenStream2, encode: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    println!("vec_ffi_exp: {name}: {t}: {mangled_t}");
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
// pub fn vec_ffi_simple_expansion(name: TokenStream2, t: &Path) -> TokenStream2 {
//     // println!("vec_ffi_simple_expansion: {name}: {}", quote!(#t));
//     let decode = quote!(rs_ffi_interfaces::from_simple_vec(self.values as *const Self::Value, self.count));
//     let encode = quote!(rs_ffi_interfaces::boxed(Self { count: obj.len(), values: rs_ffi_interfaces::boxed_vec(obj) }));
//     let drop_code = quote!({rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);});
//     vec_ffi_exp(name, quote!(#t), quote!(#t), decode, encode, drop_code)
// }
//
// pub fn vec_ffi_complex_expansion(name: TokenStream2, t: &Path) -> TokenStream2 {
//     let value_path = mangle_path(t);
//     // println!("vec_ffi_complex_expansion: {name}: {} --> {}", quote!(#t), quote!(#value_path));
//     let decode = quote!({
//         let count = self.count;
//         let values = self.values;
//         (0..count)
//             .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
//             .collect()
//     });
//     let encode = quote!({
//         rs_ffi_interfaces::boxed(Self { count: obj.len(), values: rs_ffi_interfaces::complex_vec_iterator::<Self::Value, #value_path>(obj.into_iter()) })
//     });
//     let drop_code = quote!({rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);});
//     vec_ffi_exp(name, quote!(#t), quote!(*mut #value_path), decode, encode, drop_code)
// }

pub fn map_ffi_expansion(name: TokenStream2, map: TokenStream2, k: TokenStream2, v: TokenStream2, from: TokenStream2, to: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    println!("map_ffi_expansion: {name}: {map}: {k}: {v}");
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

// pub fn map_ffi_simple_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
//     let from = quote! {
//         let ffi_ref = &*ffi;
//         rs_ffi_interfaces::from_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
//     };
//     let to = quote! {
//         rs_ffi_interfaces::boxed(Self {
//             count: obj.len(),
//             keys: rs_ffi_interfaces::to_simple_vec(obj.keys().cloned().collect()),
//             values: rs_ffi_interfaces::to_simple_vec(obj.values().cloned().collect())
//         })
//     };
//
//     let drop_code = quote! {
//         rs_ffi_interfaces::unbox_vec_ptr(self.keys, self.count);
//         rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
//     };
//     map_ffi_expansion(name, quote!(#map), quote!(#k), quote!(#v), from, to, drop_code)
// }
//
// pub fn map_ffi_simple_complex_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
//     let value_path = mangle_path(v);
//     let from = quote! {
//         let ffi_ref = &*ffi;
//         rs_ffi_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
//     };
//     let to = quote! {
//         rs_ffi_interfaces::boxed(Self {
//             count: obj.len(),
//             keys: rs_ffi_interfaces::to_simple_vec(obj.keys().cloned().collect()),
//             values: rs_ffi_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned())
//         })
//     };
//     let drop_code = quote! {
//         rs_ffi_interfaces::unbox_vec_ptr(self.keys, self.count);
//         rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
//     };
//
//     map_ffi_expansion(name, quote!(#map), quote!(#k), quote!(*mut #value_path), from, to, drop_code)
// }
//
// pub fn map_ffi_complex_simple_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
//     let key_path = mangle_path(k);
//     let from = quote! {
//         let ffi_ref = &*ffi;
//         rs_ffi_interfaces::from_complex_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
//     };
//     let to = quote! {
//         rs_ffi_interfaces::boxed(Self {
//             count: obj.len(),
//             keys: rs_ffi_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()),
//             values: rs_ffi_interfaces::to_simple_vec(obj.values().cloned().collect::<Vec<_>>())
//         })
//     };
//     let drop_code = quote! {
//         rs_ffi_interfaces::unbox_any_vec_ptr(self.keys, self.count);
//         rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
//     };
//
//     map_ffi_expansion(name, quote!(#map), quote!(*mut #key_path), quote!(#v), from, to, drop_code)
// }
//
// pub fn map_ffi_complex_expansion(name: TokenStream2, map: &Path, k: &Path, v: &Path) -> TokenStream2 {
//     let key_path = mangle_path(k);
//     let value_path = mangle_path(v);
//     let from = quote! {
//         let ffi_ref = &*ffi;
//         rs_ffi_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
//     };
//     let to = quote! {
//         rs_ffi_interfaces::boxed(Self {
//             count: obj.len(),
//             keys: rs_ffi_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()),
//             values: rs_ffi_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned())
//         })
//     };
//     let drop_code = quote! {
//         rs_ffi_interfaces::unbox_any_vec_ptr(self.keys, self.count);
//         rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
//     };
//
//     map_ffi_expansion(name, quote!(#map), quote!(*mut #key_path), quote!(*mut #value_path), from, to, drop_code)
// }

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
        let field_type = mangle_type(&full_type);
        let ffi_name = ffi_struct_name(&field_type);
        match PathConversion::from(path) {
            PathConversion::Generic(generic_conversion) => generic_conversion.expand(ffi_name),
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
        // fold std::collections::BTreeMap<crate::ffi::HashID, crate::ffi::HashID>
        // into ImportType::Original vec![crate::ffi::HashID]
        // into ImportType::External vec![std::collections::BTreeMap]
        // into ImportType::FfiType vec![crate::ffi_expansions::types::ffi::HashID_FFI]
        // into ImportType::FfiGeneric vec![crate::ffi_expansions::generics::std_collections_Map_keys_crate_ffi_HashID_values_crate_ffi_HashID_FFI]

        // fold std::collections::BTreeMap<crate::ffi::HashID, std::collections::BTreeMap<crate::ffi::HashID, crate::ffi::HashID>>
        // into ImportType::Original vec![crate::ffi::HashID]
        // into ImportType::External vec![std::collections::BTreeMap]
        // into ImportType::FfiType vec![crate::ffi_expansions::types::ffi::HashID_FFI]
        // into ImportType::FfiGeneric vec![crate::ffi_expansions::generics::std_collections_Map_keys_crate_ffi_HashID_values_crate_ffi_HashID_FFI, crate::ffi_expansions::generics::std_collections_Map_keys_crate_ffi_HashID_values_std_collections_Map_keys_crate_ffi_HashID_values_crate_ffi_HashID_FFI]

        // fold Vec<Vec<crate::ffi::HashID>>
        // into ImportType::Original vec![crate::ffi::HashID]
        // into ImportType::External vec![]
        // into ImportType::FfiType vec![crate::ffi_expansions::types::ffi::HashID_FFI]
        // into ImportType::FfiGeneric vec![crate::ffi_expansions::generics::Vec_HashID_FFI]
        let imports = generic_imports(&self.full_type);
        let vec = Vec::from_iter(imports.iter());
        println!("used_imports [{}]: {}", self.full_type.to_token_stream(), quote!(#(#vec;)*));
        imports
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