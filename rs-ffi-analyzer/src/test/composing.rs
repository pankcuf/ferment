use std::collections::{HashMap, HashSet};
use quote::quote;
use syn::parse_quote;
use syn::__private::TokenStream2;
use crate::generics::GenericConversion;
use crate::interface::Presentable;
use crate::presentation::Expansion;
use crate::scope::Scope;
use crate::scope_conversion::{ImportType, ScopeTreeCompact, ScopeTreeExportItem};


#[test]
fn decompose_module() {
    let expansion = Expansion::Scope { root: root_scope_tree_item().into() };
    println!("{}", expansion.present());
}
fn root_scope_tree_item() -> ScopeTreeCompact {
    ScopeTreeCompact {
        scope: Scope::crate_root(),
        generics: HashSet::from([]),
        imported: HashMap::from([]),
        exported: HashMap::from([
            (parse_quote!(RootStruct), ScopeTreeExportItem::Item(parse_quote!(pub struct RootStruct { pub name: String }))),
            (parse_quote!(ffi), ScopeTreeExportItem::Tree(
                HashSet::from([
                    GenericConversion::new(parse_quote!(Vec<bool>)),
                    GenericConversion::new(parse_quote!(Vec<crate::ffi::HashID>)),
                    GenericConversion::new(parse_quote!(Vec<Vec<crate::ffi::HashID>>)),
                    GenericConversion::new(parse_quote!(std::collections::BTreeMap<crate::ffi::HashID, crate::ffi::HashID>)),
                ]),
                HashMap::from([
                    (ImportType::External, vec![(parse_quote!(BTreeMap), Scope::new(parse_quote!(std::collections)))])
                ]),
                HashMap::from([
                    (parse_quote!(HashID), ScopeTreeExportItem::Item(parse_quote!(pub type HashID = [u8; 32];))),
                    (parse_quote!(KeyID), ScopeTreeExportItem::Item(parse_quote!(pub type KeyID = [u8; 20];))),
                    (parse_quote!(UsedKeyMatrix), ScopeTreeExportItem::Item(parse_quote!(pub type UsedKeyMatrix = Vec<bool>;))),
                    (parse_quote!(ArrayOfArraysOfHashes), ScopeTreeExportItem::Item(parse_quote!(pub type ArrayOfArraysOfHashes = Vec<Vec<crate::ffi::HashID>>;))),
                ]))
            ),
            (parse_quote!(chain),
             ScopeTreeExportItem::single_export(
                 parse_quote!(common),
                 ScopeTreeExportItem::single_export(
                     parse_quote!(chain_type),
                     ScopeTreeExportItem::single_export(
                         parse_quote!(ChainType),
                         ScopeTreeExportItem::Item(parse_quote!(pub enum ChainType { MainNet, TestNet })))))

            ),
            (parse_quote!(example), ScopeTreeExportItem::Tree(
                HashSet::from([]),
                HashMap::from([]),
                HashMap::from([
                    (parse_quote!(address), ScopeTreeExportItem::Tree(
                        HashSet::from([
                            GenericConversion::new(parse_quote!(Vec<u8>)),
                            GenericConversion::new(parse_quote!(std::collections::BTreeMap<crate::chain::common::chain_type::ChainType, crate::ffi::HashID>)),
                        ]),
                        HashMap::from([
                            (ImportType::External, vec![
                                (parse_quote!(BTreeMap), Scope::new(parse_quote!(std::collections))),
                            ]),
                            (ImportType::FfiType, vec![
                                (parse_quote!(ChainType_FFI), Scope::ffi_types_and(quote!(chain::common::chain_type))),
                            ]),
                        ]),
                        HashMap::from([
                            (parse_quote!(address_with_script_pubkey), ScopeTreeExportItem::Item(parse_quote!(pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> { Some(format_args!("{0:?}", script).to_string()) }))),
                            (parse_quote!(get_chain_type_string), ScopeTreeExportItem::Item(parse_quote!(pub fn get_chain_type_string(chain_type: ChainType) -> String { chain_type.get_string() }))),
                            (parse_quote!(get_chain_hashes_by_map), ScopeTreeExportItem::Item(parse_quote!(pub fn get_chain_hashes_by_map(map: BTreeMap<ChainType, HashID>) -> String { map.iter().fold(String::new(), |mut acc, (chain_type, hash_id)| { acc.add(chain_type.get_string()); acc.add(" => "); acc.add(hash_id.to_string().as_str()); acc }) } ))),
                        ]),
                    ))
                ]),
            ))
        ]),
    }
}

