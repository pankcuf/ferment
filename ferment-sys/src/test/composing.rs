use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use quote::{format_ident, quote};
use syn::parse_quote;
use syn::__private::TokenStream2;
use crate::{Config, Crate};
use crate::ast::{PathHolder, TypeHolder};
use crate::composable::TypeModel;
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeContext, ScopeContextLink, ScopeInfo, TypeChain};
use crate::kind::ObjectKind;
use crate::tree::{create_crate_root_scope_tree, ScopeTree, ScopeTreeID, ScopeTreeExportItem};


// #[test]
// fn decompose_module() {
//     println!("{}", root_scope_tree().to_token_stream());
// }
fn scope_chain(self_scope: PathHolder) -> ScopeChain {
    ScopeChain::root(ScopeInfo::attr_less(format_ident!("crate"), Scope::empty(self_scope)))
}

fn scope_ctx(self_scope: PathHolder, global_context_ptr: Arc<RwLock<GlobalContext>>) -> ScopeContextLink {
    ScopeContext::cell_with(scope_chain(self_scope), global_context_ptr)
}

fn root_scope_tree() -> ScopeTree {
    let mut global_context = GlobalContext::with_config(Config::new("crate", Crate::new("crate", PathBuf::new()), cbindgen::Config::default()));
    let root_scope = ScopeChain::crate_root(format_ident!("crate"), vec![]);
    global_context
        .scope_mut(&root_scope)
        .add_many(TypeChain::from(HashMap::from([
            (TypeHolder(parse_quote!(bool)), ObjectKind::primitive_type(parse_quote!(bool))),
            (TypeHolder(parse_quote!([u8; 20])), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!([u8; 20])))),
            (TypeHolder(parse_quote!([u8; 32])), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!([u8; 32])))),
            (TypeHolder(parse_quote!([u8; 32])), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!([u8; 32])))),
            (TypeHolder(parse_quote!(Vec)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(Vec)))),
            (TypeHolder(parse_quote!(HashID)), ObjectKind::object_model_type(TypeModel::new_default(parse_quote!(crate::nested::HashID)))),
            (TypeHolder(parse_quote!(BTreeMap)), ObjectKind::object_model_type(TypeModel::new_default(parse_quote!(std::collections::BTreeMap)))),
            (TypeHolder(parse_quote!(Vec<bool>)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(Vec<bool>)))),
            (TypeHolder(parse_quote!(Vec<HashID>)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(Vec<crate::nested::HashID>)))),
            (TypeHolder(parse_quote!(Vec<Vec<HashID>>)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(Vec<Vec<crate::nested::HashID>>)))),
            (TypeHolder(parse_quote!(BTreeMap<HashID, HashID>)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID>)))),
        ]).into_iter()).inner.into_iter());
    global_context
        .scope_mut(&scope_chain(parse_quote!(crate::example::address)))
        .add_many(TypeChain::from(HashMap::from([
            (TypeHolder(parse_quote!(u8)), ObjectKind::primitive_type(parse_quote!(u8))),
            (TypeHolder(parse_quote!(String)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(String)))),
            (TypeHolder(parse_quote!(Option)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(Option)))),
            (TypeHolder(parse_quote!(Option<String>)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(Option<String>)))),
            (TypeHolder(parse_quote!(Vec<u8>)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(Vec<u8>)))),
            (TypeHolder(parse_quote!(ChainType)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(crate::chain::common::chain_type::ChainType)))),
            (TypeHolder(parse_quote!(HashID)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(crate::nested::HashID)))),
            (TypeHolder(parse_quote!(BTreeMap)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(std::collections::BTreeMap)))),
            (TypeHolder(parse_quote!(BTreeMap<ChainType, HashID>)), ObjectKind::unknown_model_type(TypeModel::new_default(parse_quote!(std::collections::BTreeMap<crate::chain::common::chain_type::ChainType, crate::nested::HashID>)))),
        ]).into_iter()).inner.into_iter());
    let global_context_ptr = Arc::new(RwLock::new(global_context));

    create_crate_root_scope_tree(
        format_ident!("crate"),
        scope_ctx(parse_quote!(crate), global_context_ptr.clone()),
        HashSet::from([]),
        HashMap::from([
            (ScopeTreeID::Ident(parse_quote!(RootStruct)), ScopeTreeExportItem::Item(
                scope_ctx(parse_quote!(crate::RootStruct), global_context_ptr.clone()),
                parse_quote!(pub struct RootStruct { pub name: String }))),
            (ScopeTreeID::Ident(parse_quote!(ffi)), ScopeTreeExportItem::Tree(
                scope_ctx(parse_quote!(crate::ffi), global_context_ptr.clone()),
                HashSet::from([
                    parse_quote!(use std::collections::BTreeMap)
                ]),
                HashMap::from([
                    (ScopeTreeID::Ident(parse_quote!(HashID)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::HashID), global_context_ptr.clone()), parse_quote!(pub type HashID = [u8; 32];))),
                    (ScopeTreeID::Ident(parse_quote!(KeyID)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::KeyID), global_context_ptr.clone()), parse_quote!(pub type KeyID = [u8; 20];))),
                    (ScopeTreeID::Ident(parse_quote!(UsedKeyMatrix)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::UsedKeyMatrix), global_context_ptr.clone()), parse_quote!(pub type UsedKeyMatrix = Vec<bool>;))),
                    (ScopeTreeID::Ident(parse_quote!(ArrayOfArraysOfHashes)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::ArrayOfArraysOfHashes), global_context_ptr.clone()), parse_quote!(pub type ArrayOfArraysOfHashes = Vec<Vec<crate::nested::HashID>>;))),
                ]),
                vec![]
            )),
            (ScopeTreeID::Ident(parse_quote!(chain)),
             ScopeTreeExportItem::tree_with_context_and_exports(
                 scope_ctx(parse_quote!(crate::chain), global_context_ptr.clone()),
                 HashMap::from([
                     (ScopeTreeID::Ident(parse_quote!(common)), ScopeTreeExportItem::tree_with_context_and_exports(
                         scope_ctx(parse_quote!(crate::chain::common), global_context_ptr.clone()),
                         HashMap::from([
                             (ScopeTreeID::Ident(parse_quote!(chain_type)), ScopeTreeExportItem::tree_with_context_and_exports(
                                 scope_ctx(parse_quote!(crate::chain::common::chain_type), global_context_ptr.clone()),
                                 HashMap::from([
                                     (ScopeTreeID::Ident(parse_quote!(ChainType)), ScopeTreeExportItem::Item(
                                         scope_ctx(
                                             parse_quote!(crate::chain::common::chain_type::ChainType),
                                             global_context_ptr.clone()),
                                         parse_quote!(pub enum ChainType { MainNet, TestNet })))
                                 ]),
                                 vec![]))
                         ]),
                         vec![]
                     ))
                 ]),
                 vec![]),
            ),
            (ScopeTreeID::Ident(parse_quote!(example)), ScopeTreeExportItem::Tree(
                scope_ctx(parse_quote!(crate::example), global_context_ptr.clone()),
                HashSet::from([]),
                HashMap::from([
                    (ScopeTreeID::Ident(parse_quote!(address)), ScopeTreeExportItem::Tree(
                        scope_ctx(parse_quote!(crate::example::address), global_context_ptr.clone()),
                        // HashSet::from([
                        //     GenericConversion::new(ObjectKind::Type(TypeModelKind::Primitive(TypeComposition::new_default(parse_quote!(Vec<u8>))))),
                        //     GenericConversion::new(ObjectKind::Type(TypeModelKind::Primitive(TypeComposition::new_default(parse_quote!(std::collections::BTreeMap<crate::chain::common::chain_type::ChainType, crate::nested::HashID>))))),
                        // ]),
                        HashSet::from([
                            parse_quote!(use std::collections::BTreeMap),
                            parse_quote!(use chain::common::chain_type::ChainType),
                        ]),
                        HashMap::from([
                            (ScopeTreeID::Ident(parse_quote!(address_with_script_pubkey)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::example::address::address_with_script_pubkey), global_context_ptr.clone()), parse_quote!(pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> { Some(format_args!("{0:?}", script).to_string()) }))),
                            (ScopeTreeID::Ident(parse_quote!(get_chain_type_string)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::example::address::address_with_script_pubkey), global_context_ptr.clone()), parse_quote!(pub fn get_chain_type_string(chain_type: ChainType) -> String { chain_type.get_string() }))),
                            (ScopeTreeID::Ident(parse_quote!(get_chain_hashes_by_map)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::example::address::address_with_script_pubkey), global_context_ptr.clone()), parse_quote!(pub fn get_chain_hashes_by_map(map: BTreeMap<ChainType, HashID>) -> String { map.iter().fold(String::new(), |mut acc, (chain_type, hash_id)| { acc.add(chain_type.get_string()); acc.add(" => "); acc.add(hash_id.to_string().as_str()); acc }) } ))),
                        ]),
                        vec![]
                    ))
                ]),
                vec![]))
        ]),
        vec![])
}

