# rs-ffi-macro
Proc macro for FFI (work in progress)

Allows to generate an FFI-compliant equivalent for rust types (structures, enums, arrays, functions).

The project is a rust-workspace consisting of 2 crates:
1. `rs-ffi-interfaces`: A trait that provides conversion methods from/to FFI-compatible types and some helper functions and structures
2. `rs-ffi-macro-derive`: a procedural macro that uses the power of the `syn` crate to generate FFI-compatible types and their conversions.

A procedural macro consists of 2 macros:

1. `impl_ffi_conv` - for structures/enums
2. `impl_ffi_fn_conv` - for functions

**Usage**
Crate is not published yet, so use it for example locally

```toml
rs-ffi-interfaces = { path = "../../rs-ffi-macro/rs-ffi-interfaces" }
rs-ffi-macro-derive = { path = "../../rs-ffi-macro/rs-ffi-macro-derive" }
```

Using the macro implies using `cbindgen` with a configuration like:

```rust
extern crate cbindgen;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = cbindgen::Config::from_file("./cbindgen.toml").expect("Error config");
    // Here we must list the names of the crates from which the generated structures will be exported in order to include them in the final C-header
    let includes = vec![/**/];
    config.language = cbindgen::Language::C;
    config.parse = cbindgen::ParseConfig {
    parse_deps: true,
    include: Some(includes.clone()),
    extra_bindings: includes,
    ..Default::default()
    };
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("target/bindings.h");
}
```

**Examples**

For the structure labeled with `impl_ffi_conv`

```rust
#[derive(Clone)]
#[rs_ffi_macro_derive::impl_ffi_conv]
pub struct LLMQSnapshot {
    pub member_list: Vec<u8>,
    pub skip_list: Vec<i32>,
    pub skip_list_mode: crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode,
}
```
the following code with FFI-compatible fields and corresponding from/to conversions will be generated:
```rust
#[repr(C)] 
#[derive(Clone, Debug)] 
pub struct LLMQSnapshotFFI {
    pub member_list: *mut rs_ffi_interfaces::VecFFI<u8>, 
    pub skip_list: *mut rs_ffi_interfaces::VecFFI<i32>, 
    pub skip_list_mode: *mut crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipModeFFI,
} 
impl rs_ffi_interfaces::FFIConversion<LLMQSnapshot> for LLMQSnapshotFFI {
    unsafe fn ffi_from(ffi: *mut LLMQSnapshotFFI) -> LLMQSnapshot {
        let ffi_ref = &*ffi; 
        LLMQSnapshot { 
            member_list: { 
                let vec = &*ffi_ref.member_list; 
                std::slice::from_raw_parts(vec.values as *const u8, vec.count).to_vec()
            }, 
            skip_list: { 
                let vec = &*ffi_ref.skip_list; 
                std::slice::from_raw_parts(vec.values as *const i32, vec.count).to_vec()
            }, 
            skip_list_mode: rs_ffi_interfaces::FFIConversion::ffi_from(ffi_ref.skip_list_mode),
        }
    } 
    unsafe fn ffi_to(obj: LLMQSnapshot) -> *mut LLMQSnapshotFFI { 
        rs_ffi_interfaces::boxed(LLMQSnapshotFFI { 
            member_list: rs_ffi_interfaces::boxed({ 
                let vec = obj.member_list; 
                rs_ffi_interfaces::VecFFI { 
                    count: vec.len(), 
                    values: rs_ffi_interfaces::boxed_vec(vec.clone())
                } 
            }), 
            skip_list: rs_ffi_interfaces::boxed({ 
                let vec = obj.skip_list; 
                rs_ffi_interfaces::VecFFI {
                    count: vec.len(), 
                    values: rs_ffi_interfaces::boxed_vec(vec.clone())
                }
            }), 
            skip_list_mode: rs_ffi_interfaces::FFIConversion::ffi_to(obj.skip_list_mode),
        })
    } 
    unsafe fn ffi_from_opt(ffi: *mut LLMQSnapshotFFI) -> Option<LLMQSnapshot> {
        (!ffi.is_null()).then_some(<Self as rs_ffi_interfaces::FFIConversion<LLMQSnapshot>>::ffi_from(ffi))
    } 
    unsafe fn ffi_to_opt(obj: Option<LLMQSnapshot>) -> *mut LLMQSnapshotFFI {
        obj.map_or(std::ptr::null_mut(), |o| <Self as rs_ffi_interfaces::FFIConversion<LLMQSnapshot>>::ffi_to(o))
    }
    unsafe fn destroy(ffi: *mut LLMQSnapshotFFI) { 
        rs_ffi_interfaces::unbox_any(ffi); 
    }
}
impl Drop for LLMQSnapshotFFI {
    fn drop(&mut self) {
        unsafe {
            let ffi_ref = self; 
            rs_ffi_interfaces::unbox_any(ffi_ref.member_list); 
            rs_ffi_interfaces::unbox_any(ffi_ref.skip_list);
            <crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipModeFFI as rs_ffi_interfaces::FFIConversion<crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode>>::destroy(ffi_ref.skip_list_mode) ;
        }
    }
}
```

For the function labeled with `impl_ffi_fn_conv`

```rust
#[rs_ffi_macro_derive::impl_ffi_fn_conv]
pub fn address_with_script_pubkey(script: Vec<u8>, chain_type: crate::chain::common::chain_type::ChainType) -> Option<String> {
    address::with_script_pub_key(&script, &chain_type.script_map())
}
```
the following code will be generated:
```rust
#[no_mangle] 
pub unsafe extern "C" fn ffi_address_with_script_pubkey(
    script: *mut rs_ffi_interfaces::VecFFI<u8>, 
    chain_type: *mut crate::chain::common::chain_type::ChainTypeFFI) 
    -> *mut std::os::raw::c_char {
    let obj = address_with_script_pubkey(
        {
            let vec = &*script;
            std::slice::from_raw_parts(vec.values as *const u8, vec.count).to_vec()
        }, 
        rs_ffi_interfaces::FFIConversion::ffi_from(chain_type)
    );
    rs_ffi_interfaces::FFIConversion::ffi_to_opt(obj)
}
```