fn raw_expansion() -> TokenStream2 {
    quote! {
        pub struct RootStruct {
            pub name: String,
        }
        pub mod ffi {
            use std::collections::BTreeMap;
            use rs_ffi_macro_derive::ferment;
            #[ferment]
            pub type KeyID = u32;
            #[ferment]
            pub type HashID = [u8; 32];
            #[ferment]
            pub type UsedKeyMatrix = Vec<bool>;
            #[ferment]
            pub type ArrayOfArraysOfHashes = Vec<Vec<HashID>>;
            #[ferment]
            pub type MapOfHashes = BTreeMap<HashID, HashID>;
        }
        pub mod chain {
            pub mod common {
                pub mod chain_type {
                    use rs_ffi_macro_derive::ferment;
                    #[ferment]
                    pub enum ChainType { MainNet, TestNet }
                    impl ChainType {
                        pub fn get_string(&self) -> String {
                            match self { Self::MainNet => "mainnet".to_string(), Self::TestNet => "testnet".to_string() }
                        }
                    }
                }
            }
        }
        pub mod example {
            pub mod address {
                use std::collections::BTreeMap;
                use std::ops::Add;
                use crate::ffi::HashID;
                use crate::chain::common::chain_type::ChainType;
                #[rs_ffi_macro_derive::ferment]
                pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
                    Some(format_args!("{0:?}", script).to_string())
                }
                #[rs_ffi_macro_derive::ferment]
                pub fn get_chain_type_string(chain_type: ChainType) -> String {
                    chain_type.get_string()
                }
                #[rs_ffi_macro_derive::ferment]
                pub fn get_chain_hashes_by_map(map: BTreeMap<ChainType, HashID>) -> String {
                    map.iter().fold(String::new(), |mut acc, (chain_type, hash_id)| { acc.add(chain_type.get_string()); acc.add(hash_id.to_string().as_str()); acc })
                }
            }
        }
    }
}

