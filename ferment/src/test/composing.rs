use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use quote::quote;
use syn::parse_quote;
use syn::__private::TokenStream2;
use crate::Config;
use crate::composition::{GenericConversion, ImportComposition, TypeComposition};
use crate::context::{GlobalContext, Scope, ScopeChain, ScopeContext, TypeChain};
use crate::conversion::{ImportConversion, ObjectConversion, TypeConversion};
use crate::holder::{PathHolder, TypeHolder};
use crate::presentation::expansion::Expansion;
use crate::tree::{ScopeTreeCompact, ScopeTreeExportID, ScopeTreeExportItem};


#[test]
fn decompose_module() {
    let expansion = Expansion::Root { tree: root_scope_tree_item().into() };
    println!("{}", quote!(#expansion));
}
fn scope_chain(self_scope: PathHolder) -> ScopeChain {
    ScopeChain::Mod { self_scope: Scope::new(self_scope, ObjectConversion::Empty) }
}

fn scope_ctx(self_scope: PathHolder, global_context_ptr: Arc<RwLock<GlobalContext>>) -> Rc<RefCell<ScopeContext>> {
    Rc::new(RefCell::new(ScopeContext::with(scope_chain(self_scope), global_context_ptr)))
}

fn root_scope_tree_item() -> ScopeTreeCompact {
    let mut global_context = GlobalContext::with_config(Config::default());
    let root_scope = ScopeChain::crate_root();
    global_context
        .scope_mut(&root_scope)
        .add_many(TypeChain::from(HashMap::from([
            (TypeHolder(parse_quote!(bool)), ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(bool))))),
            (TypeHolder(parse_quote!([u8; 20])), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!([u8; 20]))))),
            (TypeHolder(parse_quote!([u8; 32])), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!([u8; 32]))))),
            (TypeHolder(parse_quote!([u8; 32])), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!([u8; 32]))))),
            (TypeHolder(parse_quote!(Vec)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(Vec))))),
            (TypeHolder(parse_quote!(HashID)), ObjectConversion::Type(TypeConversion::Object(TypeComposition::new_default(parse_quote!(crate::nested::HashID))))),
            (TypeHolder(parse_quote!(BTreeMap)), ObjectConversion::Type(TypeConversion::Object(TypeComposition::new_default(parse_quote!(std::collections::BTreeMap))))),
            (TypeHolder(parse_quote!(Vec<bool>)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(Vec<bool>))))),
            (TypeHolder(parse_quote!(Vec<HashID>)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(Vec<crate::nested::HashID>))))),
            (TypeHolder(parse_quote!(Vec<Vec<HashID>>)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(Vec<Vec<crate::nested::HashID>>))))),
            (TypeHolder(parse_quote!(BTreeMap<HashID, HashID>)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID>))))),
        ])));
    global_context
        .scope_mut(&scope_chain(parse_quote!(crate::example::address)))
        .add_many(TypeChain::from(HashMap::from([
            (TypeHolder(parse_quote!(u8)), ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(u8))))),
            (TypeHolder(parse_quote!(String)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(String))))),
            (TypeHolder(parse_quote!(Option)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(Option))))),
            (TypeHolder(parse_quote!(Option<String>)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(Option<String>))))),
            (TypeHolder(parse_quote!(Vec<u8>)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(Vec<u8>))))),
            (TypeHolder(parse_quote!(ChainType)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(crate::chain::common::chain_type::ChainType))))),
            (TypeHolder(parse_quote!(HashID)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(crate::nested::HashID))))),
            (TypeHolder(parse_quote!(BTreeMap)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(std::collections::BTreeMap))))),
            (TypeHolder(parse_quote!(BTreeMap<ChainType, HashID>)), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new_default(parse_quote!(std::collections::BTreeMap<crate::chain::common::chain_type::ChainType, crate::nested::HashID>))))),
        ])));
    let global_context_ptr = Arc::new(RwLock::new(global_context));
    ScopeTreeCompact {
        scope: ScopeChain::crate_root(),
        scope_context: scope_ctx(parse_quote!(crate), global_context_ptr.clone()),
        generics: HashSet::from([]),
        imported: HashMap::from([]),
        exported: HashMap::from([
            (ScopeTreeExportID::Ident(parse_quote!(RootStruct)), ScopeTreeExportItem::Item(
                scope_ctx(parse_quote!(crate::RootStruct), global_context_ptr.clone()),
                parse_quote!(pub struct RootStruct { pub name: String }))),
            (ScopeTreeExportID::Ident(parse_quote!(ffi)), ScopeTreeExportItem::Tree(
                scope_ctx(parse_quote!(crate::ffi), global_context_ptr.clone()),
                HashSet::from([
                    GenericConversion::new(ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(Vec<bool>))))),
                    GenericConversion::new(ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(Vec<crate::nested::HashID>))))),
                    GenericConversion::new(ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(Vec<Vec<crate::nested::HashID>>))))),
                    GenericConversion::new(ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID>)))))
                ]),
                HashMap::from([
                    (ImportConversion::External, HashSet::from([
                        ImportComposition::new(parse_quote!(BTreeMap), PathHolder(parse_quote!(std::collections)))]))
                ]),
                HashMap::from([
                    (ScopeTreeExportID::Ident(parse_quote!(HashID)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::HashID), global_context_ptr.clone()), parse_quote!(pub type HashID = [u8; 32];))),
                    (ScopeTreeExportID::Ident(parse_quote!(KeyID)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::KeyID), global_context_ptr.clone()), parse_quote!(pub type KeyID = [u8; 20];))),
                    (ScopeTreeExportID::Ident(parse_quote!(UsedKeyMatrix)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::UsedKeyMatrix), global_context_ptr.clone()), parse_quote!(pub type UsedKeyMatrix = Vec<bool>;))),
                    (ScopeTreeExportID::Ident(parse_quote!(ArrayOfArraysOfHashes)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::ffi::ArrayOfArraysOfHashes), global_context_ptr.clone()), parse_quote!(pub type ArrayOfArraysOfHashes = Vec<Vec<crate::nested::HashID>>;))),
                ]),
            )),
            (ScopeTreeExportID::Ident(parse_quote!(chain)),
                ScopeTreeExportItem::tree_with_context_and_export(
                    scope_ctx(parse_quote!(crate::chain), global_context_ptr.clone()),
                    HashMap::from([
                        (ScopeTreeExportID::Ident(parse_quote!(common)), ScopeTreeExportItem::tree_with_context_and_export(
                            scope_ctx(parse_quote!(crate::chain::common), global_context_ptr.clone()),
                            HashMap::from([
                                (ScopeTreeExportID::Ident(parse_quote!(chain_type)), ScopeTreeExportItem::tree_with_context_and_export(
                                    scope_ctx(parse_quote!(crate::chain::common::chain_type), global_context_ptr.clone()),
                                    HashMap::from([
                                        (ScopeTreeExportID::Ident(parse_quote!(ChainType)), ScopeTreeExportItem::Item(
                                            scope_ctx(
                                                parse_quote!(crate::chain::common::chain_type::ChainType),
                                                global_context_ptr.clone()),
                                            parse_quote!(pub enum ChainType { MainNet, TestNet })))
                                    ])
                                ))
                            ])
                        ))
                    ])),
            ),
            (ScopeTreeExportID::Ident(parse_quote!(example)), ScopeTreeExportItem::Tree(
                scope_ctx(parse_quote!(crate::example), global_context_ptr.clone()),
                HashSet::from([]),
                HashMap::from([]),
                HashMap::from([
                    (ScopeTreeExportID::Ident(parse_quote!(address)), ScopeTreeExportItem::Tree(
                        scope_ctx(parse_quote!(crate::example::address), global_context_ptr.clone()),
                        HashSet::from([
                            GenericConversion::new(ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(Vec<u8>))))),
                            GenericConversion::new(ObjectConversion::Type(TypeConversion::Primitive(TypeComposition::new_default(parse_quote!(std::collections::BTreeMap<crate::chain::common::chain_type::ChainType, crate::nested::HashID>))))),
                        ]),
                        HashMap::from([
                            (ImportConversion::External, HashSet::from([
                                ImportComposition::new(parse_quote!(BTreeMap), PathHolder(parse_quote!(std::collections)))])),
                            (ImportConversion::FfiType, HashSet::from([
                                ImportComposition::new(parse_quote!(ChainType), PathHolder::ffi_types_and(quote!(chain::common::chain_type)))])),
                        ]),
                        HashMap::from([
                            (ScopeTreeExportID::Ident(parse_quote!(address_with_script_pubkey)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::example::address::address_with_script_pubkey), global_context_ptr.clone()), parse_quote!(pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> { Some(format_args!("{0:?}", script).to_string()) }))),
                            (ScopeTreeExportID::Ident(parse_quote!(get_chain_type_string)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::example::address::address_with_script_pubkey), global_context_ptr.clone()), parse_quote!(pub fn get_chain_type_string(chain_type: ChainType) -> String { chain_type.get_string() }))),
                            (ScopeTreeExportID::Ident(parse_quote!(get_chain_hashes_by_map)), ScopeTreeExportItem::Item(scope_ctx(parse_quote!(crate::example::address::address_with_script_pubkey), global_context_ptr.clone()), parse_quote!(pub fn get_chain_hashes_by_map(map: BTreeMap<ChainType, HashID>) -> String { map.iter().fold(String::new(), |mut acc, (chain_type, hash_id)| { acc.add(chain_type.get_string()); acc.add(" => "); acc.add(hash_id.to_string().as_str()); acc }) } ))),
                        ]),
                    ))
                ]),
            ))
        ])
    }
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
            impl ferment_interfaces::FFIConversion<RootStruct> for RootStruct_FFI {
                unsafe fn ffi_from_const(ffi: *const RootStruct_FFI) -> RootStruct {
                    let ffi_ref = &*ffi;
                    RootStruct {
                        name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                    }
                }
                unsafe fn ffi_to_const(obj: RootStruct) -> *const RootStruct_FFI {
                    ferment_interfaces::boxed(RootStruct_FFI {
                        name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                    })
                }
                unsafe fn destroy(ffi: *mut RootStruct_FFI) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for RootStruct_FFI {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        <std::os::raw::c_char as ferment_interfaces::FFIConversion<&str>>::destroy(ffi_ref.name);
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
                impl ferment_interfaces::FFIConversion<KeyID> for KeyID_FFI {
                    unsafe fn ffi_from_const(ffi: *const KeyID_FFI) -> KeyID {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                    unsafe fn ffi_to_const(obj: KeyID) -> *const KeyID_FFI {
                        ferment_interfaces::boxed(KeyID_FFI(obj))
                    }
                    unsafe fn destroy(ffi: *mut KeyID_FFI) {
                        ferment_interfaces::unbox_any(ffi);
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
                impl ferment_interfaces::FFIConversion<HashID> for HashID_FFI {
                    unsafe fn ffi_from_const(ffi: *const HashID_FFI) -> HashID {
                        let ffi_ref = &*ffi;
                        *ffi_ref.0
                    }
                    unsafe fn ffi_to_const(obj: HashID) -> *const HashID_FFI {
                        ferment_interfaces::boxed(HashID_FFI(ferment_interfaces::boxed(obj)))
                    }
                    unsafe fn destroy(ffi: *mut HashID_FFI) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for HashID_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct UsedKeyMatrix_FFI(*mut Vec_bool_FFI);
                impl ferment_interfaces::FFIConversion<UsedKeyMatrix> for UsedKeyMatrix_FFI {
                    unsafe fn ffi_from_const(ffi: *const UsedKeyMatrix_FFI) -> UsedKeyMatrix {
                        let ffi_ref = &*ffi;
                        {
                            let vec = &*ffi_ref.0;
                            {
                                let vec = vec;
                                ferment_interfaces::from_primitive_vec(vec.values, vec.count)
                            }
                        }
                    }
                    unsafe fn ffi_to_const(obj: UsedKeyMatrix) -> *const UsedKeyMatrix_FFI {
                        ferment_interfaces::boxed(UsedKeyMatrix_FFI(
                            ferment_interfaces::FFIConversion::ffi_to(obj),
                        ))
                    }
                    unsafe fn destroy(ffi: *mut UsedKeyMatrix_FFI) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for UsedKeyMatrix_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct ArrayOfArraysOfHashes_FFI(*mut Vec_Vec_HashID_FFI);
                impl ferment_interfaces::FFIConversion<ArrayOfArraysOfHashes> for ArrayOfArraysOfHashes_FFI {
                    unsafe fn ffi_from_const(
                        ffi: *const ArrayOfArraysOfHashes_FFI,
                    ) -> ArrayOfArraysOfHashes {
                        let ffi_ref = &*ffi;
                        {
                            let vec = &*ffi_ref.0;
                            let count = vec.count;
                            let values = vec.values;
                            (0..count)
                                .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                                .collect()
                        }
                    }
                    unsafe fn ffi_to_const(obj: ArrayOfArraysOfHashes) -> *const ArrayOfArraysOfHashes_FFI {
                        ferment_interfaces::boxed(ArrayOfArraysOfHashes_FFI(
                            ferment_interfaces::FFIConversion::ffi_to(obj),
                        ))
                    }
                    unsafe fn destroy(ffi: *mut ArrayOfArraysOfHashes_FFI) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ArrayOfArraysOfHashes_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                pub struct MapOfHashes_FFI(*mut Map_keys_HashID_values_HashID_FFI);
                impl ferment_interfaces::FFIConversion<MapOfHashes> for MapOfHashes_FFI {
                    unsafe fn ffi_from_const(ffi: *const MapOfHashes_FFI) -> MapOfHashes {
                        let ffi_ref = &*ffi;
                        ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
                    }
                    unsafe fn ffi_to_const(obj: MapOfHashes) -> *const MapOfHashes_FFI {
                        ferment_interfaces::boxed(MapOfHashes_FFI(ferment_interfaces::FFIConversion::ffi_to(
                            obj,
                        )))
                    }
                    unsafe fn destroy(ffi: *mut MapOfHashes_FFI) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for MapOfHashes_FFI {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.0);
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
            impl ferment_interfaces::FFIConversion<Vec<HashID>> for Vec_HashID_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_HashID_FFI) -> Vec<HashID> {
                    let ffi_ref = &*ffi;
                    ferment_interfaces::FFIVecConversion::decode(ffi_ref)
                }
                unsafe fn ffi_to_const(obj: Vec<HashID>) -> *const Vec_HashID_FFI {
                    ferment_interfaces::FFIVecConversion::encode(obj)
                }
                unsafe fn destroy(ffi: *mut Vec_HashID_FFI) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl ferment_interfaces::FFIVecConversion for Vec_HashID_FFI {
                type Value = HashID;
                unsafe fn decode(&self) -> Vec<Self::Value> {
                    {
                        let count = self.count;
                        let values = self.values;
                        (0..count)
                            .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                            .collect()
                    }
                }
                unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
                    {
                        ferment_interfaces::boxed(Self {
                            count: obj.len(),
                            values: ferment_interfaces::to_complex_vec(
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
                            ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Vec_bool_FFI {
                pub count: usize,
                pub values: *mut bool,
            }
            impl ferment_interfaces::FFIConversion<Vec<bool>> for Vec_bool_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_bool_FFI) -> Vec<bool> {
                    let ffi_ref = &*ffi;
                    ferment_interfaces::FFIVecConversion::decode(ffi_ref)
                }
                unsafe fn ffi_to_const(obj: Vec<bool>) -> *const Vec_bool_FFI {
                    ferment_interfaces::FFIVecConversion::encode(obj)
                }
                unsafe fn destroy(ffi: *mut Vec_bool_FFI) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl ferment_interfaces::FFIVecConversion for Vec_bool_FFI {
                type Value = Vec<bool>;
                unsafe fn decode(&self) -> Self::Value {
                    ferment_interfaces::from_primitive_vec(self.values as *const _, self.count)
                }
                unsafe fn encode(obj: Self::Value) -> *mut Self {
                    ferment_interfaces::boxed(Self {
                        count: obj.len(),
                        values: ferment_interfaces::boxed_vec(obj),
                    })
                }
            }
            impl Drop for Vec_bool_FFI {
                fn drop(&mut self) {
                    unsafe {
                        {
                            ferment_interfaces::unbox_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Vec_Vec_HashID_FFI {
                pub count: usize,
                pub values: *mut *mut Vec_HashID_FFI,
            }
            impl ferment_interfaces::FFIConversion<Vec<Vec<HashID>>> for Vec_Vec_HashID_FFI {
                unsafe fn ffi_from_const(ffi: *const Vec_Vec_HashID_FFI) -> Vec<Vec<HashID>> {
                    let ffi_ref = &*ffi;
                    ferment_interfaces::FFIVecConversion::decode(ffi_ref)
                }
                unsafe fn ffi_to_const(obj: Vec<Vec<HashID>>) -> *const Vec_Vec_HashID_FFI {
                    ferment_interfaces::FFIVecConversion::encode(obj)
                }
                unsafe fn destroy(ffi: *mut Vec_Vec_HashID_FFI) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl ferment_interfaces::FFIVecConversion for Vec_Vec_HashID_FFI {
                type Value = Vec<HashID>;
                unsafe fn decode(&self) -> Vec<Self::Value> {
                    {
                        let count = self.count;
                        let values = self.values;
                        (0..count)
                            .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                            .collect()
                    }
                }
                unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self {
                    {
                        ferment_interfaces::boxed(Self {
                            count: obj.len(),
                            values: ferment_interfaces::to_complex_vec(
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
                            ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
                        }
                    }
                }
            }

            pub struct Map_keys_HashID_values_HashID_FFI {
                pub count: usize,
                pub keys: *mut *mut HashID_FFI,
                pub values: *mut *mut HashID_FFI,
            }
            impl ferment_interfaces::FFIConversion<BTreeMap<HashID, HashID>> for Map_keys_HashID_values_HashID_FFI
            {
                unsafe fn ffi_from_const(
                    ffi: *const Map_keys_HashID_values_HashID_FFI,
                ) -> BTreeMap<HashID, HashID> {
                    let ffi_ref = &*ffi;
                    ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
                }
                unsafe fn ffi_to_const(
                    obj: BTreeMap<HashID, HashID>,
                ) -> *const Map_keys_HashID_values_HashID_FFI {
                    ferment_interfaces::boxed(Self {
                        count: obj.len(),
                        keys: ferment_interfaces::to_complex_vec(
                            obj.keys().cloned(),
                        ),
                        values: ferment_interfaces::to_complex_vec(
                            obj.values().cloned(),
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut Map_keys_HashID_values_HashID_FFI) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Map_keys_HashID_values_HashID_FFI {
                fn drop(&mut self) {
                    unsafe {
                        ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                        ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
                    }
                }
            }
        }
    }
}


