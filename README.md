# rs-ffi-macro
Proc macro for FFI (work in progress)

Allows to generate an FFI-compliant equivalent for rust types (structures, enums, arrays, functions).

The project is a rust-workspace consisting of 2 crates:
1. `rs-ffi-interfaces`: A trait that provides conversion methods from/to FFI-compatible types and some helper functions and structures
2. `rs-ffi-macro-derive`: a procedural macro that uses the power of the `syn` crate to generate FFI-compatible types and their conversions.

A procedural macro consists of 2 macros:

1. `impl_ffi_conv` - for structures/enums
2. `impl_ffi_fn_conv` - for functions
3. `impl_ffi_ty_conv` - for type aliases

**Usage**
Crate is not published yet, so use it for example locally

```toml
rs-ffi-interfaces = { path = "../../rs-ffi-macro/rs-ffi-interfaces" }
rs-ffi-macro-derive = { path = "../../rs-ffi-macro/rs-ffi-macro-derive" }
```

Using the macro implies using `cbindgen` with a configuration like (has taken from actual apple-bindings):

```rust
extern crate cbindgen;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = cbindgen::Config::from_file("./cbindgen.toml").expect("Error config");
    // Here we must list the names of the crates from which the generated structures will be exported in order to include them in the final C-header
    let includes = vec![/**/];
    config.language = cbindgen::Language::C;
    config.parse = cbindgen::Config {
        language: cbindgen::Language::C,
        parse: cbindgen::ParseConfig {
            parse_deps: true,
            include: Some(includes.clone()),
            extra_bindings: includes.clone(),
            expand: cbindgen::ParseExpandConfig {
                crates: includes.clone(),
                all_features: false,
                default_features: false,
                features: None,
                profile: cbindgen::Profile::Debug,
            },
            ..Default::default()
        },
        enumeration: cbindgen::EnumConfig {
            prefix_with_name: true,
            ..Default::default()
        },
        braces: cbindgen::Braces::SameLine,
        line_length: 80,
        tab_width: 4,
        documentation_style: cbindgen::DocumentationStyle::C,
        include_guard: Some("dash_shared_core_h".to_string()),
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

For type aliases labeled with `impl_ffi_ty_conv`

```rust
#[rs_ffi_macro_derive::impl_ffi_ty_conv]
pub type HashID = [u8; 32];
```
the following code will be generated:
```rust
#[repr(C)]
#[derive(Clone, Debug)] 
pub struct HashIDFFI(*mut [u8; 32]); 

impl rs_ffi_interfaces::FFIConversion<HashID> for HashIDFFI {
    unsafe fn ffi_from(ffi : * mut HashIDFFI) -> HashID { 
        let ffi_ref = &*ffi; 
        *ffi_ref.0
    } 
    unsafe fn ffi_to(obj : HashID) -> *mut HashIDFFI { 
        rs_ffi_interfaces::boxed(HashIDFFI(rs_ffi_interfaces::boxed(obj))) 
    }
} 
impl Drop for HashIDFFI {
    fn drop(&mut self) { 
        unsafe { 
            rs_ffi_interfaces::unbox_any(self.0);
        }
    }
}
```

Current limitations:
- doesn't work with traits and &self
- We should mark all structures that involved into export with the macro definition
- There is some difficulty with handling type aliases. Therefore, if possible, they should be avoided. Because, in order to guarantee that it can be processed, one has to wrap it in an unnamed struct. Which is, for most cases, less efficient than using the type it uses directly. That is, `pub type KeyID = u32` becomes `pub struct KeyIDFFI(u32)` The alternative is to store a hardcoded dictionary with them.
Another alternative is to write a separate build script that collects these types before running the macro to generate this dictionary on the fly. But for now, this is too much of a complication. 

Generic mangling rules

Macro `ffi-dictionary` should wrap the scope of the application context.
It provides first-level expansion with mangled generic structures.
Conversion follows some mangling rules and gives the name for ffi structure. 
Examples for translated names:
- `Vec<u8>` -> `Vec_u8_FFI`
- `Vec<u32>` -> `Vec_u32_FFI`
- `Vec<Vec<u32>>` -> `Vec_Vec_u32_FFI`
- `BTreeMap<crate::HashID, Vec<u32>>` -> `Map_keys_crate_HashID_values_Vec_u32_FFI`
- `BTreeMap<crate::HashID, Vec<u32>>` -> `Map_keys_u32_values_Vec_u32_FFI`
- `BTreeMap<crate::HashID, BTreeMap<crate::HashID, Vec<u32>>>` -> `Map_keys_crate_HashID_values_Map_keys_crate_HashID_values_Vec_u32_FFI`
- etc
Then macro implements the necessary conversions for these structures. Example for such an expansion:
```rust
#[repr(C)] #[derive(Clone)] 
pub struct Map_keys_self_HashID_values_self_HashID_FFI {
    pub count: usize, 
    pub keys: *mut *mut self::HashIDFFI, 
    pub values: * mut * mut self::HashIDFFI,
} 
impl rs_ffi_interfaces::FFIConversion<BTreeMap<self::HashID, self::HashID>> for Map_keys_self_HashID_values_self_HashID_FFI {
    unsafe fn ffi_from_const(ffi: *const Map_keys_self_HashID_values_self_HashID_FFI) -> BTreeMap<self::HashID, self::HashID> {
        let ffi_ref = &*ffi;
        (0..ffi_ref.count).fold(BTreeMap<self::HashID, self::HashID>::new(), |mut acc, i| {
            let key = *ffi_ref.keys.add(i); 
            let value = *ffi_ref.values.add(i); 
            acc.insert(key, value); 
            acc
        })
    } 
    unsafe fn ffi_to_const(obj: BTreeMap<self::HashID, self::HashID>) -> *const Map_keys_self_HashID_values_self_HashID_FFI {
        rs_ffi_interfaces::boxed(Self { 
            count: obj.len(), 
            keys: rs_ffi_interfaces::boxed_vec(obj.keys().map(|o| <self::HashID as rs_ffi_interfaces::FFIConversion>::ffi_from_const(o)).collect()), 
            values: rs_ffi_interfaces::boxed_vec(obj.values().map(|o| <self::HashID as rs_ffi_interfaces::FFIConversion>::ffi_from_const(o)).collect())
        })
    } 
    unsafe fn destroy(ffi: *mut Map_keys_self_HashID_values_self_HashID_FFI) { 
        rs_ffi_interfaces::unbox_any(ffi); 
    }
} 
impl Drop for Map_keys_self_HashID_values_self_HashID_FFI {
    fn drop(&mut self) {
        unsafe {
            rs_ffi_interfaces::unbox_vec_ptr(self.keys, self.count);
            rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
        }
    }
}
```