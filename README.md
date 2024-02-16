**# ferment
Syntax-tree morphing tool for FFI (work in progress)

Allows to generate an FFI-compliant equivalent for rust types (structures, enums, types, functions).

The project is a rust-workspace consisting several crates:
1. `ferment-interfaces`: A traits that provide conversion methods from/to FFI-compatible types and some helper functions and structures
2. `ferment-macro`: a procedural macro that just catch target code as syn-based item.
3. `ferment-example`: provides basic example.
4. `ferment-example-nested`: provides example with dependent fermented crate.
5. `ferment`: a tool for morphing FFI-compatible syntax trees that uses the power of the `syn` crate.

A procedural macro consists of 2 macros:

1. `export` - for structures / enums / functions / types
2. `register` - for custom-defined conversions

**Usage**

Crate is not published yet, so use it for example locally

```toml
ferment-interfaces = { path = "../../ferment/ferment-interfaces" }
ferment-macro = { path = "../../ferment/ferment-macro" }
ferment = { path = "../../ferment/ferment" }
```

Using the tool implies using `cbindgen` with a configuration like this:

```rust
extern crate cbindgen;

fn main() {
    extern crate cbindgen;
    extern crate ferment;

    use std::process::Command;

    fn main() {

        match ferment::Builder::new()
            .with_mod_name("fermented")
            .with_crates(vec![])
            .generate() {
            Ok(()) => match Command::new("cbindgen")
                .args(&["--config", "cbindgen.toml", "-o", "target/example.h"])
                .status() {
                Ok(status) => println!("Bindings generated into target/example.h with status: {}", status),
                Err(err) => panic!("Can't generate bindings: {}", err)
            }
            Err(err) => panic!("Can't create FFI expansion: {}", err)
        }
    }
}
```

**Examples**

For traits marked for export like this:
```rust
#[ferment_macro::export]
pub trait IHaveChainSettings {
    fn name(&self) -> String;
}
```
You can also use macro with comma-separated trait names 
```rust
#[ferment_macro::export(IHaveChainSettings)]
pub enum ChainType {
    MainNet,
    TestNet,
    DevNet(DevnetType)
}
```
This will expose bindings for trait methods for particular types

For the structure labeled with `ferment_macro::export`

```rust
#[derive(Clone)]
#[ferment_macro::export]
pub struct LLMQSnapshot {
    pub member_list: Vec<u8>,
    pub skip_list: Vec<i32>,
    pub skip_list_mode: LLMQSnapshotSkipMode,
    pub option_vec: Option<Vec<u8>>,
}
```
the following code with FFI-compatible fields and corresponding from/to conversions will be generated:
```rust
#[doc = "FFI-representation of the "crate::model::snapshot::LLMQSnapshot""]
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLMQSnapshot {
    pub member_list: *mut crate::fermented::generics::Vec_u8,
    pub skip_list: *mut crate::fermented::generics::Vec_i32,
    pub skip_list_mode: *mut crate::fermented::types::model::snapshot::LLMQSnapshotSkipMode,
    pub option_vec: *mut crate::fermented::generics::Vec_u8,
}
impl ferment_interfaces::FFIConversion<crate::model::snapshot::LLMQSnapshot> for LLMQSnapshot {
    unsafe fn ffi_from_const(ffi: *const LLMQSnapshot) -> crate::model::snapshot::LLMQSnapshot {
        let ffi_ref = &*ffi;
        crate::model::snapshot::LLMQSnapshot {
            member_list: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.member_list),
            skip_list: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.skip_list),
            skip_list_mode: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.skip_list_mode),
            option_vec: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.option_vec),
        }
    }
    unsafe fn ffi_to_const(obj: crate::model::snapshot::LLMQSnapshot) -> *const LLMQSnapshot {
        ferment_interfaces::boxed(LLMQSnapshot {
            member_list: ferment_interfaces::FFIConversion::ffi_to(obj.member_list),
            skip_list: ferment_interfaces::FFIConversion::ffi_to(obj.skip_list),
            skip_list_mode: ferment_interfaces::FFIConversion::ffi_to(obj.skip_list_mode),
            option_vec: match obj.option_vec {
                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                None => std::ptr::null_mut(),
            },
        })
    }
    unsafe fn destroy(ffi: *mut LLMQSnapshot) {
        ferment_interfaces::unbox_any(ffi);
    }
}
impl Drop for LLMQSnapshot {
    fn drop(&mut self) {
        unsafe {
            let ffi_ref = self;
            ferment_interfaces::unbox_any(ffi_ref.member_list);
            ferment_interfaces::unbox_any(ffi_ref.skip_list);
            <crate::fermented::types::model::snapshot::LLMQSnapshotSkipMode as ferment_interfaces::FFIConversion<crate::model::snapshot::LLMQSnapshotSkipMode>>::
            destroy(ffi_ref.skip_list_mode);
            if !ffi_ref.option_vec.is_null() {
                ferment_interfaces::unbox_any(ffi_ref.option_vec);
            };
        }
    }
}
#[doc = "# Safety"]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LLMQSnapshot_ctor(
    member_list: *mut crate::fermented::generics::Vec_u8,
    skip_list: *mut crate::fermented::generics::Vec_i32,
    skip_list_mode: *mut crate::fermented::types::model::snapshot::LLMQSnapshotSkipMode,
    option_vec: *mut crate::fermented::generics::Vec_u8)
    -> *mut LLMQSnapshot {
    ferment_interfaces::boxed(LLMQSnapshot {
        member_list,
        skip_list,
        skip_list_mode,
        option_vec,
    })
}
#[doc = "# Safety"]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LLMQSnapshot_destroy(ffi: *mut LLMQSnapshot) {
    ferment_interfaces::unbox_any(ffi);
}

```

