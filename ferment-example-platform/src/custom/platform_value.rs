#[repr(C)]
#[allow(non_camel_case_types)]
#[ferment_macro::register(platform_value::Value)]
pub struct platform_value_Value {
    raw_err: *mut platform_value::Value,
}
impl ferment_interfaces::FFIConversion<platform_value::Value> for platform_value_Value {
    unsafe fn ffi_from_const(ffi: *const Self) -> platform_value::Value {
        let ffi = &*ffi;
        match &*ffi.raw_err {
            platform_value::Value::U128(o_0) => platform_value::Value::U128(*o_0),
            platform_value::Value::I128(o_0) => platform_value::Value::I128(*o_0),
            platform_value::Value::U64(o_0) => platform_value::Value::U64(*o_0),
            platform_value::Value::I64(o_0) => platform_value::Value::I64(*o_0),
            platform_value::Value::U32(o_0) => platform_value::Value::U32(*o_0),
            platform_value::Value::I32(o_0) => platform_value::Value::I32(*o_0),
            platform_value::Value::U16(o_0) => platform_value::Value::U16(*o_0),
            platform_value::Value::I16(o_0) => platform_value::Value::I16(*o_0),
            platform_value::Value::U8(o_0) => platform_value::Value::U8(*o_0),
            platform_value::Value::I8(o_0) => platform_value::Value::I8(*o_0),
            platform_value::Value::Bytes(o_0) => platform_value::Value::Bytes(o_0.clone()),
            platform_value::Value::Bytes20(o_0) => platform_value::Value::Bytes20(*o_0),
            platform_value::Value::Bytes32(o_0) => platform_value::Value::Bytes32(*o_0),
            platform_value::Value::Bytes36(o_0) => platform_value::Value::Bytes36(*o_0),
            platform_value::Value::EnumU8(o_0) => platform_value::Value::EnumU8(o_0.clone()),
            platform_value::Value::EnumString(o_0) => platform_value::Value::EnumString(o_0.clone()),
            platform_value::Value::Identifier(o_0) => platform_value::Value::Identifier(*o_0),
            platform_value::Value::Float(o_0) => platform_value::Value::Float(*o_0),
            platform_value::Value::Text(o_0) => platform_value::Value::Text(o_0.clone()),
            platform_value::Value::Bool(o_0) => platform_value::Value::Bool(*o_0),
            platform_value::Value::Null => platform_value::Value::Null,
            platform_value::Value::Array(o_0) => platform_value::Value::Array(o_0.clone()),
            platform_value::Value::Map(o_0) => platform_value::Value::Map(o_0.clone()),
            _ => panic!("platform_value::Value non exhaustive")
        }
    }
    unsafe fn ffi_to_const(obj: platform_value::Value) -> *const Self {
        ferment_interfaces::boxed(platform_value_Value { raw_err: ferment_interfaces::boxed(obj) })
    }
}

// impl Drop for platform_value_Value {
//     fn drop(&mut self) {
//         unsafe {
//             ferment_interfaces::unbox_any(self.raw_err);
//         }
//     }
// }