#[allow(unused)]
fn import_based_expansion() -> TokenStream2 {
    quote! {
        pub mod types {
            use crate::RootStruct;

            pub struct RootStruct_FFI {
                pub name: *mut std::os::raw::c_char,
            }
            impl rs_ffi_interfaces::FFIConversion<RootStruct> for RootStruct_FFI {
                unsafe fn ffi_from_const(ffi: *const RootStruct_FFI) -> RootStruct {
                    let ffi_ref = &*ffi;
                    RootStruct {
                        name: rs_ffi_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                    }
                }
                unsafe fn ffi_to_const(obj: RootStruct) -> *const RootStruct_FFI {
                    rs_ffi_interfaces::boxed(RootStruct_FFI {
                        name: rs_ffi_interfaces::FFIConversion::ffi_to(obj.name),
                    })
                }
                unsafe fn destroy(ffi: *mut RootStruct_FFI) {
                    rs_ffi_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for RootStruct_FFI {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        <std::os::raw::c_char as rs_ffi_interfaces::FFIConversion<&str>>::destroy(ffi_ref.name);
                    }
                }
            }

            pub mod ffi {
                use crate::ffi::HashID;
                use crate::ffi::KeyID;
                use crate::ffi::UsedKeyMatrix;
                use crate::ffi::ArrayOfArraysOfHashes;
                use crate::ffi::MapOfHashes;
                use crate::ffi_expansions::generics::Vec_bool_FFI;
                use crate::ffi_expansions::generics::Vec_Vec_HashID_FFI;
                use crate::ffi_expansions::generics::Map_keys_HashID_values_HashID_FFI;

                pub struct KeyID_FFI(u32);
                impl rs_ffi_interfaces::FFIConversion<KeyID> for KeyID_FFI {
                    unsafe fn ffi_from_const(ffi: *const KeyID_FFI) -> KeyID {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                    unsafe fn ffi_to_const(obj: KeyID) -> *const KeyID_FFI {
                        rs_ffi_interfaces::boxed(KeyID_FFI(obj))
                    }
                    unsafe fn destroy(ffi: *mut KeyID_FFI) {
                        rs_ffi_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for KeyID_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            {};
                        }
                    }
                }
                pub struct HashID_FFI(*mut [u8; 32]);
                impl rs_ffi_interfaces::FFIConversion<HashID> for HashID_FFI {
                    unsafe fn ffi_from_const(ffi: *const HashID_FFI) -> HashID {
                        let ffi_ref = &*ffi;
                        *ffi_ref.0
                    }
                    unsafe fn ffi_to_const(obj: HashID) -> *const HashID_FFI {
                        rs_ffi_interfaces::boxed(HashID_FFI(rs_ffi_interfaces::boxed(obj)))
                    }
                    unsafe fn destroy(ffi: *mut HashID_FFI) {
                        rs_ffi_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for HashID_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            rs_ffi_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct UsedKeyMatrix_FFI(*mut Vec_bool_FFI);
                impl rs_ffi_interfaces::FFIConversion<UsedKeyMatrix> for UsedKeyMatrix_FFI {
                    unsafe fn ffi_from_const(ffi: *const UsedKeyMatrix_FFI) -> UsedKeyMatrix {
                        let ffi_ref = &*ffi;
                        {
                            let vec = &*ffi_ref.0;
                            {
                                let vec = vec;
                                rs_ffi_interfaces::from_simple_vec(vec.values, vec.count)
                            }
                        }
                    }
                    unsafe fn ffi_to_const(obj: UsedKeyMatrix) -> *const UsedKeyMatrix_FFI {
                        rs_ffi_interfaces::boxed(UsedKeyMatrix_FFI(
                            rs_ffi_interfaces::FFIConversion::ffi_to(obj),
                        ))
                    }
                    unsafe fn destroy(ffi: *mut UsedKeyMatrix_FFI) {
                        rs_ffi_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for UsedKeyMatrix_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            rs_ffi_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct ArrayOfArraysOfHashes_FFI(*mut Vec_Vec_HashID_FFI);
                impl rs_ffi_interfaces::FFIConversion<ArrayOfArraysOfHashes> for ArrayOfArraysOfHashes_FFI {
                    unsafe fn ffi_from_const(
                        ffi: *const ArrayOfArraysOfHashes_FFI,
                    ) -> ArrayOfArraysOfHashes {
                        let ffi_ref = &*ffi;
                        {
                            let vec = &*ffi_ref.0;
                            let count = vec.count;
                            let values = vec.values;
                            (0..count)
                                .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                                .collect()
                        }
                    }
                    unsafe fn ffi_to_const(obj: ArrayOfArraysOfHashes) -> *const ArrayOfArraysOfHashes_FFI {
                        rs_ffi_interfaces::boxed(ArrayOfArraysOfHashes_FFI(
                            rs_ffi_interfaces::FFIConversion::ffi_to(obj),
                        ))
                    }
                    unsafe fn destroy(ffi: *mut ArrayOfArraysOfHashes_FFI) {
                        rs_ffi_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ArrayOfArraysOfHashes_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            rs_ffi_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct MapOfHashes_FFI(*mut Map_keys_HashID_values_HashID_FFI);
                impl rs_ffi_interfaces::FFIConversion<MapOfHashes> for MapOfHashes_FFI {
                    unsafe fn ffi_from_const(ffi: *const MapOfHashes_FFI) -> MapOfHashes {
                        let ffi_ref = &*ffi;
                        rs_ffi_interfaces::FFIConversion::ffi_from(ffi_ref.0)
                    }
                    unsafe fn ffi_to_const(obj: MapOfHashes) -> *const MapOfHashes_FFI {
                        rs_ffi_interfaces::boxed(MapOfHashes_FFI(rs_ffi_interfaces::FFIConversion::ffi_to(
                            obj,
                        )))
                    }
                    unsafe fn destroy(ffi: *mut MapOfHashes_FFI) {
                        rs_ffi_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for MapOfHashes_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            rs_ffi_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }

            }
        }
        pub mod generics {
            use std::collections::BTreeMap;
            use crate::ffi::HashID;
            use crate::ffi_expansions::types::ffi::HashID_FFI;

            pub struct Vec_HashID_FFI {
                pub count: usize,
                pub values: *mut *mut HashID_FFI,
            }
            impl rs_ffi_interfaces::FFIConversion<Vec<HashID>> for Vec_HashID_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_HashID_FFI) -> Vec<HashID> {
                    let ffi_ref = &*ffi;
                    rs_ffi_interfaces::FFIVecConversion::decode(ffi_ref)
                }
                unsafe fn ffi_to_const(obj: Vec<HashID>) -> *const Vec_HashID_FFI {
                    rs_ffi_interfaces::FFIVecConversion::encode(obj)
                }
                unsafe fn destroy(ffi: *mut Vec_HashID_FFI) {
                    rs_ffi_interfaces::unbox_any(ffi);
                }
            }
            impl rs_ffi_interfaces::FFIVecConversion for Vec_HashID_FFI {
                type Value = HashID;
                unsafe fn decode(&self) -> Vec<Self::Value> {
                    {
                        let count = self.count;
                        let values = self.values;
                        (0..count)
                            .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                            .collect()
                    }
                }
                unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
                    {
                        rs_ffi_interfaces::boxed(Self {
                            count: obj.len(),
                            values: rs_ffi_interfaces::complex_vec_iterator::<Self::Value, HashID_FFI>(
                                obj.into_iter(),
                            ),
                        })
                    }
                }
            }
            impl Drop for Vec_HashID_FFI {
                fn drop(&mut self) {
                    unsafe {
                        {
                            rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Vec_bool_FFI {
                pub count: usize,
                pub values: *mut bool,
            }
            impl rs_ffi_interfaces::FFIConversion<Vec<bool>> for Vec_bool_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_bool_FFI) -> Vec<bool> {
                    let ffi_ref = &*ffi;
                    rs_ffi_interfaces::FFIVecConversion::decode(ffi_ref)
                }
                unsafe fn ffi_to_const(obj: Vec<bool>) -> *const Vec_bool_FFI {
                    rs_ffi_interfaces::FFIVecConversion::encode(obj)
                }
                unsafe fn destroy(ffi: *mut Vec_bool_FFI) {
                    rs_ffi_interfaces::unbox_any(ffi);
                }
            }
            impl rs_ffi_interfaces::FFIVecConversion for Vec_bool_FFI {
                type Value = bool;
                unsafe fn decode(&self) -> Vec<Self::Value> {
                    rs_ffi_interfaces::from_simple_vec(self.values as *const Self::Value, self.count)
                }
                unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
                    rs_ffi_interfaces::boxed(Self {
                        count: obj.len(),
                        values: rs_ffi_interfaces::boxed_vec(obj),
                    })
                }
            }
            impl Drop for Vec_bool_FFI {
                fn drop(&mut self) {
                    unsafe {
                        {
                            rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Vec_Vec_HashID_FFI {
                pub count: usize,
                pub values: *mut *mut Vec_HashID_FFI,
            }
            impl rs_ffi_interfaces::FFIConversion<Vec<Vec<HashID>>> for Vec_Vec_HashID_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_Vec_HashID_FFI) -> Vec<Vec<HashID>> {
                    let ffi_ref = &*ffi;
                    rs_ffi_interfaces::FFIVecConversion::decode(ffi_ref)
                }
                unsafe fn ffi_to_const(obj: Vec<Vec<HashID>>) -> *const Vec_Vec_HashID_FFI {
                    rs_ffi_interfaces::FFIVecConversion::encode(obj)
                }
                unsafe fn destroy(ffi: *mut Vec_Vec_HashID_FFI) {
                    rs_ffi_interfaces::unbox_any(ffi);
                }
            }
            impl rs_ffi_interfaces::FFIVecConversion for Vec_Vec_HashID_FFI {
                type Value = Vec<HashID>;
                unsafe fn decode(&self) -> Vec<Self::Value> {
                    {
                        let count = self.count;
                        let values = self.values;
                        (0..count)
                            .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                            .collect()
                    }
                }
                unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
                    {
                        rs_ffi_interfaces::boxed(Self {
                            count: obj.len(),
                            values: rs_ffi_interfaces::complex_vec_iterator::<Self::Value, Vec_HashID_FFI>(
                                obj.into_iter(),
                            ),
                        })
                    }
                }
            }
            impl Drop for Vec_Vec_HashID_FFI {
                fn drop(&mut self) {
                    unsafe {
                        {
                            rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Map_keys_HashID_values_HashID_FFI {
                pub count: usize,
                pub keys: *mut *mut HashID_FFI,
                pub values: *mut *mut HashID_FFI,
            }
            impl rs_ffi_interfaces::FFIConversion<BTreeMap<HashID, HashID>> for Map_keys_HashID_values_HashID_FFI
            {
                unsafe fn ffi_from_const(
                    ffi: *const Map_keys_HashID_values_HashID_FFI,
                ) -> BTreeMap<HashID, HashID> {
                    let ffi_ref = &*ffi;
                    rs_ffi_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
                }
                unsafe fn ffi_to_const(
                    obj: BTreeMap<HashID, HashID>,
                ) -> *const Map_keys_HashID_values_HashID_FFI {
                    rs_ffi_interfaces::boxed(Self {
                        count: obj.len(),
                        keys: rs_ffi_interfaces::complex_vec_iterator::<HashID, HashID_FFI>(
                            obj.keys().cloned(),
                        ),
                        values: rs_ffi_interfaces::complex_vec_iterator::<HashID, HashID_FFI>(
                            obj.values().cloned(),
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut Map_keys_HashID_values_HashID_FFI) {
                    rs_ffi_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Map_keys_HashID_values_HashID_FFI {
                fn drop(&mut self) {
                    unsafe {
                        rs_ffi_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                        rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
                    }
                }
            }
        }
    }
}