For the function labeled with `export`

```rust
#[ferment_macro::export]
pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
    Some(format_args!("{0:?}", script).to_string())
}
```
the following code will be generated:
```rust
#[doc = "FFI-representation of the "address_with_script_pubkey""]
#[doc = "# Safety"]
#[no_mangle]
pub unsafe extern "C" fn ffi_address_with_script_pubkey(script: *mut crate::fermented::generics::Vec_u8) -> *mut std::os::raw::c_char {
    let conversion = ferment_interfaces::FFIConversion::ffi_from(script);
    let obj = crate::example::address::address_with_script_pubkey(conversion);
    ferment_interfaces::FFIConversion::ffi_to_opt(obj)
}
```

For type aliases labeled with `export`

```rust
#[ferment_macro::export]
pub type HashID = [u8; 32];
```
the following code will be generated in `crate::fermented::types::*` with similar conversions and bindings:
```rust
#[repr(C)]
#[derive(Clone, Debug)]
pub struct HashID(*mut [u8; 32]);
```

For traits labeled with `export`
```rust
#[ferment_macro::export]
pub trait IHaveChainSettings { 
    // ..
}
```
There will be vtable and trait obj generated
```rust
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IHaveChainSettings_VTable { 
    // ..
}
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IHaveChainSettings_TraitObject {
    pub object: *const (),
    pub vtable: *const IHaveChainSettings_VTable,
}
```
and bindings for their implementors like this:
```rust
#[doc = "# Safety"]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ChainType_as_IHaveChainSettings_TraitObject(
    obj: *const crate::chain::common::chain_type::ChainType) 
    -> IHaveChainSettings_TraitObject {
    IHaveChainSettings_TraitObject {
        object: obj as *const (),
        vtable: &ChainType_IHaveChainSettings_VTable,
    }
}
#[doc = "# Safety"]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ChainType_as_IHaveChainSettings_TraitObject_destroy(obj: IHaveChainSettings_TraitObject) {
    ferment_interfaces::unbox_any(obj.object as *mut crate::chain::common::chain_type::ChainType);
}

```
using this code cbindgen will be able to generate binding 
```
struct IHaveChainSettings_TraitObject ChainType_as_IHaveChainSettings_TraitObject(const struct ChainType *obj);
void ChainType_as_IHaveChainSettings_TraitObject_destroy(struct IHaveChainSettings_TraitObject obj);

```
Current limitations:
- We should mark all structures that involved into export with the macro definition
- There is some difficulty with handling type aliases. Therefore, if possible, they should be avoided. Because, in order to guarantee that it can be processed, one has to wrap it in an unnamed struct. Which is, for most cases, less efficient than using the type it uses directly. That is, `pub type KeyID = u32` becomes `pub struct KeyID_FFI(u32)` There will be a support at some point.

