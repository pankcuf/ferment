mod fermented;
mod model;
mod gen;
mod entry;


extern crate ferment_macro;

#[ferment_macro::export]
pub struct SomeStruct {
    pub name: String,
    pub names: &'static str,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[ferment_macro::register(std::time::Duration)]
pub struct std_time_Duration2 {
    secs: u64,
    nanos: u32,
}
ferment_interfaces::impl_custom_conversion!(std::time::Duration, std_time_Duration2,
    |value: &std_time_Duration2| std::time::Duration::new(value.secs, value.nanos),
    |value: &std::time::Duration| Self { secs: value.as_secs(), nanos: value.subsec_nanos() }
);
