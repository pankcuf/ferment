use crate::nested::HashID;

#[ferment_macro::export]
pub trait IHaveChainSettings {
    fn name(&self) -> String;
    fn genesis_hash(&self) -> HashID;
    fn genesis_height(&self) -> u32;
    fn has_genesis_hash(&self, hash: HashID) -> bool {
        self.genesis_hash() == hash
    }
    fn get_hash_by_hash(&self, hash: HashID) -> HashID {
        hash
    }
    fn should_process_llmq_of_type(&self, llmq_type: u16) -> bool;
}

#[ferment_macro::export(IHaveChainSettings)]
#[derive(Clone, PartialOrd, Ord, Hash, Eq, PartialEq)]
pub enum ChainType {
    MainNet,
    TestNet,
    DevNet(DevnetType)
}

#[derive(Clone, Default, PartialOrd, Ord, Hash, Eq, PartialEq)]
#[ferment_macro::export(IHaveChainSettings)]
pub enum DevnetType {
    JackDaniels = 0,
    Devnet333 = 1,
    Chacha = 2,
    #[default]
    Mojito = 3,
    WhiteRussian = 4,
}

impl IHaveChainSettings for ChainType {
    fn name(&self) -> String {
        match self {
            Self::MainNet => "mainnet".to_string(),
            Self::TestNet => "testnet".to_string(),
            Self::DevNet(devnet) => devnet.name()
        }
    }

    fn genesis_hash(&self) -> HashID {
        [0u8; 32]
    }

    fn genesis_height(&self) -> u32 {
        0
    }

    fn should_process_llmq_of_type(&self, llmq_type: u16) -> bool {
        llmq_type != 0
    }
}

impl IHaveChainSettings for DevnetType {
    fn name(&self) -> String {
        format!("devnet-{}", match self {
            DevnetType::JackDaniels => "jack-daniels",
            DevnetType::Devnet333 => "333",
            DevnetType::Chacha => "chacha",
            DevnetType::Mojito => "mojito",
            DevnetType::WhiteRussian => "white-russian",
        })
    }

    fn genesis_hash(&self) -> HashID {
        [0u8; 32]
    }

    fn genesis_height(&self) -> u32 {
        1
    }

    fn should_process_llmq_of_type(&self, llmq_type: u16) -> bool {
        llmq_type != 0
    }
}