**Generic mangling rules**

Conversion follows some mangling rules and gives the name for ffi structure. 
Examples for translated names:
- `Vec<u8>` -> `Vec_u8`
- `Vec<u32>` -> `Vec_u32`
- `Vec<Vec<u32>>` -> `Vec_Vec_u32`
- `BTreeMap<HashID, Vec<u32>>` -> `std_collections_Map_keys_crate_HashID_values_Vec_u32`
- `BTreeMap<HashID, Vec<u32>>` -> `std_collections_Map_keys_u32_values_Vec_u32`
- `BTreeMap<HashID, BTreeMap<HashID, Vec<u32>>>` -> `std_collections_Map_keys_crate_HashID_values_std_collections_Map_keys_crate_HashID_values_Vec_u32`
- etc

Then macro implements the necessary conversions for these structures. Example for `BTreeMap<HashID, Vec<HashID>>`:
```rust
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    pub count: usize,
    pub keys: *mut *mut crate::fermented::types::nested::HashID,
    pub values: *mut *mut crate::fermented::generics::Vec_crate_nested_HashID,
}
impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>>
for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    unsafe fn ffi_from_const(
        ffi: *const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID)
        -> std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>> {
        let ffi_ref = &*ffi;
        ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
    }
    unsafe fn ffi_to_const(
        obj: std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>)
        -> *const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
        ferment_interfaces::boxed(Self {
            count: obj.len(),
            keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
            values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
        })
    }
    unsafe fn destroy(ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID) {
        ferment_interfaces::unbox_any(ffi);
    }
}
impl Drop for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
            ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
        }
    }
}
```

The final generated code is placed in the file specified in the configuration like this:
```rust
pub mod types {
    // package relationships are inherited
    // so type like crate::some_module::SomeStruct will be expanded like this:
    pub mod some_module {
        pub struct SomeStruct {
            // ...
        }
    }
}
pub mod generics {
    // We expand generic types separately here to avoid duplication
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
        // ..
    }
}
```

