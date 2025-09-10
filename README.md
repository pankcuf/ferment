# ferment
Syntax-tree morphing tool for FFI (work in progress)

Generates FFI-compliant equivalents for Rust items (structs, enums, type aliases, functions) and for traits (vtable + trait objects, with per-implementor shims).

The project is a rust-workspace consisting several crates:
1. `ferment`: A traits that provide conversion methods from/to FFI-compatible types and some helper functions and structures
2. `ferment-sys`: a tool for morphing FFI-compatible syntax trees that uses the power of the `syn` crate.
3. `ferment-macro`: a procedural macro that captures target code as a syn item.
4. `ferment-example`: provides example of usage.

A procedural macro consists of 2 macros:

1. `export` - for structures / enums / functions / types
2. `register` - for custom-defined conversions
3. `opaque` - deprecated (objects are considered opaque by default)

**Usage**

Crate is not published yet, so use it for example locally

```toml
ferment = { path = "../../ferment/ferment" }
ferment-macro = { path = "../../ferment/ferment-macro" }
ferment-sys = { path = "../../ferment/ferment-sys" }
```

Using the tool implies using `cbindgen` with a configuration like this:

```rust
extern crate cbindgen;
extern crate ferment_sys;

fn main() {
    const SELF_NAME: &str = "example_nested";
    match ferment_sys::Ferment::with_crate_name(SELF_NAME)
        .with_default_mod_name()
        .with_cbindgen_config_from_file("cbindgen.toml")
        .with_external_crates(vec![
            "versioned-feature-core",
            "example-simple",
            "dashcore",
            "dpp",
            "platform-value",
            "platform-version"
        ])
        .with_languages(vec![
            #[cfg(feature = "objc")]
            ferment_sys::Lang::ObjC(
                ferment_sys::ObjC::new(
                    ferment_sys::XCodeConfig::new(
                        "DS",                 // class prefix
                        "DSExampleNested",    // framework name
                        SELF_NAME              // header (module) name
                    )
                )
            ),
        ])
        .generate() {
        Ok(_) => println!("[ferment] [ok]: {SELF_NAME}"),
        Err(err) => panic!("[ferment] [err]: {}", err)
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
This exposes vtable + trait object and per-implementor shims so you can call trait methods through FFI.

For a structure labeled with `ferment_macro::export`

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
impl ferment::FFIConversionFrom<crate::model::snapshot::LLMQSnapshot> for LLMQSnapshot {
    unsafe fn ffi_from_const(ffi: *const LLMQSnapshot) -> crate::model::snapshot::LLMQSnapshot {
        let ffi_ref = &*ffi;
        crate::model::snapshot::LLMQSnapshot {
            member_list: ferment::FFIConversionFrom::ffi_from(ffi_ref.member_list),
            skip_list: ferment::FFIConversionFrom::ffi_from(ffi_ref.skip_list),
            skip_list_mode: ferment::FFIConversionFrom::ffi_from(ffi_ref.skip_list_mode),
            option_vec: ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.option_vec),
        }
    }
}
impl ferment::FFIConversionTo<crate::model::snapshot::LLMQSnapshot> for LLMQSnapshot {
    unsafe fn ffi_to_const(obj: crate::model::snapshot::LLMQSnapshot) -> *const LLMQSnapshot {
        ferment::boxed(LLMQSnapshot {
            member_list: ferment::FFIConversionTo::ffi_to(obj.member_list),
            skip_list: ferment::FFIConversionTo::ffi_to(obj.skip_list),
            skip_list_mode: ferment::FFIConversionTo::ffi_to(obj.skip_list_mode),
            option_vec: match obj.option_vec {
                Some(vec) => ferment::FFIConversionTo::ffi_to(vec),
                None => std::ptr::null_mut(),
            },
        })
    }
}
impl ferment::FFIConversionDestroy<crate::model::snapshot::LLMQSnapshot> for LLMQSnapshot {
    unsafe fn destroy(ffi: *mut LLMQSnapshot) {
       ferment::unbox_any(ffi);
    }
}
impl Drop for LLMQSnapshot {
    fn drop(&mut self) {
        unsafe {
            let ffi_ref = self;
            ferment::unbox_any(ffi_ref.member_list);
           ferment::unbox_any(ffi_ref.skip_list);
            <crate::fermented::types::model::snapshot::LLMQSnapshotSkipMode as ferment::FFIConversionDestroy<crate::model::snapshot::LLMQSnapshotSkipMode>>::
            destroy(ffi_ref.skip_list_mode);
            if !ffi_ref.option_vec.is_null() {
                ferment::unbox_any(ffi_ref.option_vec);
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
   ferment::boxed(LLMQSnapshot {
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
   ferment::unbox_any(ffi);
}

```