// FFI Opaque
// Trait scope
// #[allow(non_camel_case_types)]
// #[repr(C)]
// pub struct IHaveChainSettings_VTable {
//     pub name: unsafe extern "C" fn(*const ()) -> *mut std::os::raw::c_char,
//     pub genesis_hash: unsafe extern "C" fn(*const ()) -> *mut crate::fermented::types::nested::HashID_FFI,
//     pub genesis_height: unsafe extern "C" fn(*const ()) -> u32,
//     pub should_process_llmq_of_type: unsafe extern "C" fn(*const (), llmq_type: u16) -> bool,
//     pub has_genesis_hash: unsafe extern "C" fn(*const (), hash: *const crate::fermented::types::nested::HashID_FFI) -> bool,
//     pub get_hash_by_hash: unsafe extern "C" fn(*const (), hash: *const crate::fermented::types::nested::HashID_FFI) -> *mut crate::fermented::types::nested::HashID_FFI,
// }
//
// #[allow(non_camel_case_types)]
// #[repr(C)]
// pub struct IHaveChainSettings_TraitObject {
//     pub object: *const (),
//     pub vtable: *const IHaveChainSettings_VTable,
// }
//
//
// // Trait implementation scope
// #[allow(non_snake_case)]
// static ChainType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
//     unsafe extern "C" fn ChainType_name(obj: *const ()) -> *mut std::os::raw::c_char {
//         let cast_obj = &(*(obj as *const ChainType));
//         let result = cast_obj.name();
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//     unsafe extern "C" fn ChainType_genesis_hash(obj: *const ()) -> *mut crate::fermented::types::nested::HashID_FFI {
//         let cast_obj = &(*(obj as *const ChainType));
//         let result = cast_obj.genesis_hash();
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//     unsafe extern "C" fn ChainType_genesis_height(obj: *const ()) -> u32 {
//         let cast_obj = &(*(obj as *const ChainType));
//         let result = cast_obj.genesis_height();
//         result
//     }
//     unsafe extern "C" fn ChainType_should_process_llmq_of_type(obj: *const (), llmq_type: u16) -> bool {
//         let cast_obj = &(*(obj as *const ChainType));
//         let result = cast_obj.should_process_llmq_of_type(llmq_type);
//         result
//     }
//     unsafe extern "C" fn ChainType_has_genesis_hash(obj: *const (), hash: *const crate::fermented::types::nested::HashID_FFI) -> bool {
//         let cast_obj = &(*(obj as *const ChainType));
//         let hash = ferment_interfaces::FFIConversion::ffi_from_const(hash);
//         let result = cast_obj.has_genesis_hash(hash);
//         result
//     }
//     unsafe extern "C" fn ChainType_get_hash_by_hash(obj: *const (), hash: *const crate::fermented::types::nested::HashID_FFI) -> *mut crate::fermented::types::nested::HashID_FFI {
//         let cast_obj = &(*(obj as *const ChainType));
//         let hash = ferment_interfaces::FFIConversion::ffi_from_const(hash);
//         let result = cast_obj.get_hash_by_hash(hash);
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//     IHaveChainSettings_VTable {
//         name: ChainType_name,
//         genesis_hash: ChainType_genesis_hash,
//         genesis_height: ChainType_genesis_height,
//         should_process_llmq_of_type: ChainType_should_process_llmq_of_type,
//         has_genesis_hash: ChainType_has_genesis_hash,
//         get_hash_by_hash: ChainType_get_hash_by_hash,
//     }
// };
// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn ChainType_as_IHaveChainSettings_TraitObject(obj: *const ChainType) -> IHaveChainSettings_TraitObject {
//     IHaveChainSettings_TraitObject {
//         object: obj as *const (),
//         vtable: &ChainType_IHaveChainSettings_VTable,
//     }
// }
//
// #[allow(non_snake_case)]
// static DevnetType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
//     unsafe extern "C" fn DevnetType_name(obj: *const ()) -> *mut std::os::raw::c_char {
//         let cast_obj = &(*(obj as *const DevnetType));
//         let result = cast_obj.name();
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//     unsafe extern "C" fn DevnetType_genesis_hash(obj: *const ()) -> *mut crate::fermented::types::nested::HashID_FFI {
//         let cast_obj = &(*(obj as *const DevnetType));
//         let result = cast_obj.genesis_hash();
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//     unsafe extern "C" fn DevnetType_genesis_height(obj: *const ()) -> u32 {
//         let cast_obj = &(*(obj as *const DevnetType));
//         let result = cast_obj.genesis_height();
//         result
//     }
//     unsafe extern "C" fn DevnetType_should_process_llmq_of_type(obj: *const (), llmq_type: u16) -> bool {
//         let cast_obj = &(*(obj as *const DevnetType));
//         let result = cast_obj.should_process_llmq_of_type(llmq_type);
//         result
//     }
//     unsafe extern "C" fn DevnetType_has_genesis_hash(obj: *const (), hash: *const crate::fermented::types::nested::HashID_FFI) -> bool {
//         let cast_obj = &(*(obj as *const DevnetType));
//         let hash = ferment_interfaces::FFIConversion::ffi_from_const(hash);
//         let result = cast_obj.has_genesis_hash(hash);
//         result
//     }
//     unsafe extern "C" fn DevnetType_get_hash_by_hash(obj: *const (), hash: *const crate::fermented::types::nested::HashID_FFI) -> *mut crate::fermented::types::nested::HashID_FFI {
//         let cast_obj = &(*(obj as *const DevnetType));
//         let hash = ferment_interfaces::FFIConversion::ffi_from_const(hash);
//         let result = cast_obj.get_hash_by_hash(hash);
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//     IHaveChainSettings_VTable {
//         name: DevnetType_name,
//         genesis_hash: DevnetType_genesis_hash,
//         genesis_height: DevnetType_genesis_height,
//         should_process_llmq_of_type: DevnetType_should_process_llmq_of_type,
//         has_genesis_hash: DevnetType_has_genesis_hash,
//         get_hash_by_hash: DevnetType_get_hash_by_hash,
//     }
// };
// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn DevnetType_as_IHaveChainSettings_TraitObject(obj: *const DevnetType) -> IHaveChainSettings_TraitObject {
//     IHaveChainSettings_TraitObject {
//         object: obj as *const (),
//         vtable: &DevnetType_IHaveChainSettings_VTable,
//     }
// }

#[allow(dead_code)]
pub enum ExcludedEnum {
    Variant1,
    Variant2,
}
