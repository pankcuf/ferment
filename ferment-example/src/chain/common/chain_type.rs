use crate::nested::HashID;

pub trait IHaveChainSettings {
    fn name(&self) -> String;
    fn genesis_hash(&self) -> HashID;
    fn genesis_height(&self) -> u32;
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
}

// // FFI Opaque
// // Trait scope
// #[allow(non_camel_case_types)]
// #[repr(C)]
// pub struct IHaveChainSettings_VTable {
//     pub name: unsafe extern "C" fn(*const ()) -> *mut std::os::raw::c_char,
//     pub genesis_hash: unsafe extern "C" fn(*const ()) -> *mut crate::fermented::types::nested::HashID_FFI,
//     pub genesis_height: unsafe extern "C" fn(*const ()) -> u32,
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
// static CHAIN_TYPE_VTABLE: IHaveChainSettings_VTable = {
//     unsafe extern "C" fn chain_type_name(obj: *const ()) -> *mut std::os::raw::c_char {
//         ferment_interfaces::FFIConversion::ffi_to(&(*(obj as *const ChainType)).name())
//     }
//     unsafe extern "C" fn chain_type_genesis_hash(obj: *const ()) -> *mut crate::fermented::types::nested::HashID_FFI {
//         ferment_interfaces::FFIConversion::ffi_to(&(*(obj as *const ChainType)).genesis())
//     }
//     unsafe extern "C" fn chain_type_genesis_height(obj: *const ()) -> u32 {
//         (*(obj as *const ChainType)).genesis_height()
//     }
//     IHaveChainSettings_VTable {
//         name: chain_type_name,
//         genesis_hash: chain_type_genesis_hash,
//         genesis_height: chain_type_genesis_height,
//     }
// };
// #[no_mangle]
// pub extern "C" fn chain_type_as_ihavechainsettings_trait(obj: *const ChainType) -> IHaveChainSettings_TraitObject {
//     IHaveChainSettings_TraitObject {
//         object: obj as *const (),
//         vtable: &CHAIN_TYPE_VTABLE,
//     }
// }
//
// static DEVNET_TYPE_VTABLE: IHaveChainSettings_VTable = {
//     unsafe extern "C" fn devnet_type_name(obj: *const ()) -> *mut std::os::raw::c_char {
//         ferment_interfaces::FFIConversion::ffi_to(&(*(obj as *const DevnetType)).name())
//     }
//     unsafe extern "C" fn devnet_type_genesis_hash(obj: *const ()) -> *mut crate::fermented::types::nested::HashID_FFI {
//         ferment_interfaces::FFIConversion::ffi_to(&(*(obj as *const DevnetType)).genesis())
//     }
//     unsafe extern "C" fn devnet_type_genesis_height(obj: *const ()) -> u32 {
//         (*(obj as *const DevnetType)).genesis_height()
//     }
//     IHaveChainSettings_VTable {
//         name: devnet_type_name,
//         genesis_hash: devnet_type_genesis_hash,
//         genesis_height: devnet_type_genesis_height,
//     }
// };
// #[no_mangle]
// pub extern "C" fn devnet_type_as_ihavechainsettings_trait(obj: *const DevnetType) -> IHaveChainSettings_TraitObject {
//     IHaveChainSettings_TraitObject {
//         object: obj as *const (),
//         vtable: &DEVNET_TYPE_VTABLE,
//     }
// }

#[allow(dead_code)]
pub enum ExcludedEnum {
    Variant1,
    Variant2,
}