#[allow(unused)]
fn raw_expansion() -> TokenStream2 {
    quote! {
        pub struct RootStruct {
            pub name: String,
        }
        pub mod nested {
            use std::collections::BTreeMap;
            #[ferment_macro::export]
            pub type KeyID = u32;
            #[ferment_macro::export]
            pub type HashID = [u8; 32];
            #[ferment_macro::export]
            pub type UsedKeyMatrix = Vec<bool>;
            #[ferment_macro::export]
            pub type ArrayOfArraysOfHashes = Vec<Vec<HashID>>;
            #[ferment_macro::export]
            pub type MapOfHashes = BTreeMap<HashID, HashID>;
        }
        pub mod chain {
            pub mod common {
                pub mod chain_type {
                    #[ferment_macro::export]
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
                use crate::nested::HashID;
                use crate::chain::common::chain_type::ChainType;
                #[ferment_macro::export]
                pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
                    Some(format_args!("{0:?}", script).to_string())
                }
                #[ferment_macro::export]
                pub fn get_chain_type_string(chain_type: ChainType) -> String {
                    chain_type.get_string()
                }
                #[ferment_macro::export]
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
            impl ferment::FFIConversionFrom<RootStruct> for RootStruct_FFI {
                unsafe fn ffi_from_const(ffi: *const RootStruct_FFI) -> RootStruct {
                    let ffi_ref = &*ffi;
                    RootStruct {
                        name: ferment::FFIConversionFrom::ffi_from(ffi_ref.name),
                    }
                }
            }
            impl ferment::FFIConversionTo<RootStruct> for RootStruct_FFI {
                unsafe fn ffi_to_const(obj: RootStruct) -> *const RootStruct_FFI {
                    ferment::boxed(RootStruct_FFI {
                        name: ferment::FFIConversionTo::ffi_to(obj.name),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<RootStruct> for RootStruct_FFI {
                unsafe fn destroy(ffi: *mut RootStruct_FFI) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for RootStruct_FFI {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        <std::os::raw::c_char as ferment::FFIConversionDestroy<&str>>::destroy(ffi_ref.name);
                    }
                }
            }

            pub mod nested {
                use crate::nested::HashID;
                use crate::nested::KeyID;
                use crate::nested::UsedKeyMatrix;
                use crate::nested::ArrayOfArraysOfHashes;
                use crate::nested::MapOfHashes;
                use crate::fermented::generics::Vec_bool_FFI;
                use crate::fermented::generics::Vec_Vec_HashID_FFI;
                use crate::fermented::generics::Map_keys_HashID_values_HashID_FFI;

                pub struct KeyID_FFI(u32);
                impl ferment::FFIConversionFrom<KeyID> for KeyID_FFI {
                    unsafe fn ffi_from_const(ffi: *const KeyID_FFI) -> KeyID {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                }
                impl ferment::FFIConversionTo<KeyID> for KeyID_FFI {
                    unsafe fn ffi_to_const(obj: KeyID) -> *const KeyID_FFI {
                        ferment::boxed(KeyID_FFI(obj))
                    }
                }
                impl ferment::FFIConversionDestroy<KeyID> for KeyID_FFI {
                    unsafe fn destroy(ffi: *mut KeyID_FFI) {
                        ferment::unbox_any(ffi);
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
                impl ferment::FFIConversionFrom<HashID> for HashID_FFI {
                    unsafe fn ffi_from_const(ffi: *const HashID_FFI) -> HashID {
                        let ffi_ref = &*ffi;
                        *ffi_ref.0
                    }
                }
                impl ferment::FFIConversionTo<HashID> for HashID_FFI {
                    unsafe fn ffi_to_const(obj: HashID) -> *const HashID_FFI {
                        ferment::boxed(HashID_FFI(ferment::boxed(obj)))
                    }
                }
                impl ferment::FFIConversionDestroy<HashID> for HashID_FFI {
                    unsafe fn destroy(ffi: *mut HashID_FFI) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for HashID_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct UsedKeyMatrix_FFI(*mut Vec_bool_FFI);
                impl ferment::FFIConversionFrom<UsedKeyMatrix> for UsedKeyMatrix_FFI {
                    unsafe fn ffi_from_const(ffi: *const UsedKeyMatrix_FFI) -> UsedKeyMatrix {
                        let ffi_ref = &*ffi;
                        {
                            let vec = &*ffi_ref.0;
                            {
                                let vec = vec;
                                ferment::from_primitive_vec(vec.values, vec.count)
                            }
                        }
                    }
                }
                impl ferment::FFIConversionTo<UsedKeyMatrix> for UsedKeyMatrix_FFI {
                    unsafe fn ffi_to_const(obj: UsedKeyMatrix) -> *const UsedKeyMatrix_FFI {
                        ferment::boxed(UsedKeyMatrix_FFI(
                            ferment::FFIConversionTo::ffi_to(obj),
                        ))
                    }
                }
                impl ferment::FFIConversionDestroy<UsedKeyMatrix> for UsedKeyMatrix_FFI {
                    unsafe fn destroy(ffi: *mut UsedKeyMatrix_FFI) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for UsedKeyMatrix_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct ArrayOfArraysOfHashes_FFI(*mut Vec_Vec_HashID_FFI);
                impl ferment::FFIConversionFrom<ArrayOfArraysOfHashes> for ArrayOfArraysOfHashes_FFI {
                    unsafe fn ffi_from_const(
                        ffi: *const ArrayOfArraysOfHashes_FFI,
                    ) -> ArrayOfArraysOfHashes {
                        let ffi_ref = &*ffi;
                        {
                            let vec = &*ffi_ref.0;
                            let count = vec.count;
                            let values = vec.values;
                            (0..count)
                                .map(|i| ferment::FFIConversionFrom::ffi_from_const(*values.add(i)))
                                .collect()
                        }
                    }
                }
                impl ferment::FFIConversionTo<ArrayOfArraysOfHashes> for ArrayOfArraysOfHashes_FFI {
                    unsafe fn ffi_to_const(obj: ArrayOfArraysOfHashes) -> *const ArrayOfArraysOfHashes_FFI {
                        ferment::boxed(ArrayOfArraysOfHashes_FFI(
                            ferment::FFIConversionTo::ffi_to(obj),
                        ))
                    }
                }
                impl ferment::FFIConversionDestroy<ArrayOfArraysOfHashes> for ArrayOfArraysOfHashes_FFI {
                    unsafe fn destroy(ffi: *mut ArrayOfArraysOfHashes_FFI) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for ArrayOfArraysOfHashes_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct MapOfHashes_FFI(*mut Map_keys_HashID_values_HashID_FFI);
                impl ferment::FFIConversionFrom<MapOfHashes> for MapOfHashes_FFI {
                    unsafe fn ffi_from_const(ffi: *const MapOfHashes_FFI) -> MapOfHashes {
                        let ffi_ref = &*ffi;
                        ferment::FFIConversionFrom::ffi_from(ffi_ref.0)
                    }
                }
                impl ferment::FFIConversionTo<MapOfHashes> for MapOfHashes_FFI {
                    unsafe fn ffi_to_const(obj: MapOfHashes) -> *const MapOfHashes_FFI {
                        ferment::boxed(MapOfHashes_FFI(ferment::FFIConversionTo::ffi_to(
                            obj,
                        )))
                    }
                }
                impl ferment::FFIConversionDestroy<MapOfHashes> for MapOfHashes_FFI {
                    unsafe fn destroy(ffi: *mut MapOfHashes_FFI) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for MapOfHashes_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }

            }
        }
        pub mod generics {
            use std::collections::BTreeMap;
            use crate::nested::HashID;
            use crate::fermented::types::nested::HashID_FFI;

            pub struct Vec_HashID_FFI {
                pub count: usize,
                pub values: *mut *mut HashID_FFI,
            }
            impl ferment::FFIConversionFrom<Vec<HashID>> for Vec_HashID_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_HashID_FFI) -> Vec<HashID> {
                    let ffi_ref = &*ffi;
                    ferment::FFIVecConversion::decode(ffi_ref)
                }
            }
            impl ferment::FFIConversionTo<Vec<HashID>> for Vec_HashID_FFI {
                unsafe fn ffi_to_const(obj: Vec<HashID>) -> *const Vec_HashID_FFI {
                    ferment::FFIVecConversion::encode(obj)
                }
            }
            impl ferment::FFIConversionDestroy<Vec<HashID>> for Vec_HashID_FFI {
                unsafe fn destroy(ffi: *mut Vec_HashID_FFI) {
                    ferment::unbox_any(ffi);
                }
            }
            impl ferment::FFIVecConversion for Vec_HashID_FFI {
                type Value = HashID;
                unsafe fn decode(&self) -> Vec<Self::Value> {
                    {
                        let count = self.count;
                        let values = self.values;
                        (0..count)
                            .map(|i| ferment::FFIConversionFrom::ffi_from_const(*values.add(i)))
                            .collect()
                    }
                }
                unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
                    {
                        ferment::boxed(Self {
                            count: obj.len(),
                            values: ferment::to_complex_vec(
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
                            ferment::unbox_any_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Vec_bool_FFI {
                pub count: usize,
                pub values: *mut bool,
            }
            impl ferment::FFIConversionFrom<Vec<bool>> for Vec_bool_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_bool_FFI) -> Vec<bool> {
                    let ffi_ref = &*ffi;
                    ferment::FFIVecConversion::decode(ffi_ref)
                }
            }
            impl ferment::FFIConversionTo<Vec<bool>> for Vec_bool_FFI {
                unsafe fn ffi_to_const(obj: Vec<bool>) -> *const Vec_bool_FFI {
                    ferment::FFIVecConversion::encode(obj)
                }
            }
            impl ferment::FFIConversionDestroy<Vec<bool>> for Vec_bool_FFI {
                unsafe fn destroy(ffi: *mut Vec_bool_FFI) {
                    ferment::unbox_any(ffi);
                }
            }
            impl ferment::FFIVecConversion for Vec_bool_FFI {
                type Value = Vec<bool>;
                unsafe fn decode(&self) -> Self::Value {
                    ferment::from_primitive_vec(self.values.cast_const(), self.count)
                }
                unsafe fn encode(obj: Self::Value) -> *mut Self {
                    ferment::boxed(Self {
                        count: obj.len(),
                        values: ferment::boxed_vec(obj),
                    })
                }
            }
            impl Drop for Vec_bool_FFI {
                fn drop(&mut self) {
                    unsafe {
                        {
                            ferment::unbox_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Vec_Vec_HashID_FFI {
                pub count: usize,
                pub values: *mut *mut Vec_HashID_FFI,
            }
            impl ferment::FFIConversionFrom<Vec<Vec<HashID>>> for Vec_Vec_HashID_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_Vec_HashID_FFI) -> Vec<Vec<HashID>> {
                    let ffi_ref = &*ffi;
                    ferment::FFIVecConversion::decode(ffi_ref)
                }
            }
            impl ferment::FFIConversionTo<Vec<Vec<HashID>>> for Vec_Vec_HashID_FFI {
                unsafe fn ffi_to_const(obj: Vec<Vec<HashID>>) -> *const Vec_Vec_HashID_FFI {
                    ferment::FFIVecConversion::encode(obj)
                }
            }
            impl ferment::FFIConversionDestroy<Vec<Vec<HashID>>> for Vec_Vec_HashID_FFI {
                unsafe fn destroy(ffi: *mut Vec_Vec_HashID_FFI) {
                    ferment::unbox_any(ffi);
                }
            }
            impl ferment::FFIVecConversion for Vec_Vec_HashID_FFI {
                type Value = Vec<HashID>;
                unsafe fn decode(&self) -> Vec<Self::Value> {
                    {
                        let count = self.count;
                        let values = self.values;
                        (0..count)
                            .map(|i| ferment::FFIConversionFrom::ffi_from_const(*values.add(i)))
                            .collect()
                    }
                }
                unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
                    {
                        ferment::boxed(Self {
                            count: obj.len(),
                            values: ferment::to_complex_vec(
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
                            ferment::unbox_any_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Map_keys_HashID_values_HashID_FFI {
                pub count: usize,
                pub keys: *mut *mut HashID_FFI,
                pub values: *mut *mut HashID_FFI,
            }
            impl ferment::FFIConversionFrom<BTreeMap<HashID, HashID>> for Map_keys_HashID_values_HashID_FFI
            {
                unsafe fn ffi_from_const(
                    ffi: *const Map_keys_HashID_values_HashID_FFI,
                ) -> BTreeMap<HashID, HashID> {
                    let ffi_ref = &*ffi;
                    ferment::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
                }
            }
            impl ferment::FFIConversionTo<BTreeMap<HashID, HashID>> for Map_keys_HashID_values_HashID_FFI
            {
                unsafe fn ffi_to_const(
                    obj: BTreeMap<HashID, HashID>,
                ) -> *const Map_keys_HashID_values_HashID_FFI {
                    ferment::boxed(Self {
                        count: obj.len(),
                        keys: ferment::to_complex_vec(
                            obj.keys().cloned(),
                        ),
                        values: ferment::to_complex_vec(
                            obj.values().cloned(),
                        ),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<BTreeMap<HashID, HashID>> for Map_keys_HashID_values_HashID_FFI {
                unsafe fn destroy(ffi: *mut Map_keys_HashID_values_HashID_FFI) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for Map_keys_HashID_values_HashID_FFI {
                fn drop(&mut self) {
                    unsafe {
                        ferment::unbox_any_vec_ptr(self.keys, self.count);
                        ferment::unbox_any_vec_ptr(self.values, self.count);
                    }
                }
            }
        }
    }
}