**Manual conversion support** 
- We can use `[ferment_macro::register(SomeFFIIncompatibleStructOrWhatever)]`
- It allows us to manually create custom conversions for types.
- It's especially important for non-fermentable code like types from rust std lib or from any other 3-rd party-crates.
- [Example](https://github.com/pankcuf/ferment/blob/ff10bec42c55935a3d2b5c457d50e6b5352b418c/ferment-example/src/asyn/query.rs#L1C1-L26C3)

**[TODO](https://github.com/pankcuf/ferment/blob/master/TODO.md)**

**[CHANGELOG](https://github.com/pankcuf/ferment/blob/master/CHANGELOG.md)**

**Memory Cleanup Responsibility**
Assuming we have the following structures and method:
```rust
#[ferment_macro::export]
pub struct InnerStruct {
    pub i1: u64,
    pub i2: u64,
}
#[ferment_macro::export]
pub struct OuterStruct {
    pub o1: InnerStruct,
    pub o2: InnerStruct,
}
#[ferment_macro::export]
pub fn create_outer(o1: InnerStruct, o2: InnerStruct) -> OuterStruct {
    OuterStruct {
        o1,
        o2,
    }
}
```
Ferment will produce the following FFI-compatible code:
```rust
#[doc = "FFI-representation of the [`crate::OuterStruct`]"]
#[repr(C)]
#[derive(Clone)]
pub struct OuterStruct {
    pub o1: *mut crate::fermented::types::InnerStruct,
    pub o2: *mut crate::fermented::types::InnerStruct,
}
impl ferment_interfaces::FFIConversion<crate::OuterStruct> for OuterStruct {
    unsafe fn ffi_from_const(ffi: *const OuterStruct) -> crate::OuterStruct {
        let ffi_ref = &*ffi;
        crate::OuterStruct {
            o1: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o1),
            o2: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o2),
        }
    }
    unsafe fn ffi_to_const(obj: crate::OuterStruct) -> *const OuterStruct {
        ferment_interfaces::boxed(OuterStruct {
            o1: ferment_interfaces::FFIConversion::ffi_to(obj.o1),
            o2: ferment_interfaces::FFIConversion::ffi_to(obj.o2),
        })
    }
    unsafe fn destroy(ffi: *mut OuterStruct) {
        ferment_interfaces::unbox_any(ffi);
    }
}
impl Drop for OuterStruct {
    fn drop(&mut self) {
        unsafe {
            let ffi_ref = self;
            ferment_interfaces::unbox_any(ffi_ref.o1);
            ferment_interfaces::unbox_any(ffi_ref.o2);
        }
    }
}
#[doc = r" # Safety"]
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn OuterStruct_ctor(
    o1: *mut crate::fermented::types::InnerStruct,
    o2: *mut crate::fermented::types::InnerStruct,
) -> *mut OuterStruct {
    ferment_interfaces::boxed(OuterStruct { o1, o2 })
}
#[doc = r" # Safety"]
#[no_mangle]
pub unsafe extern "C" fn OuterStruct_destroy(ffi: *mut OuterStruct) {
    ferment_interfaces::unbox_any(ffi);
}

#[doc = "FFI-representation of the [`create_outer`]"]
#[doc = r" # Safety"]
#[no_mangle]
pub unsafe extern "C" fn create_outer(
    o1: *mut crate::fermented::types::InnerStruct,
    o2: *mut crate::fermented::types::InnerStruct,
) -> *mut crate::fermented::types::OuterStruct {
    let obj = crate::create_outer(
        ferment_interfaces::FFIConversion::ffi_from(o1),
        ferment_interfaces::FFIConversion::ffi_from(o2),
    );
    ferment_interfaces::FFIConversion::ffi_to(obj)
}
```
This will produce C-bindings like this:
```c
struct OuterStruct *OuterStruct_ctor(struct InnerStruct *o1, struct InnerStruct *o2);
void OuterStruct_destroy(struct OuterStruct *ffi);
struct InnerStruct *InnerStruct_ctor(uint64_t i1, uint64_t i2);
void InnerStruct_destroy(struct InnerStruct *ffi);
struct OuterStruct *create_outer(struct InnerStruct *o1, struct InnerStruct *o2);
```
So here we have 2 different approaches, in `OuterStruct_ctor` and in `create_outer`. Although, from C perspective they look similar. This makes the difference in memory management. 

1. In the ctor approach, cloning does not occur. Instead, ownership of the pointers is transferred to the rust. We create a structure without conversions, and the ownership of pointers is transferred to the fields of the structure. And after that these pointers cannot be used in C. Accordingly, Rust is responsible for cleaning the transferred pointers. 
    ```
    struct InnerStruct* is1 = InnerStruct_ctor(1, 2);
    struct InnerStruct* is2 = InnerStruct_ctor(3, 4);
    struct OuterStruct* os1 = OuterStruct_ctor(is1, is2);
    // At this point, `is1` and `is2` should not be used or freed in C.
    OuterStruct_destroy(os1); // Rust frees `os1` and its `InnerStruct` instances.
    ```

2. Cloning occurs in the regular function approach. In this case, it will be accordingly that C will be responsible for clearing these pointers.
    ```
    struct InnerStruct* is3 = InnerStruct_ctor(5, 6);
    struct InnerStruct* is4 = InnerStruct_ctor(7, 8);
    struct OuterStruct* os2 = create_outer(is3, is4);
    // `is3` and `is4` are cloned by Rust, so C still owns `is3` and `is4` and must free them.
    InnerStruct_destroy(is3); // C frees `is3`.
    InnerStruct_destroy(is4); // C frees `is4`.
    OuterStruct_destroy(os2); // Rust kills `os2` and its cloned `InnerStruct` instances.
    ```

Back in the days decisions were made from the point of view of efficiency, it would be better to always give pointer's ownership to the rust. But to do this, you will have to write code in rust only in an FFI-compatible style (which is ridiculous), or modify the `ferment` to the state where not only FFI-compatible methods/structures are fermented, but also the code itself inside them.