For a function labeled with `export`

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
    let conversion = ferment::FFIConversionFrom::ffi_from(script);
    let obj = crate::example::address::address_with_script_pubkey(conversion);
   ferment::FFIConversionTo::ffi_to_opt(obj)
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
   ferment::unbox_any(obj.object as *mut crate::chain::common::chain_type::ChainType);
}

```
using this code cbindgen will be able to generate binding 
```
struct IHaveChainSettings_TraitObject ChainType_as_IHaveChainSettings_TraitObject(const struct ChainType *obj);
void ChainType_as_IHaveChainSettings_TraitObject_destroy(struct IHaveChainSettings_TraitObject obj);

```
Current limitations (high level):
- Mark all structures/traits/functions involved in export with the macro.
- Type aliases: supported with caveats; prefer exporting the underlying type directly when possible. Complex aliasing across crates and re-exports is not fully resolved yet.
- Path nuances: partially- and fully-qualified paths are supported in many cases, but there are edge cases under active work (see ROADMAP / TODO).

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

Then the macro implements the necessary conversions for these structures. Example for `BTreeMap<HashID, Vec<HashID>>`:
```rust
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    pub count: usize,
    pub keys: *mut *mut crate::fermented::types::nested::HashID,
    pub values: *mut *mut crate::fermented::generics::Vec_crate_nested_HashID,
}
impl ferment::FFIConversionFrom<std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>>
for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    unsafe fn ffi_from_const(
        ffi: *const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID)
        -> std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>> {
        let ffi_ref = &*ffi;
       ferment::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
    }
}
impl ferment::FFIConversionTo<std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>>
for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    unsafe fn ffi_to_const(
        obj: std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>)
        -> *const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
       ferment::boxed(Self {
            count: obj.len(),
            keys: ferment::to_complex_group(obj.keys().cloned()),
            values: ferment::to_complex_group(obj.values().cloned()),
        })
    }
}
impl ferment::FFIConversionDestroy<std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>>
for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    unsafe fn destroy(ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID) {
       ferment::unbox_any(ffi);
    }
}
impl Drop for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
    fn drop(&mut self) {
        unsafe { 
           ferment::unbox_any_vec_ptr(self.keys, self.count);
           ferment::unbox_any_vec_ptr(self.values, self.count);
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
impl ferment::FFIConversionFrom<crate::OuterStruct> for OuterStruct {
    unsafe fn ffi_from_const(ffi: *const OuterStruct) -> crate::OuterStruct {
        let ffi_ref = &*ffi;
        crate::OuterStruct {
            o1: ferment::FFIConversionFrom::ffi_from(ffi_ref.o1),
            o2: ferment::FFIConversionFrom::ffi_from(ffi_ref.o2),
        }
    }
}
impl ferment::FFIConversionTo<crate::OuterStruct> for OuterStruct {
    unsafe fn ffi_to_const(obj: crate::OuterStruct) -> *const OuterStruct {
       ferment::boxed(OuterStruct {
            o1: ferment::FFIConversionTo::ffi_to(obj.o1),
            o2: ferment::FFIConversionTo::ffi_to(obj.o2),
        })
    }
}
impl ferment::FFIConversionDestroy<crate::OuterStruct> for OuterStruct {
    unsafe fn destroy(ffi: *mut OuterStruct) {
       ferment::unbox_any(ffi);
    }
}
impl Drop for OuterStruct {
    fn drop(&mut self) {
        unsafe {
            let ffi_ref = self;
           ferment::unbox_any(ffi_ref.o1);
           ferment::unbox_any(ffi_ref.o2);
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
   ferment::boxed(OuterStruct { o1, o2 })
}
#[doc = r" # Safety"]
#[no_mangle]
pub unsafe extern "C" fn OuterStruct_destroy(ffi: *mut OuterStruct) {
   ferment::unbox_any(ffi);
}

#[doc = "FFI-representation of the [`create_outer`]"]
#[doc = r" # Safety"]
#[no_mangle]
pub unsafe extern "C" fn create_outer(
    o1: *mut crate::fermented::types::InnerStruct,
    o2: *mut crate::fermented::types::InnerStruct,
) -> *mut crate::fermented::types::OuterStruct {
    let obj = crate::create_outer(
       ferment::FFIConversionFrom::ffi_from(o1),
        ferment::FFIConversionFrom::ffi_from(o2),
    );
   ferment::FFIConversionTo::ffi_to(obj)
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




FAQ: 
- if you see no opaque pointers in cbindgen header makes sure you did include crate-owner in the list in settings 
